//! cpp `AstJsonEncoder` 逐节点帧写入器的骨架单点。
//!
//! C++ 侧 `write(AstXxx*)` 全靠同一枚模板 `writeNode(node, "AstXxx", [&]{ … })`
//! 展开；Rust 直译于是把「开 impl → 取 `base.base.location` → 帧头 → 逐字段
//! `e.write(键, &node.字段)` → 收 impl」这五行样板在 40 多个写入器里各抄一遍，
//! 每个函数只有节点类型、JSON 标签与字段名单不同。[`write_json_node!`] 只替换
//! 这层样板：字段顺序、JSON 键与取值表达式与手写逐字一致。

/// 生成一个 AST 节点的 JSON 写入器 `pub fn $fn(&mut self, node: &$ty)`。
///
/// 用法：
/// ```ignore
/// write_json_node!(write_ast_stat_assign, AstStatAssign, "AstStatAssign", [
///   "vars": vars,
///   "values": values,
/// ]);
/// ```
/// 空字段名单（C++ 只写帧头与 location 的节点）走 `|_| {}` 一支，与手写的
/// `|_| {}` 闭包一致。
macro_rules! write_json_node {
  // 无字段节点：帧体为空。
  ($fn:ident, $ty:ident, $tag:literal, []) => {
    impl $crate::records::ast_json_encoder::AstJsonEncoder {
      pub fn $fn(&mut self, node: &$ty) {
        self.write_node_ast_node_string_view_f(&node.base.base.location, $tag, |_| {});
      }
    }
  };
  // 有字段节点：按名单顺序逐字段写出。
  ($fn:ident, $ty:ident, $tag:literal, [$($key:literal : $field:ident),+ $(,)?]) => {
    impl $crate::records::ast_json_encoder::AstJsonEncoder {
      pub fn $fn(&mut self, node: &$ty) {
        self.write_node_ast_node_string_view_f(&node.base.base.location, $tag, |e| {
          $(e.write($key, &node.$field);)+
        });
      }
    }
  };
}

pub(crate) use write_json_node;
