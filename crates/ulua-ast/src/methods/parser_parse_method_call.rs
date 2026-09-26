use core::ptr::NonNull;

use crate::{
  enums::type_lexer::Type,
  functions::optional_node::node_opt,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_index_name::AstExprIndexName, ast_node::AstNode, ast_type_or_pack::AstTypeOrPack,
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

    let cst_type_arguments: Option<NonNull<CstTypeInstantiation>> = if self.options.store_cst_data {
      node_opt(self.alloc(CstTypeInstantiation::default()))
    } else {
      None
    };

    if self.lexer.current().r#type == Type::LESS && self.lexer.lookahead().r#type == Type::LESS {
      type_arguments = self.parse_type_instantiation_expr(
        cst_type_arguments.map(|p| {
          // Safety: cst_type_arguments 为 Some 时经 node_opt 折叠必非空，其值是 store_cst_data
          // 门控下 self.alloc 产出（arena alloc 恒非空，失败 handle_alloc_error
          // 中止；bump 落位后地址稳定），此刻 parser 独占该 CST 节点，重建 `&mut` 填写无别名冲突。
          unsafe { &mut *p.as_ptr() }
        }),
        None,
      );
    }

    expr = self.parse_function_args(func, true);

    if self.options.store_cst_data {
      // Safety: `expr` 是 parse_function_args 返回值——各分支均为 alloc_expr 产出的 `AstExprCall`
      // 或 report_function_args_error 的错误节点（arena alloc 恒非空，失败 handle_alloc_error 中止）；
      // `.cast::<AstNode>()` 借 #[repr(C)] 基类前缀基址重合，上转零偏移仅改视图类型；`&mut *` 重建
      // 独占基类引用交给 lookup_cst_node_mut，单线程串行解析、节点刚分配即此一处借用，无别名冲突。
      match self.lookup_cst_node_mut::<CstExprCall>(unsafe { &mut *expr.cast::<AstNode>() }) {
        Some(cst_node) => cst_node.explicit_types = cst_type_arguments,
        None => ulua_common::LUAU_ASSERT!(false),
      }
    }

    if !type_arguments.is_empty()
      // Safety: expr 是 parse_function_args 刚 arena 分配的存活表达式节点（alloc 恒非空，
      // 失败 handle_alloc_error 中止）；as_mut 对 null 会折叠为 None（等价旧判空早退），
      // 非空时重借用为独占基类可变引用（单线程串行解析、该节点尚无其他持有者，无别名）；
      // ast_node_try_as_mut 收 `&mut AstNode`（safe fn）：先按 class_index 判型，命中后
      // #[repr(C)] 基类前缀布局保证下转有效，未命中返回 None 不写。
      && let Some(node) = unsafe { expr.cast::<AstNode>().as_mut() }
      && let Some(call) = ast_node_try_as_mut::<AstExprCall>(node)
    {
      call.type_arguments = type_arguments;
    }

    expr
  }
}
