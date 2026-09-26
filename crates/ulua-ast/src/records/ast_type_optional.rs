use crate::records::ast_type::AstType;

/// cpp `AstTypeOptional`：`?` 这个「可选」标记本身在 AST 里的独立部件节点。
///
/// 与 cpp 一致，本节点**没有**内层类型成员：parser 遇到 `T?` 时把 `T` 与 `?` 拆成两个
/// 部件依次塞进 union 的 parts 列表，`AstTypeOptional` 只携带 `?` 自身的 location，
/// 于是「可空子类型」这一 cpp 里的 `nullptr` 槽位在本端口根本不存在，也就没有 null 哨兵。
#[repr(C)]
#[derive(Debug)]
pub struct AstTypeOptional {
  pub base: AstType,
}
