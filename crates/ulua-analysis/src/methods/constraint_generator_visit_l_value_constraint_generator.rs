use alloc::string::String;
use core::str::from_utf8;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_expr::AstExpr, ast_expr_constant_string::AstExprConstantString,
    ast_expr_global::AstExprGlobal, ast_expr_index_expr::AstExprIndexExpr,
    ast_expr_index_name::AstExprIndexName, ast_expr_local::AstExprLocal,
  },
  rtti::ast_node_try_as_ptr,
};
use ulua_common::macros::luau_assert::LUAU_ASSERT;

use crate::{
  enums::value::Value,
  functions::{
    arc_as_mut::arc_as_mut, as_mutable_type::as_mutable_type_id, follow_type, get_mutable_type,
    get_type::get, should_suppress_errors_type_utils::should_suppress_errors,
  },
  records::{
    arena_handle::alias, assign_index_constraint::AssignIndexConstraint,
    assign_prop_constraint::AssignPropConstraint, blocked_type::BlockedType,
    constraint_generator::ConstraintGenerator, normalization_too_complex::NormalizationTooComplex,
    subtype_constraint::SubtypeConstraint, symbol::Symbol,
  },
  type_aliases::{
    constraint_v::ConstraintV, scope_ptr_type::ScopePtr, type_error_data::TypeErrorData,
    type_id::TypeId, type_variant::TypeVariant,
  },
};

impl ConstraintGenerator {
  /// cpp `visitLValue(const ScopePtr&, AstExpr*, TypeId)`
  /// （ConstraintGenerator.cpp:3881）：按 `class_index` 恢复 C++ 的静态类型
  /// 重载分派。`expr` 为左值位置的表达式共享借用（cpp 裸指针形参的 Rust
  /// 对应），本方法只判别类型并派发，不写节点。
  pub fn visit_l_value(&mut self, scope: &ScopePtr, expr: &AstExpr, rhs_type: TypeId) {
    // match 臂的类索引与 `ast_node_try_as` 的判定完全同一（cpp `as<T>()` 命中后
    // 直接 static_cast 亦假定成功），故 `expr_downcast` 的 expect 为逻辑不可达
    // 分支；家族类索引互斥由 rtti 的 rtti_indices_unique 测试保证，臂序无关
    // 语义，与原 ast_node_is 链序一致（Local→Global→IndexName→IndexExpr→Error）。
    match expr.as_expr_ref() {
      AstExprRef::Local(local) => self.visit_l_value_local(scope, local, rhs_type),
      AstExprRef::Global(global) => self.visit_l_value_global(scope, global, rhs_type),
      AstExprRef::IndexName(index_name) => {
        self.visit_l_value_index_name(scope, index_name, rhs_type)
      }
      AstExprRef::IndexExpr(index_expr) => {
        self.visit_l_value_index_expr(scope, index_expr, rhs_type)
      }
      AstExprRef::Error(err) => {
        // If we end up with some kind of error expression in an lvalue
        // position, at least go and check the expressions so that when
        // we visit them later, there aren't any invalid assumptions.
        // （cpp:3891-3897 range-for；iter_nodes 只读遍历 arena 保活子表达式）
        for sub_expr in err.expressions.iter_nodes() {
          self.check_expr(scope, sub_expr);
        }
      }
      _ => {
        // 对照 C++:3902 `ice->ice("Unexpected lvalue expression", expr->location)`；
        // ice 句柄的存活契约收口在 Handle::get。
        self
          .ice
          .get()
          .ice_string_location("Unexpected lvalue expression", &expr.base.location);
      }
    }
  }

