use crate::{
  records::{
    ast_attr::AstAttr,
    ast_expr::AstExpr,
    ast_expr_function::AstExprFunction,
    ast_generic_type::AstGenericType,
    ast_generic_type_pack::AstGenericTypePack,
    ast_local::AstLocal,
    ast_name::AstName,
    ast_stat_block::AstStatBlock,
    ast_type_pack::AstTypePack,
    location::Location,
    node_handle::{Node, Nodes, OptNode},
  },
  rtti::AstNodeClass,
};

/// `AstExprFunction` 构造参数集合。
/// C++ 构造函数直译有 14 个参数，超出 Rust 惯用上限，打包成 struct 传递；
/// 字段与 C++ `AstExprFunction` 构造函数形参一一对应。
///
/// `self_`/`return_annotation`/`vararg_annotation` 在 cpp 都是可空槽
/// （`Ast.h:543-548`：`AstLocal* self`、`AstTypePack* returnAnnotation = nullptr`、
/// `AstTypePack* varargAnnotation`；`parseFunctionBody` 以 `Binding(Name(nameSelf, ..), nullptr)`
/// 与匿名函数不建 local 的形态填 null），此处统一用 `Option` 表达，
/// 落槽由 [`opt_node`] 单点完成。
pub struct AstExprFunctionArgs {
  pub location: Location,
  pub attributes: Nodes<AstAttr>,
  pub generics: Nodes<AstGenericType>,
  pub generic_packs: Nodes<AstGenericTypePack>,
  pub self_: OptNode<AstLocal>,
  pub args: Nodes<AstLocal>,
  pub vararg: bool,
  pub vararg_location: Location,
  pub body: Node<AstStatBlock>,
  pub function_depth: usize,
  pub debugname: AstName,
  pub return_annotation: OptNode<AstTypePack>,
  pub vararg_annotation: OptNode<AstTypePack>,
  pub arg_location: Option<Location>,
}

impl AstExprFunction {
  pub fn new(a: AstExprFunctionArgs) -> Self {
    Self {
      base: AstExpr::new(<Self as AstNodeClass>::CLASS_INDEX, a.location),
      attributes: a.attributes,
      generics: a.generics,
      generic_packs: a.generic_packs,
      self_: a.self_,
      args: a.args,
      return_annotation: a.return_annotation,
      vararg: a.vararg,
      vararg_location: a.vararg_location,
      vararg_annotation: a.vararg_annotation,
      body: a.body,
      function_depth: a.function_depth,
      debugname: a.debugname,
      arg_location: a.arg_location,
    }
  }
}
