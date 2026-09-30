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
    arena_handle::Handle, builtin_types::BuiltinTypes, function_type::FunctionType,
    generic_type::GenericType, normalized_type::NormalizedType, normalizer::Normalizer,
    type_arena::TypeArena, type_function_inference_result::TypeFunctionInferenceResult,
    type_function_instance_type::TypeFunctionInstanceType,
    type_function_reduction_guesser::TypeFunctionReductionGuesser, type_pack::TypePack,
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
  /// # Safety
  /// `instance` 须非空、对齐，指向 guesser 会话期间存活的 arena
  /// `TypeFunctionInstanceType`（由派发链 `infer_type_function_substitutions`
  /// 经 `get_type` 命中后传入）；仅读 `type_arguments`。单线程。
  pub unsafe fn infer_comparison_function(
    &mut self,
    instance: *const TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    // Safety: `instance` 由 step() 在 follow 后经 get_type::get::<TypeFunctionInstanceType>
    // 命中才传入（调用入口 LUAU_ASSERT 非空），指向 arena 中存活的实例节点载荷，
    // 猜测会话期间不移动；读 type_arguments.len() 有效。
    LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 2 });
    // Comparison functions are lt/le/eq.
    // Heuristic: these are type functions from t -> t -> bool

    // Safety: 同上——instance 指向存活节点且 len == 2 刚被断言，[0] 索引不越界；
    // 元素为 arena 存活 TypeId 句柄，follow_type_id 只读节点变体。
    let mut lhs_ty = unsafe { follow_type::follow((&(*instance).type_arguments)[0]) };
    // Safety: 与 [0] 侧逐字同前提——存活实例节点、len == 2 断言、[1] 在界内。
    let mut rhs_ty = unsafe { follow_type::follow((&(*instance).type_arguments)[1]) };

    // Safety: self.builtins 在 guesser 构造时由 TypeFunctionReducer 会话接线，
    // 非空、比 guesser 长寿、reduce 期间不再写入；提为一次共享借用，仅读 Copy 句柄。
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
  /// # Safety
  /// `instance` 须非空、对齐，指向 guesser 会话期间存活的 arena
  /// `TypeFunctionInstanceType`（唯一调用方经 `step()` 的 `get_type` 命中后传入，
  /// 同 cpp `TypeFunctionReductionGuesser.cpp` 的 NotNull 前提）；仅读
  /// `type_arguments`，写路径不触及本实例节点。单线程。
  pub unsafe fn infer_len_function(
    &mut self,
    instance: *const TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    unsafe {
      LUAU_ASSERT!((*instance).type_arguments.len() == 1);
      let mut op_ty = follow_type::follow((&(*instance).type_arguments)[0]);
      if let Some(ty) = self.try_assign_operand_type(op_ty) {
        op_ty = follow_type::follow(ty);
      }
      TypeFunctionInferenceResult {
        operand_inference: alloc::vec![op_ty],
        function_result_inference: self.builtins.get_mut().number_type,
      }
    }
  }
}

impl TypeFunctionReductionGuesser {
  /// # Safety
  /// `instance` 须非空、对齐，指向 guesser 会话期间存活的 arena
  /// `TypeFunctionInstanceType`（由派发链 `infer_type_function_substitutions`
  /// 经 `get_type` 命中后传入）；仅读 `type_arguments`。单线程。
  pub unsafe fn infer_not_function(
    &mut self,
    instance: *const TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    // Safety: `instance` 按函数级契约非 null，指向归约会话所用 type arena 中存活且对齐的
    // `TypeFunctionInstanceType` 节点；这里只读其 `type_arguments` 切片的长度。
    LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 1 });

    // Safety: 同一 `instance` 存活前提成立，`(*instance).type_arguments` 重建共享引用有效；
    // 索引 `[0]` 为切片的边界检查访问——契约保证该实例恰有 1 个实参（上方 `LUAU_ASSERT!`），
    // 越界只会 panic 而非 UB，故非 null 前提仅由 `instance` 解引用这一处 unsafe 承担。
    let op_ty = unsafe { follow_type::follow((&(*instance).type_arguments)[0]) };
    let op_ty = if let Some(ty) = self.try_assign_operand_type(op_ty) {
      follow_type::follow(ty)
    } else {
      op_ty
    };

    TypeFunctionInferenceResult {
      operand_inference: alloc::vec![op_ty],
      // Safety: `self.builtins` 是 guesser 构造期接线的 `*mut BuiltinTypes`，指向比本 guesser
      // 长寿的内置类型单例、全程存活非空；此处只读取 Copy 的 `boolean_type`（`TypeId`），
      // 单线程归约内无并存可变借用。
      function_result_inference: self.builtins.get_mut().boolean_type,
    }
  }
}

impl TypeFunctionReductionGuesser {
  /// # Safety
  /// `instance` 须非空、对齐，指向 guesser 会话期间存活的 arena
  /// `TypeFunctionInstanceType`（由派发链 `infer_type_function_substitutions`
  /// 经 `get_type` 命中后传入）；仅读 `function.name` 与 `type_arguments`。单线程。
  pub unsafe fn infer_numeric_binop_function(
    &mut self,
    instance: *const TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    // Safety: `instance` 由派发链 `infer_type_function_substitutions` 在
    // `get_type::get::<TypeFunctionInstanceType>` 命中并经非空守卫后传入，指向类型
    // arena bump 分块中存活节点；此处只读其 `type_arguments` Vec 的长度做前置断言。
    LUAU_ASSERT!(unsafe { (*instance).type_arguments.len() == 2 });

    let builtins = self.builtins.get();
    TypeFunctionInferenceResult {
      operand_inference: alloc::vec![builtins.number_type, builtins.number_type],
      function_result_inference: builtins.number_type,
    }
  }
}

impl TypeFunctionReductionGuesser {
  /// # Safety
  /// `instance` 须非空、对齐，指向 guesser 会话期间存活的 arena
  /// `TypeFunctionInstanceType`（唯一调用方经 `step()` 的 `get_type` 命中后传入，
  /// 同 cpp `TypeFunctionReductionGuesser.cpp` 的 NotNull 前提）；仅读
  /// `type_arguments`，写路径不触及本实例节点。单线程。
  pub unsafe fn infer_unary_minus_function(
    &mut self,
    instance: *const TypeFunctionInstanceType,
  ) -> TypeFunctionInferenceResult {
    unsafe {
      LUAU_ASSERT!((*instance).type_arguments.len() == 1);
      let mut op_ty = follow_type::follow((&(*instance).type_arguments)[0]);
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
    // Safety: `normalizer` 由检查会话构造 guesser 时接线（构造函数入参，指向
    // 会话持有的 Normalizer），覆盖 guesser 生命周期且期内地址稳定，仅此解引用。
    unsafe { (*normalizer).try_normalize(ty) }
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
      // &T 隐式转 *const，命中清单外旧签名
      unsafe { self.infer_type_function_substitutions(t, tf) };
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
