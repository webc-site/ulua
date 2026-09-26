use ulua_ast::{
  records::{
    ast_expr_binary::AstExprBinary, ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_nil::AstExprConstantNil,
    ast_expr_constant_number::AstExprConstantNumber,
    ast_expr_constant_string::AstExprConstantString, ast_expr_error::AstExprError,
    ast_expr_function::AstExprFunction, ast_expr_global::AstExprGlobal,
    ast_expr_group::AstExprGroup, ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr, ast_expr_index_name::AstExprIndexName,
    ast_expr_interp_string::AstExprInterpString, ast_expr_local::AstExprLocal,
    ast_expr_table::AstExprTable, ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::AstExprUnary, ast_expr_varargs::AstExprVarargs, ast_stat_assign::AstStatAssign,
    ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
    ast_stat_compound_assign::AstStatCompoundAssign, ast_stat_continue::AstStatContinue,
    ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_while::AstStatWhile, ast_type_error::AstTypeError, ast_type_function::AstTypeFunction,
    ast_type_group::AstTypeGroup, ast_type_intersection::AstTypeIntersection,
    ast_type_optional::AstTypeOptional, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_pack_generic::AstTypePackGeneric,
    ast_type_pack_variadic::AstTypePackVariadic, ast_type_reference::AstTypeReference,
    ast_type_singleton_bool::AstTypeSingletonBool,
    ast_type_singleton_string::AstTypeSingletonString, ast_type_table::AstTypeTable,
    ast_type_typeof::AstTypeTypeof, ast_type_union::AstTypeUnion,
  },
  rtti::AstNodePtr,
};

use crate::{macros::json_visit_delegator, records::ast_json_encoder::AstJsonEncoder};

impl AstJsonEncoder {
  pub fn visit_ast_type_group(&mut self, node: &AstTypeGroup) -> bool {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstTypeGroup", |e| {
      e.write("inner", &node.type_);
    });
    false
  }
}

json_visit_delegator!(
  pub,
  visit_ast_stat_break,
  write_ast_stat_break,
  AstStatBreak
);

json_visit_delegator!(
  pub,
  visit_ast_stat_continue,
  write_ast_stat_continue,
  AstStatContinue
);

json_visit_delegator!(pub(crate), visit_ast_stat_return, write_ast_stat_return, AstStatReturn);

json_visit_delegator!(pub(crate), visit_ast_stat_expr, write_ast_stat_expr, AstStatExpr);

json_visit_delegator!(pub(crate), visit_ast_stat_local, write_ast_stat_local, AstStatLocal);

json_visit_delegator!(pub(crate), visit_ast_stat_for, write_ast_stat_for, AstStatFor);

json_visit_delegator!(pub(crate), visit_ast_stat_for_in, write_ast_stat_for_in, AstStatForIn);

json_visit_delegator!(pub(crate), visit_ast_stat_assign, write_ast_stat_assign, AstStatAssign);

json_visit_delegator!(
  pub(crate),
  visit_ast_stat_compound_assign,
  write_ast_stat_compound_assign,
  AstStatCompoundAssign
);

json_visit_delegator!(
  pub(crate), visit_ast_stat_function, write_ast_stat_function, AstStatFunction
);

json_visit_delegator!(
  pub(crate),
  visit_ast_stat_local_function,
  write_ast_stat_local_function,
  AstStatLocalFunction
);

json_visit_delegator!(
  pub(crate), visit_ast_stat_type_alias, write_ast_stat_type_alias, AstStatTypeAlias
);

json_visit_delegator!(
  pub(crate),
  visit_ast_stat_declare_function,
  write_ast_stat_declare_function,
  AstStatDeclareFunction
);

json_visit_delegator!(
  pub(crate),
  visit_ast_stat_declare_global,
  write_ast_stat_declare_global,
  AstStatDeclareGlobal
);

json_visit_delegator!(
  pub(crate),
  visit_ast_stat_declare_extern_type,
  write_ast_stat_declare_extern_type,
  AstStatDeclareExternType
);

json_visit_delegator!(pub(crate), visit_ast_stat_error, write_ast_stat_error, AstStatError);

json_visit_delegator!(
  pub(crate), visit_ast_type_reference, write_ast_type_reference, AstTypeReference
);

json_visit_delegator!(pub(crate), visit_ast_type_table, write_ast_type_table, AstTypeTable);

json_visit_delegator!(
  pub(crate), visit_ast_type_function, write_ast_type_function, AstTypeFunction
);

json_visit_delegator!(pub(crate), visit_ast_type_typeof, write_ast_type_typeof, AstTypeTypeof);

json_visit_delegator!(
  pub,
  visit_ast_type_optional,
  write_ast_type_optional,
  AstTypeOptional
);

json_visit_delegator!(pub(crate), visit_ast_type_union, write_ast_type_union, AstTypeUnion);

json_visit_delegator!(
  pub(crate),
  visit_ast_type_intersection,
  write_ast_type_intersection,
  AstTypeIntersection
);

json_visit_delegator!(pub(crate), visit_ast_type_error, write_ast_type_error, AstTypeError);

