use alloc::{boxed::Box, sync::Arc, vec::Vec};

use ulua_analysis::{
  enums::solver_mode::SolverMode,
  records::{
    arena_handle::Handle, builtin_types::BuiltinTypes, function_type::FunctionType,
    internal_error_reporter::InternalErrorReporter, intersection_type::IntersectionType,
    normalizer::Normalizer, overload_resolver::OverloadResolver, scope::Scope,
    type_arena::TypeArena, type_check_limits::TypeCheckLimits,
    type_function_runtime::TypeFunctionRuntime, unifier_shared_state::UnifierSharedState,
  },
  type_aliases::type_id::TypeId,
};
use ulua_ast::records::{ast_expr_constant_nil::AstExprConstantNil, location::Location};
use ulua_common::{fflag, records::dense_hash_set::DenseHashSet};

use crate::records::{fixture::Fixture, overload_resolver_fixture::OverloadResolverFixture};
impl OverloadResolverFixture {
  /// `arena` 裸指针的唯一解引用收口入口（cpp `fixture.arena->addType/
  /// addTypePack` 同形的独占写视图）。
  ///
  /// 返回借用生命周期与 `&self` 解耦（`'a` 独立，slots_mut 同法）：数据活在
  /// `arena_` Box 保活的堆块而非 fixture 句柄自身。
  ///
  /// # Safety
  /// `arena` 须指向本 fixture `arena_` Box 保活的 TypeArena 堆块（即构造完成后
  /// `Self::arena` 字段的值），且返回借用存续期内该 arena 无其他活动借用
  /// （测试单线程、fixture 方法串行独占写，逐字对应 cpp 版纪律）。
  #[inline]
  pub(crate) unsafe fn arena_view<'a>(&self) -> &'a mut TypeArena {
    // Safety: 转调即函数级 `# Safety` 契约本身。
    unsafe { &mut *self.arena }
  }

  pub fn new() -> Self {
    let mut base = Box::new(Fixture::default());
    let mut arena_ = Box::new(TypeArena::default());

    let mut builtin_types = Box::new(BuiltinTypes::new());
    let builtin_types_ptr = builtin_types.as_mut() as *mut BuiltinTypes;
    base.builtin_types = builtin_types_ptr;

    let mut ice_reporter = Box::new(InternalErrorReporter::default());
    let ice_reporter_ptr = ice_reporter.as_mut() as *mut InternalErrorReporter;

    let mut limits = Box::new(TypeCheckLimits::default());
    let limits_ptr = limits.as_mut() as *mut TypeCheckLimits;

    let base_ice_ptr = &mut base.ice as *mut InternalErrorReporter;
    let mut shared_state = Box::new(UnifierSharedState::new(base_ice_ptr));
    let shared_state_ptr = shared_state.as_mut() as *mut UnifierSharedState;

    let solver_mode = if !fflag::DebugLuauForceOldSolver.get() {
      SolverMode::New
    } else {
      SolverMode::Old
    };
    let mut normalizer = Box::new(Normalizer::new(
      Some(Handle::from_mut(arena_.as_mut())),
      Handle::from_mut(builtin_types.as_mut()),
      Handle::from_opt_ptr(shared_state_ptr),
      solver_mode,
      false,
    ));

    let mut root_scope = Box::new(Scope::scope_type_pack_id(builtin_types.empty_type_pack));

    let runtime_root_scope = Arc::new(Scope::scope_type_pack_id(builtin_types.empty_type_pack));
    let mut type_function_runtime = Box::new(TypeFunctionRuntime::new(
      ice_reporter.as_ref(),
      limits.as_ref(),
      runtime_root_scope,
    ));

    let call_location = Location::default();
    // Safety: 各指针实参均为本帧 Box 堆块地址（builtin_types/ice_reporter/limits
    // 判空于 as_mut，normalizer/type_function_runtime/root_scope 同），存活至
    // Self 构造完成并搬入后由对应 Box 字段长期保活；OverloadResolver::new 依
    // cpp 契约仅存储不解引用，故本调用点参数全部合法。
    let resolver = unsafe {
      OverloadResolver::new(
        Handle::from_mut(builtin_types.as_mut()),
        Handle::from_mut(arena_.as_mut()),
        normalizer.as_mut() as *mut Normalizer,
        type_function_runtime.as_mut() as *mut TypeFunctionRuntime,
        root_scope.as_mut() as *mut Scope,
        ice_reporter_ptr,
        limits_ptr,
        call_location,
      )
    };

    let mut k_empty_set = Box::new(DenseHashSet::default());
    let empty_set = k_empty_set.as_mut() as *mut DenseHashSet<TypeId>;

    let k_dummy_location = Location::default();
    let k_dummy_expr = AstExprConstantNil::new(k_dummy_location);

    let number_type = builtin_types.number_type;
    let string_type = builtin_types.string_type;

    // arena 写全部走 `arena_.as_mut()` 独占借用（Box 尚未搬入 Self，借用即
    // 存活证明），`arena` 字段仅存其堆地址供 fixture 方法期复用。
    let arena = arena_.as_mut() as *mut TypeArena;
    let number_to_number = add_function_type(arena_.as_mut(), &[number_type], &[number_type]);
    let number_number_to_number =
      add_function_type(arena_.as_mut(), &[number_type, number_type], &[number_type]);
    let number_to_string = add_function_type(arena_.as_mut(), &[number_type], &[string_type]);
    let string_to_string = add_function_type(arena_.as_mut(), &[string_type], &[string_type]);

    let number_to_number_and_string_to_string = arena_.as_mut().add_type(IntersectionType {
      parts: alloc::vec![number_to_number, string_to_string],
    });
    let number_to_number_and_number_number_to_number = arena_.as_mut().add_type(IntersectionType {
      parts: alloc::vec![number_to_number, number_number_to_number],
    });

    Self {
      arena_,
      arena,
      builtin_types,
      shared_state,
      normalizer,
      ice_reporter,
      limits,
      type_function_runtime,
      root_scope,
      call_location,
      resolver,
      k_empty_set,
      empty_set,
      k_dummy_location,
      k_dummy_expr,
      k_empty_exprs: Vec::new(),
      number_to_number,
      number_number_to_number,
      number_to_string,
      string_to_string,
      number_to_number_and_string_to_string,
      number_to_number_and_number_number_to_number,
      base,
    }
  }
}

impl Default for OverloadResolverFixture {
  fn default() -> Self {
    Self::new()
  }
}

/// 对应 C++ `OverloadResolverFixture::fn`（tests/OverloadResolver.test.cpp:63-66）：
/// 在 arena 上顺序追加 `(args) -> rets` 的函数类型（三笔独占写）。
pub(crate) fn add_function_type(arena: &mut TypeArena, args: &[TypeId], rets: &[TypeId]) -> TypeId {
  let arg_pack = arena.add_type_pack_initializer_list_type_id(args);
  let ret_pack = arena.add_type_pack_initializer_list_type_id(rets);
  arena.add_type(FunctionType::function_type_new(
    arg_pack, ret_pack, None, false,
  ))
}
