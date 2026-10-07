use core::ptr::NonNull;

use crate::{
  enums::type_lexer::Type,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_index_name::AstExprIndexName, ast_type_or_pack::AstTypeOrPack,
    cst_expr_call::CstExprCall, cst_type_instantiation::CstTypeInstantiation, location::Location,
    name::Name, node_handle::Node, parser::Parser, position::Position,
  },
  rtti::ast_node_try_as_mut,
};

impl Parser {
  pub fn parse_method_call(&mut self, start: Position, mut expr: *mut AstExpr) -> *mut AstExpr {
    let op_position = self.lexer.current().location.begin;
    self.next_lexeme();

    let index: Name = self.parse_index_name("method name", &op_position);

    // expr 出自调用方 parse_primary_expr 线的 arena 分配（恒非空）。
    let func = self.alloc_expr(AstExprIndexName::new(
      Location::new(start, index.location.end),
      Node::from_raw(expr),
      index.name,
      index.location,
      op_position,
      b':',
    ));

    // C++ `AstArray<AstTypeOrPack*>{nullptr, 0}`：空类型实参
    let mut type_arguments: AstArray<AstTypeOrPack> = AstArray::EMPTY;

    // CST 出参槽走 alloc_mut 单一构造门面（会话级独占写契约见 records::parser），
    // `None` 即 store_cst_data 关闭、不记录。
    let mut cst_type_arguments: Option<&mut CstTypeInstantiation> = if self.options.store_cst_data {
      Some(self.alloc_mut(CstTypeInstantiation::default()))
    } else {
      None
    };

    if self.lexer.current().r#type == Type::LESS && self.lexer.lookahead().r#type == Type::LESS {
      type_arguments = self.parse_type_instantiation_expr(cst_type_arguments.as_deref_mut(), None);
    }

    expr = self.parse_function_args(func, true);

    // expr 的句柄视图：parse_function_args 返回值——各分支均为 alloc_expr 产出的
    // `AstExprCall` 或 report_function_args_error 的错误节点（arena alloc 恒非空，
    // 失败 handle_alloc_error 中止）；CST 写穿与判型下转全经该句柄的安全借用。
    let mut call_expr = Node::from_raw(expr);

    if self.options.store_cst_data {
      match self.lookup_cst_node_mut::<CstExprCall>(&mut call_expr.get_mut().base) {
        // 槽位值落 `Option<NonNull>`：从独占引用取地址（NonNull::from，safe），
        // printer 侧 `node_ref` 门面按同一 arena 存活契约读回共享引用。
        Some(cst_node) => {
          cst_node.explicit_types = cst_type_arguments.as_mut().map(|c| NonNull::from(&mut **c));
        }
        None => ulua_common::LUAU_ASSERT!(false),
      }
    }

    if !type_arguments.is_empty()
      // ast_node_try_as_mut 收 `&mut AstNode`（safe fn）：先按 class_index 判型，
      // 命中后 #[repr(C)] 基类前缀布局保证下转有效，未命中返回 None 不写。
      && let Some(call) = ast_node_try_as_mut::<AstExprCall>(call_expr.get_mut())
    {
      call.type_arguments = type_arguments;
    }

    expr
  }
}
