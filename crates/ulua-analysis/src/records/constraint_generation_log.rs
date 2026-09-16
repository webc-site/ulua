use alloc::{string::String, vec::Vec};

use crate::records::{
  annotation_types_at_location::AnnotationTypesAtLocation, error_snapshot::ErrorSnapshot,
  expr_types_at_location::ExprTypesAtLocation,
};
#[derive(Debug, Clone, Default)]
pub struct ConstraintGenerationLog {
  pub source: String,
  pub errors: Vec<ErrorSnapshot>,
  pub expr_type_locations: Vec<ExprTypesAtLocation>,
  pub annotation_type_locations: Vec<AnnotationTypesAtLocation>,
}
