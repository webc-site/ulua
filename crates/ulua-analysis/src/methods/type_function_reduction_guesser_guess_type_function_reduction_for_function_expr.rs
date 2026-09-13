use alloc::{
  string::{String, ToString},
  vec::Vec,
};
// C++ `struct InstanceCollector2 : TypeOnceVisitor` (TypeFunctionReductionGuesser.cpp).
// The virtual `visit(...)`/`cycle(...)` overrides live as the
// `GenericTypeVisitorTrait` impl so `traverse` dispatches into them; the bodies
// delegate to the inherent methods declared on the record / sibling files.
use core::ffi::CStr;
use core::{
  ffi::c_void,
  mem::replace,
  ptr::{null, null_mut},
};
use std::mem::take;

use ulua_ast::records::ast_expr_function::AstExprFunction;
use ulua_common::records::dense_hash_set::DenseHashSet;

use crate::{
  functions::{
    begin_type_pack::begin, end_type_pack::end, follow_type::follow_type_id,
    get_type_alt_j::get_type_id,
  },
  records::{
    extern_type::ExternType,
    function_type::FunctionType,
    generic_type_visitor::{GenericTypeVisitor, GenericTypeVisitorTrait},
    instance_collector_2::InstanceCollector2,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack,
    type_function_reduction_guess_result::TypeFunctionReductionGuessResult,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl GenericTypeVisitorTrait for InstanceCollector2 {
  type Seen = DenseHashSet<*mut c_void>;

  fn visitor_base(&mut self) -> &mut GenericTypeVisitor<Self::Seen> {
    &mut self.base.base
  }

  fn cycle_type_id(&mut self, ty: TypeId) {
    InstanceCollector2::cycle(self, ty);
  }

  fn visit_type_id_type_function_instance_type(
    &mut self,
    ty: TypeId,
    it: &TypeFunctionInstanceType,
  ) -> bool {
    InstanceCollector2::visit_type_id_type_function_instance_type(self, ty, it)
  }

  fn visit_type_id_extern_type(&mut self, ty: TypeId, et: &ExternType) -> bool {
    InstanceCollector2::visit_type_id_extern_type(self, ty, et)
  }

  fn visit_type_pack_id_type_function_instance_type_pack(
    &mut self,
    tp: TypePackId,
    itp: &TypeFunctionInstanceTypePack,
  ) -> bool {
    InstanceCollector2::visit_type_pack_id_type_function_instance_type_pack(self, tp, itp)
  }
}

impl TypeFunctionReductionGuesser {
  pub fn guess_type_function_reduction_for_function_expr(
    &mut self,
    expr: &AstExprFunction,
    ftv: &FunctionType,
    ret_ty: TypeId,
  ) -> TypeFunctionReductionGuessResult {
    let mut collector = InstanceCollector2::new();
    collector.traverse_type_id(ret_ty);
    self.to_infer = take(&mut collector.tys);
    self.cyclic_instances = replace(
      &mut collector.cyclic_instance,
      DenseHashSet::new(null_mut()),
    );

    if self.is_function_generics_saturated(ftv, &mut collector.instance_arguments) {
      return TypeFunctionReductionGuessResult {
        guessed_function_annotations: Vec::new(),
        guessed_return_type: null(),
        should_recommend_annotation: false,
      };
    }
    self.infer();

    let mut results: Vec<(String, TypeId)> = Vec::new();
    let mut args: Vec<TypeId> = Vec::new();
    let mut it = begin(ftv.arg_types);
    let end_it = end(ftv.arg_types);
    while it.operator_ne(&end_it) {
      args.push(*it.operator_deref());
      it.operator_inc();
    }

    // Submit a guess for arg types
    for (i, &local) in expr.args.as_slice().iter().enumerate() {
      if i >= args.len() {
        continue;
      }

      let arg_ty = args[i];
      let guessed_type = self.guess_type(arg_ty);
      let guessed_type = match guessed_type {
        Some(g) => g,
        None => continue,
      };
      let guess = follow_type_id(guessed_type);
      if !get_type_id::<TypeFunctionInstanceType>(guess).is_none() {
        continue;
      }

      let name = unsafe {
        if (*local).name.value.is_null() {
          String::new()
        } else {
          CStr::from_ptr((*local).name.value)
            .to_string_lossy()
            .to_string()
        }
      };
      results.push((name, guess));
    }

    // Submit a guess for return types

    let guessed_return_type = self.guess_type(ret_ty);
    let recommended_annotation: TypeId = match guessed_return_type {
      None => unsafe { (*self.builtins).unknown_type },
      Some(g) => follow_type_id(g),
    };
    let recommended_annotation =
      if !get_type_id::<TypeFunctionInstanceType>(recommended_annotation).is_none() {
        unsafe { (*self.builtins).unknown_type }
      } else {
        recommended_annotation
      };

    self.to_infer.clear();
    self.cyclic_instances.clear();
    self.function_reduces_to.clear();
    self.substitutable.clear();

    TypeFunctionReductionGuessResult {
      guessed_function_annotations: results,
      guessed_return_type: recommended_annotation,
      should_recommend_annotation: true,
    }
  }
}
