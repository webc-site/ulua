use alloc::sync::Arc;

use crate::{
  functions::{attach_magic_function::attach_magic_function, attach_tag_type::attach_tag},
  methods::{
    magic_require_handle_old_solver::magic_require_handle_old_solver,
    magic_require_infer::magic_require_infer,
  },
  records::{
    magic_function::MagicFunction,
    magic_function_type_check_context::MagicFunctionTypeCheckContext,
    magic_refinement_context::MagicRefinementContext,
  },
  type_aliases::type_id::TypeId,
};

fn noop_refine(_context: &MagicRefinementContext) {}

fn noop_type_check(_context: &MagicFunctionTypeCheckContext) -> bool {
  false
}

pub fn attach_require_magic(require_ty: TypeId) {
  attach_tag(require_ty, "require");
  attach_magic_function(
    require_ty,
    Arc::new(MagicFunction {
      handle_old_solver: magic_require_handle_old_solver,
      infer: magic_require_infer,
      refine: noop_refine,
      type_check: noop_type_check,
    }),
  );
}
