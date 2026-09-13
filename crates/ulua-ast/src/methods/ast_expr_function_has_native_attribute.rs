use crate::records::{ast_attr::AstAttrType::Native, ast_expr_function::AstExprFunction};

impl AstExprFunction {
  pub fn has_native_attribute(&self) -> bool {
    // 迭代 AstArray 满足其"data 与 size 一致"不变式；逐个检查非空指针指向的节点。
    self.attributes.iter().any(|&attribute_ptr| {
      // SAFETY: 非空指针指向 arena 中存活的 AstAttr 节点。
      unsafe {
        attribute_ptr
          .as_ref()
          .is_some_and(|attribute| attribute.r#type == Native)
      }
    })
  }
}

pub fn ast_expr_function_has_native_attribute(this: &AstExprFunction) -> bool {
  this.has_native_attribute()
}
