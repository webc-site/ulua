//! Source: `Analysis/src/ConstraintGenerator.cpp:3223-3279` (hand-ported)
//! C++ `Inference ConstraintGenerator::checkIndexName(scope, key, indexee, index, indexLocation)`.
use alloc::string::String;
use core::ptr::null;

use ulua_ast::records::{ast_expr::AstExpr, location::Location};

use crate::{
  enums::value_context::ValueContext,
  functions::{get_mutable_type, get_table_type::get_table_type, in_conditional::in_conditional},
  records::{
    blocked_type::BlockedType, constraint_generator::ConstraintGenerator,
    has_prop_constraint::HasPropConstraint, inference::Inference, refinement_key::RefinementKey,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn check_index_name(
    &mut self,
    scope: &ScopePtr,
    key: *const RefinementKey,
    indexee: *mut AstExpr,
    index: &String,
    index_location: Location,
  ) -> Inference {
    unsafe {
      let obj = self.check_expr(scope, &*indexee).ty; // indexee 存活由本 unsafe fn 契约担保
      let mut result: TypeId = null();

      // We optimize away the HasProp constraint in simple cases so that we can
      // reason about updates to unsealed tables more accurately.

      let mut tt = get_table_type(obj);

      // This is a little bit iffy but I *believe* it is okay because, if the
      // local's domain is going to be extended at all, it will be someplace after
      // the current lexical position within the script.
      if tt.is_none()
        && let Some(local_domain) = self.local_types.find(&obj)
        && local_domain.size() == 1
      {
        let first = local_domain.order[0];
        tt = get_table_type(first);
      }

      if let Some(tt) = tt
        && let Some(prop) = tt.props.get(index)
        && let Some(read_ty) = prop.read_ty
      {
        result = read_ty;
      }

      if let Some(cached_has_prop_result) = self.prop_index_pairs_seen.find(&(obj, index.clone())) {
        result = *cached_has_prop_result;
      }

      if result.is_null() {
        result = self.arena.get_mut().add_type(BlockedType::default());

        let c = self.add_constraint_scope_ptr_location_constraint_v(
          scope,
          (*indexee).base.location,
          ConstraintV::HasProp(HasPropConstraint {
            result_type: result,
            subject_type: obj,
            prop: index.clone(),
            context: ValueContext::RValue,
            in_conditional: in_conditional(self.type_context),
            suppress_simplification: false,
          }),
        );
        // result 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3909
        // `getMutable<BlockedType>(propTy)->setOwner(apc)`
        let blocked = get_mutable_type::get_mutable::<BlockedType>(result)
          .expect("result 刚由 add_type(BlockedType) 分配，必命中（cpp:3909）");
        blocked.set_owner(c as *const _);
        *self
          .prop_index_pairs_seen
          .get_or_insert((obj, index.clone())) = result;
      }

      if !key.is_null() {
        if let Some(ty) = self.lookup(scope, index_location, (*key).def, false) {
          let refinement = self
            .refinement_arena
            .proposition_refinement_key_type_id(key, self.builtin_types.get().truthy_type);
          // §2：`None`（原 `Inference{ty, nullptr}` 形态）收口到 `no_refinement` 构造器。
          return refinement.map_or_else(
            || Inference::no_refinement(ty),
            |r| Inference::inference_type_id_refinement_id(ty, r),
          );
        }

        self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, (*key).def, result);
      }

      if !key.is_null() {
        let refinement = self
          .refinement_arena
          .proposition_refinement_key_type_id(key, self.builtin_types.get().truthy_type);
        // §2：`None`（原 `Inference{result, nullptr}` 形态）收口到 `no_refinement` 构造器。
        refinement.map_or_else(
          || Inference::no_refinement(result),
          |r| Inference::inference_type_id_refinement_id(result, r),
        )
      } else {
        Inference::no_refinement(result)
      }
    }
  }
}
