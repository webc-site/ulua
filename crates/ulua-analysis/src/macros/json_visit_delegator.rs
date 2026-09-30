//! cpp `AstJsonEncoder` 节点 visit 委托的骨架单点。
//!
//! C++ 侧 `AstJsonEncoder` 继承 `AstVisitor`，对每种节点的 `visit` 覆写都只是
//! 转发同名 `write(AstXxx*)` 后 `return false`（继续遍历的开关恒关）。Rust
//! 直译于是把「开 impl → `pub((crate))? fn visit_X(&mut self, node: &X) ->
//! bool { self.write_X(node); false } → 收 impl」这七行样板在 50 多个委托里
//! 各抄一遍，只有可见性、方法名与节点类型三要素不同。[`json_visit_delegator!`]
//! 只替换这层样板：签名、可见性与「转发后返回 false」的行为逐字保持。

/// 生成一个恒转发到 `$write` 并返回 `false` 的 visit 委托
/// `$vis fn $visit(&mut self, node: &$ty) -> bool`。
///
/// 用法：
/// ```ignore
/// json_visit_delegator!(pub(crate), visit_ast_stat_return, write_ast_stat_return, AstStatReturn);
/// ```
/// `$write` 目标方法（通常由 [`crate::macros::write_json_node`] 生成）仍由
/// 调用方所在模块可见；节点类型 `$ty` 需在调用处导入。
macro_rules! json_visit_delegator {
  ($vis:vis, $visit:ident, $write:ident, $ty:ident) => {
    impl $crate::records::ast_json_encoder::AstJsonEncoder {
      $vis fn $visit(&mut self, node: &$ty) -> bool {
        self.$write(node);
        false
      }
    }
  };
}

pub(crate) use json_visit_delegator;
