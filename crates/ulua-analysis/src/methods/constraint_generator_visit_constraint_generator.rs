use alloc::{string::String, sync::Arc, vec::Vec};
use core::{
  iter::repeat_with,
  mem::take,
  ptr::{eq, from_mut, from_ref},
};

use ulua_ast::{
  enums::ast_table_access::AstTableAccess,
  functions::optional_node::{slot_opt, slot_ref},
  records::{
    ast_attr::AstAttrType, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_error::AstExprError, ast_expr_global::AstExprGlobal,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
    ast_expr_table::AstExprTable, ast_node::AstNode, ast_stat::AstStat,
    ast_stat_assign::AstStatAssign, ast_stat_block::AstStatBlock, ast_stat_break::AstStatBreak,
    ast_stat_class::AstStatClass, ast_stat_compound_assign::AstStatCompoundAssign,
    ast_stat_continue::AstStatContinue, ast_stat_declare_extern_type::AstStatDeclareExternType,
    ast_stat_declare_function::AstStatDeclareFunction,
    ast_stat_declare_global::AstStatDeclareGlobal, ast_stat_error::AstStatError,
    ast_stat_expr::AstStatExpr, ast_stat_for::AstStatFor, ast_stat_for_in::AstStatForIn,
    ast_stat_function::AstStatFunction, ast_stat_if::AstStatIf, ast_stat_local::AstStatLocal,
    ast_stat_local_function::AstStatLocalFunction, ast_stat_repeat::AstStatRepeat,
    ast_stat_return::AstStatReturn, ast_stat_type_alias::AstStatTypeAlias,
    ast_stat_type_function::AstStatTypeFunction, ast_stat_while::AstStatWhile, location::Location,
  },
  rtti::{AstNodeClass, ast_node_is, ast_node_try_as},
};
use ulua_common::{dfint, fflag, functions::format::format, macros::luau_assert::LUAU_ASSERT};

use super::constraint_generator_prototype_type_definitions::make_binding;
use crate::{
  enums::{
    control_flow::ControlFlow, polarity::Polarity, table_state::TableState,
    type_context::TypeContext,
  },
  functions::{
    add_all_as_dependencies::add_all_as_dependencies,
    add_all_as_dependencies_and_chain_returns::add_all_as_dependencies_and_chain_returns,
    add_all_as_reverse_dependencies::add_all_as_reverse_dependencies,
    arc_as_mut::arc_as_mut,
    arena_slots::{
      add_deprecated_dependency, bind_type, block_owner_at, chain_deprecated_dependencies,
      propagate_deprecated_attribute,
    },
    ast_node_downcast::ast_node_downcast as stat_downcast,
    begin_type_pack::begin,
    checkpoint::checkpoint,
    does_call_error::does_call_error,
    end_type_pack::end_type_pack_id,
    flatten_type_pack::flatten_type_pack_id,
    follow_type,
    for_each_constraint::for_each_constraint,
    get_mutable_type, get_type,
    magic_names::{K_TYPEOF, is_reserved_type_alias_name},
    match_require::match_require,
    match_set_metatable::match_set_metatable,
    matches::matches,
    module_slots::{record_declared_global, record_expr_type, record_stat_type},
    occurs_check_type_utils::occurs_check_type_id_type_id,
    scope_slots::{
      bind, bind_local, bind_with_lvalue, check_function_signature, check_function_signature_in,
      check_pack_expr, inherit_assignments, inherit_refinements, lvalue_type,
      resolve_generic_defaults, scope_mut, set_lvalue_type,
    },
    type_pack_from_iterator::type_pack_from_iterator,
  },
  records::{
    arena_handle::{alias, alias_ref},
    blocked_type::BlockedType,
    checkpoint::Checkpoint,
    class_decl_record::ClassDeclRecord,
    constraint_generator::{ConstraintGenerator, InferredBinding},
    extern_type::ExternType,
    function_argument::FunctionArgument,
    function_definition::FunctionDefinition,
    function_type::{self, FunctionType},
    generalization_constraint::GeneralizationConstraint,
    generic_error::GenericError,
    in_conditional_context::InConditionalContext,
    intersection_type::IntersectionType,
    iterable_constraint::IterableConstraint,
    name_constraint::NameConstraint,
    occurs_check_failed::OccursCheckFailed,
    pack_subtype_constraint::PackSubtypeConstraint,
    property_type::Property,
    push_function_type_constraint::PushFunctionTypeConstraint,
    recursion_counter::RecursionCounter,
    reduce_constraint::ReduceConstraint,
    reserved_identifier::ReservedIdentifier,
    scope::Scope,
    scope_registry::intern_scope,
    subtype_constraint::SubtypeConstraint,
    symbol::Symbol,
    table_indexer::TableIndexer,
    table_type::TableType,
    type_fun::TypeFun,
    type_function::TypeFunction,
    type_ids::TypeIds,
    unknown_symbol::{Context, UnknownSymbol},
    unpack_constraint::UnpackConstraint,
  },
  type_aliases::{
    constraint_v::ConstraintV, def_id_def::DefId, name_type::Name, props_type::Props,
    refinement_id_refinement::RefinementId, scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId,
  },
};

