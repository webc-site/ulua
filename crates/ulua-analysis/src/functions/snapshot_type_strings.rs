use alloc::string::String;

use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::to_string_to_string::to_string_type_id_to_string_options,
  records::{
    annotation_types_at_location::AnnotationTypesAtLocation,
    expr_types_at_location::ExprTypesAtLocation, to_string_options::ToStringOptions,
  },
};

pub fn snapshot_type_strings(
  interested_exprs: &[ExprTypesAtLocation],
  interested_annots: &[AnnotationTypesAtLocation],
  map: &mut DenseHashMap<*const (), String>,
  opts: &mut ToStringOptions,
) {
  for tys in interested_exprs {
    let s = to_string_type_id_to_string_options(tys.ty, opts);
    map.try_insert(tys.ty as *const (), s);

    if let Some(expected_ty) = tys.expected_ty {
      let s = to_string_type_id_to_string_options(expected_ty, opts);
      map.try_insert(expected_ty as *const (), s);
    }
  }

  for tys in interested_annots {
    let s = to_string_type_id_to_string_options(tys.resolved_ty, opts);
    map.try_insert(tys.resolved_ty as *const (), s);
  }
}
