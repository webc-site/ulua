use alloc::vec::Vec;
use core::ptr::{null, null_mut};

use ulua_ast::{
  functions::is_l_value::is_l_value,
  records::{
    ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_index_name::AstExprIndexName,
    ast_expr_local::AstExprLocal, ast_expr_varargs::AstExprVarargs, ast_node::AstNode,
    location::Location,
  },
  rtti::{ast_node_is_ptr, ast_node_try_as_ptr},
};
use ulua_common::{LUAU_ASSERT, fflag};

use super::constraint_generator_prototype_type_definitions::make_binding;
use crate::{
  enums::{polarity::Polarity, type_context::TypeContext, value::Value},
  functions::{
    add_all_as_dependencies::add_all_as_dependencies, arc_as_mut::arc_as_mut,
    begin_type::begin_union_type, checkpoint::checkpoint, extend_type_pack::extend_type_pack,
    follow_type::follow, for_each_constraint::for_each_constraint, get_mutable_type,
    get_mutable_type_pack, get_type, is_table_union::is_table_union, match_assert::match_assert,
    match_is_instance_guard::match_is_instance_guard, match_set_metatable::match_set_metatable,
    should_suppress_errors_type_utils::should_suppress_errors,
    should_typestate_for_first_argument::should_typestate_for_first_argument,
  },
  records::{
    arena_handle::Handle, blocked_type::BlockedType, blocked_type_pack::BlockedTypePack,
    checkpoint::Checkpoint, constraint::Constraint, constraint_generator::ConstraintGenerator,
    function_call_constraint::FunctionCallConstraint,
    function_check_constraint::FunctionCheckConstraint,
    in_conditional_context::InConditionalContext, inference_pack::InferencePack,
    metatable_type::MetatableType, scope::Scope, symbol::Symbol, type_pack::TypePack,
    union_builder::UnionBuilder, union_type::UnionType, unpack_constraint::UnpackConstraint,
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

    // SAFETY: 经 Arc 而非共享引用派生可变指针：后续会经 scope_raw 写入 bindings。
    let scope_raw: *mut Scope = arc_as_mut(scope);

    let mut expr_args: Vec<*mut AstExpr> = Vec::new();

    let mut return_refinements: Vec<RefinementId> = Vec::new();
    let mut discriminant_types: Vec<Option<TypeId>> = Vec::new();

    if call.self_ {
      // cpp `expr->func->as<AstExprIndexName>()` 判型+判空折叠为 Option；
      // 未命中经 ice_string 抛出 InternalCompilerError（内部 panic_any，
      // 与原代码 null 分支后不可达的解引用同理不再可能触达）。
      // SAFETY: call.func 是存活节点名下的 arena 子指针；try_as_ptr 先判空
      // 再按类索引甄别，命中即存活只读借用。
      let index_expr = match unsafe { ast_node_try_as_ptr::<AstExprIndexName>(call.func) } {
        Some(i) => i,
        None => {
          self
            .ice
            .get()
            .ice_string("method call expression has no 'self'");
          unreachable!("ice_string 内部 panic_any，不外返")
        }
      };

      // expr 已句柄化恒非空；行走链/身份键为既有裸指针 API，经 as_ptr 桥接。
      expr_args.push(index_expr.expr.as_ptr());

      let key = self
        .dfg_ref()
        .get_refinement_key(index_expr.expr.as_ptr().cast_const());
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

    for &arg in call.args.iter() {
      expr_args.push(arg);

      let key = self.dfg_ref().get_refinement_key(arg as *const AstExpr);
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

    let expected_types_for_call: Vec<Option<TypeId>> =
      self.get_expected_call_types_for_function_overloads(fn_type);

    if let Some(module) = &self.module {
      let module_ptr = arc_as_mut(module);
      // SAFETY: arc_as_mut 惯用法下的模块共享写句柄，身份指针仅作
      // ast_original_call_types 的键。对照 cpp `module->astOriginalCallTypes[expr->func]`。
      unsafe {
        *(*module_ptr)
          .ast_original_call_types
          .get_or_insert(call.func as *const AstNode) = fn_type;
      }
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
          let module_ptr = arc_as_mut(module);
          // SAFETY: 共享写句柄惯用法；此处仅按身份键只读查 ast_types。
          unsafe {
            (*module_ptr)
              .ast_types
              .find(&(arg as *const AstExpr))
              .copied()
          }
        } else {
          None
        };
        if let Some(ty) = self_ty {
          args.push(ty);
        } else {
          args.push(self.fresh_type(scope, Polarity::Negative));
        }
      } else if i < expr_args.len() - 1
        || !(unsafe { ast_node_is_ptr::<AstExprCall>(arg) }
          || unsafe { ast_node_is_ptr::<AstExprVarargs>(arg) })
      {
        let expected_type = expected_types_for_call.get(i).copied().flatten();
        // cpp:3022 InConditionalContext 仅在该分支包裹 check 调用。
        // SAFETY: 裸指针派生自 `&mut self.type_context` 真实借用，守卫仅存活
        // 于本迭代轮（cpp RAII flipper 同款作用域）。
        let _flipper = (i == 0 && match_assert(call))
          .then(|| InConditionalContext::new(&mut self.type_context, TypeContext::Condition));
        // SAFETY: arg 是 call.args/方法 self 实参链上的 arena 存活子节点，
        // 只取共享引用递归（cpp 直接透传同一指针）。
        let inference = self.check_expr_full(scope, unsafe { &*arg }, expected_type, false, false);
        args.push(inference.ty);
        argument_refinements.push(inference.refinement);
      } else {
        let mut expected_types: Vec<Option<TypeId>> = Vec::new();
        if let Some(rest) = expected_types_for_call.get(i..) {
          expected_types.extend_from_slice(rest);
        }
        // SAFETY: arg 同上为存活子表达式，满足 check_pack 的 cpp 指针契约。
        let pack = unsafe {
          self.check_pack_scope_ptr_ast_expr_vector_optional_type_id_bool(
            scope,
            arg,
            &expected_types,
            true,
          )
        };
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
        // SAFETY: arena 为构造期注入的存活句柄（可变借用止于本调用，crate
        // 不变量 2）；Handle::from_ptr 实参为非空 builtin_types 单例；t 是
        // 存活的 arg_tail TypePackId（cpp extendTypePack 同款实参）。
        arg_tail_pack = unsafe {
          extend_type_pack(
            self.arena.get_mut(),
            Handle::from_ptr(self.builtin_types.as_ptr()),
            t,
            2 - args.len(),
            Vec::new(),
          )
        };
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
      // SAFETY: normalizer 为构造期注入的存活句柄，满足 should_suppress_errors
      // 自身的指针契约（cpp `shouldSuppressErrors(normalizer, mt)`）。
      if unsafe { should_suppress_errors(self.normalizer.as_ptr(), mt) }.value == Value::Suppress {
        mt = self.builtin_types.get().any_type;
      }

      // 进入本分支要求 match_set_metatable 命中，其判定包含
      // 「func 为 setmetatable 名字且 args.size >= 1」（cpp 同款前提），
      // 此处只读出该元素指针值作后续身份键/判型实参。
      let target_expr = call.args[0];

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

      // SAFETY: cpp `targetExpr->as<AstExprLocal>()`——target_expr 是存活
      // arena 子节点；try_as_ptr 先判空再按类索引甄别，命中即存活只读借用。
      if let Some(target_local) = unsafe { ast_node_try_as_ptr::<AstExprLocal>(target_expr) } {
        // local 槽已句柄化恒非空；Symbol::from_local 为既有裸指针 API，经 as_ptr 桥接。
        let symbol = Symbol::from_local(target_local.local.as_ptr());
        // C++ `scope->bindings[targetLocal->local].typeId = resultTy` — the
        // operator[] default-constructs a Binding when absent.
        // SAFETY: scope_raw 为本函数开头按 arc_as_mut 契约派生的存活写句柄，
        // 此处单线程独占写 bindings 表。
        unsafe {
          if let Some(binding) = (*scope_raw).bindings.get_mut(&symbol) {
            binding.type_id = result_ty;
          } else {
            (*scope_raw)
              .bindings
              .insert(symbol, make_binding(result_ty, Location::default()));
          }
        }

        let def = self.dfg_ref().get_def(target_expr as *const AstExpr);
        // SAFETY: 同上，scope_raw 存活写句柄，独占写 lvalue_types 一张表。
        unsafe {
          *(*scope_raw).lvalue_types.get_or_insert(def) = result_ty; // TODO: typestates: track this as an assignment
          self.update_r_value_refinements_scope_def_id_type_id(scope_raw, def, result_ty);
        } // TODO: typestates: track this as an assignment

        // HACK: If we have a targetLocal, it has already been added to the
        // inferredBindings table.  We want to replace it so that we don't
        // infer a weird union like tbl | { @metatable something, tbl }
        let ib_symbol = Symbol::from_local(target_local.local.as_ptr());
        if let Some(ib) = self.inferred_bindings.find_mut(&ib_symbol) {
          ib.types.erase_type_id(target);
        }

        self.record_inferred_binding(target_local.local.as_ptr(), result_ty);
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

      if let Some(def) = self
        .dfg_ref()
        .get_def_optional(target_expr as *const AstExpr)
      {
        // SAFETY: scope_raw 存活写句柄，独占写 lvalue_types 表（cpp
        // `scope->lvalueTypes[*def] = resultTy`）。
        unsafe {
          *(*scope_raw).lvalue_types.get_or_insert(def) = result_ty;
          self.update_r_value_refinements_scope_def_id_type_id(scope_raw, def, result_ty);
        }
      }
    }

    if match_assert(call) && !argument_refinements.is_empty() {
      // SAFETY: match_assert 命中蕴含 args 非空（其判定含 is_empty 分支），
      // 元素为存活子表达式，仅读 location（cpp `call->args.data[0]->location`）。
      let first_loc = unsafe { (*call.args[0]).base.location };
      self.apply_refinements(scope, first_loc, argument_refinements[0]);
    }

    // TODO: How do expectedTypes play into this?  Do they?
    let rets: TypePackId = self.arena.get_mut().add_type_pack_t(BlockedTypePack {
      index: 0,
      owner: null_mut(),
    });
    let arg_pack: TypePackId = self.add_type_pack(args, arg_tail);

    let (explicit_type_ids, explicit_type_pack_ids): (Vec<TypeId>, Vec<TypePackId>) =
      if fflag::LuauExplicitTypeInstantiationSupport.get() && call.type_arguments.size != 0 {
        // scope_raw 为 arc_as_mut 契约下的存活写句柄，被调 safe fn 按其签名
        // 消费（cpp `resolveTypeArguments(scope, call->typeArguments)`）。
        self.resolve_type_arguments(scope_raw, call.type_arguments)
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

    // SAFETY: `(*(*call.func))` 的 location 读——func 是存活节点名下 arena 子指针。
    let func_location = unsafe { (*call.func).base.location };

    let check_constraint: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
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
            let mp = arc_as_mut(m);
            // SAFETY: arc_as_mut 契约下的共享写句柄，仅取其字段地址作只读
            // 表指针存入约束（cpp `NotNull{&module->astTypes}`）。
            unsafe { &(*mp).ast_types as *const _ }
          })
          .unwrap_or(null()),
        ast_expected_types: self
          .module
          .as_ref()
          .map(|m| {
            let mp = arc_as_mut(m);
            // SAFETY: 同上。
            unsafe { &(*mp).ast_expected_types as *const _ }
          })
          .unwrap_or(null()),
      }),
    );

    if fflag::LuauConstraintGraph.get() {
      add_all_as_dependencies(func_begin, func_end, self, check_constraint);
    } else {
      for_each_constraint(func_begin, func_end, self, |constraint| {
        // SAFETY: 区间内约束皆 `Box::into_raw` 堆分配、会话内存活；向
        // check_constraint（区间外新建）的依赖表追加，与被遍历数组互不重叠。
        unsafe { (*check_constraint).deprecated_dependencies.push(constraint) };
      });
    }

    let call_constraint: *mut Constraint = self.add_constraint_scope_ptr_location_constraint_v(
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
            let mp = arc_as_mut(m);
            // SAFETY: arc_as_mut 契约下的共享写句柄取字段地址（cpp 存
            // `&module->astOverloadResolvedTypes`）。
            unsafe { &mut (*mp).ast_overload_resolved_types as *mut _ }
          })
          .unwrap_or(null_mut()),
      }),
    );

    // rets 刚由 add_type_pack_t(BlockedTypePack) 分配，必命中；对照 C++:3085
    // `getMutable<BlockedTypePack>(rets)->owner = callConstraint.get();`
    get_mutable_type_pack::get_mutable::<BlockedTypePack>(rets)
      .expect("rets 刚由 add_type_pack_t(BlockedTypePack) 分配，必命中（对照 C++:3085）")
      .owner = call_constraint;

    if fflag::LuauConstraintGraph.get() {
      // SAFETY: 本分支由 `LuauConstraintGraph` flag 守卫，宿主仅在 flag 开启时
      // 注入非空 `self.cgraph`；check/call 两枚约束为本函数刚经 addConstraint
      // 造出的存活堆分配节点，区间内 `constraint` 同理由 for_each_constraint
      // 顺序读出（cpp `cgraph->addDependencyOf` 同款实参）。
      unsafe {
        (*self.cgraph)
          .add_dependency_of_constraint_constraint(&mut *check_constraint, &mut *call_constraint);
      }
      for_each_constraint(
        arg_begin_checkpoint,
        arg_end_checkpoint,
        self,
        |constraint| {
          // SAFETY: 同上——flag 守卫下 cgraph 非空存活，三枚约束指针皆会话内存活节点。
          unsafe {
            (*self.cgraph)
              .add_dependency_of_constraint_constraint(&mut *check_constraint, &mut *constraint);
            (*self.cgraph)
              .add_dependency_of_constraint_constraint(&mut *constraint, &mut *call_constraint);
          }
        },
      );
    } else {
      // SAFETY: call/check 两枚约束为本函数刚造出的存活堆分配节点。
      unsafe {
        (*call_constraint)
          .deprecated_dependencies
          .push(check_constraint);
      }
      for_each_constraint(
        arg_begin_checkpoint,
        arg_end_checkpoint,
        self,
        |constraint| {
          // SAFETY: 区间内 constraint 与会话内存活的 check/call 同法解引用，
          // 仅写各自的 deprecated_dependencies 字段（cpp 降级依赖链）。
          unsafe {
            (*constraint).deprecated_dependencies.push(check_constraint);
            (*call_constraint).deprecated_dependencies.push(constraint);
          }
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
