use super::chain_step::ChainStep;

pub(crate) struct ChainDef {
    pub(crate) pre_scroll: bool,
    pub(crate) steps: &'static [ChainStep],
    pub(crate) suggestion: &'static str,
}
