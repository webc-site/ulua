use alloc::vec::Vec;
use core::ptr::{NonNull, null_mut};

use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  functions::{get_mutable_type_pack, get_type},
  records::{
    blocked_type_pack::BlockedTypePack, constraint::Constraint,
    constraint_solver::ConstraintSolver, function_call_constraint::FunctionCallConstraint,
    function_type::FunctionType, iterable_constraint::IterableConstraint,
  },
  type_aliases::{constraint_v::ConstraintV, type_id::TypeId},
};
impl ConstraintSolver {
  pub fn try_dispatch_iterable_function(
    &mut self,
    next_ty: TypeId,
    table_ty: TypeId,
    c: &IterableConstraint,
    constraint: &Constraint,
  ) -> bool {
    LUAU_ASSERT!(get_type::get::<FunctionType>(next_ty).is_some());

    unsafe { (*(*c.ast_for_in_next_types).get_or_insert(c.next_ast_fragment)) = next_ty };

    let table_ty_pack = self
      .arena_mut()
      .add_type_pack_initializer_list_type_id(&[table_ty]);

    let variables_pack = {
      let mut btp = BlockedTypePack {
        index: 0,
        owner: null_mut(),
      };
      btp.blocked_type_pack_blocked_type_pack();
      self.arena_mut().add_type_pack_t(btp)
    };

    let call_constraint = self.push_constraint(
      // Safety: Constraint.scope 按 cpp NotNull<Scope> 登记，恒非空。
      NonNull::new(constraint.scope).expect("Constraint.scope 为 NotNull 登记值，恒非空"),
      constraint.location,
      ConstraintV::FunctionCall(FunctionCallConstraint {
        fn_type: next_ty,
        args_pack: table_ty_pack,
        result: variables_pack,
        call_site: null_mut(),
        discriminant_types: Vec::new(),
        type_arguments: Vec::new(),
        type_pack_arguments: Vec::new(),
        ast_overload_resolved_types: null_mut(),
      }),
    );

    // variables_pack 刚 emplace 为 BlockedTypePack，下转必命中
    // （C++ `getMutable<BlockedTypePack>` 后直解引用）。
    get_mutable_type_pack::get_mutable::<BlockedTypePack>(variables_pack)
      .expect("fresh blocked type pack")
      .owner = call_constraint.as_ptr();

    let unpack_constraint = self.unpack_and_assign(
      c.variables.clone(),
      variables_pack,
      // & 引用反推 NonNull 天然非空，免 const→mut 裸转（同址同语义）。
      NonNull::from(constraint),
    );

    self.inherit_blocks(constraint as *const Constraint, call_constraint.as_ptr());
    self.inherit_blocks(unpack_constraint.as_ptr(), call_constraint.as_ptr());

    true
  }
}
