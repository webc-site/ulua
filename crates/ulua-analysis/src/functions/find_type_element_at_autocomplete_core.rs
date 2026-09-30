use ulua_ast::{
  records::{
    ast_type::AstType, ast_type_error::AstTypeError, ast_type_function::AstTypeFunction,
    ast_type_list::AstTypeList, ast_type_pack::AstTypePack,
    ast_type_pack_explicit::AstTypePackExplicit, ast_type_pack_variadic::AstTypePackVariadic,
    ast_type_reference::AstTypeReference, position::Position,
  },
  rtti::{AstNodePtr, ast_node_is, ast_node_is_ptr, ast_node_try_as, ast_node_try_as_ptr},
};

use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type, follow_type_pack, get_type, get_type_pack,
  },
  records::{function_type::FunctionType, variadic_type_pack::VariadicTypePack},
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

pub(crate) fn find_type_element_at_ast_type_list_type_pack_id_position(
  ast_type_list: &AstTypeList,
  tp: TypePackId,
  position: Position,
) -> Option<TypeId> {
  let types = ast_type_list.types.as_slice();

  for (i, &type_) in types.iter().enumerate() {
    // Safety: `types` 数组槽位为 parser 写入的非空存活 `*mut AstType`，此处仅 Copy
    // 读其 location 字段。
    let location = unsafe { (*type_).base.location };
    if location.contains_closed(position) {
      let (head, _) = flatten_type_pack_id(tp);

      if i < head.len() {
        // Safety: `type_` 为上面确认的存活非空 AstType，`head[i]`（i<head.len()）是与该
        // AST 槽位对应的 arena 存活 TypeId，满足被调 unsafe fn 的入参契约。
        return unsafe { find_type_element_at_ast_type_type_id_position(type_, head[i], position) };
      }
    }
  }

  if !ast_type_list.tail_type.is_null() {
    // Safety: 上一行已排除空指针，tail_type 指向 arena 内存活 AstTypePack，共享再借用
    // 仅供只读遍历。
    let arg_tp = unsafe { &*ast_type_list.tail_type };

    // safe 引用门面：`ast_node_try_as` 甄别 class_index（repr(C) 基址重合），命中进 Option、未命中 None。
    if let Some(variadic) = ast_node_try_as::<AstTypePackVariadic>(&arg_tp.base) {
      let location = variadic.base.base.location;
      if location.contains_closed(position) {
        let (_, tail) = flatten_type_pack_id(tp);

        if let Some(tail_id) = tail {
          let follow_tp = follow_type_pack::follow(tail_id);
          if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(follow_tp) {
            // variadic_type 是 variadic 的 parser 写入表达式槽位指针，只读指针值。
            let variadic_type = variadic.variadic_type;
            return unsafe {
              // Safety: variadic_type 为 arena 内存活 AstType，vtp.ty 为对应存活 TypeId，
              // 满足被调 unsafe fn 契约。
              find_type_element_at_ast_type_type_id_position(variadic_type, vtp.ty, position)
            };
          }
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
  unsafe {
    // Safety: `ast_type_pack` 按函数级契约为 null 或存活非空 AstTypePack，非空分支先判
    // null；`as *mut AstTypePackExplicit`/`AstTypePackVariadic` 由 ast_node_is RTTI 判型
    // 背书（repr(C) 基址重合），读 type_list/variadic_type/location 均为字段 Copy；
    // tp/position 原样转发给满足其契约的下游函数。
    if !ast_type_pack.is_null() {
      let node = ast_type_pack.as_ast_node();
      if ast_node_is::<AstTypePackExplicit>(&*node) {
        let explicit = ast_type_pack.cast::<AstTypePackExplicit>();
        let type_list = (*explicit).type_list;
        return find_type_element_at_ast_type_list_type_pack_id_position(&type_list, tp, position);
      } else if ast_node_is::<AstTypePackVariadic>(&*node) {
        let variadic = ast_type_pack.cast::<AstTypePackVariadic>();
        let loc = (*ast_type_pack).base.location;
        if loc.contains_closed(position) {
          let (_, tail) = flatten_type_pack_id(tp);

          if let Some(tail_id) = tail {
            let follow_tp = follow_type_pack::follow(tail_id);
            if let Some(vtp) = get_type_pack::get::<VariadicTypePack>(follow_tp) {
              let variadic_type = (*variadic).variadic_type;
              return find_type_element_at_ast_type_type_id_position(
                variadic_type,
                vtp.ty,
                position,
              );
            }
          }
        }
      }
    }
  }
  None
}

/// # Safety
/// `ast_type` 必须非空并指向 arena 内存活、RTTI 有效的 `AstType`；`ty` 必须是与 `ast_type`
/// 结构对应的存活 `TypeId`（命中 FunctionType 时按其解引用读 arg/ret 类型组）。本 pass
/// 单线程、AST arena 只读，且 `ast_type` 与 `ty` 描述同一类型构造。
pub(crate) unsafe fn find_type_element_at_ast_type_type_id_position(
  ast_type: *mut AstType,
  ty: TypeId,
  position: Position,
) -> Option<TypeId> {
  let ty = follow_type::follow(ty);

  // Safety: `ast_type` 按函数级契约为存活非空 AstType；`ast_node_is` 判型只读基类索引；
  // `ast_node_try_as_ptr` 判型成功提供合法 AstTypeFunction 只读借用；
  // `ty` 为 follow 后的存活 TypeId，`get_type::get::<FunctionType>` 命中才读 arg_types/return_types
  // 并转发给契约相符的递归/列表函数，全程 arena 只读、无并存可变借用。
  if unsafe { ast_node_is_ptr::<AstTypeReference>(ast_type) }
    || unsafe { ast_node_is_ptr::<AstTypeError>(ast_type) }
  {
    return Some(ty);
  }

  if let Some(type_function) = unsafe { ast_node_try_as_ptr::<AstTypeFunction>(ast_type) } {
    let ftv = get_type::get::<FunctionType>(ty)?;

    let arg_types = &type_function.arg_types;
    let arg_types_tp = ftv.arg_types;

    if let Some(element) =
      find_type_element_at_ast_type_list_type_pack_id_position(arg_types, arg_types_tp, position)
    {
      return Some(element);
    }

    let return_types = type_function.return_types;
    let ret_types_tp = ftv.ret_types;

    if let Some(element) = unsafe {
      find_type_element_at_ast_type_pack_type_pack_id_position(return_types, ret_types_tp, position)
    } {
      return Some(element);
    }
  }

  None
}
