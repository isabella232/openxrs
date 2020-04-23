use std::mem::MaybeUninit;
use std::{ptr, sync::Arc};

use crate::*;

pub use sys::HandMSFT as Hand;

pub struct HandTracker {
    session: Arc<session::SessionInner>,
    handle: sys::HandTrackerMSFT,
}

impl HandTracker {
    #[inline]
    pub fn as_raw(&self) -> sys::HandTrackerMSFT {
        self.handle
    }

    #[inline]
    pub unsafe fn from_raw<G: Graphics>(
        session: &Session<G>,
        handle: sys::HandTrackerMSFT,
    ) -> Self {
        Self {
            handle,
            session: session.inner.clone(),
        }
    }

    pub fn create<G: Graphics>(session: &Session<G>, hand: Hand) -> Result<Self> {
        let fp = session
            .inner
            .instance
            .exts()
            .msft_hand_tracking_preview
            .as_ref();
        let fp = if let Some(fp) = fp {
            fp
        } else {
            return Err(sys::Result::ERROR_EXTENSION_NOT_PRESENT);
        };

        let mut out = MaybeUninit::uninit();
        let info = sys::HandTrackerCreateInfoMSFT {
            ty: sys::HandTrackerCreateInfoMSFT::TYPE,
            next: ptr::null(),
            hand,
        };
        let handle = unsafe {
            cvt((fp.create_hand_tracker)(session.as_raw(), &info, out.as_mut_ptr()))?;
            out.assume_init()
        };
        Ok(HandTracker {
            session: session.inner.clone(),
            handle,
        })
    }

    #[inline]
    fn fp(&self) -> &raw::HandTrackingPreviewMSFT {
        self.session
            .instance
            .exts()
            .msft_hand_tracking_preview
            .as_ref()
            .expect(
                "Somehow created HandTracker without XR_MSFT_hand_tracking_preview being enabled",
            )
    }
}

impl Drop for HandTracker {
    fn drop(&mut self) {
        unsafe {
            (self.fp().destroy_hand_tracker)(self.handle);
        }
    }
}