impl ConstraintGenerator {
  /// 语句 RTTI 分发（cpp `visit(const ScopePtr&, AstStat*)`）。
  ///
  /// `stat` 为分析期存活、由 arena 持有的语句节点共享借用（cpp 裸指针形参的
  /// Rust 对应）；具体子节点字段仍是裸指针，进入各 `visit_stat_*` 后按其自身
  /// 契约处理。
  pub fn visit_stat(&mut self, scope: &ScopePtr, stat: &AstStat) -> ControlFlow {
    // SAFETY: 计数指针派生自 `&mut self.recursion_count`，RAII 守卫在本作用域
    // 结束时归还计数（cpp `RecursionCounter counter{&recursionCount}` 同构）。
    let _counter = RecursionCounter::recursion_counter_i32(&mut self.recursion_count);

    // cpp RecursionLimiter 的超限路径由下方显式判断替代
    // （report_code_too_complex 而非抛 RecursionLimitException）。
    if self.recursion_count >= dfint::LuauConstraintGeneratorRecursionLimit.get() {
      self.report_code_too_complex(stat.base.location);
      return ControlFlow::None;
    }

    let node: &AstNode = &stat.base;

    // match 臂的类索引与 `ast_node_try_as` 的判定完全同一（cpp `as<T>()` 命中后
    // 直接 static_cast 亦假定成功），故下转必然成功，expect 为逻辑不可达分支；
    // 家族类索引互斥由 rtti 的 rtti_indices_unique 测试保证，臂序无关语义。
    match node.class_index {
      AstStatBlock::CLASS_INDEX => {
        self.visit_stat_block(scope, stat_downcast::<AstStatBlock>(node))
      }
      AstStatIf::CLASS_INDEX => self.visit_stat_if(scope, stat_downcast::<AstStatIf>(node)),
      AstStatWhile::CLASS_INDEX => {
        self.visit_stat_while(scope, stat_downcast::<AstStatWhile>(node))
      }
      AstStatRepeat::CLASS_INDEX => {
        self.visit_stat_repeat(scope, stat_downcast::<AstStatRepeat>(node))
      }
      AstStatBreak::CLASS_INDEX => ControlFlow::Breaks,
      AstStatContinue::CLASS_INDEX => ControlFlow::Continues,
      AstStatReturn::CLASS_INDEX => {
        self.visit_stat_return(scope.clone(), stat_downcast::<AstStatReturn>(node))
      }
      AstStatExpr::CLASS_INDEX => self.visit_stat_expr(scope, stat_downcast::<AstStatExpr>(node)),
      AstStatLocal::CLASS_INDEX => {
        self.visit_stat_local(scope, stat_downcast::<AstStatLocal>(node))
      }
      AstStatFor::CLASS_INDEX => self.visit_stat_for(scope, stat_downcast::<AstStatFor>(node)),
      AstStatForIn::CLASS_INDEX => {
        self.visit_stat_for_in(scope, stat_downcast::<AstStatForIn>(node))
      }
      AstStatAssign::CLASS_INDEX => {
        self.visit_stat_assign(scope, stat_downcast::<AstStatAssign>(node))
      }
      AstStatCompoundAssign::CLASS_INDEX => {
        self.visit_stat_compound_assign(scope, stat_downcast::<AstStatCompoundAssign>(node))
      }
      AstStatFunction::CLASS_INDEX => {
        self.visit_stat_function(scope, stat_downcast::<AstStatFunction>(node))
      }
      AstStatLocalFunction::CLASS_INDEX => {
        self.visit_stat_local_function(scope, stat_downcast::<AstStatLocalFunction>(node))
      }
      AstStatTypeAlias::CLASS_INDEX => {
        self.visit_stat_type_alias(scope, stat_downcast::<AstStatTypeAlias>(node))
      }
      AstStatTypeFunction::CLASS_INDEX => {
        self.visit_stat_type_function(scope, stat_downcast::<AstStatTypeFunction>(node))
      }
      AstStatDeclareGlobal::CLASS_INDEX => {
        self.visit_stat_declare_global(scope, stat_downcast::<AstStatDeclareGlobal>(node))
      }
      AstStatDeclareFunction::CLASS_INDEX => {
        self.visit_stat_declare_function(scope, stat_downcast::<AstStatDeclareFunction>(node))
      }
      AstStatDeclareExternType::CLASS_INDEX => {
        self.visit_stat_declare_extern_type(scope, stat_downcast::<AstStatDeclareExternType>(node))
      }
      AstStatClass::CLASS_INDEX => {
        LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());
        self.visit_stat_class(scope, stat_downcast::<AstStatClass>(node))
      }
      AstStatError::CLASS_INDEX => {
        self.visit_stat_error(scope, stat_downcast::<AstStatError>(node))
      }
      _ => {
        LUAU_ASSERT!(false, "Internal error: Unknown AstStat type");
        ControlFlow::None
      }
    }
  }

  /// 表达式语句（cpp `visit` 分发中 `AstStatExpr` 分支的内联体）。
  pub fn visit_stat_expr(&mut self, scope: &ScopePtr, stat: &AstStatExpr) -> ControlFlow {
    // §2：stat.expr 已句柄化（node_handle::Node），`.get()` 即安全只读视图，
    // 原 `alias_ref` 裸指针门面与手写契约注释一并消失。
    let expr = stat.expr.get();
    check_pack_expr(self, scope, expr, &Vec::new(), true);
    // 判型+下转走安全的精确 RTTI 门面（cpp `e->expr->as<AstExprCall>()`）。
    if ast_node_try_as::<AstExprCall>(&expr.base).is_some_and(does_call_error) {
      return ControlFlow::Throws;
    }
    ControlFlow::None
  }

  /// cpp `visit(const ScopePtr&, AstStatLocal*)`：`stat_local` 为分发层经 RTTI
  /// 校验后传入的节点共享借用，本方法只读取其字段（vars/values）。
  pub fn visit_stat_local(&mut self, scope: &ScopePtr, stat_local: &AstStatLocal) -> ControlFlow {
    let scope_raw = arc_as_mut(scope);

    let mut annotated_types = Vec::with_capacity(stat_local.vars.size);
    let mut has_annotation = false;

    let mut expected_types = Vec::with_capacity(stat_local.vars.size);

    let mut assignees = Vec::with_capacity(stat_local.vars.size);

    let mut first_value_type = None;

    let unknown_type = self.builtin_types.get().unknown_type;

    for local in stat_local.vars.iter_nodes() {
      // 符号/dfg 键要的是节点身份：由同一存活借用还原地址（cpp 的 AstLocal* 键）。
      let local_ptr = from_ref(local).cast_mut();
      let location = local.location;

      // arena 是 Handle 收口的构造期句柄，add_type 本身即安全方法。
      let assignee = self.arena.get_mut().add_type(BlockedType::default());
      self.local_types.try_insert(assignee, TypeIds::new());
      assignees.push(assignee);

      first_value_type.get_or_insert(assignee);

      // §2：可空标注槽位以 Option 判空（cpp `if (local->annotation)`）。
      if slot_opt(local.annotation).is_some() {
        has_annotation = true;
        let annotation_ty = self.resolve_type(
          scope_raw,
          local.annotation,
          false,
          false,
          Polarity::Positive,
        );
        annotated_types.push(annotation_ty);
        expected_types.push(Some(annotation_ty));
        bind(
          scope,
          Symbol::from_local(local_ptr),
          make_binding(annotation_ty, location),
        );
      } else {
        annotated_types.push(unknown_type);
        expected_types.push(None);
        bind(
          scope,
          Symbol::from_local(local_ptr),
          make_binding(unknown_type, location),
        );

        let mut types = TypeIds::new();
        types.insert_type_id(assignee);
        self.inferred_bindings.try_insert(
          Symbol::from_local(local_ptr),
          InferredBinding {
            scope: scope_raw,
            location,
            types,
          },
        );
      }

      let def = self.dfg_ref().get_def_local(from_ref(local));
      set_lvalue_type(scope, def, assignee);
    }

    let start = checkpoint(self);
    let rvalue_pack = self
      .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(
        scope,
        stat_local.values,
        &expected_types,
      )
      .tp;
    let end = checkpoint(self);

    let mut deferred_types = Vec::new();
    let (head, tail) = flatten_type_pack_id(rvalue_pack);
    let mut fresh_blocked_types: Vec<&'static mut BlockedType> = Vec::new();

    for (i, local_ref) in stat_local.vars.iter_nodes().enumerate() {
      LUAU_ASSERT!(get_mutable_type::get_mutable::<BlockedType>(assignees[i]).is_some());
      let local_domain = self
        .local_types
        .find_mut(&assignees[i])
        .expect("local assignee domain should exist");

      if slot_opt(local_ref.annotation).is_some() {
        local_domain.insert_type_id(annotated_types[i]);
        if i >= head.len() && tail.is_some() {
          deferred_types.push(annotated_types[i]);
        }
      } else if i < head.len() {
        local_domain.insert_type_id(head[i]);
      } else if tail.is_some() {
        // SAFETY: arena 同上（构造期非空句柄），此处为变长尾新分配 deferred 占位类型。
        let deferred = self.arena.get_mut().add_type(BlockedType::default());
        deferred_types.push(deferred);
        local_domain.insert_type_id(deferred);
        // deferred 刚由 add_type(BlockedType) 分配，必命中；对照 C++:1509
        fresh_blocked_types.push(get_mutable_type::get_mutable::<BlockedType>(deferred).unwrap());
      } else {
        // SAFETY: builtin_types 指向构造期布线的内建单例，此处只读取 nil_type。
        local_domain.insert_type_id({ self.builtin_types.get().nil_type });
      }
    }

    if has_annotation {
      // SAFETY: arena 为构造期非空句柄；annotated_types 是本函数局部 Vec，
      // 移入分配类型包后不再按引用使用。
      let annotated_pack = {
        self
          .arena
          .get_mut()
          .add_type_pack_vector_type_id_optional_type_pack_id(annotated_types, None)
      };
      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        stat_local.base.base.location,
        ConstraintV::PackSubtype(PackSubtypeConstraint {
          sub_pack: rvalue_pack,
          super_pack: annotated_pack,
          returns: false,
        }),
      );
    }

    if !deferred_types.is_empty() {
      LUAU_ASSERT!(tail.is_some());
      let uc = self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        stat_local.base.base.location,
        ConstraintV::Unpack(UnpackConstraint {
          result_pack: deferred_types,
          source_pack: tail.unwrap(),
        }),
      );

      if fflag::LuauConstraintGraph.get() {
        add_all_as_dependencies(start, end, self, uc);
      } else {
        // 退化依赖：区间内每条约束都须在 uc 之前派发（解引用收在门面内）。
        for_each_constraint(start, end, self, |run_before| {
          add_deprecated_dependency(uc, run_before);
        });
      }

      // 对照 C++:1546 `for (BlockedType* bt : freshBlockedTypes) bt->setOwner(uc);`
      for bt in fresh_blocked_types {
        bt.set_owner(uc);
      }
    }

    if stat_local.vars.size == 1
      && stat_local.values.size == 1
      && let Some(first_value_type) = first_value_type
      && eq(scope_raw, arc_as_mut(self.root()))
      && !has_annotation
    {
      // cpp 契约：vars/values 各 1 个；切片取元素后由 alias_ref 折成共享借用。
      let var_ref = alias_ref(stat_local.vars.as_slice()[0]);
      let value_ref = alias_ref(stat_local.values.as_slice()[0]);
      let should_name = ast_node_is::<AstExprTable>(&value_ref.base)
        || ast_node_try_as::<AstExprCall>(&value_ref.base).is_some_and(match_set_metatable);

      if should_name {
        let name = var_ref.name.as_str_or_empty().to_string();
        self.add_constraint_scope_ptr_location_constraint_v(
          scope,
          value_ref.base.location,
          ConstraintV::Name(NameConstraint {
            named_type: first_value_type,
            name,
            synthetic: true,
            type_parameters: Vec::new(),
            type_pack_parameters: Vec::new(),
          }),
        );
      }
    }

    if stat_local.values.size > 0 {
      // zip 在较短一侧结束，等价于 min(values.size, vars.size)
      for (value, local) in stat_local
        .values
        .iter_nodes()
        .zip(stat_local.vars.iter_nodes())
      {
        // 判型+下转为安全的精确 RTTI 门面（cpp `value->as<AstExprCall>()`）。
        let Some(call) = ast_node_try_as::<AstExprCall>(&value.base) else {
          continue;
        };
        let Some(require) = match_require(call) else {
          continue;
        };

        let module_name = self.module.as_ref().unwrap().name.clone();
        // §2：require 指向 fragment AST 的 `AstExpr` 节点（C++ 同契约）。
        let require_expr = alias_ref(require);
        let module_info = self
          .module_resolver_ref()
          .resolve_module_info(&module_name, require_expr);
        let Some(module_info) = module_info else {
          continue;
        };

        let required_module = self.module_resolver_ref().get_module(&module_info.name);
        let Some(required_module) = required_module else {
          continue;
        };

        // local 为 stat_local.vars 的 arena 存活节点（iter_nodes 只读遍历），
        // name 字段读取安全。
        let name = local.name.as_str_or_empty().to_string();
        let imports = scope_mut(scope);
        imports
          .imported_type_bindings
          .insert(name.clone(), required_module.exported_type_bindings.clone());
        imports
          .imported_modules
          .insert(name.clone(), module_info.name.clone());

        for cycle in &self.require_cycles {
          if cycle.path.is_empty() || cycle.path[0] != module_info.name {
            continue;
          }

          if let Some(bindings) = scope_mut(scope).imported_type_bindings.get_mut(&name) {
            for tf in bindings.values_mut() {
              *tf = TypeFun {
                type_params: Vec::new(),
                type_pack_params: Vec::new(),
                r#type: self.builtin_types.get().any_type,
                definition_location: None,
              };
            }
          }
        }
      }
    }

    ControlFlow::None
  }

  /// cpp `visit(const ScopePtr&, AstStatFor*)`：`for_` 为分发层经 RTTI 校验后
  /// 传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_for(&mut self, scope: &ScopePtr, for_: &AstStatFor) -> ControlFlow {
    // var 已句柄化为 Node（push_local alloc 恒非空由类型层承载），`.get()`
    // 直出共享引用，alias_ref 门面消失。
    let var = for_.var.get();

    let mut annotation_ty: TypeId = self.builtin_types.get().number_type;
    // §2：可空标注槽位以 Option 判空（cpp `if (var->annotation)`）。
    if slot_opt(var.annotation).is_some() {
      annotation_ty = self.resolve_type(
        arc_as_mut(scope),
        var.annotation,
        /* in_type_arguments */ false,
        /* replace_error_with_fresh */ false,
        Polarity::Positive,
      );
    }

    // C++ inferNumber lambda：形参收为共享借用，判空职责移到调用侧的 Option。
    let number_ty: TypeId = self.builtin_types.get().number_type;
    let infer_number = |this: &mut Self, expr: &AstExpr| {
      let t = this.check_expr(scope, expr).ty;
      this.add_constraint_scope_ptr_location_constraint_v(
        scope,
        expr.base.location,
        ConstraintV::Subtype(SubtypeConstraint {
          sub_type: t,
          super_type: number_ty,
        }),
      );
    };

    // from/to 已句柄化为非空 Node（parser 必建上下界），step 落可空 OptNode：
    // cpp `inferNumber` 对 nullptr 直接 return 的守卫随类型折叠进 `get()`，
    // 顺序仍为 from → to → step。
    infer_number(self, for_.from.get());
    infer_number(self, for_.to.get());
    if let Some(step) = for_.step.get() {
      infer_number(self, step);
    }

    // child_scope 读取 node 内嵌 AstNode 基类的 location 并登记（cpp
    // AstStatFor* 透传同构）；for_ 为存活借用、scope 为调用方持有的存活 Arc。
    let for_scope: ScopePtr = self.child_scope(&for_.base.base, scope);

    bind(
      &for_scope,
      Symbol::from_local(from_ref(var).cast_mut()),
      make_binding(annotation_ty, var.location),
    );

    let def = self.dfg_ref().get_def_local(from_ref(var));
    set_lvalue_type(&for_scope, def, annotation_ty);
    self.update_r_value_refinements_scope_ptr_def_id_type_id(&for_scope, def, annotation_ty);

    // §2：for_.body 已句柄化为非空 Node（文法保证 for 必有循环体）。
    self.visit_stat_block(&for_scope, for_.body.get());

    inherit_assignments(scope, &for_scope);

    ControlFlow::None
  }

  /// cpp `visit(const ScopePtr&, AstStatForIn*)`：`for_in` 为分发层经 RTTI 校验
  /// 后传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_for_in(&mut self, scope: &ScopePtr, for_in: &AstStatForIn) -> ControlFlow {
    // child_scope 读取 node 内嵌 AstNode 基类的 location 并登记（cpp
    // AstStatForIn* 透传同构）；for_in 为存活借用、scope 为调用方持有的存活 Arc。
    let loop_scope: ScopePtr = self.child_scope(&for_in.base.base, scope);

    let values = for_in.values;
    let iterator: TypePackId = self
      .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(scope, values, &Vec::new())
      .tp;

    let mut variable_types: Vec<TypeId> = Vec::with_capacity(for_in.vars.size);

    for var_ref in for_in.vars.iter_nodes() {
      // 符号/dfg 键要的是节点身份：由同一存活借用还原地址。
      let var_ptr = from_ref(var_ref).cast_mut();
      let loop_var: TypeId = self.arena.get_mut().add_type(BlockedType::default());
      variable_types.push(loop_var);

      if fflag::LuauPropagateTypeAnnotationsInForInLoops.get() {
        let def = self.dfg_ref().get_def_local(from_ref(var_ref));

        // §2：可空标注槽位以 Option 判空（cpp `if (var->annotation)`）。
        if slot_opt(var_ref.annotation).is_some() {
          let annotation_ty = self.resolve_type(
            arc_as_mut(&loop_scope),
            var_ref.annotation,
            /* in_type_arguments */ false,
            /* replace_error_with_fresh */ false,
            Polarity::Positive,
          );
          bind(
            &loop_scope,
            Symbol::from_local(var_ptr),
            make_binding(annotation_ty, var_ref.location),
          );
          self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            var_ref.location,
            ConstraintV::Subtype(SubtypeConstraint {
              sub_type: loop_var,
              super_type: annotation_ty,
            }),
          );
          set_lvalue_type(&loop_scope, def, annotation_ty);
        } else {
          bind(
            &loop_scope,
            Symbol::from_local(var_ptr),
            make_binding(loop_var, var_ref.location),
          );
          set_lvalue_type(&loop_scope, def, loop_var);
        }
      } else {
        if slot_opt(var_ref.annotation).is_some() {
          let annotation_ty = self.resolve_type(
            arc_as_mut(&loop_scope),
            var_ref.annotation,
            /* in_type_arguments */ false,
            /* replace_error_with_fresh */ false,
            Polarity::Positive,
          );
          bind(
            &loop_scope,
            Symbol::from_local(var_ptr),
            make_binding(annotation_ty, var_ref.location),
          );
          self.add_constraint_scope_ptr_location_constraint_v(
            scope,
            var_ref.location,
            ConstraintV::Subtype(SubtypeConstraint {
              sub_type: loop_var,
              super_type: annotation_ty,
            }),
          );
        } else {
          bind(
            &loop_scope,
            Symbol::from_local(var_ptr),
            make_binding(loop_var, var_ref.location),
          );
        }

        let def = self.dfg_ref().get_def_local(from_ref(var_ref));
        set_lvalue_type(&loop_scope, def, loop_var);
      }
    }

    // cpp 契约 values.size >= 1（visit 判定）；借用切片免逐处指针加法。
    let values_slice = values.as_slice();
    let next_ast_fragment = from_ref(alias_ref(values_slice[0])).cast::<AstNode>();
    let ast_for_in_next_types = {
      let module = self.module.as_ref().unwrap();
      // 该字段地址随 iterable 约束长期保存、在 solver 派发期回写，与 cpp
      // `&module->astForInNextTypes` 同一成员生命周期契约（Arc 由 self 持有）。
      from_mut(&mut alias(arc_as_mut(module)).ast_for_in_next_types)
    };

    // C++ getLocation(forIn->values): span from first expr begin to last expr end.
    let values_location = {
      let first_ref = alias_ref(values_slice[0]);
      let last_ref = alias_ref(values_slice[values_slice.len() - 1]);
      Location {
        begin: first_ref.base.location.begin,
        end: last_ref.base.location.end,
      }
    };

    let iterable = self.add_constraint_scope_ptr_location_constraint_v(
      &loop_scope,
      values_location,
      ConstraintV::Iterable(IterableConstraint {
        iterator,
        variables: variable_types.clone(),
        next_ast_fragment,
        ast_for_in_next_types,
      }),
    );

    // Add an intersection ReduceConstraint for the key variable to denote that it can't be nil
    let key_var_ref = alias_ref(for_in.vars.as_slice()[0]);
    let key_def = self.dfg_ref().get_def_local(from_ref(key_var_ref));
    // cpp `loopScope->lvalueTypes.getOrInsert(keyDef)`：上方 vars 循环已写入该项，
    // 缺失时按 getOrInsert 语义插入默认值再读回。
    let loop_var: TypeId = lvalue_type(&loop_scope, key_def);

    let intersection_ty: TypeId = {
      // SAFETY: builtin_types 指向 arena 外存活的内建单例，此处只读。
      let bt = self.builtin_types.get();
      let intersect_func: &TypeFunction = &bt.type_functions.intersect_func;
      let not_nil_ty = bt.not_nil_type;
      self.create_type_function_instance(
        intersect_func,
        alloc::vec![loop_var, not_nil_ty],
        Vec::new(),
        &loop_scope,
        key_var_ref.location,
      )
    };

    bind(
      &loop_scope,
      Symbol::from_local(from_ref(key_var_ref).cast_mut()),
      make_binding(intersection_ty, key_var_ref.location),
    );
    set_lvalue_type(&loop_scope, key_def, intersection_ty);

    let c = self.add_constraint_scope_ptr_location_constraint_v(
      &loop_scope,
      key_var_ref.location,
      ConstraintV::Reduce(ReduceConstraint {
        ty: intersection_ty,
      }),
    );
    if fflag::LuauConstraintGraph.get() {
      // iterable 与 c 为紧邻上方两次 add_constraint 返回的约束句柄（generator 持有）。
      alias(self.cgraph).add_dependency_of_constraint_constraint(alias(iterable), alias(c));
    } else {
      add_deprecated_dependency(c, iterable);
    }

    for var in &variable_types {
      // variable_types 均来自 add_type(BlockedType)，必命中；对照 C++:1700-1704
      // `BlockedType* bt = getMutable<BlockedType>(var); LUAU_ASSERT(bt); bt->setOwner(iterable);`
      let bt = get_mutable_type::get_mutable::<BlockedType>(*var)
        .expect("variableTypes 元素应为 BlockedType");
      bt.set_owner(iterable.cast_const());
    }

    let start = checkpoint(self);
    // §2：for_in.body 是文法保证非空的 arena AstStatBlock（forin 必有循环体）。
    // body 已句柄化为非空 Node（文法保证 for-in 必有循环体），`.get()` 直出引用。
    self.visit_stat_block(&loop_scope, for_in.body.get());
    let end = checkpoint(self);

    // C++ scope->inheritAssignments(loopScope)
    inherit_assignments(scope, &loop_scope);

    // This iter constraint must dispatch first.
    if fflag::LuauConstraintGraph.get() {
      add_all_as_reverse_dependencies(start, end, self, iterable);
    } else {
      for_each_constraint(start, end, self, |run_later| {
        add_deprecated_dependency(run_later, iterable);
      });
    }

    ControlFlow::None
  }

  pub fn visit_stat_while(&mut self, scope: &ScopePtr, while_: &AstStatWhile) -> ControlFlow {
    let condition = while_.condition.get();
    let refinement: RefinementId = self.check_expr(scope, condition).refinement;

    // child_scope 读取 node 内嵌 AstNode 基类的 location 并登记（cpp
    // AstStatWhile* 透传同构）；while_ 为存活借用、scope 为调用方借出的存活 Arc。
    let while_scope: ScopePtr = self.child_scope(&while_.base.base, scope);
    self.apply_refinements(&while_scope, condition.base.location, refinement);

    self.visit_stat_block(&while_scope, while_.body.get());

    inherit_assignments(scope, &while_scope);

    ControlFlow::None
  }

  /// cpp `visit(const ScopePtr&, AstStatRepeat*)`：`repeat` 为分发层经 RTTI
  /// 校验后传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_repeat(&mut self, scope: &ScopePtr, repeat: &AstStatRepeat) -> ControlFlow {
    // child_scope 读取 node 内嵌 AstNode 基类的 location 并登记（cpp
    // AstStatRepeat* 透传同构）；repeat 为存活借用、scope 由调用方持有。
    let repeat_scope: ScopePtr = self.child_scope(&repeat.base.base, scope);

    // repeat_scope 为局部存活 Arc；body/condition 已句柄化为 Node（文法保证
    // 非空由类型层承载），`.get()` 直出共享引用，alias_ref 门面消失。
    self.visit_block_without_child_scope(&repeat_scope, repeat.body.get());

    // §2：repeat.condition 为文法保证非空的 arena 子表达式（repeat 必有条件）。
    self.check_expr(&repeat_scope, repeat.condition.get());

    inherit_assignments(scope, &repeat_scope);

    ControlFlow::None
  }

  /// cpp `visit(const ScopePtr&, AstStatLocalFunction*)`：`function` 为分发层经
  /// RTTI 校验后传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_local_function(
    &mut self,
    scope: &ScopePtr,
    function: &AstStatLocalFunction,
  ) -> ControlFlow {
    // §2：name/func 已句柄化为 Node（文法保证非空，类型层即证明），`.get()` 直读。
    let name_local = function.name.get();
    let func_body = function.func.get();

    // The parser ensures that every local function has a distinct Symbol for its name.
    // lookup_symbol 是 &self 安全方法：经 Arc 的 Deref 读取，不再派生写句柄。
    let ty = scope.lookup_symbol(Symbol::from_local(from_ref(name_local).cast_mut()));
    LUAU_ASSERT!(ty.is_none());

    // arena 为构造期非空句柄，function_type 是本次新分配的 BlockedType。
    let function_type: TypeId = self.arena.get_mut().add_type(BlockedType::default());
    bind_local(scope, name_local, function_type);

    // enclosing_class 的 cpp `nullptr` 形态即「非类体内声明」（被调方以
    // DebugLuauUserDefinedClasses 断言兜底，ConstraintGenerator.cpp:4285 同前提），
    // 门面里折成 `None`。
    let sig = check_function_signature(self, scope, func_body, None, Some(name_local.location));
    bind_local(&sig.body_scope, name_local, sig.signature);

    let def = self.dfg_ref().get_def_local(from_ref(name_local));
    set_lvalue_type(scope, def, function_type);
    self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, def, function_type);
    set_lvalue_type(&sig.body_scope, def, sig.signature);
    self.update_r_value_refinements_scope_ptr_def_id_type_id(&sig.body_scope, def, sig.signature);

    let start = checkpoint(self);
    // 与取得 sig 时为同一存活 AST 节点（对照 C++ checkFunctionBody(sig.bodyScope,
    // function->func)）。
    self.check_function_body(&sig.body_scope, func_body);
    let end = checkpoint(self);

    // constraintScope = sig.signatureScope ? sig.signatureScope : sig.bodyScope.
    let constraint_scope: &ScopePtr = &sig.signature_scope;

    let c = self.add_constraint_scope_ptr_location_constraint_v(
      constraint_scope,
      name_local.location,
      ConstraintV::Generalization(GeneralizationConstraint {
        generalized_type: function_type,
        source_type: sig.signature,
        interior_types: Vec::new(),
        has_deprecated_attribute: false,
        deprecated_info: Default::default(),
        no_generics: false,
      }),
    );

    propagate_deprecated_attribute(c, func_body);

    if fflag::LuauConstraintGraph.get() {
      add_all_as_dependencies_and_chain_returns(start, end, self, c);
    } else {
      // 关闭约束图时的退化依赖：区间内逐条前置 + PackSubtype{returns} 串联，
      // 四处同款循环收口到 arena_slots 门面。
      chain_deprecated_dependencies(self, start, end, c);
    }

    // 对照 C++:2625 `getMutable<BlockedType>(functionType)->setOwner(genConstraint)`
    // （function_type 刚由 add_type(BlockedType) 分配，必命中）。
    block_owner_at(function_type, c);

    // module->ast_types[function->func] = function_type;
    record_expr_type(self.module.as_ref().unwrap(), func_body, function_type);

    ControlFlow::None
  }

  /// cpp `visit(const ScopePtr&, AstStatFunction*)`：`function` 为分发层经 RTTI
  /// 校验后传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_function(
    &mut self,
    scope: &ScopePtr,
    function: &AstStatFunction,
  ) -> ControlFlow {
    let name_expr = function.name;
    // §2：名字节点/函数体节点已句柄化为 Node（parser 保证非空由类型层承载），
    // `.get()` 读成共享借用后全函数复用；基类视图经首字段链取（cpp
    // `static_cast<AstNode*>`）。
    let name_ref = name_expr.get();
    let name_node_ref = &name_ref.base;
    let body_fn = function.func.get();

    let start = self.cg_checkpoint();
    let sig = check_function_signature(self, scope, body_fn, None, Some(name_node_ref.location));

    let def = self.dfg_ref().get_def(name_expr.as_ptr().cast_const());

    let local_name = ast_node_try_as::<AstExprLocal>(name_node_ref);
    let global_name = ast_node_try_as::<AstExprGlobal>(name_node_ref);
    if let Some(local_name) = local_name {
      bind_with_lvalue(
        &sig.body_scope,
        Symbol::from_local(local_name.local.as_ptr()),
        make_binding(sig.signature, local_name.base.base.location),
        def,
        sig.signature,
      );
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&sig.body_scope, def, sig.signature);
    } else if let Some(global_name) = global_name {
      bind_with_lvalue(
        &sig.body_scope,
        Symbol::from_global(global_name.name),
        make_binding(sig.signature, global_name.base.base.location),
        def,
        sig.signature,
      );
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&sig.body_scope, def, sig.signature);
    } else if ast_node_is::<AstExprIndexName>(name_node_ref) {
      self.update_r_value_refinements_scope_ptr_def_id_type_id(&sig.body_scope, def, sig.signature);
    }

    if let Some(index_name) = ast_node_try_as::<AstExprIndexName>(name_node_ref) {
      let begin_prop = self.cg_checkpoint();
      // name_ref 即上方已确认存活的 name_expr 节点（cpp 复用同一指针）。
      let fn_ty = self.check_expr(scope, name_ref).ty;
      let end_prop = self.cg_checkpoint();
      let pftc = self.add_constraint_scope_ptr_location_constraint_v(
        &sig.signature_scope,
        body_fn.base.base.location,
        ConstraintV::PushFunctionType(PushFunctionTypeConstraint {
          expected_function_type: fn_ty,
          function_type: sig.signature,
          expr: function.func.as_ptr(),
          is_self: index_name.op == b':',
        }),
      );

      if fflag::LuauConstraintGraph.get() {
        add_all_as_dependencies(begin_prop, end_prop, self, pftc);

        let begin_body = self.cg_checkpoint();
        self.check_function_body(&sig.body_scope, body_fn);
        let end_body = self.cg_checkpoint();

        add_all_as_reverse_dependencies(begin_body, end_body, self, pftc);
      } else {
        for_each_constraint(begin_prop, end_prop, self, |c| {
          add_deprecated_dependency(pftc, c);
        });
        let begin_body = self.cg_checkpoint();
        self.check_function_body(&sig.body_scope, body_fn);
        let end_body = self.cg_checkpoint();
        for_each_constraint(begin_body, end_body, self, |c| {
          add_deprecated_dependency(c, pftc);
        });
      }
    } else {
      self.check_function_body(&sig.body_scope, body_fn);
    }

    let end = self.cg_checkpoint();

    // arena 构造期布线非空，generalized_type 为本次新分配的 BlockedType 句柄。
    let mut generalized_type: TypeId = self.arena.get_mut().add_type(BlockedType::default());
    // constraintScope = sig.signatureScope ? sig.signatureScope : sig.bodyScope.
    let constraint_scope: &ScopePtr = &sig.signature_scope;

    let c = self.add_constraint_scope_ptr_location_constraint_v(
      constraint_scope,
      name_node_ref.location,
      ConstraintV::Generalization(GeneralizationConstraint {
        generalized_type,
        source_type: sig.signature,
        interior_types: Vec::new(),
        has_deprecated_attribute: false,
        deprecated_info: Default::default(),
        no_generics: false,
      }),
    );
    // 对照 C++:2248 `getMutable<BlockedType>(generalizedType)->setOwner(gc)`
    // （generalized_type 刚由 add_type(BlockedType) 分配，必命中）。
    block_owner_at(generalized_type, c);

    propagate_deprecated_attribute(c, body_fn);

    if fflag::LuauConstraintGraph.get() {
      add_all_as_dependencies_and_chain_returns(start, end, self, c);
    } else {
      chain_deprecated_dependencies(self, start, end, c);
    }

    let existing_function_ty: Option<TypeId> = self
      .lookup(scope, name_node_ref.location, def, false)
      .map(follow_type::follow);

    if let Some(local_name) = ast_node_try_as::<AstExprLocal>(name_node_ref) {
      // scope 为调用方借出的存活 Arc，name_ref 已确认为存活的 AstExprLocal
      // 共享借用，generalized_type 是本函数持有的 arena 句柄（cpp visitLValue
      // 同构前提）。
      self.visit_l_value(scope, name_ref, generalized_type);

      bind_with_lvalue(
        scope,
        Symbol::from_local(local_name.local.as_ptr()),
        make_binding(sig.signature, local_name.base.base.location),
        def,
        sig.signature,
      );
    } else if let Some(global_name) = ast_node_try_as::<AstExprGlobal>(name_node_ref) {
      if existing_function_ty.is_none() {
        // ice 为 Handle 收口的构造期单例句柄。
        {
          self.ice.get().ice_string_location(
            "prepopulateGlobalScope did not populate a global name",
            &global_name.base.base.location,
          );
        }
      }

      if let Some(existing) = existing_function_ty {
        let global_sym = global_name.name;
        // 对照 C++：`if (auto bt = get<BlockedType>(existing); bt && uninitializedGlobals.contains(...))`
        if let Some(bt) = get_type::get::<BlockedType>(existing)
          && self.uninitialized_globals.contains(&global_sym)
        {
          LUAU_ASSERT!(bt.get_owner().is_null());
          self.uninitialized_globals.erase(&global_sym);
          // existing 是 arena 内的类型句柄（cpp asMutable 同契约）。
          bind_type(existing, generalized_type);
        }
      }

      bind_with_lvalue(
        scope,
        Symbol::from_global(global_name.name),
        make_binding(sig.signature, global_name.base.base.location),
        def,
        sig.signature,
      );
    } else if ast_node_try_as::<AstExprIndexName>(name_node_ref).is_some() {
      // 此分支 name_ref 已确认为存活的 AstExprIndexName（借用即证），
      // scope/generalized_type 与 local 分支同源，满足被调门面的指针契约。
      self.visit_l_value(scope, name_ref, generalized_type);
    } else if ast_node_is::<AstExprError>(name_node_ref) {
      // builtin_types 为 Handle 收口的构造期单例。
      generalized_type = self.builtin_types.get().error_type;
    }

    if generalized_type.is_null() {
      // ice 为 Handle 收口的构造期单例句柄。
      {
        self
          .ice
          .get()
          .ice_string_location("generalizedType == nullptr", &function.base.base.location);
      }
    }

    self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, def, generalized_type);

    ControlFlow::None
  }

  /// `checkpoint(self)` 快照（visit 六处同款）。
  fn cg_checkpoint(&self) -> Checkpoint {
    checkpoint(self)
  }

  // ConstraintGenerator::visit(const ScopePtr&, AstStatReturn*)
  // (ConstraintGenerator.cpp:1961).
  /// cpp `visit(const ScopePtr&, AstStatReturn*)`：`ret` 为分发层经 RTTI 校验后
  /// 传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_return(&mut self, scope: ScopePtr, ret: &AstStatReturn) -> ControlFlow {
    // At this point, the only way scope->returnType should have anything
    // interesting in it is if the function has an explicit return annotation.
    // If this is the case, then we can expect that the return expression
    // conforms to that.
    let scope_ref = scope.as_ref();
    let return_type = scope_ref.return_type;
    let expected_types: Vec<Option<TypeId>> = begin(return_type).map(Some).collect();

    let list = ret.list;
    let expr_types = self
      .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(
        &scope,
        list,
        &expected_types,
      )
      .tp;

    self.add_constraint_scope_ptr_location_constraint_v(
      &scope,
      ret.base.base.location,
      ConstraintV::PackSubtype(PackSubtypeConstraint {
        sub_pack: expr_types,
        super_pack: return_type,
        returns: true,
      }),
    );

    ControlFlow::Returns
  }

  // ConstraintGenerator::visit(const ScopePtr&, AstStatBlock*) (ConstraintGenerator.cpp).
  /// cpp `visit(const ScopePtr&, AstStatBlock*)`：`block` 为调用方交出的存活
  /// AstStatBlock 共享借用，本方法只读取其字段。
  pub(crate) fn visit_stat_block(&mut self, scope: &ScopePtr, block: &AstStatBlock) -> ControlFlow {
    // child_scope 读取 block 内嵌 AstNode 基类的 location 并登记为 ast_scopes
    // 键；block/scope 均为调用方交出的存活共享借用，Arc<Scope> 由 self.scopes 保活。
    let inner_scope = self.child_scope(&block.base.base, scope);
    let flow = self.visit_block_without_child_scope(&inner_scope, block);

    // An AstStatBlock has linear control flow, i.e. one entry and one exit, so we
    // can inherit all the changes to the environment occurred by the statements in
    // that block.
    // §2：inherit_* 的「Arc 写句柄 + 解引用」收口到 scope_slots 门面。
    inherit_assignments(scope, &inner_scope);
    inherit_refinements(scope, &inner_scope);

    flow
  }

  /// cpp `visit(const ScopePtr&, AstStatAssign*)`：`assign` 为分发层经 RTTI 校验
  /// 后传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_assign(&mut self, scope: &ScopePtr, assign: &AstStatAssign) -> ControlFlow {
    let result_pack = self
      .check_pack_scope_ptr_ast_array_ast_expr_vector_optional_type_id(scope, assign.values, &[])
      .tp;
    let mut value_types = Vec::new();
    let (head, _) = flatten_type_pack_id(result_pack);

    // 借用切片读取 vars，免逐处裸指针加法（cpp `assign->vars.data[i]`）。
    let vars = assign.vars.as_slice();
    let vars_size = vars.len();
    if head.len() >= vars_size {
      value_types.extend_from_slice(&head[..vars_size]);
    } else {
      value_types.extend(
        repeat_with(|| self.arena.get_mut().add_type(BlockedType::default())).take(vars_size),
      );
      let uc = self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        assign.base.base.location,
        ConstraintV::Unpack(UnpackConstraint {
          result_pack: value_types.clone(),
          source_pack: result_pack,
        }),
      );
      for &t in &value_types {
        // cpp `getMutable<BlockedType>(t)->setOwner(uc)`（未命中即跳过，安全方向）。
        block_owner_at(t, uc);
      }
    }
    // i < value_types.len() <= vars_size，等价地在较短的 value_types 一侧结束。
    for (&vt, var_ref) in value_types.iter().zip(assign.vars.iter_nodes()) {
      self.visit_l_value(scope, var_ref, vt);
    }
    ControlFlow::None
  }

  /// cpp `visit(const ScopePtr&, AstStatIf*)`：`if_statement` 为分发层经 RTTI
  /// 校验后传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_if(&mut self, scope: &ScopePtr, if_statement: &AstStatIf) -> ControlFlow {
    // §2：condition 是文法保证非空的 arena 槽位，共享借用全函数复用
    // （cpp 直接 `ifStatement->condition`）。
    let condition = if_statement.condition.get();
    let condition_location = condition.base.location;

    let refinement: RefinementId = {
      // SAFETY: InConditionalContext::new 需要 c 指向有效 TypeContext 且在 Drop 前
      // 不被其它途径改写——此处传 self.type_context 字段的可变借用指针，唯一且
      // 由作用域借用保证；_flipper 存活期间无第二个句柄。
      let _flipper = InConditionalContext::new(&mut self.type_context, TypeContext::Condition);
      self.check_expr_expected(scope, condition, None).refinement
    };

    // SAFETY: thenbody 文法保证非空（if 必有 then 分支）且存活；
    // child_scope 仅登记该节点。
    let then_scope: ScopePtr =
      self.child_scope(alias_ref(if_statement.thenbody.as_ptr().cast()), scope);
    self.apply_refinements(&then_scope, condition_location, refinement);

    let else_node: &AstNode = if let Some(else_body) = if_statement.elsebody.get() {
      &else_body.base
    } else {
      &if_statement.base.base
    };
    // 两支均非空——要么通过上方判空的 elsebody，要么取 if_statement 自身基类
    // （repr(C) 偏移 0）；两支都随 arena 存活，child_scope 仅读取 location 并登记。
    let else_scope: ScopePtr = self.child_scope(else_node, scope);
    let else_refinement_location = if_statement.else_location.unwrap_or(condition_location);
    let negated = self.refinement_arena.negation(refinement);
    // §2：`None`（原 null 哨兵）与 apply_refinements 入口的判空 no-op 同义。
    if let Some(negated) = negated {
      self.apply_refinements(&else_scope, else_refinement_location, negated);
    }

    // thenbody/elsebody 的基类位于 repr(C) 偏移 0：取基类共享引用后由
    // visit_stat 重新 RTTI 分发（cpp 同一形态）。
    let thencf = self.visit_stat(&then_scope, &if_statement.thenbody.base);
    let mut elsecf = ControlFlow::None;
    if let Some(else_body) = if_statement.elsebody.get() {
      elsecf = self.visit_stat(&else_scope, else_body);
    }

    if thencf != ControlFlow::None && elsecf == ControlFlow::None {
      inherit_refinements(scope, &else_scope);
    } else if thencf == ControlFlow::None && elsecf != ControlFlow::None {
      inherit_refinements(scope, &then_scope);
    } else if thencf == ControlFlow::None && elsecf == ControlFlow::None {
      self.inherit_shared_refinements(scope, condition_location, &then_scope, &else_scope);
    }

    if thencf == ControlFlow::None {
      inherit_assignments(scope, &then_scope);
    }
    if elsecf == ControlFlow::None {
      inherit_assignments(scope, &else_scope);
    }

    if thencf == elsecf {
      thencf
    } else if (matches(thencf, ControlFlow::Returns) || matches(thencf, ControlFlow::Throws))
      && (matches(elsecf, ControlFlow::Returns) || matches(elsecf, ControlFlow::Throws))
    {
      ControlFlow::Returns
    } else {
      ControlFlow::None
    }
  }

  // ConstraintGenerator::visit(const ScopePtr&, AstStatTypeAlias*)
  // (ConstraintGenerator.cpp).
  /// cpp `visit(const ScopePtr&, AstStatTypeAlias*)`：`alias` 为分发层经 RTTI
  /// 校验后传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_type_alias(
    &mut self,
    scope: &ScopePtr,
    alias: &AstStatTypeAlias,
  ) -> ControlFlow {
    // AstName 由词法器 intern，必为合法 ASCII/UTF-8。
    // 保留别名名不参与绑定：`typeof` 额外报 ReservedIdentifier（cpp 两臂逐字保真）。
    let name_bytes = alias.name.as_bytes();
    if is_reserved_type_alias_name(name_bytes) {
      if name_bytes == K_TYPEOF {
        self.report_error(
          alias.base.base.location,
          TypeErrorData::ReservedIdentifier(ReservedIdentifier::new(String::from("typeof"))),
        );
      }
      return ControlFlow::None;
    }

    let name_key: String = alias.name.as_str_or_empty().to_string();
    // §2：Scope 槽位写入统一经 scope_mut 门面（Arc 写句柄 + 解引用不再外渗）。
    {
      let locations = scope_mut(scope);
      locations
        .type_alias_locations
        .insert(name_key.clone(), alias.base.base.location);
      locations
        .type_alias_name_locations
        .insert(name_key.clone(), alias.name_location);
    }

    // 键即节点身份：由存活借用还原地址（cpp `AstStatTypeAlias*` 键）。
    let defn_scope_opt = self
      .ast_type_alias_defining_scopes
      .find(&from_ref(alias))
      .cloned();

    // These will be undefined if the alias was a duplicate definition, in which
    // case we just skip over it.
    // 只读走 Arc 的 Deref（cpp `const Scope&`），不再派生写句柄；exported 只是
    // 选择读取两张绑定表之一，结果 cloned 脱离借用。
    let binding_it = if alias.exported {
      scope.exported_type_bindings.get(&name_key).cloned()
    } else {
      scope.private_type_bindings.get(&name_key).cloned()
    };

    let (fun, defn_scope) = match (binding_it, defn_scope_opt) {
      (Some(b), Some(Some(s))) => (b, s),
      _ => return ControlFlow::None,
    };
    let defn_scope_raw = arc_as_mut(&defn_scope);

    // 形参契约（defn_scope 独占可写、alias 存活只读、fun 为该 alias 的 TypeFun）
    // 收口在 scope_slots::resolve_generic_defaults。
    resolve_generic_defaults(self, &defn_scope, alias, &fun);

    let ty = self.resolve_type(
      defn_scope_raw,
      alias.type_ptr,
      /* in_type_arguments */ false,
      /* replace_error_with_fresh */ false,
      Polarity::Positive,
    );

    let alias_ty = fun.r#type();
    // get_type_id 已 safe；对照 C++ `LUAU_ASSERT(is<BlockedType>(aliasTy))`
    LUAU_ASSERT!(get_type::get::<BlockedType>(alias_ty,).is_some());

    if occurs_check_type_id_type_id(alias_ty, ty) {
      // alias_ty 在紧邻上方已断言为 arena 内 BlockedType；改写 Bound 指向即
      // cpp asMutable 同一契约（收口在 arena_slots::bind_type）。
      bind_type(alias_ty, self.builtin_types.get().any_type);
      self.report_error(
        alias.name_location,
        TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
      );
    } else {
      // 同上：alias_ty 为 arena 内存活的 BlockedType 句柄，ty 是本函数
      // resolve_type 刚产出的 arena 类型。
      bind_type(alias_ty, ty);
    }

    let type_params: Vec<TypeId> = self
      .create_generics(
        &defn_scope,
        alias.generics,
        /* use_cache */ true,
        /* add_types */ false,
      )
      .into_iter()
      .map(|(_, def)| def.ty)
      .collect();

    let type_pack_params: Vec<TypePackId> = self
      .create_generic_packs(
        &defn_scope,
        alias.generic_packs,
        /* use_cache */ true,
        /* add_types */ false,
      )
      .into_iter()
      .map(|(_, def)| def.tp)
      .collect();

    self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      // 类型别名文法必有右端类型节点 type_ptr（resolve_type 已在上方读同一节点）。
      slot_ref(alias.type_ptr).base.location,
      ConstraintV::Name(NameConstraint {
        named_type: ty,
        name: name_key,
        synthetic: false,
        type_parameters: type_params,
        type_pack_parameters: type_pack_params,
      }),
    );

    ControlFlow::None
  }

  /// cpp `visit(const ScopePtr&, AstStatTypeFunction*)`：`function` 为分发层经
  /// RTTI 校验后传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_type_function(
    &mut self,
    scope: &ScopePtr,
    function: &AstStatTypeFunction,
  ) -> ControlFlow {
    // function->name == "typeof"
    let name_bytes = function.name.as_bytes();
    if name_bytes == K_TYPEOF {
      self.report_error(
        function.base.base.location,
        TypeErrorData::ReservedIdentifier(ReservedIdentifier::new(String::from("typeof"))),
      );
    }

    // 键即节点身份：由存活借用还原地址（cpp `AstStatTypeFunction*` 键）。
    let scope_it = self
      .ast_type_function_environment_scopes
      .find(&from_ref(function))
      .cloned();
    LUAU_ASSERT!(scope_it.is_some());

    let environment_scope: ScopePtr = scope_it.unwrap().unwrap();

    // function.body 文法保证非空（类型函数必有定义体）：共享借用全函数复用。
    let body = slot_ref(function.body);

    let start_checkpoint = checkpoint(self);
    let sig = check_function_signature(self, &environment_scope, body, None, None);

    // Place this function as a child of the non-type function scope.
    scope_mut(scope)
      .children
      .push(intern_scope(&sig.signature_scope));
    self.interior_free_types.push(Default::default());
    // 与取得 sig 时同一存活节点（对照 C++ checkFunctionBody(sig.bodyScope,
    // function->body)）。
    self.check_function_body(&sig.body_scope, body);
    let end_checkpoint = checkpoint(self);

    // arena 构造期非空句柄；generalized_ty 为本次新分配的 BlockedType。
    let generalized_ty: TypeId = self.arena.get_mut().add_type(BlockedType::default());
    let gc = self.add_constraint_scope_ptr_location_constraint_v(
      &sig.signature_scope,
      function.base.base.location,
      ConstraintV::Generalization(GeneralizationConstraint {
        generalized_type: generalized_ty,
        source_type: sig.signature,
        interior_types: Vec::new(),
        has_deprecated_attribute: false,
        deprecated_info: Default::default(),
        no_generics: false,
      }),
    );

    // signature_scope 的 Arc 由 sig 持有、本函数内存活；interior_free_types 栈
    // 刚 push 了新帧、尚未 pop，last_mut().unwrap() 必非 None。
    {
      let signature_scope = scope_mut(&sig.signature_scope);
      signature_scope.interior_free_types = Some(take(
        &mut self.interior_free_types.last_mut().unwrap().types,
      ));
      signature_scope.interior_free_type_packs = Some(take(
        &mut self.interior_free_types.last_mut().unwrap().type_packs,
      ));
    }

    // 对照 C++:2248 `getMutable<BlockedType>(generalizedType)->setOwner(gc)`
    // （generalized_ty 刚由 add_type(BlockedType) 分配，必命中）。
    block_owner_at(generalized_ty, gc);
    self.interior_free_types.pop();

    if fflag::LuauConstraintGraph.get() {
      add_all_as_dependencies_and_chain_returns(start_checkpoint, end_checkpoint, self, gc);
    } else {
      chain_deprecated_dependencies(self, start_checkpoint, end_checkpoint, gc);
    }

    // lookup_symbol 是 &self 安全方法：经 Arc 的 Deref 读取（cpp `const Scope&`）。
    let existing_function_ty = environment_scope.lookup_symbol(Symbol::from_global(function.name));

    if existing_function_ty.is_none() {
      // SAFETY: self.ice.as_ptr() 为构造期布线的报告器，非空且随 generator 存活（C++ ICE
      // 引用成员同契约）；name_location 取自存活 function。
      {
        self.ice.get().ice_string_location(
          "checkAliases did not populate type function name",
          &function.name_location,
        );
      }
    }

    let unpacked_ty = follow_type::follow(existing_function_ty.unwrap());

    // 对照 C++:2259 `if (auto bt = get<BlockedType>(unpackedTy); bt && nullptr == bt->getOwner())`
    if let Some(bt) = get_type::get::<BlockedType>(unpacked_ty)
      && bt.get_owner().is_null()
    {
      // unpacked_ty 是 arena 内的类型句柄（cpp asMutable 同契约）。
      bind_type(unpacked_ty, generalized_ty);
    }

    ControlFlow::None
  }

  // ConstraintGenerator::visit(const ScopePtr&, AstStatDeclareGlobal*)
  // (ConstraintGenerator.cpp:2273).
  /// cpp `visit(const ScopePtr&, AstStatDeclareGlobal*)`：`global` 为分发层经
  /// RTTI 校验后传入的节点共享借用；`scope` 与兄弟方法统一收为 `&ScopePtr`，
  /// 写句柄在本函数内经 `arc_as_mut` 派生。
  pub fn visit_stat_declare_global(
    &mut self,
    scope: &ScopePtr,
    global: &AstStatDeclareGlobal,
  ) -> ControlFlow {
    LUAU_ASSERT!(!global.type_.is_null());

    // arc_as_mut 派生的写句柄在调用方借用的 Arc 期内存活（crate 惯用法）。
    let scope_raw = arc_as_mut(scope);
    let global_ty: TypeId =
      self.resolve_type(scope_raw, global.type_, false, false, Polarity::Positive);
    let global_name: Name = global.name.as_str_or_empty().to_string();

    // module->declaredGlobals[globalName] = globalTy;
    record_declared_global(self.module.as_ref().unwrap(), global_name, global_ty);

    // rootScope->bindings[global->name] = Binding{globalTy, global->location};
    // root 句柄由 `root()` 收口、随 generator 存活；Symbol::from_global 只取
    // 值语义的 AstName（对应 C++ `rootScope->bindings[global->name] = Binding{..}`）。
    bind(
      self.root(),
      Symbol::from_global(global.name),
      make_binding(global_ty, global.base.base.location),
    );

    // dfg 由 NonNull 字段收口（dfg_ref），键为存活节点的地址（cpp 同值）。
    let def: DefId = self.dfg_ref().get_def_declare_global(from_ref(global));
    set_lvalue_type(self.root(), def, global_ty);
    // (scope,def,type) 三元合法性：def 刚取自 dfg、root 有效（C++ 同构调用）。
    self.update_r_value_refinements_scope_ptr_def_id_type_id(self.root(), def, global_ty);

    ControlFlow::None
  }

  /// cpp `visit(const ScopePtr&, AstStatDeclareExternType*)`：`decl` 为分发层经
  /// RTTI 校验后传入的节点共享借用；`scope` 与兄弟方法统一收为 `&ScopePtr`，
  /// 写句柄在本函数内经 `arc_as_mut` 派生，不再让裸指针形参渗进签名。
  pub(crate) fn visit_stat_declare_extern_type(
    &mut self,
    scope: &ScopePtr,
    decl: &AstStatDeclareExternType,
  ) -> ControlFlow {
    // 本函数逐句对应 C++ `visit(const ScopePtr&, AstStatDeclareExternType*)`：
    // 句柄存活前提（scope 的 Arc、builtin_types/arena 构造期单例、decl 名下 arena
    // 字段与 props 数组只读遍历）由各收口门面（scope_slots / arena_slots /
    // module_slots / slot_opt）的模块级契约保证，函数体内不再出现裸解引用。
    // §2：只读视图直接用 Arc 的 Deref（cpp `const Scope&`）；写句柄只在需要
    // `Scope*` 形参（resolve_type / TableType 登记）处经 arc_as_mut 派生。
    let scope_ref: &Scope = scope;
    let scope_raw = arc_as_mut(scope);

    // If a class with the same name was already defined, we skip over.
    let name_key: Name = decl.name.as_str_or_empty().to_string();
    let binding_it = match scope_ref.exported_type_bindings.get(&name_key) {
      Some(b) => b.clone(),
      None => return ControlFlow::None,
    };

    let mut super_ty: Option<TypeId> = Some(self.builtin_types.get().extern_type);

    if let Some(super_name_node) = decl.super_name.as_ref() {
      let super_name: Name = super_name_node.as_str_or_empty().to_string();

      let lookup_type = scope_ref.lookup_type(&super_name);

      if lookup_type.is_none() {
        self.report_error(
          decl.base.base.location,
          TypeErrorData::UnknownSymbol(UnknownSymbol::new(super_name, Context::Type)),
        );
        return ControlFlow::None;
      }

      let lookup_type = lookup_type.unwrap();

      // We don't have generic extern type_arguments, so this assertion
      // _should_ never be hit.
      LUAU_ASSERT!(
        lookup_type.type_params().is_empty() && lookup_type.type_pack_params().is_empty()
      );

      let followed = follow_type::follow(lookup_type.r#type());
      super_ty = Some(followed);

      if get_type::get::<ExternType>(follow_type::follow(super_ty.unwrap())).is_none() {
        self.report_error(
          decl.base.base.location,
          TypeErrorData::GenericError(GenericError::new(format(format_args!(
            "Cannot use non-class type '{}' as a superclass of class '{}'",
            super_name.as_str(),
            name_key.as_str()
          )))),
        );

        // If we don't emplace an error type here, then later we'll be
        // exposing a blocked type in this file's type interface. This
        // is _normally_ harmless.
        let class_bind_ty = binding_it.r#type();
        bind_type(class_bind_ty, self.builtin_types.get().error_type);

        return ControlFlow::None;
      }
    }

    let class_name: Name = name_key.clone();

    let module_name = self.module.as_ref().unwrap().name.clone();

    let extern_ty: TypeId = self.arena.get_mut().add_type(ExternType {
      name: class_name,
      props: Default::default(),
      parent: super_ty,
      metatable: None,
      tags: Default::default(),
      user_data: None,
      definition_module_name: module_name,
      definition_location: Some(decl.base.base.location),
      indexer: None,
      relation: None,
    });
    // extern_ty / meta_ty 刚由 add_type 分配，必命中；对照 C++:2338-2342
    // `ExternType* etv = getMutable<ExternType>(externTy); TableType* metatable = ...`
    let etv = get_mutable_type::get_mutable::<ExternType>(extern_ty).unwrap();

    let meta_ty: TypeId =
      self
        .arena
        .get_mut()
        .add_type(TableType::table_type_table_state_type_level_scope(
          TableState::Sealed,
          scope_ref.level,
          scope_raw,
        ));
    let metatable = get_mutable_type::get_mutable::<TableType>(meta_ty).unwrap();

    etv.metatable = Some(meta_ty);

    let class_bind_ty = binding_it.r#type();
    bind_type(class_bind_ty, extern_ty);

    // §2：可空 indexer 槽位折成 Option（cpp 判空后直接解引用，null 即 UB）。
    if let Some(indexer) = slot_opt(decl.indexer) {
      if self.recursion_count >= dfint::LuauConstraintGeneratorRecursionLimit.get() {
        self.report_code_too_complex(indexer.location);
      } else {
        // I don't think extern types can *be* generic, but if they
        // have an indexer over those generics, the polarity is
        // mixed.
        let index_type =
          self.resolve_type(scope_raw, indexer.index_type, false, false, Polarity::Mixed);
        let index_result_type = self.resolve_type(
          scope_raw,
          indexer.result_type,
          false,
          false,
          Polarity::Mixed,
        );
        etv.indexer = Some(TableIndexer {
          index_type,
          index_result_type,
          is_read_only: false,
        });
      }
    }

    for extern_prop in decl.props.as_slice() {
      let prop_name: Name = extern_prop.name.as_str_or_empty().to_string();
      let prop_ty = self.resolve_type(scope_raw, extern_prop.ty, false, false, Polarity::Mixed);

      let assign_to_metatable = is_metamethod_mut(&prop_name);

      // Function type_arguments always take 'self', but this isn't
      // reflected in the parsed annotation. Add it here.
      if extern_prop.is_method {
        // 对照 C++:2382 `if (FunctionType* ftv = getMutable<FunctionType>(propTy))`
        if let Some(ftv) = get_mutable_type::get_mutable::<FunctionType>(prop_ty) {
          ftv.arg_names.insert(
            0,
            Some(FunctionArgument {
              name: "self".into(),
              location: Default::default(),
            }),
          );
          ftv.arg_types = self.add_type_pack(alloc::vec![extern_ty], Some(ftv.arg_types));

          ftv.has_self = true;

          let defn = FunctionDefinition {
            definition_module_name: Some(self.module.as_ref().unwrap().name.clone()),
            definition_location: extern_prop.location,
            // No data is preserved for vararg_location
            vararg_location: None,
            original_name_location: extern_prop.name_location,
          };

          ftv.definition = Some(defn);
        }
      }

      let props: &mut Props = if assign_to_metatable {
        &mut metatable.props
      } else {
        &mut etv.props
      };

      if !props.contains_key(&prop_name) {
        let mut table_prop = if extern_prop.access == AstTableAccess::Read {
          Property::readonly(prop_ty)
        } else if extern_prop.access == AstTableAccess::Write {
          Property::writeonly(prop_ty)
        } else {
          Property::rw_type_id(prop_ty)
        };

        table_prop.location = Some(extern_prop.location);

        props.insert(prop_name.clone(), table_prop);
      } else {
        let prop = props.get_mut(&prop_name).unwrap();
        let mut added_write_type_by_overload = false;

        if let Some(read_ty) = prop.read_ty {
          // We special-case this logic to keep the intersection
          // flat; otherwise we would create a ton of nested
          // intersection type_arguments.
          // 对照 C++：`if (const IntersectionType* itv = get<IntersectionType>(readTy))`
          if let Some(itv) = get_type::get::<IntersectionType>(read_ty) {
            let mut options = itv.parts.clone();
            options.push(prop_ty);
            let new_itv = self
              .arena
              .get_mut()
              .add_type(IntersectionType { parts: options });
            prop.read_ty = Some(new_itv);
          } else if get_type::get::<FunctionType>(read_ty).is_some() {
            let intersection = self.arena.get_mut().add_type(IntersectionType {
              parts: alloc::vec![read_ty, prop_ty],
            });
            prop.read_ty = Some(intersection);
          } else if extern_prop.access == AstTableAccess::Write && prop.write_ty.is_none() {
            prop.write_ty = Some(prop_ty);
            added_write_type_by_overload = true;
          } else {
            self.report_error(
              decl.base.base.location,
              TypeErrorData::GenericError(GenericError::new(format(format_args!(
                "Cannot overload read type of non-function extern type member '{}'",
                prop_name.as_str()
              )))),
            );
          }
        }

        if let Some(write_ty) = prop.write_ty
          && !added_write_type_by_overload
        {
          // We special-case this logic to keep the
          // intersection flat; otherwise we would create a ton
          // of nested intersection type_arguments.
          // 对照 C++：`if (const IntersectionType* itv = get<IntersectionType>(writeTy))`
          if let Some(itv) = get_type::get::<IntersectionType>(write_ty) {
            let mut options = itv.parts.clone();
            options.push(prop_ty);
            let new_itv = self
              .arena
              .get_mut()
              .add_type(IntersectionType { parts: options });
            prop.write_ty = Some(new_itv);
          } else if get_type::get::<FunctionType>(write_ty).is_some() {
            let intersection = self.arena.get_mut().add_type(IntersectionType {
              parts: alloc::vec![write_ty, prop_ty],
            });
            prop.write_ty = Some(intersection);
          } else if extern_prop.access == AstTableAccess::Read && prop.read_ty.is_none() {
            prop.read_ty = Some(prop_ty);
          } else {
            self.report_error(
              decl.base.base.location,
              TypeErrorData::GenericError(GenericError::new(format(format_args!(
                "Cannot overload write type of non-function extern type member '{}'",
                prop_name.as_str()
              )))),
            );
          }
        }
      }
    }

    ControlFlow::None
  }
}