impl AstJsonEncoder {
  /// # Safety
  /// `node` 允许为 null（对应 C++ 对空 type pack 槽位的编码，
  /// `write_ast_node` 契约接受 null）；非 null 时须指向存活的
  /// `AstTypePack` 或其派生具体 pack 节点，且所在 arena 在本轮编码内独占
  /// 存活。
  pub fn visit_ast_type_pack(&mut self, node: *mut AstTypePack) -> bool {
    // Safety: `node as *mut AstNode` 落在 `AstTypePack.base: AstNode` 的
    // 偏移 0，不改变指针值与对齐；null 与存活节点两种形态都在
    // `write_ast_node`（转发 `ast_node_visit`）的契约覆盖内。
    unsafe { self.write_ast_node(node.as_ast_node()) };
    false
  }
}

json_visit_delegator!(
  pub(crate),
  visit_ast_type_pack_explicit,
  write_ast_type_pack_explicit,
  AstTypePackExplicit
);

impl AstJsonEncoder {
  pub fn visit_ast_type_singleton_bool(&mut self, node: &AstTypeSingletonBool) -> bool {
    self.write_node_ast_node_string_view_f(&node.base.base.location, "AstTypeSingletonBool", |e| {
      e.write("value", &node.value);
    });
    false
  }
}

json_visit_delegator!(
  pub(crate),
  visit_ast_type_pack_variadic,
  write_ast_type_pack_variadic,
  AstTypePackVariadic
);

json_visit_delegator!(
  pub(crate),
  visit_ast_type_pack_generic,
  write_ast_type_pack_generic,
  AstTypePackGeneric
);

impl AstJsonEncoder {
  pub fn visit_ast_type_singleton_string(&mut self, node: &AstTypeSingletonString) -> bool {
    self.write_node_ast_node_string_view_f(
      &node.base.base.location,
      "AstTypeSingletonString",
      |e| {
        e.write("value", &node.value);
      },
    );
    false
  }
}

json_visit_delegator!(pub(crate), visit_ast_expr_group, write_ast_expr_group, AstExprGroup);

json_visit_delegator!(
  pub,
  visit_ast_expr_constant_nil,
  write_ast_expr_constant_nil,
  AstExprConstantNil
);

json_visit_delegator!(
  pub(crate),
  visit_ast_expr_constant_bool,
  write_ast_expr_constant_bool,
  AstExprConstantBool
);

json_visit_delegator!(
  pub(crate),
  visit_ast_expr_constant_number,
  write_ast_expr_constant_number,
  AstExprConstantNumber
);

json_visit_delegator!(
  pub(crate),
  visit_ast_expr_constant_string,
  write_ast_expr_constant_string,
  AstExprConstantString
);

json_visit_delegator!(pub(crate), visit_ast_expr_if_else, write_ast_expr_if_else, AstExprIfElse);

json_visit_delegator!(
  pub(crate),
  visit_ast_expr_interp_string,
  write_ast_expr_interp_string,
  AstExprInterpString
);

json_visit_delegator!(pub(crate), visit_ast_expr_local, write_ast_expr_local, AstExprLocal);

json_visit_delegator!(pub(crate), visit_ast_expr_global, write_ast_expr_global, AstExprGlobal);

json_visit_delegator!(
  pub,
  visit_ast_expr_varargs,
  write_ast_expr_varargs,
  AstExprVarargs
);

json_visit_delegator!(pub(crate), visit_ast_expr_call, write_ast_expr_call, AstExprCall);

json_visit_delegator!(
  pub(crate), visit_ast_expr_index_name, write_ast_expr_index_name, AstExprIndexName
);

json_visit_delegator!(
  pub(crate), visit_ast_expr_index_expr, write_ast_expr_index_expr, AstExprIndexExpr
);

json_visit_delegator!(
  pub(crate), visit_ast_expr_function, write_ast_expr_function, AstExprFunction
);

json_visit_delegator!(pub(crate), visit_ast_expr_table, write_ast_expr_table, AstExprTable);

json_visit_delegator!(pub(crate), visit_ast_expr_unary, write_ast_expr_unary, AstExprUnary);

json_visit_delegator!(pub(crate), visit_ast_expr_binary, write_ast_expr_binary, AstExprBinary);

json_visit_delegator!(
  pub(crate),
  visit_ast_expr_type_assertion,
  write_ast_expr_type_assertion,
  AstExprTypeAssertion
);

json_visit_delegator!(pub(crate), visit_ast_expr_error, write_ast_expr_error, AstExprError);

json_visit_delegator!(pub(crate), visit_ast_stat_block, write_ast_stat_block, AstStatBlock);

json_visit_delegator!(pub(crate), visit_ast_stat_if, write_ast_stat_if, AstStatIf);

json_visit_delegator!(pub(crate), visit_ast_stat_while, write_ast_stat_while, AstStatWhile);

json_visit_delegator!(pub(crate), visit_ast_stat_repeat, write_ast_stat_repeat, AstStatRepeat);
