use alloc::vec;

use crate::{
  functions::{
    as_mutable_type_pack_alt_d::as_mutable_type_pack, flatten_type_pack::flatten_type_pack_id,
    follow_type::follow_type_id, get_type_alt_j::get_type_id,
  },
  records::{
    function_type::FunctionType, magic_function_call_context::MagicFunctionCallContext,
    type_pack::TypePack,
  },
  type_aliases::type_pack_variant::TypePackVariant,
};

pub fn magic_pcall_infer(ctx: &MagicFunctionCallContext) -> bool {
  let (arg_head, _arg_tail) = flatten_type_pack_id(ctx.arguments);

  if arg_head.is_empty() {
    return false;
  }

  let fn_ty = follow_type_id(arg_head[0]);
  let Some(fn_ptr) = get_type_id::<FunctionType>(fn_ty) else {
    return false;
  };

  let (fn_return_head, fn_return_tail) = flatten_type_pack_id(fn_ptr.ret_types);
  if !fn_return_head.is_empty() || fn_return_tail.is_some() {
    return false;
  }

  let solver = unsafe { ctx.solver.as_ref() };
  let res = unsafe {
    let builtin_types = &*solver.builtin_types;
    (*solver.arena).add_type_pack_t(TypePack {
      head: vec![builtin_types.boolean_type, builtin_types.unknown_type],
      tail: None,
    })
  };

  let result_mut = as_mutable_type_pack(ctx.result);
  unsafe {
    (*result_mut).ty = TypePackVariant::Bound(res);
  }

  true
}
