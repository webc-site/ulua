use alloc::vec::Vec;

use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_local::AstExprLocal, node_handle::OptNode},
  rtti::ast_node_try_as,
};

use crate::{
  functions::{
    begin_type::begin_union_type, finite::finite, follow_type, get_type,
    is_table_intersection::is_table_intersection, is_table_union::is_table_union,
    shared_mut::shared_mut, size_type_pack::size,
  },
  records::{
    any_type::AnyType,
    arena_handle::alias_ref,
    binding::Binding,
    cannot_extend_table::{self, CannotExtendTable},
    generic_error::GenericError,
    metatable_type::MetatableType,
    symbol::Symbol,
    table_type::TableType,
    type_checker::TypeChecker,
    type_error::TypeError,
    type_pack::TypePack,
    union_type::UnionType,
    with_predicate::WithPredicate,
  },
  type_aliases::{
    error_type::ErrorType, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId, type_pack_id::TypePackId,
  },
};
pub fn magic_set_metatable_handle_old_solver(
  typechecker: &mut TypeChecker,
  scope: &ScopePtr,
  expr: &AstExprCall,
  with_predicate: WithPredicate<TypePackId>,
) -> Option<WithPredicate<TypePackId>> {
  let param_pack = with_predicate.r#type;

  // param_pack 指向类型 pack arena（bump 块，地址不移动）中存活至本次类型
  // 检查结束的 TypePackVar；size 为 Option<&TxnLog> 形参，传 None 即等价 C++
  // 默认实参 TxnLog* log = nullptr，走全局 follow 分支、不解引用空指针。
  let param_count = size(param_pack, None);
  // finite 已 Option<&TxnLog> 化：None 即 C++ 默认 nullptr 形参，只读遍历 pack 链；
  // 保持原 `size < 2 && finite` 短路顺序，finite 仅在 param_count < 2 时求值。
  if param_count < 2 && finite(param_pack, None) {
    return None;
  }

  let module = typechecker.current_module.as_ref()?.clone();
  // Safety: shared_mut 从刚 clone 出的 ModulePtr(Arc<Module>) 取共享堆分配的写句柄，
  // 与 C++ 直接经 shared_ptr<Module> 改写 *internalTypes 的语义一致；分析单线程串行，
  // 本借用存续期间无第二个 Module 内部视图存活，&mut 重建无别名冲突。
  let arena = { &mut (shared_mut(&module)).internal_types };

  let expected_args = typechecker.un_type_pack(scope, param_pack, 2, &expr.base.base.location);
  let target = follow_type::follow(expected_args[0]);
  let mt = follow_type::follow(expected_args[1]);

  typechecker.tablify(target);
  typechecker.tablify(mt);

  if let Some(tab_ref) = get_type::get::<TableType>(target) {
    if alias_ref(target).persistent {
      typechecker.report_error_type_error(&TypeError::type_error_location_type_error_data(
        expr.base.base.location,
        TypeErrorData::CannotExtendTable(CannotExtendTable {
          table_type: target,
          context: cannot_extend_table::Context::Metatable,
          prop: String::new(),
        }),
      ));
    } else {
      let mt_ttv = get_type::get::<TableType>(mt);
      let mut mtv = MetatableType {
        table: target,
        metatable: mt,
        synthetic_name: None,
      };

      if (tab_ref.name.is_some() || tab_ref.synthetic_name.is_some())
        && mt_ttv.is_some_and(|mt_ttv| mt_ttv.name.is_some() || mt_ttv.synthetic_name.is_some())
      {
        let table_name = tab_ref
          .name
          .as_ref()
          .or(tab_ref.synthetic_name.as_ref())
          .expect("上方复合判据蕴含 name/synthetic_name 至少一枚 Some");
        let metatable_name = mt_ttv
          .and_then(|mt_ttv| mt_ttv.name.as_ref().or(mt_ttv.synthetic_name.as_ref()))
          .expect("上方 is_some_and 判据蕴含 mt_ttv Some 且双名至少一枚 Some");

        if table_name == metatable_name {
          mtv.synthetic_name = Some(table_name.clone());
        }
      }

      let mt_ty = arena.add_type(mtv);

      if expr.args.is_empty() {
        return None;
      }

      if !expr.self_ {
        // `args[0]` 槽位仍是裸指针：经句柄门面 `OptNode::from_ptr` 折叠可空性，
        // 判型下转走生命周期正确的 [`ast_node_try_as`]（C++ `->as<AstExprLocal>()`
        // 同语义：不命中/为 null 一并折叠为 None 不解引用），借用半径由局部句柄
        // 供给，不锻造假 'static。
        let target_node = OptNode::from_ptr(expr.args[0]);
        if let Some(target_local) = target_node
          .get()
          .and_then(|e| ast_node_try_as::<AstExprLocal>(e))
        {
          let scope_ptr = shared_mut(scope);
          {
            // Safety: scope_ptr 由调用方借出的 &ScopePtr(Arc<Scope>) 经 shared_mut 派生，
            // Arc 在本借用期内存活且单线程独占写（crate 惯用法），bindings 插入无别名；
            // target_local 已由 try_as 命中，读取 .local 字段类型正确、节点存活。
            scope_ptr.bindings.insert(
              Symbol::from_local(target_local.local.as_ptr()),
              Binding {
                type_id: mt_ty,
                location: expr.base.base.location,
                deprecated: false,
                deprecated_suggestion: String::new(),
                documentation_symbol: None,
              },
            );
          }
        }
      }

      return Some(WithPredicate::with_predicate_t(
        arena.add_type_pack_t(TypePack::single(mt_ty)),
      ));
    }
  } else if get_type::get::<AnyType>(target).is_some()
    || get_type::get::<ErrorType>(target).is_some()
    || is_table_intersection(target)
  {
  } else if is_table_union(target) {
    // is_table_union 已判定为 UnionType，get 必命中；None（不可达）落回尾部兜底。
    if let Some(ut_ref) = get_type::get::<UnionType>(target) {
      // C++ `for (TypeId ty : ut)`——UnionTypeIterator 展平嵌套 union 并
      // follow,裸遍历 options 会漏掉嵌套成员。
      let mut result_parts: Vec<TypeId> = Vec::new();
      for ty in begin_union_type(ut_ref) {
        result_parts.push(arena.add_type(MetatableType {
          table: ty,
          metatable: mt,
          synthetic_name: None,
        }));
      }

      let result_union = arena.add_type(UnionType {
        options: result_parts,
      });
      return Some(WithPredicate::with_predicate_t(
        arena.add_type_pack_t(TypePack::single(result_union)),
      ));
    }
  } else {
    typechecker.report_error_type_error(&TypeError::type_error_location_type_error_data(
      expr.base.base.location,
      TypeErrorData::GenericError(GenericError::new(
        "setmetatable should take a table".to_string(),
      )),
    ));
  }

  Some(WithPredicate::with_predicate_t(
    arena.add_type_pack_t(TypePack::single(target)),
  ))
}
