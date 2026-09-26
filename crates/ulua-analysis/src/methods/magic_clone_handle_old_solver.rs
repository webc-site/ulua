// ({+ +}) -> {+ +}
// <T: {}>(T) -> T
use alloc::{sync::Arc, vec::Vec};

use ulua_ast::records::ast_expr_call::AstExprCall;
use ulua_common::records::dense_hash_map::DenseHashMap;

use crate::{
  functions::{
    arc_as_mut::arc_as_mut, begin_type::begin_intersection_type,
    extend_type_pack::extend_type_pack, follow_type, get_type, shallow_clone_clone::shallow_clone,
  },
  records::{
    clone_state::CloneState, count_mismatch::CountMismatch, intersection_type::IntersectionType,
    scope::Scope, table_type::TableType, type_checker::TypeChecker, type_pack::TypePack,
    with_predicate::WithPredicate,
  },
  type_aliases::{type_error_data::TypeErrorData, type_pack_id::TypePackId},
};
pub fn magic_clone_handle_old_solver(
  typechecker: &mut TypeChecker,
  _scope: &Arc<Scope>,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let param_pack = with_predicate.r#type;

  let builtin_types = typechecker.builtin_types;
  let module = typechecker.current_module.as_ref()?;
  // Safety: `arc_as_mut(module)` 返回 Arc<Module> 内嵌 Module 的裸地址（Arc 由
  // `current_module` 持有、`module` 借用覆盖全函数，指针非空对齐且存活）。取
  // `&mut internal_types` 是 C++ `asMutable(module)->internalTypes` 的等价惯用法：
  // 全函数单线程串行，此 arena 无第二处可变句柄并存。
  let arena = unsafe { &mut (*(arc_as_mut(module))).internal_types };

  // in the old solver, nonstrict in particular is really bad about inferring `...any` for things that are definitely present
  // and the only real way for us to deal with this is to just be more permissive here
  // Safety: 满足 extend_type_pack 的入参契约——arena 为上方独占写句柄（其内部
  // add_type 追加只前移 bump 指针，不移动既有块）；builtin_types 是 TypeChecker
  // 构造接线的 NotNull 会话指针；param_pack 为 with_predicate 携带的存活
  // TypePackId，沿链只读；单线程串行下调用期间无其它 arena 借用。
  let extended = unsafe { extend_type_pack(arena, builtin_types, param_pack, 1, Vec::new()) };
  let param_types = extended.head;
  if param_types.is_empty() || expr.args.size == 0 {
    typechecker.report_error_location_type_error_data(
      &expr.arg_location,
      TypeErrorData::CountMismatch(CountMismatch {
        expected: 1,
        actual: 0,
        ..Default::default()
      }),
    );
    return None;
  }

  let input_type = follow_type::follow(param_types[0]);

  let table_ty = get_type::get::<TableType>(input_type);
  let intersection_ty = get_type::get::<IntersectionType>(input_type);
  if table_ty.is_none() && intersection_ty.is_none() {
    return None;
  }

  if let Some(intersection_ty) = intersection_ty {
    // C++ `for (auto ty : intersectionTy)` — IntersectionTypeIterator 防环展平
    // 并 follow Bound,裸遍历 parts 会漏掉 Bound 穿透与嵌套 intersection。
    for ty in begin_intersection_type(intersection_ty) {
      get_type::get::<TableType>(ty)?;
    }
  }

  let mut clone_state = CloneState {
    builtin_types,
    seen_types: DenseHashMap::default(),
    seen_type_packs: DenseHashMap::default(),
  };
  // Safety: 满足 shallow_clone 的入参契约——input_type 是 follow 后的存活 arena
  // TypeId（源自 param_types[0]）；dest 为本函数独占的 module arena 写句柄，
  // clone_state 的 seen 映射为本地新建（null 哨兵即 C++ 空 map 直译，首次
  // insert 前只判空不写），调用期间单线程、无并存别名。
  let result_type = unsafe {
    shallow_clone(
      input_type,
      arena,
      &mut clone_state,
      /* clonePersistentTypes */ false,
    )
  };

  let cloned_type_pack = arena.add_type_pack_t(TypePack::single(result_type));
  Some(WithPredicate::with_predicate_t(cloned_type_pack))
}