use crate::functions::is_metamethod_constraint_generator::is_metamethod_mut;

impl ConstraintGenerator {
  /// cpp `visit(const ScopePtr&, AstStatDeclareFunction*)`：`global` 为分发层经
  /// RTTI 校验后传入的声明函数节点共享借用，本方法只读取其字段。
  pub fn visit_stat_declare_function(
    &mut self,
    scope: &ScopePtr,
    global: &AstStatDeclareFunction,
  ) -> ControlFlow {
    let generics = self.create_generics(scope, global.generics, false, true);
    let generic_packs = self.create_generic_packs(scope, global.generic_packs, false, true);

    let generic_tys = generics.iter().map(|(_, generic)| generic.ty).collect();
    let generic_tps = generic_packs
      .iter()
      .map(|(_, generic_pack)| generic_pack.tp)
      .collect();

    let fun_scope = if !generics.is_empty() || !generic_packs.is_empty() {
      // node 取 global 内嵌 AstNode 基类（repr(C) 偏移 0）的共享借用——被调方
      // 仅读取其 location 并登记（cpp AstStatDeclareFunction* 透传同构），
      // 该引用在借用期内存活；scope 为调用方持有的存活 Arc。
      self.child_scope(&global.base.base, scope)
    } else {
      scope.clone()
    };
    let fun_scope_raw = arc_as_mut(&fun_scope);

    let param_pack = self.resolve_type_pack_scope_ptr_ast_type_list_bool_bool_polarity(
      fun_scope_raw,
      &global.params,
      false,
      false,
      Polarity::Negative,
    );
    let ret_pack = self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
      fun_scope_raw,
      global.ret_types,
      false,
      false,
      Polarity::Positive,
    );

