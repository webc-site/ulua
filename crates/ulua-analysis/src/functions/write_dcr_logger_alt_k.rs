use crate::{
  functions::to_pointer_id_dcr_logger::to_pointer_id,
  records::{expr_types_at_location::ExprTypesAtLocation, json_emitter::JsonEmitter},
};

pub fn write_json_emitter_expr_types_at_location(
  emitter: &mut JsonEmitter,
  tys: &ExprTypesAtLocation,
) {
  let mut o = emitter.write_object();
  o.write_pair("location", tys.location);
  o.write_pair("ty", to_pointer_id(tys.ty as *const _));
  if let Some(expected_ty) = tys.expected_ty {
    o.write_pair("expectedTy", to_pointer_id(expected_ty as *const _));
  }
  o.finish();
}
