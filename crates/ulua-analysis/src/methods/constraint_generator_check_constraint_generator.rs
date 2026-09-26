use alloc::{string::String, vec::Vec};
use core::{mem::take, ptr::null_mut, str::from_utf8};

use ulua_ast::{
  records::{
    ast_expr::AstExpr,
    ast_expr_constant_bool::AstExprConstantBool,
    ast_expr_constant_integer::AstExprConstantInteger,
    ast_expr_constant_string::AstExprConstantString,
    ast_expr_function::AstExprFunction,
    ast_expr_global::AstExprGlobal,
    ast_expr_if_else::AstExprIfElse,
    ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName,
    ast_expr_instantiate::AstExprInstantiate,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_local::AstExprLocal,
    ast_expr_table::AstExprTable,
    ast_expr_type_assertion::AstExprTypeAssertion,
    ast_expr_unary::{AstExprUnary, AstExprUnaryOp},
  },
  rtti::{AstNodePtr, ast_node_is, ast_node_try_as_ptr},
};
use ulua_common::{fflag, fint, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::{polarity::Polarity, table_state::TableState, type_context::TypeContext},
  functions::{
    add_all_as_dependencies_and_chain_returns::add_all_as_dependencies_and_chain_returns,
    add_all_as_reverse_dependencies::add_all_as_reverse_dependencies, arc_as_mut::arc_as_mut,
    checkpoint::checkpoint, follow_type, for_each_constraint::for_each_constraint,
    get_mutable_type, has_free_type::has_free_type,
    inference_with_refinement::inference_with_refinement,
  },
  records::{
    arena_handle::alias_ref, blocked_type::BlockedType, constraint::Constraint,
    constraint_generator::ConstraintGenerator, free_type::FreeType,
    generalization_constraint::GeneralizationConstraint,
    has_indexer_constraint::HasIndexerConstraint, in_conditional_context::InConditionalContext,
    inference::Inference, primitive_type_constraint::PrimitiveTypeConstraint,
    property_type::Property, push_type_constraint::PushTypeConstraint,
    singleton_type::SingletonType, string_singleton::StringSingleton, table_indexer::TableIndexer,
    table_type::TableType, type_instantiation_constraint::TypeInstantiationConstraint,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, singleton_variant::SingletonVariant,
    type_id::TypeId,
  },
};