  /// cpp `visitLValue(const ScopePtr&, AstExprLocal*, TypeId)`
  /// （ConstraintGenerator.cpp:3905 重载）：`local` 为分发层经类索引校验后
  /// 传入的共享借用，只读取其字段。
  pub fn visit_l_value_local(&mut self, scope: &ScopePtr, local: &AstExprLocal, rhs_type: TypeId) {
    // local 槽已句柄化恒非空（解析器 buildAllLocals 阶段写入的活 AstLocal，
    // C++:3920 同样无条件使用）；下游查表/符号为既有裸指针形态，经 as_ptr 桥接。
    let local_ptr = local.local.as_ptr();
    let annotated_ty = scope.lookup_symbol(Symbol::from_local(local_ptr));
    LUAU_ASSERT!(annotated_ty.is_some());

    // get_def 只按指针身份查表；repr(C) 基址重合保证该地址与 cpp
    // `dfg->getDef(local)`（C++:3923）的键同一。
    let def_id = self
      .dfg_ref()
      .get_def((local as *const AstExprLocal).cast::<AstExpr>());
    let mut ty = scope.as_ref().lookup_unrefined_type(def_id);

    if let Some(ty_val) = ty {
      if let Some(local_domain) = self.local_types.find_mut(&ty_val)
        && !local.upvalue
      {
        local_domain.insert_type_id(rhs_type);
      }
    } else {
      // SAFETY: arena 字段为构造期写入的活 TypeArena 指针；可变借用仅存活于
      // 本语句，不与其它 arena 引用并生（crate 不变量 2）。对照 C++:3934。
      let new_ty = self.arena.get_mut().add_type(BlockedType::default());
      self
        .local_types
        .get_or_insert(new_ty)
        .insert_type_id(rhs_type);
      ty = Some(new_ty);

      if let Some(annotated_ty_val) = annotated_ty {
        // SAFETY: normalizer 字段由构造参数（NonNull）写入且 generator 期间
        // 有效，作为实参满足被调 `should_suppress_errors` 自身的指针契约。
        // 对照 C++:3939 `shouldSuppressErrors(normalizer, *annotatedTy)`。
        match unsafe { should_suppress_errors(self.normalizer.as_ptr(), annotated_ty_val) }.value {
          Value::DoNotSuppress => {}
          Value::Suppress => {
            // SAFETY: error_type 读自构造期写入的 builtin_types 单例句柄；
            // location 现经共享引用只读（C++:3944）。
            ty = Some(
              self.simplify_union(scope.clone(), local.base.base.location, new_ty, {
                self.builtin_types.get().error_type
              }),
            );
          }
          Value::NormalizationFailed => {
            // SAFETY: local_ptr 是本函数契约保证的活 AstLocal（解析器写入），
            // 此分支前提是 annotated_ty 为 Some（符号确有标注），annotation
            // 指针随之非空（C++:3947 `local->local->annotation->location`
            // 同款无条件解引用），仅读 location。
            self.report_error(
              unsafe { (*local_ptr).annotation.as_ref().unwrap().base.location },
              TypeErrorData::NormalizationTooComplex(NormalizationTooComplex::default()),
            );
          }
        }
      }

      // SAFETY: scope 是调用方持有的 &ScopePtr（Arc<Scope>），按 arc_as_mut
      // 惯用法自 Arc::as_ptr 派生写句柄（Arc 在本借用期间存活、单线程独占写，
      // 见 functions/arc_as_mut.rs 契约）；只写 lvalue_types 这一张表，无并存
      // 可变别名。对照 C++:3952 `scope->lvalueTypes[defId] = *ty`。
      unsafe {
        let scope_raw = arc_as_mut(scope);
        *(*scope_raw).lvalue_types.get_or_insert(def_id) = ty.unwrap();
      }
    }

    let assigned_ty = ty.unwrap();
    self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, def_id, assigned_ty);
    self.record_inferred_binding(local_ptr, assigned_ty);

