//! `type_function_reduction_guesser` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use std::sync::Arc;

use ulua_common::{
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, vec_deque::VecDeque},
};

use crate::{
  functions::{
    flatten_type_pack::flatten_type_pack_id, follow_type, follow_type_utils::follow_optional_ty,
    get_type, get_type::get, is_prim::is_number,
  },
  records::{
    arena_handle::{Handle, alias},
    builtin_types::BuiltinTypes,
    function_type::FunctionType,
    generic_type::GenericType,
    normalized_type::NormalizedType,
    normalizer::Normalizer,
    type_arena::TypeArena,
    type_function_inference_result::TypeFunctionInferenceResult,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser,
    type_pack::TypePack,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeFunctionReductionGuesser {
  pub fn done(&self) -> bool {
    self.to_infer.empty()
  }
}

impl TypeFunctionReductionGuesser {
  pub fn guess_type(&mut self, arg: TypeId) -> Option<TypeId> {
    let t = follow_type::follow(arg);

    if self.substitutable.contains(&t) {
      // 上方 `contains(&t)` 判定蕴含 find 命中，双写合一直接绑定非空句柄。
      let subst0 = self
        .substitutable
        .find(&t)
        .copied()
        .expect("上方 contains 判定蕴含 find 命中");
      let subst = follow_optional_ty(Some(subst0)).unwrap_or(subst0);
      if subst == t
        || self.substitutable.contains(&subst)
        || get_type::get::<TypeFunctionInstanceType>(subst).is_none()
      {
        return Some(subst);
      } else {
        return self.guess_type(subst);
      }
    }

    if get_type::get::<TypeFunctionInstanceType>(t).is_some()
      && self.function_reduces_to.contains(&t)
    {
      return self.function_reduces_to.find(&t).copied();
    }

    None
  }
}

impl TypeFunctionReductionGuesser {
  /// C++ `TypeFunctionReductionGuesser.cpp:96 guess(TypeId)`。
  pub fn guess_type_id(&mut self, typ: TypeId) -> Option<TypeId> {
    let guessed_type = self.guess_type(typ)?;

    let guess = follow_type::follow(guessed_type);
    // C++: `if (get<TypeFunctionInstanceType>(guess)) return {};`
    if get_type::get::<TypeFunctionInstanceType>(guess).is_some() {
      return None;
    }

    Some(guess)
  }

  /// C++ `TypeFunctionReductionGuesser.cpp:110 guess(TypePackId)`。
  pub fn guess_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
    let (head, tail) = flatten_type_pack_id(tp);

    let mut guessed_head: Vec<TypeId> = Vec::with_capacity(head.len());

    for typ in head.iter().copied() {
      let guessed_type = self.guess_type(typ)?;

      // C++: `if (get<TypeFunctionInstanceType>(guess)) return {};`
      let guess = follow_type::follow(guessed_type);
      if get_type::get::<TypeFunctionInstanceType>(guess).is_some() {
        return None;
      }

      // C++: `guessedHead.push_back(*guessedType)`（存原始猜测值，非 follow 结果）
      guessed_head.push(guessed_type);
    }

    // SAFETY: arena 由构造方保证有效，同 C++ `arena->addTypePack`。
    Some(
      self
        .arena
        .get_mut()
        .add_type_pack_t(TypePack::new(guessed_head, tail)),
    )
  }
}

impl TypeFunctionReductionGuesser {
  pub fn infer(&mut self) {
    while !self.done() {
      self.step();
    }
  }
}

impl TypeFunctionReductionGuesser {
  pub fn infer_comparison_function(
    &mut self,
    instance: &TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    LUAU_ASSERT!(instance.type_arguments.len() == 2);
    // Comparison functions are lt/le/eq.
    // Heuristic: these are type functions from t -> t -> bool

    let mut lhs_ty = follow_type::follow(instance.type_arguments[0]);
    let mut rhs_ty = follow_type::follow(instance.type_arguments[1]);

    let builtins = self.builtins.get();
    let boolean_ty = builtins.boolean_type;
    let comparison_inference = |op: TypeId| -> TypeFunctionInferenceResult {
      TypeFunctionInferenceResult {
        operand_inference: alloc::vec![op, op],
        function_result_inference: boolean_ty,
      }
    };

    if let Some(ty) = self.try_assign_operand_type(lhs_ty) {
      lhs_ty = follow_type::follow(ty);
    }
    if let Some(ty) = self.try_assign_operand_type(rhs_ty) {
      rhs_ty = follow_type::follow(ty);
    }
    if self.operand_is_assignable(lhs_ty) && !self.operand_is_assignable(rhs_ty) {
      return comparison_inference(rhs_ty);
    }
    if self.operand_is_assignable(rhs_ty) && !self.operand_is_assignable(lhs_ty) {
      return comparison_inference(lhs_ty);
    }
    comparison_inference(builtins.number_type)
  }
}

impl TypeFunctionReductionGuesser {
  pub fn infer_len_function(
    &mut self,
    instance: &TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    LUAU_ASSERT!(instance.type_arguments.len() == 1);
    let mut op_ty = follow_type::follow(instance.type_arguments[0]);
    if let Some(ty) = self.try_assign_operand_type(op_ty) {
      op_ty = follow_type::follow(ty);
    }
    TypeFunctionInferenceResult {
      operand_inference: alloc::vec![op_ty],
      function_result_inference: self.builtins.get_mut().number_type,
    }
  }
}

impl TypeFunctionReductionGuesser {
  pub fn infer_not_function(
    &mut self,
    instance: &TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    LUAU_ASSERT!(instance.type_arguments.len() == 1);

    let op_ty = follow_type::follow(instance.type_arguments[0]);
    let op_ty = if let Some(ty) = self.try_assign_operand_type(op_ty) {
      follow_type::follow(ty)
    } else {
      op_ty
    };

    TypeFunctionInferenceResult {
      operand_inference: alloc::vec![op_ty],
      function_result_inference: self.builtins.get_mut().boolean_type,
    }
  }
}

impl TypeFunctionReductionGuesser {
  pub fn infer_numeric_binop_function(
    &mut self,
    instance: &TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    LUAU_ASSERT!(instance.type_arguments.len() == 2);

    let builtins = self.builtins.get();
    TypeFunctionInferenceResult {
      operand_inference: alloc::vec![builtins.number_type, builtins.number_type],
      function_result_inference: builtins.number_type,
    }
  }
}

impl TypeFunctionReductionGuesser {
  pub fn infer_unary_minus_function(
    &mut self,
    instance: &TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    LUAU_ASSERT!(instance.type_arguments.len() == 1);
    let mut op_ty = follow_type::follow(instance.type_arguments[0]);
    if let Some(ty) = self.try_assign_operand_type(op_ty) {
      op_ty = follow_type::follow(ty);
    }
    if is_number(op_ty) {
      TypeFunctionInferenceResult {
        operand_inference: alloc::vec![op_ty],
        function_result_inference: self.builtins.get_mut().number_type,
      }
    } else {
      TypeFunctionInferenceResult {
        operand_inference: alloc::vec![self.builtins.get_mut().unknown_type],
        function_result_inference: self.builtins.get_mut().number_type,
      }
    }
  }
}

impl TypeFunctionReductionGuesser {
  pub fn is_comparison_function(&self, instance: &TypeFunctionInstanceType) -> bool {
    matches!(instance.function().name.as_str(), "lt" | "le" | "eq")
  }
}

impl TypeFunctionReductionGuesser {
  pub fn is_function_generics_saturated(
    &self,
    ftv: &FunctionType,
    args_used: &mut DenseHashSet<TypeId>,
  ) -> bool {
    let same_size = ftv.generics.len() == args_used.size();
    let mut all_generics_appear = true;
    for &gt in &ftv.generics {
      all_generics_appear = all_generics_appear && args_used.contains(&gt);
    }
    same_size && all_generics_appear
  }
}

impl TypeFunctionReductionGuesser {
  pub fn is_len_function(&self, instance: &TypeFunctionInstanceType) -> bool {
    instance.function().name == "len"
  }
}

impl TypeFunctionReductionGuesser {
  pub fn is_not_function(&self, instance: &TypeFunctionInstanceType) -> bool {
    instance.function().name == "not"
  }
}

impl TypeFunctionReductionGuesser {
  pub fn is_numeric_binop_function(&self, instance: &TypeFunctionInstanceType) -> bool {
    // 单值 matches!（字节 DFA），替代 7 次顺序字符串比较；null 名 as_bytes 为
    // 空切片，与旧比较链一致地落 false
    matches!(
      instance.function().name.as_bytes(),
      b"add" | b"sub" | b"mul" | b"div" | b"idiv" | b"pow" | b"mod"
    )
  }
}

impl TypeFunctionReductionGuesser {
  pub fn is_or_and_function(&self, instance: &TypeFunctionInstanceType) -> bool {
    matches!(instance.function().name.as_str(), "or" | "and")
  }
}

impl TypeFunctionReductionGuesser {
  pub fn is_unary_minus(&self, instance: &TypeFunctionInstanceType) -> bool {
    instance.function().name == "unm"
  }
}

impl TypeFunctionReductionGuesser {
  /// C++ 中归一化失败返回空 shared_ptr，此处以 None 表达
  pub fn normalize(&mut self, ty: TypeId) -> Option<Arc<NormalizedType>> {
    let normalizer = self.normalizer;
    alias(normalizer).try_normalize(ty)
  }
}

impl TypeFunctionReductionGuesser {
  pub fn operand_is_assignable(&self, ty: TypeId) -> bool {
    if get_type::get::<TypeFunctionInstanceType>(ty).is_some() {
      return true;
    }
    if get_type::get::<GenericType>(ty).is_some() {
      return true;
    }
    if self.cyclic_instances.contains(&ty) {
      return true;
    }
    false
  }
}

impl TypeFunctionReductionGuesser {
  /// C++ `TypeFunctionReductionGuesser.cpp:283 step()`。
  pub fn step(&mut self) {
    let t: TypeId = { *self.to_infer.front_mut() };
    self.to_infer.pop_front();
    let t = follow_type::follow(t);
    if let Some(tf) = get_type::get::<TypeFunctionInstanceType>(t) {
      self.infer_type_function_substitutions(t, tf);
    }
  }
}

impl TypeFunctionReductionGuesser {
  pub fn try_assign_operand_type(&mut self, ty: TypeId) -> Option<TypeId> {
    // Because we collect innermost instances first, if we see a type function instance as an operand,
    // We try to check if we guessed a type for it
    if get::<TypeFunctionInstanceType>(ty).is_some()
      && let Some(value) = self.function_reduces_to.find(&ty)
    {
      return Some(*value);
    }

    // If ty is a generic, we need to check if we inferred a substitution
    if get::<GenericType>(ty).is_some()
      && let Some(value) = self.substitutable.find(&ty)
    {
      return Some(*value);
    }

    // If we cannot substitute a type for this value, we return an empty optional
    None
  }
}

impl TypeFunctionReductionGuesser {
  pub fn type_function_reduction_guesser_type_function_reduction_guesser(
    arena: Handle<TypeArena>,
    builtins: Handle<BuiltinTypes>,
    normalizer: *mut Normalizer,
  ) -> Self {
    Self {
      function_reduces_to: DenseHashMap::default(),
      substitutable: DenseHashMap::default(),
      to_infer: VecDeque::new(),
      cyclic_instances: DenseHashSet::default(),
      arena,
      builtins,
      normalizer,
    }
  }
}
