#[cfg(target_os = "macos")]
mod imp {
    use accessibility_sys::AXUIElementRef;
    use core_foundation::base::{CFRelease, CFRetain, CFTypeRef};

    pub struct AXElement(pub(crate) AXUIElementRef);

    impl Drop for AXElement {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe { CFRelease(self.0 as CFTypeRef) }
            }
        }
    }

    impl Clone for AXElement {
        fn clone(&self) -> Self {
            if !self.0.is_null() {
                unsafe { CFRetain(self.0 as CFTypeRef) };
            }
            AXElement(self.0)
        }
    }
}

#[cfg(not(target_os = "macos"))]
mod imp {
    pub struct AXElement(pub(crate) *const std::ffi::c_void);

    impl Drop for AXElement {
        fn drop(&mut self) {}
    }

    impl Clone for AXElement {
        fn clone(&self) -> Self {
            AXElement(self.0)
        }
    }
}

pub use imp::AXElement;
