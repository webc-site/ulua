use crate::enums::interesting_edge_case::InterestingEdgeCase;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstantiateGenericsOnNonFunction {
  pub(crate) interesting_edge_case: InterestingEdgeCase,
}

impl InstantiateGenericsOnNonFunction {
  pub const NONE: InterestingEdgeCase = InterestingEdgeCase::None;
  pub const METATABLE_CALL: InterestingEdgeCase = InterestingEdgeCase::MetatableCall;
  pub const INTERSECTION: InterestingEdgeCase = InterestingEdgeCase::Intersection;
}
