//! `TypeId ConstraintSolver::instantiateFunctionType(TypeId functionTypeId,
//!   const std::vector<TypeId>& type_arguments, const std::vector<TypePackId>& typePackArguments,
//!   NotNull<Scope> scope, const Location& location)`
//! (`Analysis/src/ConstraintSolver.cpp:3193-3267`, hand-ported faithfully).

use std::ptr::eq;

use ulua_ast::records::location::Location;
use ulua_common::{macros::luau_assert::LUAU_ASSERT, records::dense_hash_map::DenseHashMap};

use crate::{
  enums::polarity::Polarity,
  functions::{
    follow_type, fresh_type::fresh_type, get_mutable_type, get_type,
    shallow_clone_clone::shallow_clone,
  },
  records::{
    clone_state::CloneState, constraint_solver::ConstraintSolver, function_type::FunctionType,
    replacer::Replacer, scope::Scope,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};
impl ConstraintSolver {
  pub fn instantiate_function_type(
    &mut self,
    function_type_id: TypeId,
    type_arguments: &[TypeId],
    type_pack_arguments: &[TypePackId],
    scope: *mut Scope,
    _location: &Location,
  ) -> TypeId {
    let function_type_id = follow_type::follow(function_type_id);

    // no work to be done if we're not instantiating with anything
    if type_arguments.is_empty() && type_pack_arguments.is_empty() {
      return function_type_id;
    }

    let Some(ft) = get_type::get::<FunctionType>(function_type_id) else {
      return function_type_id;
    };

    let mut replacements: DenseHashMap<TypeId, TypeId> = DenseHashMap::default();
    let generics = &ft.generics;

    for (&type_argument, &generic) in type_arguments.iter().zip(generics.iter()) {
      *replacements.get_or_insert(generic) = type_argument;
    }

    // 类型实参不足的泛型形参以 fresh type 补齐（zip 已消费 min(len) 个）
    for &generic in &generics[type_arguments.len().min(generics.len())..] {
      let fresh = fresh_type(
        // Safety: self.arena.as_ptr() 在 ConstraintSolver 构造时从 (*normalizer).arena 接线
        // ——非空、对齐、指向会话存活 TypeArena；单线程串行下此刻无并存借用，
        // &mut 重建仅覆盖 fresh_type 调用语句（其内只向稳定 bump 块追加 FreeType）。
        { self.arena.get_mut() },
        // Safety: self.builtin_types.as_ptr() 同一构造点接线，指向检查期只读的 BuiltinTypes；
        // 共享借用只读 Copy 句柄，与 arena 写入互不重叠。
        { self.builtin_types.get() },
        scope,
        Polarity::Mixed,
      );
      *replacements.get_or_insert(generic) = fresh;
    }

    let mut replacement_packs: DenseHashMap<TypePackId, TypePackId> = DenseHashMap::default();
    let generic_packs = &ft.generic_packs;
    for (&type_pack_argument, &generic_pack) in type_pack_arguments.iter().zip(generic_packs.iter())
    {
      *replacement_packs.get_or_insert(generic_pack) = type_pack_argument;
    }

    let mut r = Replacer::new(
      self.arena,
      &mut replacements as *mut DenseHashMap<TypeId, TypeId>,
      &mut replacement_packs as *mut DenseHashMap<TypePackId, TypePackId>,
    );

    let mut cs = CloneState {
      builtin_types: self.builtin_types,
      seen_types: DenseHashMap::default(),
      seen_type_packs: DenseHashMap::default(),
    };

    // We clone persistent types here to enable instantiation for generic
    // builtins like `table.find`; otherwise, the lines after would
    // immediately corrupt the definitions of the original function.
    // Safety: shallow_clone 的前置条件——cloned 源 function_type_id 是 follow 后的
    // arena 存活节点句柄（get_type::get::<FunctionType> 已命中）；`self.arena.get_mut()`
    // 由构造期接线的非空指针重建，此刻单线程串行下无并存借用（replacements/cs 均
    // 不触碰 arena）；cs 是本帧独占局部，其 seen 表随克隆写入自有内存。
    let cloned_function_type_id = unsafe {
      shallow_clone(
        function_type_id,
        self.arena.get_mut(),
        &mut cs,
        /* clonePersistentTypes */ true,
      )
    };
    // C++ 在 assert 后直接解引用 ft2（shallow_clone 一个 FunctionType 必然仍是
    // FunctionType），此处同契约，用 unwrap 表达必命中。
    let ft2 = get_mutable_type::get_mutable::<FunctionType>(cloned_function_type_id)
      .expect("shallow_clone 保留了 FunctionType 变体");
    LUAU_ASSERT!(!eq(ft as *const FunctionType, ft2 as *const FunctionType));

    // We instantiate all generics, replacing any with free types.
    ft2.generics.clear();

    // However, we only instantiate as many type pack arguments as are given.
    if !ft2.generic_packs.is_empty() && type_pack_arguments.len() < ft2.generic_packs.len() {
      ft2.generic_packs.drain(0..type_pack_arguments.len());
    } else {
      ft2.generic_packs.clear();
    }

    match r.substitute_type_id(cloned_function_type_id) {
      Some(result) => result,
      // Safety: builtin_types 构造期接线（同上），非空、比 solver 长寿、检查期只读，
      // 此处仅取 error_type 的 Copy 句柄。
      None => self.builtin_types.get().error_type,
    }
  }
}
