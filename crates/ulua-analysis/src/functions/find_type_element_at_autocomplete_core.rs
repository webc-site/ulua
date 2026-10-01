use ulua_ast::{
  records::{
    ast_type::AstType, ast_type_error::AstTypeError, ast_type_function::AstTypeFunction,
    ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_pack_variadic::AstTypePackVariadic,
    ast_type_reference::AstTypeReference, position::Position,
  },
  rtti::{AstNodePtr, ast_node_is, ast_node_try_as},
};

use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type, follow_type_pack, get_type, get_type_pack,
  },
  records::{
    arena_handle::{alias_opt, alias_ref},
    function_type::FunctionType,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub(crate) fn find_type_element_at_ast_type_list_type_pack_id_position(
  ast_type_list: &AstTypeList,
  tp: TypePackId,
  position: Position,
) -> Option<TypeId> {
  let types = ast_type_list.types.as_slice();

  for (i, &type_) in types.iter().enumerate() {
    let Some(type_ref) = alias_opt(type_) else {
      continue;
    };
    if type_ref.base.location.contains_closed(position) {
      let (head, _) = flatten_type_pack_id(tp);

      if i < head.len() {
        return find_type_element_at_ast_type_type_id_position(type_ref, head[i], position);
      }
    }
  }

  if !ast_type_list.tail_type.is_null() {
    let arg_tp = alias_ref(ast_type_list.tail_type);

    // safe 引用门面：`ast_node_try_as` 甄别 class_index（repr(C) 基址重合），命中进 Option、未命中 None。
    if let Some(variadic) = ast_node_try_as::<AstTypePackVariadic>(&arg_tp.base) {
      let location = variadic.base.base.location;
      if location.contains_closed(position) {
        let (_, tail) = flatten_type_pack_id(tp);

        if let Some(tail_id) = tail
          && let Some(vtp) =
            get_type_pack::get::<VariadicTypePack>(follow_type_pack::follow(tail_id))
        {
          // variadic_type 槽已句柄化（`...T` 文法必建 T），get() 直接给出存活引用。
          let variadic_type = variadic.variadic_type.get();
          return find_type_element_at_ast_type_type_id_position(variadic_type, vtp.ty, position);
        }
      }
    }
  }

  None
}

/// # Safety
/// 直译 cpp `findTypeElementAt`（autocomplete）：`ast_type_pack` 可为 null；非空时必须
/// 指向分析会话 arena 内存活、RTTI 有效的 `AstTypePack`。`tp` 为与之对应的存活 `TypePackId`，
/// `position` 为查询光标位置；本 pass 单线程、AST arena 只读。
pub(crate) unsafe fn find_type_element_at_ast_type_pack_type_pack_id_position(
  ast_type_pack: *mut AstTypePack,
  tp: TypePackId,
  position: Position,
) -> Option<TypeId> {
  if !ast_type_pack.is_null() {
    let node = ast_type_pack.as_ast_node();
    if ast_node_is::<AstTypePackExplicit>(alias_ref(node)) {
      let explicit = ast_type_pack.cast::<AstTypePackExplicit>();
      let type_list = alias_ref(explicit).type_list;
      return find_type_element_at_ast_type_list_type_pack_id_position(&type_list, tp, position);
    } else if ast_node_is::<AstTypePackVariadic>(alias_ref(node)) {
      let variadic = ast_type_pack.cast::<AstTypePackVariadic>();
      let loc = alias_ref(ast_type_pack).base.location;
      if loc.contains_closed(position) {
        let (_, tail) = flatten_type_pack_id(tp);

        if let Some(tail_id) = tail {
          let follow_tp = follow_type_pack::follow(tail_id);
          if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(follow_tp) {
            // variadic_type 槽已句柄化（`...T` 文法必建 T），get() 直接给出存活引用。
            let variadic_type = alias_ref(variadic).variadic_type.get();
            return find_type_element_at_ast_type_type_id_position(variadic_type, vtp.ty, position);
          }
        }
      }
    }
  }
  None
}

pub(crate) fn find_type_element_at_ast_type_type_id_position(
  ast_type: &AstType,
  ty: TypeId,
  position: Position,
) -> Option<TypeId> {
  let ty = follow_type::follow(ty);

  if ast_node_is::<AstTypeReference>(&ast_type.base) || ast_node_is::<AstTypeError>(&ast_type.base)
  {
    return Some(ty);
  }

  if let Some(type_function) = ast_node_try_as::<AstTypeFunction>(&ast_type.base) {
    let ftv = get_type::get::<FunctionType>(ty)?;

    let arg_types = &type_function.arg_types;
    let arg_types_tp = ftv.arg_types;

    if let Some(element) =
      find_type_element_at_ast_type_list_type_pack_id_position(arg_types, arg_types_tp, position)
    {
      return Some(element);
    }

    // return_types 槽已句柄化（parseReturnType/补建空 pack 恒非空），as_ptr 交出
    // 既有指针形参 API 的原址。
    let return_types = type_function.return_types.as_ptr();
    let ret_types_tp = ftv.ret_types;

    if let Some(element) = unsafe {
      find_type_element_at_ast_type_pack_type_pack_id_position(return_types, ret_types_tp, position)
    } {
      return Some(element);
    }
  }

  None
}
