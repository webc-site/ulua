use alloc::string::String;
use core::ptr::NonNull;

use ulua_ast::records::location::Location;

use crate::{
  functions::reduce_type_functions_type_function::reduce_type_functions,
  records::{
    function_graph_reduction_result::FunctionGraphReductionResult, scope::Scope,
    subtyping::Subtyping, type_error::TypeError, type_function_context::TypeFunctionContext,
    type_function_instance_type::TypeFunctionInstanceType,
    uninhabited_type_function::UninhabitedTypeFunction,
  },
  type_aliases::{error_vec::ErrorVec, type_error_data::TypeErrorData, type_id::TypeId},
};
impl Subtyping {
  /// # Safety
  /// 调用方须保证满足 C++ 原实现的调用契约。
  pub unsafe fn handle_type_function_reduction_result(
    &mut self,
    function_instance: &TypeFunctionInstanceType,
    scope: *mut Scope,
  ) -> (TypeId, ErrorVec) {
    let mut subtyping = Subtyping::subtyping_owned(
      self.builtin_types,
      self.arena,
      self.normalizer,
      self.type_function_runtime,
      self.ice_reporter,
    );
    let context = TypeFunctionContext::from_components(
      unsafe { NonNull::new_unchecked(self.arena) },
      unsafe { NonNull::new_unchecked(self.builtin_types) },
      unsafe { NonNull::new_unchecked(scope) },
      unsafe { NonNull::new_unchecked(self.normalizer) },
      unsafe { NonNull::new_unchecked(self.type_function_runtime) },
      unsafe { NonNull::new_unchecked(self.ice_reporter) },
      NonNull::from(&mut self.limits),
      NonNull::from(&mut subtyping),
    );
    let mut context = context;

    let function = unsafe { (*self.arena).add_type(function_instance.clone()) };
    let result: FunctionGraphReductionResult = reduce_type_functions(
      function,
      Location::default(),
      NonNull::from(&mut context),
      true,
    );
    let mut errors: ErrorVec = ErrorVec::new();
    if result.blocked_types.size() != 0 || result.blocked_packs.size() != 0 {
      errors.push(TypeError {
        location: Location::default(),
        module_name: String::new(),
        data: TypeErrorData::UninhabitedTypeFunction(UninhabitedTypeFunction { ty: function }),
      });
      return (unsafe { (*self.builtin_types).never_type }, errors);
    }
    if result.reduced_types.contains(&function) {
      return (function, errors);
    }
    (unsafe { (*self.builtin_types).never_type }, errors)
  }
}
