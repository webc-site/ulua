use ulua_ast::{records::ast_node::AstNode, visit::dispatch_node};

use crate::records::type_map_visitor::{TypeMapVisitor, TypeMapVisitorArgs};

/// 对应 cpp `buildTypeMap`（cpp/Compiler/src/Types.cpp:948）：以 `TypeMapVisitor`
/// 遍历 `root`（编译入口的 `AstStatBlock`）子树，读取各节点 `typename` 字段，把函数/
/// 局部/表达式三类类型别名映射写入 `args` 各 map（均以裸 AST 指针为地址键的句柄模型）。
pub(crate) fn build_type_map(root: &mut AstNode, args: TypeMapVisitorArgs<'_, '_>) {
  let mut visitor = TypeMapVisitor::new(args);

  // `dispatch_node` 以调用方独占的 `&mut root` 沿 parser 接线的子指针只读分派遍历，
  // 不长期构造指向 AST 的其他 `&mut`；`visitor` 独占持有 `args` 的可变借用且止于本
  // 函数，无别名冲突。
  dispatch_node(root, &mut visitor);
}
