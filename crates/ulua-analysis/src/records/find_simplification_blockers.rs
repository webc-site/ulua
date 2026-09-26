use crate::records::iterative_type_visitor::IterativeTypeVisitor;

/// C++ `struct FindSimplificationBlockers : IterativeTypeVisitor`
/// (ConstraintGenerator.cpp:760)。visit 分发实现在 methods/ 下的
/// find_simplification_blockers_* 文件（经 IterativeTypeVisitorTrait 派发）。
#[derive(Debug, Clone)]
pub struct FindSimplificationBlockers {
  pub base: IterativeTypeVisitor,
  pub found: bool,
}