    if let Some(annotated_ty_val) = annotated_ty {
      // 对照 C++:3958 的 addConstraint(scope, local->location, ...)；
      // location 经共享引用只读。
      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        local.base.base.location,
        ConstraintV::Subtype(SubtypeConstraint {
          sub_type: rhs_type,
          super_type: annotated_ty_val,
        }),
      );
    }
  }

  /// cpp `visitLValue(const ScopePtr&, AstExprGlobal*, TypeId)`
  /// （ConstraintGenerator.cpp:3961 重载）：`global` 为分发层经类索引校验后
  /// 传入的共享借用，只读取其 `name`/`location` 字段。
  pub fn visit_l_value_global(
    &mut self,
    scope: &ScopePtr,
    global: &AstExprGlobal,
    rhs_type: TypeId,
  ) {
    // name 是模块 AstNameTable 持有的值拷贝（对照 C++:3963 `Symbol{global->name}`）。
    let global_name = global.name;
    let annotated_ty = scope.lookup_symbol(Symbol::from_global(global_name));
    if let Some(annotated_ty_val) = annotated_ty {
      // get_def 只按指针身份查表；repr(C) 基址重合保证地址与 cpp
      // `dfg->getDef(global)`（C++:3966）的键同一。
      let def = self
        .dfg_ref()
        .get_def((global as *const AstExprGlobal).cast::<AstExpr>());
      // root 写句柄随 generator 存活（`root()` 收口）；单线程内独占写
      // lvalue_types，无并存可变别名。对照 C++:3967
      // `rootScope->lvalueTypes[def] = rhsType`。
      *alias(arc_as_mut(self.root()))
        .lvalue_types
        .get_or_insert(def) = rhs_type;

      // Ignore possible self-assignment, it doesn't create a new constraint.
      let followed_rhs = follow_type::follow(rhs_type);
      if annotated_ty_val == followed_rhs {
        return;
      }

      let followed_annotation = follow_type::follow(annotated_ty_val);
      // 对照 C++:3886 `if (auto bt = get<BlockedType>(...); bt && uninitializedGlobals.contains(...))`
      if let Some(bt) = get::<BlockedType>(followed_annotation)
        && self.uninitialized_globals.contains(&global_name)
      {
        LUAU_ASSERT!(bt.get_owner().is_null());
        self.uninitialized_globals.erase(&global_name);
        // SAFETY: followed_annotation 是 arena 内的类型句柄，与 C++ asMutable 同契约。
        unsafe {
          (*as_mutable_type_id(followed_annotation)).ty = TypeVariant::Bound(rhs_type);
        }
      }

      // 对照 C++:3982 的 addConstraint(scope, global->location, ...)。
      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        global.base.base.location,
        ConstraintV::Subtype(SubtypeConstraint {
          sub_type: rhs_type,
          super_type: annotated_ty_val,
        }),
      );
    }
  }

  /// cpp `visitLValue(const ScopePtr&, AstExprIndexName*, TypeId)`
  /// （ConstraintGenerator.cpp:3986 重载）：`expr` 为分发层经类索引校验后
  /// 传入的共享借用，其 `expr`/`index` 字段由解析器填为非空活节点。
  pub fn visit_l_value_index_name(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprIndexName,
    rhs_type: TypeId,
  ) {
    // expr（被索引对象表达式）已句柄化恒非空：.get() 共享引用供 check 递归。
    // 对照 C++:3988。
    let subject = expr.expr.get();
    let lhs_ty = self.check_expr(scope, subject).ty;
    // SAFETY: arena 字段活（crate 不变量 2），可变借用仅在本语句内，
    // 新类型句柄随后才对外可见。对照 C++:3989 `arena->addType(BlockedType{})`。
    let prop_ty = self.arena.get_mut().add_type(BlockedType::default());

    if let Some(module) = &self.module {
      let module_ptr = arc_as_mut(module);
      // SAFETY: module 是 self.module 中存活的 Arc（本借用持有），按
      // arc_as_mut 惯用法派生独占写句柄；身份指针仅作 ast_types 的键写入，
      // 不触碰节点内存。对照 C++:3990 `module->astTypes[expr] = propTy`。
      unsafe {
        *(*module_ptr)
          .ast_types
          .get_or_insert((expr as *const AstExprIndexName).cast::<AstExpr>()) = prop_ty;
      }
    }

    let incremented = self.record_property_assignment(lhs_ty);

    // index 是模块名表持有的 AstName（其 C 串由 Arc<AstNameTable> 保活，本
    // module 借用期间有效），此处只读取为 String（对照 C++:3995 `expr->index.value`）。
    let prop_name: String = expr.index.as_str_or_empty().to_string();

    // 对照 C++:3994-3995 的 addConstraint 实参；location 均经共享引用只读。
    let apc = self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      expr.base.base.location,
      ConstraintV::AssignProp(AssignPropConstraint {
        lhs_type: lhs_ty,
        prop_name,
        rhs_type,
        prop_location: Some(expr.index_location),
        prop_type: prop_ty,
        decrement_prop_count: incremented,
      }),
    );

    // prop_ty 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3904
    // `getMutable<BlockedType>(propTy)->setOwner(apc)`
    let blocked = get_mutable_type::get_mutable::<BlockedType>(prop_ty).unwrap();
    blocked.set_owner(apc as *const _);
  }

  /// cpp `visitLValue(const ScopePtr&, AstExprIndexExpr*, TypeId)`
  /// （ConstraintGenerator.cpp:3999 重载）：`expr` 为分发层经类索引校验后
  /// 传入的共享借用，其 `expr`/`index` 子槽位已句柄化（Node）恒非空。
  pub fn visit_l_value_index_expr(
    &mut self,
    scope: &ScopePtr,
    expr: &AstExprIndexExpr,
    rhs_type: TypeId,
  ) {
    let index = expr.index;
    // SAFETY: index 为句柄化的非空子表达式槽位（同一解析器 arena 保活），
    // 门面判型+下转+判空一步折叠为 Option（cpp
    // `expr->index->as<AstExprConstantString>()`），不命中仅得 None。
    let constant_string = unsafe { ast_node_try_as_ptr::<AstExprConstantString>(index) };
    if let Some(constant_string) = constant_string {
      // .expr 已句柄化：.get() 安全共享引用供 check 递归。对照 C++:4003。
      let subject = expr.expr.get();
      let lhs_ty = self.check_expr(scope, subject).ty;
      // SAFETY: arena 字段活，可变借用止于本语句（crate 不变量 2）。
      // 对照 C++:4004。
      let prop_ty = self.arena.get_mut().add_type(BlockedType::default());

      if let Some(module) = &self.module {
        let module_ptr = arc_as_mut(module);
        // SAFETY: module 为 self.module 存活的 Arc，经 arc_as_mut 派生独占写
        // 句柄；两枚身份指针仅作 ast_types 的键，builtin_types 字段为活的
        // 构造期指针、此处只读 string_type。对照 C++:4005-4006 两行 astTypes 写入。
        unsafe {
          *(*module_ptr)
            .ast_types
            .get_or_insert((expr as *const AstExprIndexExpr).cast::<AstExpr>()) = prop_ty;
          // FIXME? Singleton strings exist.
          *(*module_ptr)
            .ast_types
            .get_or_insert(index.as_ptr().cast_const()) = self.builtin_types.get().string_type;
        }
      }

      // SAFETY: constant_string 门面命中分支，value 是解析器在模块字符串
      // 池内保活的字节数组（data+size 成对写入），只读拷贝。对照 C++:4007
      // `std::string propName{constantString->value.data, constantString->value.size}`。
      let prop_name: String =
        String::from(from_utf8(constant_string.value.as_bytes()).unwrap_or(""));

      let incremented = self.record_property_assignment(lhs_ty);

      // index 已句柄化恒非空，location 字段只读拷贝（对应 C++:4012 的
      // expr->index->location）。
      let apc = self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        expr.base.base.location,
        ConstraintV::AssignProp(AssignPropConstraint {
          lhs_type: lhs_ty,
          prop_name,
          rhs_type,
          prop_location: Some(index.base.location),
          prop_type: prop_ty,
          decrement_prop_count: incremented,
        }),
      );

      // prop_ty 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3927
      // `getMutable<BlockedType>(propTy)->setOwner(apc)`
      let blocked = get_mutable_type::get_mutable::<BlockedType>(prop_ty).unwrap();
      blocked.set_owner(apc as *const _);

      return;
    }

    // .expr/.index 已句柄化：.get() 安全共享引用供 check 递归。对照
    // C++:4019-4020 `check(scope, expr->expr)` /
    // `check(scope, expr->index)`。
    let subject = expr.expr.get();
    let index_node = index.get();
    let lhs_ty = self.check_expr(scope, subject).ty;
    let index_ty = self.check_expr(scope, index_node).ty;
    // SAFETY: arena 字段活，可变借用止于本语句。对照 C++:4021。
    let prop_ty = self.arena.get_mut().add_type(BlockedType::default());

    if let Some(module) = &self.module {
      let module_ptr = arc_as_mut(module);
      // SAFETY: 同常量串分支的 ast_types 写入论证——Arc 存活、单线程独占写、
      // 身份指针仅作键。对照 C++:4022 `module->astTypes[expr] = propTy`。
      unsafe {
        *(*module_ptr)
          .ast_types
          .get_or_insert((expr as *const AstExprIndexExpr).cast::<AstExpr>()) = prop_ty;
      }
    }

    // 对照 C++:4023 的 addConstraint；location 经共享引用只读。
    let aic = self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      expr.base.base.location,
      ConstraintV::AssignIndex(AssignIndexConstraint {
        lhs_type: lhs_ty,
        index_type: index_ty,
        rhs_type,
        prop_type: prop_ty,
      }),
    );

    // prop_ty 刚由 add_type(BlockedType) 分配，必命中；对照 C++:3937
    // `getMutable<BlockedType>(propTy)->setOwner(aic)`
    let blocked = get_mutable_type::get_mutable::<BlockedType>(prop_ty).unwrap();
    blocked.set_owner(aic as *const _);
  }
}
