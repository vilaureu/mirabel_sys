//! Wrapper around [`game_init`].

use std::{
    borrow::Borrow,
    ffi::c_void,
    mem::transmute,
    ops::Deref,
    ptr::{null, null_mut},
};

use crate::{
    sys::{
        game_init, game_init_create_standard, sl_game_init_info_serializer,
        GAME_INIT_SOURCE_TYPE_E_GAME_INIT_SOURCE_TYPE_STANDARD as SOURCE_TYPE_STANDARD,
        GSIT_E_GSIT_COPY, GSIT_E_GSIT_DESTROY, LS_ERR,
    },
    ValidCStr,
};

// TODO: Provide safe wrapper for game_init internals.
/// Wraps a valid [`game_init`].
#[repr(transparent)]
pub struct GameInit(game_init);

impl GameInit {
    /// Create a new [`GameInit`] from a [`game_init`].
    ///
    /// # Safety
    /// The supplied `init_info` must be valid.
    #[inline]
    pub unsafe fn new(init_info: game_init) -> Self {
        Self(init_info)
    }

    /// Create a new [`GameInit`] from a [`game_init`] by references.
    ///
    /// This is valid because [`GameInit`] is `repr(transparent)`.
    ///
    /// # Safety
    /// The supplied `init_info` must be valid.
    #[inline]
    pub unsafe fn from_ref<'l>(init_info: &'l game_init) -> &'l Self {
        transmute::<&'l game_init, &'l Self>(init_info)
    }
}

impl Deref for GameInit {
    type Target = game_init;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ToOwned for GameInit {
    type Owned = OwnedGameInit;

    fn to_owned(&self) -> Self::Owned {
        let original: *const game_init = &**self;
        // Prevent having uninitialized memory by providing initialized memory
        // to start with.
        let mut copy = game_init::default();
        let copy_ptr: *mut game_init = &mut copy;
        unsafe {
            // original is not actually mutated.
            assert_ne!(
                LS_ERR,
                sl_game_init_info_serializer(
                    GSIT_E_GSIT_COPY,
                    original.cast_mut().cast::<c_void>(),
                    copy_ptr.cast::<c_void>(),
                    null_mut(),
                    null_mut(),
                )
            );
            OwnedGameInit::new(GameInit::new(copy))
        }
    }
}

/// An owned variant of [`GameInit`].
///
/// It will destroy the [`game_init`] on drop.
pub struct OwnedGameInit(GameInit);

impl OwnedGameInit {
    /// Takes ownership of `init_info`.
    ///
    /// # Safety
    /// The data pointed to by `init_info` must not be concurrently accessed nor
    /// used after this [`OwnedGameInit`] is dropped.
    #[inline]
    unsafe fn new(init_info: GameInit) -> Self {
        OwnedGameInit(init_info)
    }

    /// Clones the underlying [`game_init`] while forcing the supplied state.
    pub fn clone_with_state(&self, state: Option<ValidCStr>) -> Self {
        let (opts, legacy) = if self.source_type == SOURCE_TYPE_STANDARD {
            unsafe { (self.source.standard.opts, self.source.standard.legacy) }
        } else {
            (null(), null())
        };
        let state = match state {
            Some(s) => s.into(),
            None => null(),
        };

        // Prevent having uninitialized memory by providing initialized memory
        // to start with.
        let mut copy = game_init::default();
        let copy_ptr: *mut game_init = &mut copy;
        unsafe {
            game_init_create_standard(copy_ptr, opts, legacy, state);
            OwnedGameInit::new(GameInit::new(copy))
        }
    }
}

impl Borrow<GameInit> for OwnedGameInit {
    #[inline]
    fn borrow(&self) -> &GameInit {
        &self.0
    }
}

impl Deref for OwnedGameInit {
    type Target = GameInit;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.borrow()
    }
}

impl Drop for OwnedGameInit {
    fn drop(&mut self) {
        let ptr: *mut game_init = &mut (self.0).0;
        unsafe {
            // Unclear how to handle errors of destroy here. Ignore them.
            sl_game_init_info_serializer(
                GSIT_E_GSIT_DESTROY,
                ptr.cast::<c_void>(),
                null_mut(),
                null_mut(),
                null_mut(),
            );
        }
    }
}
