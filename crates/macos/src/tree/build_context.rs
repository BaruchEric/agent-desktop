use super::AXElement;

pub struct TreeBuildContext {
    pub(crate) focused: Option<AXElement>,
}

impl TreeBuildContext {
    pub fn for_pid(pid: i32) -> Self {
        let app = super::element_for_pid(pid);
        Self {
            focused: super::copy_element_attr(&app, "AXFocusedUIElement"),
        }
    }

    pub fn empty() -> Self {
        Self { focused: None }
    }
}
