use crate::{
  functions::to_pointer_id_dcr_logger::to_pointer_id,
  records::{annotation_types_at_location::AnnotationTypesAtLocation, json_emitter::JsonEmitter},
};

pub fn write_json_emitter_annotation_types_at_location(
  emitter: &mut JsonEmitter,
  tys: &AnnotationTypesAtLocation,
) {
  let mut o = emitter.write_object();
  o.write_pair("location", tys.location);
  o.write_pair("resolvedTy", to_pointer_id(tys.resolved_ty as *const _));
  o.finish();
}
