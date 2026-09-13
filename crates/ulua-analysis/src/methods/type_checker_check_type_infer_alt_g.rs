use alloc::{sync::Arc, vec::Vec};

use ulua_ast::records::ast_stat_return::AstStatReturn;

use crate::{
  enums::control_flow::ControlFlow,
  functions::{
    begin_type_pack::begin, end_type_pack::end, follow_type_pack::follow_type_pack_id,
    get_type_pack::get_type_pack_id,
  },
  records::{
    count_mismatch::CountMismatchContext, demoter::Demoter, module::Module, scope::Scope,
    type_arena::TypeArena, type_checker::TypeChecker, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{scope_ptr_type::ScopePtr, type_id::TypeId},
};
impl TypeChecker {
  pub fn check_scope_ptr_ast_stat_return(
    &mut self,
    scope: &ScopePtr,
    return_: &AstStatReturn,
  ) -> ControlFlow {
    let mut expected_types: Vec<Option<TypeId>> = Vec::with_capacity(return_.list.size);

    let mut expected_ret_curr = begin(scope.return_type);
    let expected_ret_end = end(scope.return_type);

    for _i in 0..return_.list.size {
      if expected_ret_curr.operator_ne(&expected_ret_end) {
        expected_types.push(Some(*expected_ret_curr.operator_deref()));
        expected_ret_curr.operator_inc();
      } else if let Some(expected_args_tail) = expected_ret_curr.tail()
        && let Some(vtp) =
          get_type_pack_id::<VariadicTypePack>(unsafe { follow_type_pack_id(expected_args_tail) })
      {
        expected_types.push(Some(vtp.ty));
      }
    }

    let arena = unsafe {
      &mut (*(Arc::as_ptr(self.current_module.as_ref().unwrap()) as *mut Module)).internal_types
        as *mut TypeArena
    };
    let mut demoter = Demoter {
      arena,
      builtins: self.builtin_types,
    };
    demoter.demote(&mut expected_types);

    let ret_pack = self
      .check_expr_list(
        scope,
        &return_.base.base.location,
        &return_.list,
        false,
        &Vec::new(),
        &expected_types,
      )
      .r#type;

    // HACK: Nonstrict mode gets a bit too smart and strict for us when we
    // start typechecking everything across module boundaries.
    let module_return_type = {
      (*self.current_module.as_ref().unwrap())
        .get_module_scope()
        .return_type
    };
    if self.is_nonstrict_mode()
      && unsafe { follow_type_pack_id(scope.return_type) }
        == unsafe { follow_type_pack_id(module_return_type) }
    {
      let errors = self.try_unify_type_pack_id_type_pack_id_scope_ptr_location(
        ret_pack,
        scope.return_type,
        scope.clone(),
        &return_.base.base.location,
      );

      if !errors.is_empty() {
        let any_pack = self.add_type_pack_initializer_list_type_id(&[self.any_type]);
        unsafe {
          let module_scope = (*self.current_module.as_ref().unwrap()).get_module_scope();
          let module_scope_mut = Arc::as_ptr(&module_scope) as *mut Scope;
          (*module_scope_mut).return_type = any_pack;
        }
      }

      return ControlFlow::Returns;
    }

    self.unify_type_pack_id_type_pack_id_scope_ptr_location_count_mismatch_context(
      ret_pack,
      scope.return_type,
      scope,
      &return_.base.base.location,
      CountMismatchContext::Return,
    );

    ControlFlow::Returns
  }
}