    let module_name = self.module.as_ref().unwrap().name.clone();
    let defn = FunctionDefinition {
      definition_module_name: Some(module_name),
      definition_location: global.base.base.location,
      vararg_location: if global.vararg {
        Some(global.vararg_location)
      } else {
        None
      },
      original_name_location: global.name_location,
    };

    let mut function_type =
      FunctionType::function_type_new(param_pack, ret_pack, Some(defn), false);
    function_type.generics = generic_tys;
    function_type.generic_packs = generic_tps;
    function_type.is_checked_function = global.is_checked_function();

    let deprecated_attr = global.get_attribute(AstAttrType::Deprecated);
    // §2：可空属性槽位折成 Option（cpp `if (attr)`）；deprecated_info 是 &self
    // 安全方法，仅读取属性字段。
    if let Some(deprecated) = slot_opt(deprecated_attr) {
      function_type.is_deprecated_function = true;
      function_type.deprecated_info = Some(Arc::new(deprecated.deprecated_info()));
    }

    function_type.arg_names = Vec::with_capacity(global.param_names.size);
    for &(name, location) in &global.param_names {
      function_type.arg_names.push(Some(FunctionArgument {
        name: (name.as_str_or_empty().to_string()),
        location,
      }));
    }

    // arena 构造期非空句柄，fn_type 为登记函数类型的新句柄。
    let fn_type = self.arena.get_mut().add_type(function_type);
    let fn_name: Name = global.name.as_str_or_empty().to_string();

