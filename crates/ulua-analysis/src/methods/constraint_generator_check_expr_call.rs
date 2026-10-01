use alloc::vec::Vec;
use core::ptr::{NonNull, from_ref, null, null_mut};

use ulua_ast::{
  functions::is_l_value::is_l_value,
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
    location::Location,
  },
  rtti::{ast_node_is, ast_node_try_as},
};
use ulua_common::{LUAU_ASSERT, fflag};

use super::constraint_generator_prototype_type_definitions::make_binding;
use crate::{
  enums::{polarity::Polarity, type_context::TypeContext, value::Value},
  functions::{
    add_all_as_dependencies::add_all_as_dependencies, begin_type::begin_union_type,
    checkpoint::checkpoint, extend_type_pack::extend_type_pack, follow_type::follow,
    for_each_constraint::for_each_constraint, get_mutable_type, get_mutable_type_pack, get_type,
    is_table_union::is_table_union, match_assert::match_assert,
    match_is_instance_guard::match_is_instance_guard, match_set_metatable::match_set_metatable,
    shared_mut::shared_mut, should_suppress_errors_type_utils::should_suppress_errors,
    should_typestate_for_first_argument::should_typestate_for_first_argument,
  },
  records::{
    arena_handle::{Handle, alias, alias_ref},
    blocked_type::BlockedType,
    blocked_type_pack::BlockedTypePack,
    checkpoint::Checkpoint,
    constraint::Constraint,
    constraint_generator::ConstraintGenerator,
    function_call_constraint::FunctionCallConstraint,
    function_check_constraint::FunctionCheckConstraint,
    in_conditional_context::InConditionalContext,
    inference_pack::InferencePack,
    metatable_type::MetatableType,
    symbol::Symbol,
    type_pack::TypePack,
    union_builder::UnionBuilder,
    union_type::UnionType,
    unpack_constraint::UnpackConstraint,
  },
  type_aliases::{
    constraint_v::ConstraintV,
    refinement_id_refinement::{NULL_REFINEMENT_ID, RefinementId},
    scope_ptr_type::ScopePtr,
    type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl ConstraintGenerator {
  /// cpp `checkPack` 内部的 `checkExprCall`（ConstraintGenerator.cpp:2916 起）：
  /// `call` 为分派链上受检的调用表达式共享借用（cpp `AstExprCall*` 形参的
  /// Rust 对应），本函数只读取其字段；写入全部落在 Scope/arena/约束表上。
  /// 约束/模块记录内以裸指针为键/值的槽位（`call_site`、`ast_types` 键、
  /// `cgraph` 句柄等）保持记录布局，仅在取地址/还原地址处收敛。
  pub fn check_expr_call(
    &mut self,
    scope: &ScopePtr,
    call: &AstExprCall,
    fn_type: TypeId,
    func_begin: Checkpoint,
    func_end: Checkpoint,
  ) -> InferencePack {
    // 约束实体的 call_site 裸指针：cpp 约束里存的就是 `AstExprCall*`，求解侧
    // 只读其 location；地址派生自本共享借用，从不据此写入。
    let call_ptr: *mut AstExprCall = (call as *const AstExprCall).cast_mut();

    // §2：实参行走链由裸指针数组收口为共享借用数组——各元素皆存活节点名下的
    // arena 子表达式（iter_nodes/句柄 get 给出引用）；ast_types/getRefinementKey
    // 等身份键在消费点经 from_ref 从引用还原地址（与 cpp 指针同一）。
    let mut expr_args: Vec<&AstExpr> = Vec::new();

    let mut return_refinements: Vec<RefinementId> = Vec::new();
    let mut discriminant_types: Vec<Option<TypeId>> = Vec::new();

    if call.self_ {
      // cpp `expr->func->as<AstExprIndexName>()` 判型+判空折叠为 Option；
      // 未命中经 ice_string 抛出 InternalCompilerError（内部 panic_any，
      // 与原代码 null 分支后不可达的解引用同理不再可能触达）。
      // call.func 是存活节点名下的 arena 子指针；alias_ref 收口后 ast_node_try_as
      // 判型下转（引用进、Option<&T> 出）。
      let index_expr = match ast_node_try_as::<AstExprIndexName>(alias_ref(call.func)) {
        Some(i) => i,
        None => {
          self
            .ice
            .get()
            .ice_string("method call expression has no 'self'");
          unreachable!("ice_string 内部 panic_any，不外返")
        }
      };

      // expr 已句柄化恒非空，.get() 直出共享借用入行走链。
      let receiver = index_expr.expr.get();
      expr_args.push(receiver);

      let key = self.dfg_ref().get_refinement_key(from_ref(receiver));
      if !key.is_null() {
        let discriminant_ty = self.arena.get_mut().add_type(BlockedType::default());
        return_refinements.push(
          self
            .refinement_arena
            .implicit_proposition_refinement_key_type_id(key, discriminant_ty)
            // §2：`return_refinements` 为直存可空句柄的数据槽，`None`（原 null）
            // 以定义处收口的具名哨兵落槽；非空 key 守卫下此处恒为 `Some`。
            .unwrap_or(NULL_REFINEMENT_ID),
        );
        discriminant_types.push(Some(discriminant_ty));
      } else {
        discriminant_types.push(None);
      }
    }

    for arg in call.args.iter_nodes() {
      expr_args.push(arg);

      let key = self.dfg_ref().get_refinement_key(from_ref(arg));
      if !key.is_null() {
        let discriminant_ty = self.arena.get_mut().add_type(BlockedType::default());
        return_refinements.push(
          self
            .refinement_arena
            .implicit_proposition_refinement_key_type_id(key, discriminant_ty)
            // §2：同上——具名哨兵落槽。
            .unwrap_or(NULL_REFINEMENT_ID),
        );
        discriminant_types.push(Some(discriminant_ty));
      } else {
        discriminant_types.push(None);
      }
    }

    let expected_types_for_call: Vec<Option<TypeId>> =
      self.get_expected_call_types_for_function_overloads(fn_type);

    if let Some(module) = &self.module {
      let module_ptr = shared_mut(module);
      // alias 收口下的模块写句柄，身份指针仅作 ast_original_call_types 的键。
      // 对照 cpp `module->astOriginalCallTypes[expr->func]`。
      *alias(module_ptr)
        .ast_original_call_types
        .get_or_insert(call.func as *const AstNode) = fn_type;
    }

    let arg_begin_checkpoint = checkpoint(self);

    let mut args: Vec<TypeId> = Vec::new();
    let mut arg_tail: Option<TypePackId> = None;
    let mut argument_refinements: Vec<RefinementId> = Vec::new();

    for (i, &arg) in expr_args.iter().enumerate() {
      if i == 0 && call.self_ {
        // The self type has already been computed as a side effect of
        // computing fnType.  If computing that did not cause us to exceed a
        // recursion limit, we can fetch it from ast_types rather than
        // recomputing it.
        let self_ty: Option<TypeId> = if let Some(module) = &self.module {
          let module_ptr = shared_mut(module);
          // alias 收口写句柄；此处仅按身份键只读查 ast_types（from_ref 还原地址）。
          alias(module_ptr).ast_types.find(&from_ref(arg)).copied()
        } else {
          None
        };
        if let Some(ty) = self_ty {
          args.push(ty);
        } else {
          args.push(self.fresh_type(scope, Polarity::Negative));
        }
      } else if i < expr_args.len() - 1
        || !(ast_node_is::<AstExprCall>(arg) || ast_node_is::<AstExprVarargs>(arg))
      {
        let expected_type = expected_types_for_call.get(i).copied().flatten();
        // cpp:3022 InConditionalContext 仅在该分支包裹 check 调用。
        let _flipper = (i == 0 && match_assert(call))
          .then(|| InConditionalContext::new(&mut self.type_context, TypeContext::Condition));
        // arg 是 call.args/方法 self 实参链上的 arena 存活子节点引用，直传递归
        // （cpp 直接透传同一指针）。
        let inference = self.check_expr_full(scope, arg, expected_type, false, false);
        args.push(inference.ty);
        argument_refinements.push(inference.refinement);
      } else {
        let mut expected_types: Vec<Option<TypeId>> = Vec::new();
        if let Some(rest) = expected_types_for_call.get(i..) {
          expected_types.extend_from_slice(rest);
        }
        // arg 同上为存活子表达式引用，直传满足 check_pack 引用契约。
        let pack = self.check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
          scope,
          arg,
          &expected_types,
          true,
        );
        arg_tail = Some(pack.tp);
        argument_refinements.extend(pack.refinements.iter().copied());
      }
    }

    let arg_end_checkpoint = checkpoint(self);

    if fflag::DebugLuauUserDefinedClasses.get() {
      let instance_guard = match_is_instance_guard(call, self.dfg_ref());
      if !instance_guard.is_null() && args.len() >= 2 {
        // The class type may not be solved yet (e.g. `A.Point` from a
        // required module).
        let objectof_inst = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.objectof_func,
          alloc::vec![args[1]],
          Vec::new(),
          scope,
          call.base.base.location,
        );
        return_refinements.push(
          self
            .refinement_arena
            .implicit_proposition_refinement_key_type_id(instance_guard, objectof_inst)
            // §2：同上——具名哨兵落槽；非空 instance_guard 下恒为 `Some`。
            .unwrap_or(NULL_REFINEMENT_ID),
        );
      }
    }

    if match_set_metatable(call) {
      let mut arg_tail_pack = TypePack::empty();
      if args.len() < 2
        && let Some(t) = arg_tail
      {
        // extend_type_pack 已 safe 化（形参全为受检类型）。
        arg_tail_pack = extend_type_pack(
          self.arena.get_mut(),
          Handle::from_ptr(self.builtin_types.as_ptr()),
          t,
          2 - args.len(),
          Vec::new(),
        );
      }

      let mut target: TypeId;
      let mut mt: TypeId;

      if args.len() + arg_tail_pack.head.len() == 2 {
        target = if !args.is_empty() {
          args[0]
        } else {
          arg_tail_pack.head[0]
        };
        mt = if args.len() > 1 {
          args[1]
        } else {
          arg_tail_pack.head[if args.is_empty() { 1 } else { 0 }]
        };
      } else {
        let mut unpacked_types: Vec<TypeId> = Vec::new();
        if !args.is_empty() {
          target = follow(args[0]);
        } else {
          target = self.arena.get_mut().add_type(BlockedType::default());
          unpacked_types.push(target);
        }

        mt = self.arena.get_mut().add_type(BlockedType::default());
        unpacked_types.push(mt);

        let c = self.add_constraint_scope_ptr_location_constraint_v(
          scope,
          call.base.base.location,
          ConstraintV::Unpack(UnpackConstraint {
            result_pack: unpacked_types,
            source_pack: arg_tail
              .expect("cpp 同位 `*argTail` 直 deref：setmetatable 变参形态必有尾包"),
          }),
        );
        // mt 刚由 add_type(BlockedType) 分配，必命中；对照 C++:2965
        // `getMutable<BlockedType>(mt)->setOwner(c);`
        get_mutable_type::get_mutable::<BlockedType>(mt)
          .expect("mt 刚由 add_type(BlockedType) 分配，必命中（对照 C++:2965）")
          .set_owner(c as *const Constraint);
        // 对照 C++:2966 `if (auto bt = getMutable<BlockedType>(target); bt && bt->getOwner() == nullptr)`
        if let Some(b) = get_mutable_type::get_mutable::<BlockedType>(target)
          && b.get_owner().is_null()
        {
          b.set_owner(c as *const Constraint);
        }
      }

      LUAU_ASSERT!(!target.is_null());
      LUAU_ASSERT!(!mt.is_null());

      target = follow(target);
      // normalizer 为构造期注入的存活句柄，经 Handle::get_mut 以 &mut 直传
      // should_suppress_errors（cpp `shouldSuppressErrors(normalizer, mt)`）。
      if should_suppress_errors(self.normalizer.get_mut(), mt).value == Value::Suppress {
        mt = self.builtin_types.get().any_type;
      }

      // 进入本分支要求 match_set_metatable 命中，其判定包含
      // 「func 为 setmetatable 名字且 args.size >= 1」（cpp 同款前提），
      // alias_ref 收口为存活子节点共享引用，判型直用、身份键处 from_ref 还原地址。
      let target_expr = alias_ref(call.args[0]);

      let result_ty: TypeId = if is_table_union(target) {
        // 对照 C++:2983 `const UnionType* targetUnion = get<UnionType>(target);`
        let target_union = get_type::get::<UnionType>(target)
          .expect("上一行 is_table_union(target) 判据蕴含 UnionType");
        let mut ub = UnionBuilder::new(self.arena, self.builtin_types);

        // C++ `for (TypeId ty : targetUnion)`——UnionTypeIterator 展平
        // 嵌套 union 并 follow,裸遍历 options 会漏掉嵌套成员。
        for ty in begin_union_type(target_union) {
          ub.add(self.arena.get_mut().add_type(MetatableType {
            table: ty,
            metatable: mt,
            synthetic_name: None,
          }));
        }

        ub.build()
      } else {
        self.arena.get_mut().add_type(MetatableType {
          table: target,
          metatable: mt,
          synthetic_name: None,
        })
      };

      // cpp `targetExpr->as<AstExprLocal>()`——target_expr 是存活
      // arena 子节点的共享引用；ast_node_try_as 判型下转。
      if let Some(target_local) = ast_node_try_as::<AstExprLocal>(target_expr) {
        // local 槽已句柄化恒非空；Symbol::from_local 为既有裸指针 API，经 as_ptr 桥接。
        let symbol = Symbol::from_local(target_local.local.as_ptr());
        // C++ `scope->bindings[targetLocal->local].typeId = resultTy` — the
        // operator[] default-constructs a Binding when absent.
        // 写窗口经 alias 门面即时物化，单线程独占写 bindings 表。
        let scope_write = alias(shared_mut(scope));
        if let Some(binding) = scope_write.bindings.get_mut(&symbol) {
          binding.type_id = result_ty;
        } else {
          scope_write
            .bindings
            .insert(symbol, make_binding(result_ty, Location::default()));
        }

        let def = self.dfg_ref().get_def(from_ref(target_expr));
        *scope_write.lvalue_types.get_or_insert(def) = result_ty; // TODO: typestates: track this as an assignment
        self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, def, result_ty);
        // TODO: typestates: track this as an assignment

        // HACK: If we have a targetLocal, it has already been added to the
        // inferredBindings table.  We want to replace it so that we don't
        // infer a weird union like tbl | { @metatable something, tbl }
        let ib_symbol = Symbol::from_local(target_local.local.as_ptr());
        if let Some(ib) = self.inferred_bindings.find_mut(&ib_symbol) {
          ib.types.erase_type_id(target);
        }

        self.record_inferred_binding(target_local.local.get(), result_ty);
      }

      return InferencePack {
        tp: self
          .arena
          .get_mut()
          .add_type_pack_initializer_list_type_id(&[result_ty]),
        refinements: alloc::vec![
          self
            .refinement_arena
            .variadic_refinement_ids(&return_refinements)
            // §2：cpp `InferencePack{ty, {variadic(...)}}` 恒存一个（可空）元素，
            // 且下游按位取 `argument_refinements[0]`——`None`（原 null）须以
            // 具名哨兵落槽保持元素位次不变。
            .unwrap_or(NULL_REFINEMENT_ID)
        ],
      };
    }

    if should_typestate_for_first_argument(call)
      && let Some(target_expr) = call.args.iter_nodes().next()
      && is_l_value(target_expr)
    {
      let result_ty = self.arena.get_mut().add_type(BlockedType::default());

      if let Some(def) = self.dfg_ref().get_def_optional(from_ref(target_expr)) {
        // 写窗口经 alias 门面即时物化（cpp `scope->lvalueTypes[*def] = resultTy`）。
        *alias(shared_mut(scope)).lvalue_types.get_or_insert(def) = result_ty;
        self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, def, result_ty);
      }
    }

    if match_assert(call) && !argument_refinements.is_empty() {
      // match_assert 命中蕴含 args 非空（其判定含 is_empty 分支），
      // 元素为存活子表达式，仅读 location（cpp `call->args.data[0]->location`）。
      let first_loc = alias_ref(call.args[0]).base.location;
      self.apply_refinements(scope, first_loc, argument_refinements[0]);
    }

    // TODO: How do expectedTypes play into this?  Do they?
    let rets: TypePackId = self.arena.get_mut().add_type_pack_t(BlockedTypePack {
      index: 0,
      owner: None,
    });
    let arg_pack: TypePackId = self.add_type_pack(args, arg_tail);

    let (explicit_type_ids, explicit_type_pack_ids): (Vec<TypeId>, Vec<TypePackId>) =
      if fflag::LuauExplicitTypeInstantiationSupport.get() && call.type_arguments.size != 0 {
        self.resolve_type_arguments(scope, call.type_arguments)
      } else {
        (Vec::new(), Vec::new())
      };

    // cpp ConstraintGenerator.cpp:3111 的局部 ftv 在 C++ 中即未被后续使用，不移植。

    /*
     * To make bidirectional type checking work, we need to solve these constraints in a particular order:
     *
     * 1. Solve the function type
     * 2. Propagate type information from the function type to the argument type_arguments
     * 3. Solve the argument type_arguments
     * 4. Solve the call
     */

    // func 是存活节点名下 arena 子指针，alias_ref 收口读 location。
    let func_location = alias_ref(call.func).base.location;

    let check_constraint = self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      func_location,
      ConstraintV::FunctionCheck(FunctionCheckConstraint {
        fn_type,
        args_pack: arg_pack,
        call_site: call_ptr,
        ast_types: self
          .module
          .as_ref()
          .map(|m| {
            let mp = shared_mut(m);
            // alias 收口写句柄，仅取其字段地址作只读表指针存入约束
            //（cpp `NotNull{&module->astTypes}`）。
            &alias(mp).ast_types as *const _
          })
          .unwrap_or(null()),
        ast_expected_types: self
          .module
          .as_ref()
          .map(|m| {
            let mp = shared_mut(m);
            // 同上。
            &alias(mp).ast_expected_types as *const _
          })
          .unwrap_or(null()),
      }),
    );

    if fflag::LuauConstraintGraph.get() {
      add_all_as_dependencies(func_begin, func_end, self, check_constraint);
    } else {
      for_each_constraint(func_begin, func_end, self, |constraint| {
        // 区间内约束皆 `Box::into_raw` 堆分配、会话内存活；向
        // check_constraint（区间外新建）的依赖表追加，与被遍历数组互不重叠。
        alias(check_constraint)
          .deprecated_dependencies
          .push(constraint);
      });
    }

    let call_constraint = self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      func_location,
      ConstraintV::FunctionCall(FunctionCallConstraint {
        fn_type,
        args_pack: arg_pack,
        result: rets,
        call_site: call_ptr,
        discriminant_types,
        type_arguments: explicit_type_ids,
        type_pack_arguments: explicit_type_pack_ids,
        ast_overload_resolved_types: self
          .module
          .as_ref()
          .map(|m| {
            let mp = shared_mut(m);
            // alias 收口写句柄取字段地址（cpp 存 `&module->astOverloadResolvedTypes`）。
            &mut alias(mp).ast_overload_resolved_types as *mut _
          })
          .unwrap_or(null_mut()),
      }),
    );

    // rets 刚由 add_type_pack_t(BlockedTypePack) 分配，必命中；对照 C++:3085
    // `getMutable<BlockedTypePack>(rets)->owner = callConstraint.get();`
    get_mutable_type_pack::get_mutable::<BlockedTypePack>(rets)
      .expect("rets 刚由 add_type_pack_t(BlockedTypePack) 分配，必命中（对照 C++:3085）")
      .owner = NonNull::new(call_constraint);

    if fflag::LuauConstraintGraph.get() {
      // 本分支由 `LuauConstraintGraph` flag 守卫，宿主仅在 flag 开启时
      // 注入非空 `self.cgraph`（记录布局保持裸指针句柄），解引用经 alias 收口；
      // check/call 两枚约束为本函数刚经 addConstraint 造出的存活堆分配节点，
      // 区间内 `constraint` 同理由 for_each_constraint 顺序读出
      //（cpp `cgraph->addDependencyOf` 同款实参）。
      let cgraph = alias(self.cgraph);
      cgraph
        .add_dependency_of_constraint_constraint(alias(check_constraint), alias(call_constraint));
      for_each_constraint(
        arg_begin_checkpoint,
        arg_end_checkpoint,
        self,
        |constraint| {
          let constraint = alias(constraint);
          cgraph.add_dependency_of_constraint_constraint(alias(check_constraint), constraint);
          cgraph.add_dependency_of_constraint_constraint(constraint, alias(call_constraint));
        },
      );
    } else {
      // call/check 两枚约束为本函数刚造出的存活堆分配节点，alias 收口写依赖链。
      alias(call_constraint)
        .deprecated_dependencies
        .push(check_constraint);
      for_each_constraint(
        arg_begin_checkpoint,
        arg_end_checkpoint,
        self,
        |constraint| {
          // 区间内 constraint 与会话内存活的 check/call 同法经 alias 解引用，
          // 仅写各自的 deprecated_dependencies 字段（cpp 降级依赖链）。
          alias(constraint)
            .deprecated_dependencies
            .push(check_constraint);
          alias(call_constraint)
            .deprecated_dependencies
            .push(constraint);
        },
      );
    }

    InferencePack {
      tp: rets,
      refinements: alloc::vec![
        self
          .refinement_arena
          .variadic_refinement_ids(&return_refinements)
          // §2：同前——`None`（原 null）以具名哨兵落槽，位次不变。
          .unwrap_or(NULL_REFINEMENT_ID)
      ],
    }
  }
}
