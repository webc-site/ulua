//! Source: `Analysis/src/ConstraintGenerator.cpp:3223-3279` (hand-ported)
//! C++ `Inference ConstraintGenerator::checkIndexName(scope, key, indexee, index, indexLocation)`.
use core::ptr::null;

use ulua_ast::records::{ast_expr::AstExpr, location::Location};

use crate::{
  enums::value_context::ValueContext,
  functions::{
    get_mutable_type, get_table_type::get_table_type, in_conditional::in_conditional,
    inference_with_refinement::inference_with_refinement,
  },
  records::{
    arena_handle::alias_ref, blocked_type::BlockedType, constraint_generator::ConstraintGenerator,
    has_prop_constraint::HasPropConstraint, inference::Inference, refinement_key::RefinementKey,
  },
  type_aliases::{constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl ConstraintGenerator {
  /// C++ `checkIndexName(scope, key, indexee, index, indexLocation)`。形参链已
  /// 引用化（原 `# Safety` 契约由签名承担）：`indexee` 为 parse arena 存活节点
  /// 共享借用；`key` 为可空 `RefinementKey` 身份句柄（约束/细化 arena 记录键，
  /// 布局不改），解引用经 alias_ref 收口。
  pub fn check_index_name(
    &mut self,
    scope: &ScopePtr,
    key: *const RefinementKey,
    indexee: &AstExpr,
    index: &str,
    index_location: Location,
  ) -> Inference {
    {
      let obj = self.check_expr(scope, indexee).ty;
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

      if let Some(cached_has_prop_result) =
        self.prop_index_pairs_seen.find(&(obj, index.to_owned()))
      {
        result = *cached_has_prop_result;
      }

      if result.is_null() {
        result = self.arena.get_mut().add_type(BlockedType::default());

        let c = self.add_constraint_scope_ptr_location_constraint_v(
          scope,
          indexee.base.location,
          ConstraintV::HasProp(HasPropConstraint {
            result_type: result,
            subject_type: obj,
            prop: index.to_owned(),
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
          .get_or_insert((obj, index.to_owned())) = result;
      }

      if !key.is_null() {
        if let Some(ty) = self.lookup(scope, index_location, alias_ref(key).def, false) {
          let refinement = self
            .refinement_arena
            .proposition_refinement_key_type_id(key, self.builtin_types.get().truthy_type);
          // §2：`None`（原 `Inference{ty, nullptr}` 形态）收口到 Option 构造器。
          return inference_with_refinement(ty, refinement);
        }

        self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, alias_ref(key).def, result);
      }

      if !key.is_null() {
        let refinement = self
          .refinement_arena
          .proposition_refinement_key_type_id(key, self.builtin_types.get().truthy_type);
        // §2：`None`（原 `Inference{result, nullptr}` 形态）收口到 Option 构造器。
        inference_with_refinement(result, refinement)
      } else {
        Inference::no_refinement(result)
      }
    }
  }
}
