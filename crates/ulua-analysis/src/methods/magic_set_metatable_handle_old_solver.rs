use alloc::vec::Vec;
use core::ptr::null_mut;

use ulua_ast::{
  records::{ast_expr_call::AstExprCall, ast_expr_local::AstExprLocal},
  rtti::ast_node_try_as_ptr,
};

use crate::{
  functions::{
    arc_as_mut::arc_as_mut, begin_type::begin_union_type, finite::finite, follow_type, get_type,
    is_table_intersection::is_table_intersection, is_table_union::is_table_union,
    size_type_pack::size,
  },
  records::{
    any_type::AnyType,
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

  // Safety: param_pack 指向类型 pack arena（bump 块，地址不移动）中存活至本次类型
  // 检查结束的 TypePackVar；size 内部对 log 显式判空走全局 follow 分支，传 null_mut()
  // 即等价 C++ 默认实参 TxnLog* log = nullptr，不会解引用该空指针。
  let param_count = size(param_pack, None);
  // Safety: 同上——finite 只读遍历 pack 链且 null log 命中其 is_null 分支；保持原
  // `size < 2 && finite` 短路顺序，finite 仅在 param_count < 2 时求值。
  if param_count < 2 && unsafe { finite(param_pack, null_mut()) } {
    return None;
  }

  let module = typechecker.current_module.as_ref()?.clone();
  // Safety: arc_as_mut 从刚 clone 出的 ModulePtr(Arc<Module>) 取共享堆分配的写句柄，
  // 与 C++ 直接经 shared_ptr<Module> 改写 *internalTypes 的语义一致；分析单线程串行，
  // 本借用存续期间无第二个 Module 内部视图存活，&mut 重建无别名冲突。
  let arena = unsafe { &mut (*(arc_as_mut(&module))).internal_types };

  let expected_args = typechecker.un_type_pack(scope, param_pack, 2, &expr.base.base.location);
  let target = follow_type::follow(expected_args[0]);
  let mt = follow_type::follow(expected_args[1]);

  typechecker.tablify(target);
  typechecker.tablify(mt);

  if let Some(tab_ref) = get_type::get::<TableType>(target) {
    // Safety: target 是 follow_type_id 结果，指向类型 arena（bump 块地址不移动）中
    // 存活至本次检查结束的 Type 节点；此处仅只读 persistent 标志，与 get_type_id
    // 命中判定的对象同址同源。
    if unsafe { (*target).persistent } {
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
        let target_expr = expr.args[0];
        // Safety: try_as_ptr 按节点 class_index 分派——不命中返回 None、命中即与
        // C++ `->as<AstExprLocal>()` 同址同型的存活节点只读借用，从不解引用未命中指针。
        if let Some(target_local) = unsafe { ast_node_try_as_ptr::<AstExprLocal>(target_expr) } {
          let scope_ptr = arc_as_mut(scope);
          unsafe {
            // Safety: scope_ptr 由调用方借出的 &ScopePtr(Arc<Scope>) 经 arc_as_mut 派生，
            // Arc 在本借用期内存活且单线程独占写（crate 惯用法），bindings 插入无别名；
            // target_local 已由 try_as_ptr 命中，读取 .local 字段类型正确、节点存活。
            (*scope_ptr).bindings.insert(
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
