#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct AstArrayPredicate;

unsafe impl Send for AstArrayPredicate {}
unsafe impl Sync for AstArrayPredicate {}