impl ConstraintGenerator {
  /// cpp `check(const ScopePtr&, AstExprConstantString*, optional<TypeId>,
  /// bool forceSingleton)`（ConstraintGenerator.cpp:3259）。
  pub fn check_expr_constant_string(
    &mut self,
    scope: &ScopePtr,
    string: &AstExprConstantString,
    expected_type: Option<TypeId>,
    force_singleton: bool,
  ) -> Inference {
    let string_str = from_utf8(string.value.as_bytes()).unwrap_or("");
    let string_singleton = StringSingleton::new(String::from(string_str));
    let singleton_type = SingletonType::new(SingletonVariant::V1(string_singleton));

    if force_singleton {
      return Inference::no_refinement(self.arena.get_mut().add_type(singleton_type));
    }

    if self.large_table_depth > 0 {
      return Inference::no_refinement(self.builtin_types.get().string_type);
    }

    let free_ty = self.fresh_type(scope, Polarity::Positive);
    // fresh_type 刚分配的必是 FreeType，对照 C++ `getMutable<FreeType>(freeTy)` 后的 LUAU_ASSERT
    let ft = get_mutable_type::get_mutable::<FreeType>(free_ty);
    LUAU_ASSERT!(ft.is_some());
    let ft = ft.expect("fresh_type 刚分配的必是 FreeType（cpp LUAU_ASSERT(ftv)）");
    ft.lower_bound = self.arena.get_mut().add_type(singleton_type);
    ft.upper_bound = self.builtin_types.get().string_type;

    self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      string.base.base.location,
      ConstraintV::PrimitiveType(PrimitiveTypeConstraint {
        free_type: free_ty,
        expected_type,
        primitive_type: self.builtin_types.get().string_type,
      }),
    );
    Inference::no_refinement(free_ty)
  }

  /// cpp `check(const ScopePtr&, AstExprConstantBool*, optional<TypeId>,
  /// bool forceSingleton)`（ConstraintGenerator.cpp:3285）。
  pub fn check_expr_constant_bool(
    &mut self,
    scope: &ScopePtr,
    bool_expr: &AstExprConstantBool,
    expected_type: Option<TypeId>,
    force_singleton: bool,
  ) -> Inference {
    let singleton_type = if bool_expr.value {
      self.builtin_types.get().true_type
    } else {
      self.builtin_types.get().false_type
    };
    if force_singleton {
      return Inference::no_refinement(singleton_type);
    }

    if self.large_table_depth > 0 {
      return Inference::no_refinement(self.builtin_types.get().boolean_type);
    }

    let free_ty = self.fresh_type(scope, Polarity::Positive);
    // fresh_type 刚分配的必是 FreeType，对照 C++ `getMutable<FreeType>(freeTy)->...` 直接解引用
    let ft = get_mutable_type::get_mutable::<FreeType>(free_ty)
      .expect("fresh_type 刚分配的必是 FreeType（cpp 直 deref getMutable<FreeType>）");
    ft.lower_bound = singleton_type;
    ft.upper_bound = self.builtin_types.get().boolean_type;

    self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      bool_expr.base.base.location,
      ConstraintV::PrimitiveType(PrimitiveTypeConstraint {
        free_type: free_ty,
        expected_type,
        primitive_type: self.builtin_types.get().boolean_type,
      }),
    );
    Inference::no_refinement(free_ty)
  }

  /// cpp `check(const ScopePtr&, AstExprLocal*)`（ConstraintGenerator.cpp:3316）。
  /// `local` 为分派层经类索引校验后传入的共享借用，本方法只读取其字段。
  pub fn check_expr_local(&mut self, scope: &ScopePtr, local: &AstExprLocal) -> Inference {
    // get_refinement_key 只按指针身份查表、不解引用；repr(C) 基址重合保证
    // 该地址与 cpp `dfg->getRefinementKey(local)` 的键同一。
    let key = self
      .dfg_ref()
      .get_refinement_key((local as *const AstExprLocal).cast::<AstExpr>());
    LUAU_ASSERT!(!key.is_null());

    let mut maybe_ty: Option<TypeId> = None;

    // if we have a refinement key, we can look up its type.
    // SAFETY: key 的非空由上一行断言给出；DFG 返回的 RefinementKey 驻留其
    // 内部 arena、随会话存活，此处只读取 `def`（cpp `key->def`）。
    if !key.is_null() {
      let def = unsafe { (*key).def };
      // C++ default `prototype = true`.
      maybe_ty = self.lookup(scope, local.base.base.location, def, true);
    }

    if let Some(ty) = maybe_ty {
      let ty = follow_type::follow(ty);

      // local 槽已句柄化恒非空；record_inferred_binding 为既有裸指针 API，经 as_ptr 桥接。
      self.record_inferred_binding(local.local.as_ptr(), ty);

      let refinement = self
        .refinement_arena
        .proposition_refinement_key_type_id(key, self.builtin_types.get().truthy_type);
      // §2：`None`（原 `Inference{ty, nullptr}` 形态）收口到 Option 构造器。
      inference_with_refinement(ty, refinement)
    } else {
      self
        .ice
        .get()
        .ice_string("CG: AstExprLocal came before its declaration?");
      Inference::no_refinement(self.builtin_types.get().error_type)
    }
  }

  /// cpp `check(const ScopePtr&, AstExprGlobal*)`（ConstraintGenerator.cpp:3344）。
  /// `global` 为分派层经类索引校验后传入的共享借用，本方法只读取其字段。
  pub fn check_expr_global(&mut self, scope: &ScopePtr, global: &AstExprGlobal) -> Inference {
    let key = self
      .dfg_ref()
      .get_refinement_key((global as *const AstExprGlobal).cast::<AstExpr>());
    LUAU_ASSERT!(!key.is_null());

    // SAFETY: 上一行断言保证 key 非空；RefinementKey 驻留 DFG 内部 arena、
    // 随会话存活，此处只读 `def`（cpp `key->def`）。
    let def = unsafe { (*key).def };

    // prepopulateGlobalScope() has already added all global functions to the environment by this point, so any
    // global that is not already in-scope is definitely an unknown symbol.
    if let Some(ty) = self.lookup(
      scope,
      global.base.base.location,
      def,
      /*prototype=*/ false,
    ) {
      let refinement = self
        .refinement_arena
        .proposition_refinement_key_type_id(key, self.builtin_types.get().truthy_type);
      // §2：`None`（原 `Inference{ty, nullptr}` 形态）收口到 Option 构造器。
      inference_with_refinement(ty, refinement)
    } else {
      Inference::no_refinement(self.builtin_types.get().error_type)
    }
  }

  /// cpp `check(const ScopePtr&, AstExprIndexName*)`（ConstraintGenerator.cpp:3420）。
  /// 与 local/global 不同，refinement key 允许为 null（cpp `checkIndexName` 显式
  /// 分支处理空 key，ConstraintGenerator.cpp:3367-3417），由被调方按该契约消费。
  pub fn check_expr_index_name(
    &mut self,
    scope: &ScopePtr,
    index_name: &AstExprIndexName,
  ) -> Inference {
    let key = self
      .dfg_ref()
      .get_refinement_key((index_name as *const AstExprIndexName).cast::<AstExpr>());
    let index: String = index_name.index.as_str_or_empty().to_string();
    // SAFETY: check_index_name 为 unsafe fn；index_name.expr 已句柄化恒非空，
    // 既有裸指针 API 经 as_ptr 桥接；key 为 DFG 内部只读节点或 null，cpp
    // checkIndexName 同款可空契约；index 为本地只读串。
    unsafe {
      self.check_index_name(
        scope,
        key,
        index_name.expr.as_ptr(),
        &index,
        index_name.index_location,
      )
    }
  }

  /// cpp `check(const ScopePtr&, AstExprIndexExpr*)`（ConstraintGenerator.cpp:3426）。
  /// `index_expr` 为分派层经类索引校验后传入的共享借用，本方法只读取其字段。
  pub fn check_expr_index_expr(
    &mut self,
    scope: &ScopePtr,
    index_expr: &AstExprIndexExpr,
  ) -> Inference {
    // SAFETY: index_expr.index 是存活节点名下的 arena 子指针；try_as_ptr 先判空
    // 再按类索引甄别，命中即存活 AstExprConstantString 的只读借用（cpp
    // `index->as<AstExprConstantString>()`）。
    if let Some(constant_string) =
      unsafe { ast_node_try_as_ptr::<AstExprConstantString>(index_expr.index) }
    {
      if let Some(module) = &self.module {
        let module_ptr = arc_as_mut(module);
        // SAFETY: module_ptr 为 arc_as_mut 契约下的模块共享写句柄，此处按 cpp
        // `module->astTypes[index] = builtinTypes->stringType` 写一条身份键表项。
        unsafe {
          // index 已句柄化恒非空；ast_types 身份键为既有指针形态，经 as_ptr 桥接。
          *(*module_ptr)
            .ast_types
            .get_or_insert(index_expr.index.as_ptr()) = self.builtin_types.get().string_type;
        }
      }
      let key = self
        .dfg_ref()
        .get_refinement_key((index_expr as *const AstExprIndexExpr).cast::<AstExpr>());
      let index: String = String::from(from_utf8(constant_string.value.as_bytes()).unwrap_or(""));
      // SAFETY: index_expr.expr 为存活节点名下的 arena 子表达式；key 可空由
      // check_index_name 的消费契约覆盖（cpp 同款）。
      return unsafe {
        self.check_index_name(
          scope,
          key,
          // expr 已句柄化；check_index_name 为既有裸指针 API，经 as_ptr 桥接。
          index_expr.expr.as_ptr(),
          &index,
          index_expr.base.base.location,
        )
      };
    }

    // .expr/.index 已句柄化：get() 只读借用出自存活 &AstExprIndexExpr，
    // 供递归与 location 读取（cpp 直接透传同一指针）。
    let subject = index_expr.expr.get();
    let index_node = index_expr.index.get();
    let obj = self.check_expr(scope, subject).ty;
    let index_type = self.check_expr(scope, index_node).ty;

    let result = self.arena.get_mut().add_type(BlockedType::default());

    let key = self
      .dfg_ref()
      .get_refinement_key((index_expr as *const AstExprIndexExpr).cast::<AstExpr>());
    if !key.is_null() {
      // SAFETY: key 非空刚判、驻留 DFG 内部 arena 随会话存活，只读 `def`。
      let def = unsafe { (*key).def };
      // C++ default `prototype = true`.
      if let Some(ty) = self.lookup(scope, index_expr.base.base.location, def, true) {
        let refinement = self
          .refinement_arena
          .proposition_refinement_key_type_id(key, self.builtin_types.get().truthy_type);
        // §2：`None`（原 `Inference{ty, nullptr}` 形态）收口到 Option 构造器。
        return inference_with_refinement(ty, refinement);
      }
      self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, def, result);
    }

    let c = self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      subject.base.location,
      ConstraintV::HasIndexer(HasIndexerConstraint {
        result_type: result,
        subject_type: obj,
        index_type,
      }),
    );
    // result 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3349
    // `getMutable<BlockedType>(result)->setOwner(c)`
    let blocked = get_mutable_type::get_mutable::<BlockedType>(result)
      .expect("result 刚由 add_type(BlockedType) 分配，必命中（cpp:3349）");
    blocked.set_owner(c as *const _);

    if !key.is_null() {
      let refinement = self
        .refinement_arena
        .proposition_refinement_key_type_id(key, self.builtin_types.get().truthy_type);
      // §2：`None`（原 `Inference{result, nullptr}` 形态）收口到 Option 构造器。
      inference_with_refinement(result, refinement)
    } else {
      Inference::no_refinement(result)
    }
  }

  /// cpp `check(const ScopePtr&, AstExprFunction*, optional<TypeId>, bool
  /// generalize)`（ConstraintGenerator.cpp:3457）。`func` 为分派层经类索引校验
  /// 后传入的共享借用，本方法只读取其字段。
  pub fn check_expr_function(
    &mut self,
    scope: &ScopePtr,
    func: &AstExprFunction,
    expected_type: Option<TypeId>,
    generalize: bool,
  ) -> Inference {
    // SAFETY: 裸指针派生自 `&mut self.type_context` 真实借用；守卫是本函数最先
    // 声明的局部量、函数末尾最后析构，其 Drop 回写即该字段的收尾访问，析构时
    // 不再有存活的 `self` 借用（C++ `InConditionalContext inContext(&typeContext)`
    // RAII 等价）。
    let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

    // 被调 check_function_signature（另一波次的 *mut 契约移植）仍按 cpp 引用
    // 形参收裸指针；约束生成期 AST 只读，该指针仅被只读消费。
    let func_ptr: *mut AstExprFunction = (func as *const AstExprFunction).cast_mut();

    let start_checkpoint = checkpoint(self);
    // SAFETY: func_ptr 指向会话存活 AstExprFunction（派生自本共享借用）；
    // enclosing_class 传 null_mut() 对应 C++ `checkFunctionSignature(scope,
    // nullptr, func, ...)` 的非类方法上下文（被调在 `DebugLuauUserDefinedClasses`
    // 关闭时断言该参数必须为空）。
    let sig =
      unsafe { self.check_function_signature(scope, null_mut(), func_ptr, expected_type, None) };

    self.interior_free_types.push(Default::default());
    self.check_function_body(&sig.body_scope, func);
    let end_checkpoint = checkpoint(self);

    let generalized_ty = self.arena.get_mut().add_type(BlockedType::default());
    let gc = self.add_constraint_scope_ptr_location_constraint_v(
      &sig.signature_scope,
      func.base.base.location,
      ConstraintV::Generalization(GeneralizationConstraint {
        generalized_type: generalized_ty,
        source_type: sig.signature,
        interior_types: Vec::new(),
        has_deprecated_attribute: false,
        deprecated_info: Default::default(),
        no_generics: false,
      }),
    );

    // SAFETY: `sig.signature_scope` 的 Arc 由局部 `sig` 持有、存活至函数结束，
    // `arc_as_mut` 得到本 crate 约定的 C++ const_cast 式共享写句柄
    // （C++ `sig.signatureScope->interiorFreeTypes =
    // std::move(interiorFreeTypes.back().types)`，单线程独占写，此刻无其它 `&mut
    // Scope` 别名）；栈上 `interior_free_types` 在本函数 push 后无嵌套残留 pop，
    // `last_mut().expect()` 必命中，`take` 即 C++ `std::move`。
    unsafe {
      let signature_scope = arc_as_mut(&sig.signature_scope);
      (*signature_scope).interior_free_types = Some(take(
        &mut self
          .interior_free_types
          .last_mut()
          .expect("本函数 push 后无嵌套 pop，栈帧配对必命中")
          .types,
      ));
      (*signature_scope).interior_free_type_packs = Some(take(
        &mut self
          .interior_free_types
          .last_mut()
          .expect("本函数 push 后无嵌套 pop，栈帧配对必命中")
          .type_packs,
      ));
    }
    self.interior_free_types.pop();

    // generalized_ty 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3432
    // `getMutable<BlockedType>(generalizedTy)->setOwner(gc)`
    let blocked = get_mutable_type::get_mutable::<BlockedType>(generalized_ty)
      .expect("generalized_ty 刚由 add_type(BlockedType) 分配，必命中（cpp:3432）");
    blocked.set_owner(gc as *const _);

    if fflag::LuauConstraintGraph.get() {
      // SAFETY: 本分支由 `LuauConstraintGraph` flag 守卫，宿主（`check_frontend`）仅在
      // flag 开启时构造 `ConstraintGraph` 并注入非空 `self.cgraph` 句柄、随会话存活；
      add_all_as_dependencies_and_chain_returns(start_checkpoint, end_checkpoint, self, gc);
    } else {
      let mut previous: *mut Constraint = null_mut();
      for_each_constraint(
        start_checkpoint,
        end_checkpoint,
        self,
        |constraint: *mut Constraint| {
          // SAFETY: `constraint` 由 `for_each_constraint` 从
          // `self.constraints[start.offset..end.offset]` 顺序读出，皆指向
          // `add_constraint` 中 `Box::into_raw` 堆分配、会话内持续存活的 `Constraint`；
          // `gc` 指向同法新建、落在区间之外的约束，向其 `deprecated_dependencies`
          // 追加与被借只读的指针数组互不重叠（对应 C++ 图关闭时的降级依赖链）。
          unsafe { (*gc).deprecated_dependencies.push(constraint) };

          // SAFETY: 同上，`constraint` 为区间内存活节点，此处仅对其 `.c` 字段做
          // 只读借用判别 returns 位。
          if let ConstraintV::PackSubtype(psc) = unsafe { &(*constraint).c }
            && psc.returns
          {
            if !previous.is_null() {
              // SAFETY: `previous` 来自同一遍历中上一轮回调保存的存活约束指针，
              // 判空后才写入 `constraint` 自身的 `deprecated_dependencies` 字段——与
              // 上述 `.c` 只读借用字段不相交，与 C++ 逐字段读写一致。
              unsafe { (*constraint).deprecated_dependencies.push(previous) };
            }
            previous = constraint;
          }
        },
      );
    }

    if generalize && has_free_type(sig.signature) {
      Inference::no_refinement(generalized_ty)
    } else {
      Inference::no_refinement(sig.signature)
    }
  }

  /// cpp `check(const ScopePtr&, AstExprUnary*)`（ConstraintGenerator.cpp:3497）。
  /// `unary` 为分派层经类索引校验后传入的共享借用，本方法只读取其字段。
  pub fn check_expr_unary(&mut self, scope: &ScopePtr, unary: &AstExprUnary) -> Inference {
    let op = unary.op;

    // SAFETY: 裸指针由 `&mut self.type_context` 真实借用派生，守卫在函数末尾
    // 析构、回写为该字段收尾访问（C++ `std::optional<InConditionalContext>` 同款 RAII）。
    let _in_context = if op != AstExprUnaryOp::Not {
      Some(InConditionalContext::new(
        &mut self.type_context,
        TypeContext::Default,
      ))
    } else {
      None
    };

    // expr 已句柄化：get() 只读借用出自存活 &AstExprUnary（cpp 直接透传）。
    let operand = unary.expr.get();
    let inf = self.check_expr(scope, operand);
    let operand_type = inf.ty;
    let refinement = inf.refinement;

    match op {
      AstExprUnaryOp::Not => {
        // BuiltinTypes 为进程级单例（Handle::get 类型契约），not_func 是常量槽，
        // 只读借用交 create_type_function_instance 消费。
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.not_func,
          alloc::vec![operand_type],
          Vec::new(),
          scope,
          unary.base.base.location,
        );
        let negated = self.refinement_arena.negation_refinement_id(refinement);
        // §2：`None`（原 `Inference{ty, nullptr}` 形态）收口到 Option 构造器。
        inference_with_refinement(result_type, negated)
      }
      AstExprUnaryOp::Len => {
        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.len_func,
          alloc::vec![operand_type],
          Vec::new(),
          scope,
          unary.base.base.location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
      AstExprUnaryOp::Minus => {
        // compileExprUnary folds `-1i` into one negative constant, so a negated integer literal is a value rather than
        // an operation. A non-literal integer still reaches the runtime, which has no __unm, so it keeps the check.
        if ast_node_is::<AstExprConstantInteger>(&operand.base) {
          return Inference::inference_type_id_refinement_id(
            self.builtin_types.get().integer_type,
            refinement,
          );
        }

        let result_type = self.create_type_function_instance(
          &self.builtin_types.get().type_functions.unm_func,
          alloc::vec![operand_type],
          Vec::new(),
          scope,
          unary.base.base.location,
        );
        Inference::inference_type_id_refinement_id(result_type, refinement)
      }
    }
  }

  /// cpp `check(const ScopePtr&, AstExprIfElse*, optional<TypeId>)`
  /// （ConstraintGenerator.cpp:3630）。`if_else` 为分派层经类索引校验后传入的
  /// 共享借用，其三分支子表达式在此一次性取共享引用复用。
  pub fn check_expr_if_else(
    &mut self,
    scope: &ScopePtr,
    if_else: &AstExprIfElse,
    expected_type: Option<TypeId>,
  ) -> Inference {
    let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

    // 三子节点已句柄化恒非空：.get() 共享借用出自 if_else 存活引用（cpp
    // `check(scope, ifElse->condition)` 同款路径），整块 unsafe 契约随类型消失。
    let condition = if_else.condition.get();
    let true_expr = if_else.true_expr.get();
    let false_expr = if_else.false_expr.get();

    let refinement = {
      // C++ `InConditionalContext flipper{&typeContext}` -> default newValue is Condition.
      let _flipper = InConditionalContext::new(&mut self.type_context, TypeContext::Condition);
      let cond_scope = self.child_scope(alias_ref(if_else.condition.as_ast_node()), scope);
      self.check_expr(&cond_scope, condition).refinement
    };

    let then_scope = self.child_scope(alias_ref(if_else.true_expr.as_ast_node()), scope);
    self.apply_refinements(&then_scope, true_expr.base.location, refinement);
    let then_type = self
      .check_expr_expected(&then_scope, true_expr, expected_type)
      .ty;

    let else_scope = self.child_scope(alias_ref(if_else.false_expr.as_ast_node()), scope);
    let negated = self.refinement_arena.negation_refinement_id(refinement);
    // §2：`None`（原 null 哨兵）与 apply_refinements 入口的判空 no-op 同义。
    if let Some(negated) = negated {
      self.apply_refinements(&else_scope, false_expr.base.location, negated);
    }
    let else_type = self
      .check_expr_expected(&else_scope, false_expr, expected_type)
      .ty;

    let union = self.make_union_scope_ptr_location_type_id_type_id(
      arc_as_mut(scope),
      if_else.base.base.location,
      then_type,
      else_type,
    );
    Inference::no_refinement(union)
  }

  /// cpp `check(const ScopePtr&, AstExprTypeAssertion*)`
  /// （ConstraintGenerator.cpp:3687）。
  pub fn check_expr_type_assertion(
    &mut self,
    scope: &ScopePtr,
    type_assert: &AstExprTypeAssertion,
  ) -> Inference {
    // expr 已句柄化恒非空：.get() 共享引用供递归（cpp `check(scope, typeAssert->expr)`）。
    self.check_expr(scope, type_assert.expr.get());
    Inference::no_refinement(self.resolve_type(
      arc_as_mut(scope),
      // annotation 已句柄化恒非空；resolve_type 为既有裸指针 API，经 as_ptr 桥接。
      type_assert.annotation.as_ptr(),
      false,
      false,
      Polarity::Positive,
    ))
  }

  /// cpp `check(const ScopePtr&, AstExprInterpString*)`
  /// （ConstraintGenerator.cpp:3693）。
  pub fn check_expr_interp_string(
    &mut self,
    scope: &ScopePtr,
    interp_string: &AstExprInterpString,
  ) -> Inference {
    // SAFETY: 裸指针源自 `&mut self.type_context` 借用、守卫随本函数末析构回写
    // （cpp RAII flipper 同款）。
    let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

    for expr in interp_string.expressions.iter_nodes() {
      self.check_expr(scope, expr);
    }

    Inference::no_refinement(self.builtin_types.get().string_type)
  }

  /// cpp `check(const ScopePtr&, AstExprInstantiate*)`
  /// （ConstraintGenerator.cpp:3703）。`instantiate` 为分派层经类索引校验后
  /// 传入的共享借用。
  pub fn check_expr_instantiate(
    &mut self,
    scope: &ScopePtr,
    instantiate: &AstExprInstantiate,
  ) -> Inference {
    if !fflag::LuauExplicitTypeInstantiationSupport.get() {
      // SAFETY: instantiate.expr 为存活节点名下的 arena 子表达式，只取共享引用
      // 递归（cpp flag 关闭时直接 `check(scope, expr->expr)`）。
      return self.check_expr(scope, unsafe { &*instantiate.expr });
    }

    // SAFETY: 同上，只取共享引用递归；type_arguments 数组 data/size 由 arena
    // 成对写入，交 resolve_type_arguments 只读消费。
    let function_type = {
      let expr = unsafe { &*instantiate.expr };
      self.check_expr_expected(scope, expr, None).ty
    };

    let (explicit_type_ids, explicit_type_pack_ids) =
      self.resolve_type_arguments(arc_as_mut(scope), instantiate.type_arguments);

    let placeholder_type = self.arena.get_mut().add_type(BlockedType::default());

    let constraint = self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      instantiate.base.base.location,
      ConstraintV::TypeInstantiation(TypeInstantiationConstraint {
        function_type,
        placeholder_type,
        type_arguments: explicit_type_ids,
        type_pack_arguments: explicit_type_pack_ids,
      }),
    );

    // placeholder_type 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3630
    // `getMutable<BlockedType>(placeholderType)->setOwner(constraint)`
    let blocked = get_mutable_type::get_mutable::<BlockedType>(placeholder_type)
      .expect("placeholder_type 刚由 add_type(BlockedType) 分配，必命中（cpp:3630）");
    blocked.set_owner(constraint as *const _);

    Inference::no_refinement(placeholder_type)
  }

  /// cpp `check(const ScopePtr&, AstExprTable*, optional<TypeId>)`
  /// （ConstraintGenerator.cpp:4027）。`table` 为分派层经类索引校验后传入的
  /// 共享借用；item 的 `.value`/`.key` 仍是裸子指针，按下述契约逐项处理。
  pub fn check_expr_table(
    &mut self,
    scope: &ScopePtr,
    table: &AstExprTable,
    expected_type: Option<TypeId>,
  ) -> Inference {
    // SAFETY: 裸指针由 `&mut self.type_context` 真实借用派生、非空对齐；守卫先声明后
    // 析构，Drop 回写发生时对本字段再无存活借用（C++ RAII flipper 同款次序）。
    let _in_context = InConditionalContext::new(&mut self.type_context, TypeContext::Default);

    // SAFETY: table.items 数组的 data/size 为 arena 成对写入，各 item 的
    // `.value`/`.key`（可空）指向存活节点；`self.arena`/`self.builtin_types` 是
    // 构造期注入的会话级句柄（add_type 独占写、number_type 读）；`self.module`
    // 的 Arc 由 generator 持有，`ttv.scope`/`PushTypeConstraint` 存入的
    // `arc_as_mut` 裸句柄指向随模块与约束集存活到求解结束的共享对象——
    // 逐一对应 C++ `NotNull{&module->astTypes}`、`scope.get()` 等成员直写；
    // `ttv` 命中性由 `table_ty` 刚以 TableType 分配保证。
    unsafe {
      let table_ty = self.arena.get_mut().add_type(TableType::new());
      // table_ty 刚由 add_type(TableType) 分配，必命中；对照 C++ `getMutable<TableType>(tableTy)` 后的 LUAU_ASSERT
      let ttv = get_mutable_type::get_mutable::<TableType>(table_ty);
      LUAU_ASSERT!(ttv.is_some());
      let ttv = ttv.expect("table_ty 刚由 add_type(TableType) 分配，必命中（cpp LUAU_ASSERT）");

      ttv.state = TableState::Unsealed;
      if let Some(module) = &self.module {
        ttv.definition_module_name = module.name.clone();
      }
      ttv.definition_location = table.base.base.location;
      ttv.scope = arc_as_mut(scope);

      let primitive_limit = fint::LuauPrimitiveInferenceInTableLimit.get();
      let large_table = primitive_limit > 0 && table.items.size > primitive_limit as usize;
      if large_table {
        self.large_table_depth += 1;
      }

      if let Some(interior) = self.interior_free_types.last_mut() {
        interior.types.push(table_ty);
      }

      let mut index_key_lower_bound: Vec<TypeId> = Vec::new();
      let mut index_value_lower_bound: Vec<TypeId> = Vec::new();

      let mut create_indexer = |current_index_type: TypeId, current_result_type: TypeId| {
        let key = follow_type::follow(current_index_type);
        if !index_key_lower_bound.contains(&key) {
          index_key_lower_bound.push(key);
        }

        let value = follow_type::follow(current_result_type);
        if !index_value_lower_bound.contains(&value) {
          index_value_lower_bound.push(value);
        }
      };

      let start = checkpoint(self);

      for item in table.items.as_slice() {
        let item_ty = self
          .check_expr_full(scope, &*item.value, None, false, false)
          .ty;

        if !item.key.is_null() {
          let key_ty = self.check_expr(scope, &*item.key).ty;
          // cpp `item.key->as<AstExprConstantString>()`：门面命中走属性名，
          // 未命中退化为 indexer（原 is_null 两分支语义不变）。
          if let Some(key) = ast_node_try_as_ptr::<AstExprConstantString>(item.key) {
            let prop_name = String::from(from_utf8(key.value.as_bytes()).unwrap_or(""));
            let mut prop = Property::rw_type_id(item_ty);
            prop.location = Some(key.base.base.location);
            ttv.props.insert(prop_name, prop);
          } else {
            create_indexer(key_ty, item_ty);
          }
        } else {
          create_indexer(self.builtin_types.get().number_type, item_ty);
        }
      }

      let end = checkpoint(self);

      if !index_key_lower_bound.is_empty() {
        LUAU_ASSERT!(!index_value_lower_bound.is_empty());

        let index_key = match index_key_lower_bound.as_slice() {
          [only] => *only,
          _ => self.make_union_vector_type_id(index_key_lower_bound),
        };

        let index_value = match index_value_lower_bound.as_slice() {
          [only] => *only,
          _ => self.make_union_vector_type_id(index_value_lower_bound),
        };

        ttv.indexer = Some(TableIndexer {
          index_type: index_key,
          index_result_type: index_value,
          is_read_only: false,
        });
      }

      if let Some(expected_type) = expected_type
        && let Some(module) = &self.module
      {
        let module_ptr = arc_as_mut(module);
        let ptc = self.add_constraint_scope_ptr_location_constraint_v(
          scope,
          table.base.base.location,
          ConstraintV::PushType(PushTypeConstraint {
            expected_type,
            target_type: table_ty,
            ast_types: &(*module_ptr).ast_types as *const _,
            ast_expected_types: &(*module_ptr).ast_expected_types as *const _,
            // 身份键：cpp `NotNull{expr}` 存的就是被检节点地址，求解侧只读。
            expr: (table as *const AstExprTable).cast::<AstExpr>(),
          }),
        );

        if fflag::LuauConstraintGraph.get() {
          add_all_as_reverse_dependencies(start, end, self, ptc);
        } else {
          for_each_constraint(start, end, self, |c| {
            (*c).deprecated_dependencies.push(ptc);
          });
        }
      }

      if large_table {
        self.large_table_depth -= 1;
      }

      Inference::no_refinement(table_ty)
    }
  }
}
