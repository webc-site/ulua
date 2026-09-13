//! Source: `Analysis/src/OverloadResolver.cpp:147-173` (hand-ported)
use crate::{
  enums::pack_field::PackField,
  functions::{flatten_type_pack::flatten_type_pack_id, is_optional::is_optional},
  records::path::Path,
  type_aliases::{
    component::Component, subtyping_reasonings::SubtypingReasonings, type_pack_id::TypePackId,
  },
};

pub fn are_unsatisfied_arguments_optional(
  reasonings: &SubtypingReasonings,
  arg_pack: TypePackId,
  func_arg_pack: TypePackId,
) -> bool {
  // If the two argument lists are incompatible solely because of the argument
  // counts, the reasonings will simply point at the argument lists
  // themselves. If the reasonings point into a pack, it's because that
  // specific argument has an incompatible type.
  if 1 != reasonings.size() {
    return false;
  }

  let just_arguments = Path::from_component(Component::PackField(PackField::Arguments));
  let reason = match reasonings.iter().next() {
    Some(r) => r,
    None => return false,
  };
  if !reason.sub_path.operator_eq(&just_arguments)
    || !reason.super_path.operator_eq(&just_arguments)
  {
    return false;
  }

  let (arg_head, _arg_tail) = flatten_type_pack_id(arg_pack);
  let (fun_arg_head, _fun_arg_tail) = flatten_type_pack_id(func_arg_pack);

  if arg_head.len() >= fun_arg_head.len() {
    return false;
  }

  fun_arg_head[arg_head.len()..]
    .iter()
    .all(|arg| is_optional(*arg))
}
