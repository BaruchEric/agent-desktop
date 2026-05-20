use crate::adapter::{NativeHandle, PlatformAdapter};

pub(crate) struct ResolvedElement<'a> {
    adapter: &'a dyn PlatformAdapter,
    handle: NativeHandle,
}

impl<'a> ResolvedElement<'a> {
    pub(crate) fn new(adapter: &'a dyn PlatformAdapter, handle: NativeHandle) -> Self {
        Self { adapter, handle }
    }

    pub(crate) fn handle(&self) -> &NativeHandle {
        &self.handle
    }
}

impl Drop for ResolvedElement<'_> {
    fn drop(&mut self) {
        let _ = self.adapter.release_handle(&self.handle);
    }
}
