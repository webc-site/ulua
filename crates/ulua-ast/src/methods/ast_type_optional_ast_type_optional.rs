use crate::{
  records::{ast_type::AstType, ast_type_optional::AstTypeOptional, location::Location},
  rtti::AstNodeClass,
};

impl AstTypeOptional {
  /// 只带 `?` 自身位置的部件节点（cpp `alloc<AstTypeOptional>(loc)`）：无子类型字段。
  pub fn new(location: Location) -> Self {
    Self {
      base: AstType::new(<Self as AstNodeClass>::CLASS_INDEX, location),
    }
  }
}
