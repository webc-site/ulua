use ulua_ast::{
  records::{
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_table::{Item, ItemKind},
    node_handle::OptNode,
  },
  rtti::ast_node_is,
};
pub fn is_record(item: &Item) -> bool {
  if item.kind == ItemKind::Record {
    true
  } else if item.kind == ItemKind::General {
    // `key` 仍是 records 引用化波次前的裸指针字段：经句柄门面
    // `OptNode::from_ptr` 把可空性折叠为 `Option`（null → false，与原
    // is_null 守卫 + is_ptr 形态逐字等价），判型走生命周期正确的安全门面
    // [`ast_node_is`]——只读 repr(C) 基类偏移 0 的 class_index，不外传借用。
    let key = OptNode::from_ptr(item.key);
    key.get().is_some_and(ast_node_is::<AstExprConstantString>)
  } else {
    false
  }
}