    // module/scope/root 三处登记：写入分别收口到 module_slots / scope_slots 门面，
    // def 由 dfg_ref 取得，(scope,def,type) 三元合法（C++ 同构调用）。
    record_declared_global(self.module.as_ref().unwrap(), fn_name, fn_type);

    bind(
      scope,
      Symbol::from_global(global.name),
      make_binding(fn_type, global.base.base.location),
    );

    let def = self
      .dfg_ref()
      .get_def_for_declare_function(from_ref(global));
    set_lvalue_type(self.root(), def, fn_type);
    self.update_r_value_refinements_scope_ptr_def_id_type_id(self.root(), def, fn_type);

    ControlFlow::None
  }

  /// cpp `visit(const ScopePtr&, AstStatClass*)`：`stat_class` 为分发层经 RTTI
  /// 校验后传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_class(&mut self, scope: &ScopePtr, stat_class: &AstStatClass) -> ControlFlow {
    LUAU_ASSERT!(fflag::DebugLuauUserDefinedClasses.get());

    // §2：可空超类槽位折成 Option（cpp `if (super_)` 后直接解引用）。
    if let Some(super_) = slot_opt(stat_class.super_) {
      self.check_expr(scope, super_);
    }

    let Some(mut class_decl_record) = self.class_decl_records.find(&stat_class.name).cloned()
    else {
      // C++: this is unpopulated in fragment autocomplete.
      return ControlFlow::None;
    };

    // resolve_type 的 cpp 形参是 Scope 句柄：沿用 crate 的 arc_as_mut 写句柄惯用法。
    let scope_raw = arc_as_mut(scope);

    for member in stat_class.members.as_slice() {
      if let Some(class_prop) = member.get_if_0() {
        // AstClassProperty：解析标注并把占位 BlockedType 绑定为目标类型。
        let Some(&entry) = class_decl_record.member_types.find(&class_prop.name) else {
          LUAU_ASSERT!(false);
          continue;
        };

        let blocked_ty = follow_type::follow(entry);
        if get_type::get::<BlockedType>(blocked_ty).is_none() {
          continue;
        }

        let target = if !class_prop.ty.is_null() {
          self.resolve_type(scope_raw, class_prop.ty, false, false, Polarity::Unknown)
        } else {
          self.builtin_types.get().any_type
        };
        bind_type(blocked_ty, target);
      } else if let Some(method) = member.get_if_1() {
        // AstClassMethod：登记签名约束；`__init` 额外派生 `.new` 构造签名。
        let Some(&function_type) = class_decl_record.member_types.find(&method.function_name)
        else {
          LUAU_ASSERT!(false);
          continue;
        };

        let function_type = follow_type::follow(function_type);
        // 同名成员在原型阶段共用一个占位；若已被更早的成员解析，则跳过。
        if get_type::get::<BlockedType>(function_type).is_none() {
          continue;
        }

        let method_name = method.function_name.as_str_or_empty().to_string();

        // method.function 非空（类方法必有函数体）：共享借用全分支复用。
        let method_fn = slot_ref(method.function);
        // 类体内声明：enclosing_class 交 `Some(&mut record)`（cpp
        // `ClassDeclRecord*`），record 为本函数 find().cloned() 出的局部值。
        let sig = check_function_signature_in(
          self,
          scope,
          Some(&mut class_decl_record),
          method_fn,
          None,
          Some(method_fn.base.base.location),
        );

        let start = checkpoint(self);
        self.check_function_body(&sig.body_scope, method_fn);
        let end = checkpoint(self);

        let c = self.add_constraint_scope_ptr_location_constraint_v(
          &sig.signature_scope,
          method_fn.base.base.location,
          ConstraintV::Generalization(GeneralizationConstraint {
            generalized_type: function_type,
            source_type: sig.signature,
            interior_types: Vec::new(),
            has_deprecated_attribute: false,
            deprecated_info: Default::default(),
            no_generics: false,
          }),
        );

        propagate_deprecated_attribute(c, method_fn);

        if fflag::LuauConstraintGraph.get() {
          add_all_as_dependencies_and_chain_returns(start, end, self, c);
        } else {
          chain_deprecated_dependencies(self, start, end, c);
        }

        // 对照 C++:2618 `getMutable<BlockedType>(functionType)->setOwner(genConstraint)`
        // （function_type 来自 BlockedType 记录，必命中）。
        block_owner_at(function_type, c);

        if method_name == "__init"
          && let Some(new_blocked_ty) = class_decl_record.new_blocked_ty
        {
          self.bind_new_constructor(&mut class_decl_record, sig.signature, new_blocked_ty);
        }
      } else {
        LUAU_ASSERT!(false);
      }
    }

    ControlFlow::None
  }

  /// 由 `__init` 的签名派生 `ClassName.new` 的签名：去掉首参 self、保留变长尾，
  /// 返回值固定为类实例，并从泛型表中裁掉 self 与返回包。
  ///
  /// 前提（由调用点保证，与 cpp 同构）：`record` 为调用方 `&mut` 借用的局部记录、
  /// `new_blocked_ty` 是登记在该记录上的 arena BlockedType 占位；`signature` 若非
  /// FunctionType 则整段跳过（cpp `get<FunctionType>` 判空后提前返回）。
  fn bind_new_constructor(
    &mut self,
    record: &mut ClassDeclRecord,
    signature: TypeId,
    new_blocked_ty: TypeId,
  ) {
    // 对照 C++:2624 `const FunctionType* initFn = get<FunctionType>(sig.signature);`
    let Some(init_fn) = get_type::get::<function_type::FunctionType>(signature) else {
      return;
    };

    // C++:2629-2648：跳过 self 后按迭代器重建参数包（变长尾原样保留）。
    let mut iter = begin(init_fn.arg_types);
    let end_iter = end_type_pack_id(init_fn.arg_types);
    let new_args = if iter == end_iter {
      iter
        .tail()
        .unwrap_or(self.builtin_types.get().empty_type_pack)
    } else {
      iter.advance();
      type_pack_from_iterator(self.arena.get_mut(), &mut iter, &end_iter)
    };

    let mut new_fn_type = init_fn.clone();
    new_fn_type.arg_types = new_args;
    new_fn_type.ret_types = self
      .arena
      .get_mut()
      .add_type_pack_initializer_list_type_id(&[record.ty]);
    new_fn_type.has_self = false;

    // C++:2655-2662 从泛型列表里裁掉类实例类型与 `__init` 的返回包。
    new_fn_type.generics.retain(|g| *g != record.ty);
    new_fn_type
      .generic_packs
      .retain(|g| *g != init_fn.ret_types);

    let new_fn = self.arena.get_mut().add_type(new_fn_type);
    bind_type(new_blocked_ty, new_fn);
  }

  /// cpp `visit(const ScopePtr&, AstStatError*)`：`error` 为分发层经 RTTI 校验
  /// 后传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_error(&mut self, scope: &ScopePtr, error: &AstStatError) -> ControlFlow {
    // SAFETY: 循环元素取自 error 名下 statements/expressions 两个 AstArray，其
    // 元素均为 arena 存活节点；visit_stat/check_expr 收到同树节点
    // 满足其 cpp 同构契约。
    for stat in error.statements.iter_nodes() {
      self.visit_stat(scope, stat);
    }
    for expr in error.expressions.iter_nodes() {
      self.check_expr(scope, expr);
    }
    ControlFlow::None
  }

  // ConstraintGenerator::visit(const ScopePtr&, AstStatCompoundAssign*)
  // (ConstraintGenerator.cpp:2046).
  /// cpp `visit(const ScopePtr&, AstStatCompoundAssign*)`：`assign` 为分发层经
  /// RTTI 校验后传入的节点共享借用，本方法只读取其字段。
  pub fn visit_stat_compound_assign(
    &mut self,
    scope: &ScopePtr,
    assign: &AstStatCompoundAssign,
  ) -> ControlFlow {
    let result_ty: TypeId = self
      .check_ast_expr_binary(
        scope,
        assign.base.base.location,
        assign.op,
        assign.var.as_ptr(),
        assign.value.as_ptr(),
        None,
      )
      .ty;
    // module->astCompoundAssignResultTypes[assign] = resultTy（键为节点地址，
    // 与 cpp 同值；收口在 module_slots 门面）。
    record_stat_type(self.module.as_ref().unwrap(), assign, result_ty);
    // NOTE: We do not update lvalues for compound assignments. This is
    // intentional.
    ControlFlow::None
  }
}
