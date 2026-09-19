use core::{ffi::c_char, ptr::null_mut};

use crate::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_index_name::AstExprIndexName, ast_node::AstNode, ast_type_or_pack::AstTypeOrPack,
    cst_expr_call::CstExprCall, cst_type_instantiation::CstTypeInstantiation, lexeme::Type,
    location::Location, name::Name, parser::Parser, position::Position,
  },
  rtti::ast_node_try_as_mut,
};

impl Parser {
  pub fn parse_method_call(&mut self, start: Position, mut expr: *mut AstExpr) -> *mut AstExpr {
    let op_position = self.lexer.current().location.begin;
    self.next_lexeme();

    let index: Name = self.parse_index_name("method name", &op_position);

    let func = unsafe {
      (*self.allocator).alloc(AstExprIndexName::new(
        Location::new(start, index.location.end),
        expr,
        index.name,
        index.location,
        op_position,
        ':' as c_char,
      ))
    };

    // C++ `AstArray<AstTypeOrPack*>{nullptr, 0}`：空类型实参
    let mut type_arguments: AstArray<AstTypeOrPack> = AstArray::EMPTY;

    let cst_type_arguments: *mut CstTypeInstantiation = if self.options.store_cst_data {
      unsafe { (*self.allocator).alloc(CstTypeInstantiation::default()) }
    } else {
      null_mut()
    };

    if self.lexer.current().r#type == Type('<' as i32)
      && self.lexer.lookahead().r#type == Type('<' as i32)
    {
      type_arguments = self.parse_type_instantiation_expr(cst_type_arguments, None);
    }

    expr = self.parse_function_args(func as *mut AstExpr, true);

    if self.options.store_cst_data {
      // SAFETY: lookup_cst_node_mut 收口判型与下转，映射值指向 arena 存活节点
      match self.lookup_cst_node_mut::<CstExprCall>(unsafe { &mut *expr.cast::<AstNode>() }) {
        Some(cst_node) => cst_node.explicit_types = cst_type_arguments,
        None => ulua_common::LUAU_ASSERT!(false),
      }
    }

    if !expr.is_null() && !type_arguments.is_empty() {
      // SAFETY: expr 判空后指向 arena 存活节点，ast_node_try_as_mut 合并判型下转
      if let Some(call) =
        unsafe { ast_node_try_as_mut::<AstExprCall>(&mut *expr.cast::<AstNode>()) }
      {
        call.type_arguments = type_arguments;
      }
    }

    expr
  }
}
