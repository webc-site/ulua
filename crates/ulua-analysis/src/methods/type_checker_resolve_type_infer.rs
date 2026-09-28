use crate::{
  enums::table_state::TableState,
  functions::{
    add_refinement::add_refinement,
    arc_as_mut::arc_as_mut,
    follow_type, get_type,
    has_tag_type::has_tag_type_id,
    is_boolean::is_boolean,
    is_overloaded_function::is_overloaded_function,
    is_prim::{is_buffer, is_integer, is_nil, is_number, is_thread},
    is_string::is_string,
    is_table_intersection::is_table_intersection,
    is_undecidable::is_undecidable,
    maybe_singleton::maybe_singleton,
  },
  records::{
    and_predicate::AndPredicate, eq_predicate::EqPredicate, extern_type::ExternType,
    function_type::FunctionType, is_a_predicate::IsAPredicate, metatable_type::MetatableType,
    not_predicate::NotPredicate, or_predicate::OrPredicate, singleton_type::SingletonType,
    table_type::TableType, truthy_predicate::TruthyPredicate, type_checker::TypeChecker,
    type_guard_predicate::TypeGuardPredicate, union_type::UnionType, unknown_type::UnknownType,
  },
  type_aliases::{
    l_value::LValue,
    predicate::{Predicate, PredicateMember},
    predicate_vec::PredicateVec,
    refinement_map::RefinementMap,
    scope_ptr_type::ScopePtr,
    type_id::TypeId,
  },
};

impl TypeChecker {
  pub fn resolve_predicate_vec_scope_ptr_bool(
    &mut self,
    predicates: &PredicateVec,
    scope: &ScopePtr,
    sense: bool,
  ) {
    let scope_mut = arc_as_mut(scope);
    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      predicates,
      // Safety: `scope_mut` 由 arc_as_mut(scope) 派生，指向 `scope` 这个 Arc 内嵌
      // 的 Scope；`scope` 借用同参数传入并存活至被调返回，故指针非空、对齐且已
      // 初始化。这是 C++ `asMutable(scope)->refinements` 的等价惯用法：执行全程
      // 单线程串行，本行取出的 `&mut` 是调用窗内对 refinements 的唯一可变句柄，
      // 被调链对 scope 的其余访问均为 Arc 只读遍历（parent/bindings/refinements
      // 查询），不并存第二个可变借用。
      unsafe { &mut (*scope_mut).refinements },
      scope,
      sense,
      false,
    );
  }

  pub fn resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
    &mut self,
    predicates: &PredicateVec,
    refis: &mut RefinementMap,
    scope: &ScopePtr,
    sense: bool,
    from_or: bool,
  ) {
    for c in predicates {
      self.resolve_predicate_refinement_map_scope_ptr_bool_bool(c, refis, scope, sense, from_or);
    }
  }

  pub fn resolve_predicate_refinement_map_scope_ptr_bool_bool(
    &mut self,
    predicate: &Predicate,
    refis: &mut RefinementMap,
    scope: &ScopePtr,
    sense: bool,
    from_or: bool,
  ) {
    // get_predicate<T>().as_ref() 的安全等价式：get_if 直接返回派生自 `predicate`
    // 借用的 Option<&T>，与 C++ `get<T>(predicate)` 判空后取引用逐一对应。
    if let Some(truthy_p) = TruthyPredicate::get_if(predicate) {
      self.resolve_truthy_predicate_refinement_map_scope_ptr_bool_bool(
        truthy_p,
        refis,
        scope.clone(),
        sense,
        from_or,
      );
    } else if let Some(and_p) = AndPredicate::get_if(predicate) {
      self.resolve_and_predicate_refinement_map_scope_ptr_bool(and_p, refis, scope, sense);
    } else if let Some(or_p) = OrPredicate::get_if(predicate) {
      self.resolve_or_predicate_refinement_map_scope_ptr_bool(or_p, refis, scope, sense);
    } else if let Some(not_p) = NotPredicate::get_if(predicate) {
      self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
        &not_p.predicates,
        refis,
        scope,
        !sense,
        from_or,
      );
    } else if let Some(isa_p) = IsAPredicate::get_if(predicate) {
      self.resolve_is_a_predicate_refinement_map_scope_ptr_bool(isa_p, refis, scope.clone(), sense);
    } else if let Some(typeguard_p) = TypeGuardPredicate::get_if(predicate) {
      self.resolve_type_guard_predicate_refinement_map_scope_ptr_bool(
        typeguard_p,
        refis,
        scope.clone(),
        sense,
      );
    } else if let Some(eq_p) = EqPredicate::get_if(predicate) {
      self.resolve_eq_predicate_refinement_map_scope_ptr_bool(eq_p, refis, scope.clone(), sense);
    } else {
      self.ice_string("Unhandled predicate kind");
    }
  }

  pub fn resolve_truthy_predicate_refinement_map_scope_ptr_bool_bool(
    &mut self,
    truthy_p: &TruthyPredicate,
    refis: &mut RefinementMap,
    scope: ScopePtr,
    sense: bool,
    from_or: bool,
  ) {
    let ty =
      self.resolve_l_value_refinement_map_scope_ptr_l_value(refis, scope.clone(), &truthy_p.lvalue);

    if let Some(ty) = ty
      && from_or
    {
      add_refinement(refis, &truthy_p.lvalue, ty);
      return;
    }

    let mut predicate = self.mk_truthy_predicate(sense, self.nil_type);
    self.refine_l_value(&truthy_p.lvalue, refis, scope, &mut predicate);
  }

  pub fn resolve_and_predicate_refinement_map_scope_ptr_bool(
    &mut self,
    and_p: &AndPredicate,
    refis: &mut RefinementMap,
    scope: &ScopePtr,
    sense: bool,
  ) {
    if !sense {
      let or_p = OrPredicate {
        lhs: alloc::vec![Predicate::Not(NotPredicate {
          predicates: and_p.lhs.clone(),
        })],
        rhs: alloc::vec![Predicate::Not(NotPredicate {
          predicates: and_p.rhs.clone(),
        })],
      };

      self.resolve_or_predicate_refinement_map_scope_ptr_bool(&or_p, refis, scope, !sense);
      return;
    }

    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      &and_p.lhs, refis, scope, sense, false,
    );
    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      &and_p.rhs, refis, scope, sense, false,
    );
  }

  pub fn resolve_or_predicate_refinement_map_scope_ptr_bool(
    &mut self,
    or_p: &OrPredicate,
    refis: &mut RefinementMap,
    scope: &ScopePtr,
    sense: bool,
  ) {
    if !sense {
      let and_p = AndPredicate {
        lhs: alloc::vec![Predicate::Not(NotPredicate {
          predicates: or_p.lhs.clone(),
        })],
        rhs: alloc::vec![Predicate::Not(NotPredicate {
          predicates: or_p.rhs.clone(),
        })],
      };

      self.resolve_and_predicate_refinement_map_scope_ptr_bool(&and_p, refis, scope, !sense);
      return;
    }

    let mut left_refis = RefinementMap::new();
    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      &or_p.lhs,
      &mut left_refis,
      scope,
      sense,
      false,
    );

    let mut right_refis = RefinementMap::new();
    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      &or_p.lhs,
      &mut right_refis,
      scope,
      !sense,
      false,
    );
    self.resolve_predicate_vec_refinement_map_scope_ptr_bool_bool(
      &or_p.rhs,
      &mut right_refis,
      scope,
      sense,
      true,
    );

    self.merge(refis, &left_refis);
    self.merge(refis, &right_refis);
  }

  /// C++ `TypeChecker::resolve(const IsAPredicate&, ...)` (TypeInfer.cpp:6459).
  pub fn resolve_is_a_predicate_refinement_map_scope_ptr_bool(
    &mut self,
    isa_p: &IsAPredicate,
    refis: &mut RefinementMap,
    scope: ScopePtr,
    sense: bool,
  ) {
    // 上游 lambda 捕获 `this` 后回调 `canUnify`（TypeInfer.cpp:6459-6500），会改写
    // 类型检查器状态。端口把 TypeChecker 作为 predicate 的入参，闭包不持有 self 的
    // 裸指针别名，别名窗口只存在于单次回调内。
    let isa_ty = isa_p.ty;
    let location = isa_p.location;
    let scope_for_predicate = scope.clone();

    let mut predicate = move |tc: &mut TypeChecker, option: TypeId| -> Option<TypeId> {
      // This by itself is not truly enough to determine that A is stronger than B or vice versa.
      let option_is_subtype = tc
        .can_unify_type_infer(option, isa_ty, &scope_for_predicate, &location)
        .is_empty();
      let target_is_subtype = tc
        .can_unify_type_infer(isa_ty, option, &scope_for_predicate, &location)
        .is_empty();

      // If A is a superset of B, then if sense is true, we promote A to B, otherwise we keep A.
      if !option_is_subtype && target_is_subtype {
        return if sense { Some(isa_ty) } else { Some(option) };
      }

      // If A is a subset of B, then if sense is true we pick A, otherwise we eliminate A.
      if option_is_subtype && !target_is_subtype {
        return if sense { Some(option) } else { None };
      }

      // If neither has any relationship, we only return A if sense is false.
      if !option_is_subtype && !target_is_subtype {
        return if sense { None } else { Some(option) };
      }

      // If both are subtypes, then we're in one of the two situations described in
      // the C++ source. We look at whether the option is undecidable or a free table.
      if option_is_subtype && target_is_subtype {
        let is_free_table =
          get_type::get::<TableType>(option).is_some_and(|ttv| ttv.state == TableState::Free);
        if is_undecidable(option) || is_free_table {
          return if sense { Some(isa_ty) } else { Some(option) };
        }

        if sense {
          return Some(isa_ty);
        }
      }

      None
    };

    self.refine_l_value(&isa_p.lvalue, refis, scope, &mut predicate);
  }
}

/// `kTypeofRootTag` (Type.h:1257).
const K_TYPEOF_ROOT_TAG: &str = "typeofRoot";

fn is_table_like(ty: TypeId) -> bool {
  is_table_intersection(ty)
    || get_type::get::<TableType>(ty).is_some()
    || get_type::get::<MetatableType>(ty).is_some()
}

fn is_function_like(ty: TypeId) -> bool {
  is_overloaded_function(ty) || get_type::get::<FunctionType>(ty).is_some()
}

fn is_userdata_like(ty: TypeId) -> bool {
  get_type::get::<ExternType>(ty).is_some()
}

impl TypeChecker {
  /// C++ helper lambda `refine` inside `resolve(const TypeGuardPredicate&, ...)`.
  fn type_guard_refine(
    &mut self,
    lvalue: &LValue,
    refis: &mut RefinementMap,
    scope: ScopePtr,
    sense: bool,
    f: fn(TypeId) -> bool,
    maps_to: Option<TypeId>,
  ) {
    let mut predicate = move |_tc: &mut TypeChecker, ty: TypeId| -> Option<TypeId> {
      if sense && get_type::get::<UnknownType>(ty).is_some() {
        return maps_to.or(Some(ty));
      }

      if f(ty) == sense {
        return Some(ty);
      }

      if is_undecidable(ty) {
        return maps_to.or(Some(ty));
      }

      None
    };

    self.refine_l_value(lvalue, refis, scope, &mut predicate);
  }

  pub fn resolve_type_guard_predicate_refinement_map_scope_ptr_bool(
    &mut self,
    typeguard_p: &TypeGuardPredicate,
    refis: &mut RefinementMap,
    scope: ScopePtr,
    sense: bool,
  ) {
    // Rewrite the predicate 'type(foo) == "vector"' to be 'typeof(foo) == "Vector3"'.
    // They're exactly identical.
    if !typeguard_p.is_typeof && typeguard_p.kind == "vector" {
      return self.resolve_type_guard_predicate_refinement_map_scope_ptr_bool(
        &TypeGuardPredicate {
          lvalue: typeguard_p.lvalue.clone(),
          location: typeguard_p.location,
          kind: "Vector3".to_string(),
          is_typeof: true,
        },
        refis,
        scope,
        sense,
      );
    }

    let ty = self.resolve_l_value_refinement_map_scope_ptr_l_value(
      refis,
      scope.clone(),
      &typeguard_p.lvalue,
    );
    if ty.is_none() {
      return;
    }

    // In certain cases, the value may actually be nil, but Luau doesn't know about it.
    // So we whitelist this.
    if sense && typeguard_p.kind == "nil" {
      add_refinement(refis, &typeguard_p.lvalue, self.nil_type);
      return;
    }

    // Note: "vector" never happens here at this point.
    // kind 关键字单值派发：一次 match（字符串 DFA）选出 (细化谓词, 目标类型)，
    // 替代 10 路 else-if 顺序字符串比较；分支本就互斥，语义逐字保持。
    let kind = typeguard_p.kind.as_str();
    let refine = match kind {
      "nil" => Some((is_nil as fn(TypeId) -> bool, Some(self.nil_type))),
      "string" => Some((is_string as fn(TypeId) -> bool, Some(self.string_type))),
      "number" => Some((is_number as fn(TypeId) -> bool, Some(self.number_type))),
      "integer" => Some((is_integer as fn(TypeId) -> bool, Some(self.integer_type))),
      "boolean" => Some((is_boolean as fn(TypeId) -> bool, Some(self.boolean_type))),
      "thread" => Some((is_thread as fn(TypeId) -> bool, Some(self.thread_type))),
      "buffer" => Some((is_buffer as fn(TypeId) -> bool, Some(self.buffer_type))),
      "table" => Some((is_table_like as fn(TypeId) -> bool, None)),
      "function" => Some((is_function_like as fn(TypeId) -> bool, None)),
      "userdata" => Some((is_userdata_like as fn(TypeId) -> bool, None)),
      _ => None,
    };
    if let Some((predicate, maps_to)) = refine {
      // This can still happen when sense is false! (nil 分支注记保留)
      return self.type_guard_refine(&typeguard_p.lvalue, refis, scope, sense, predicate, maps_to);
    }

    if !typeguard_p.is_typeof {
      let err = self.error_recovery_type_scope_ptr(&scope);
      add_refinement(refis, &typeguard_p.lvalue, err);
      return;
    }

    // Safety: `self.global_scope` 为构造期注入的 `*const ScopePtr`，对应 C++
    // `const ScopePtr&` 成员，指向会话级存活的全局 Arc<Scope>；两层解引用取
    // &Scope 仅做只读 lookup_type，无写路径。
    let global_scope = unsafe { &**self.global_scope };
    let type_fun = global_scope.lookup_type(&typeguard_p.kind);
    let type_fun = match type_fun {
      Some(tf) if tf.type_params().is_empty() && tf.type_pack_params().is_empty() => tf,
      _ => {
        let err = self.error_recovery_type_scope_ptr(&scope);
        add_refinement(refis, &typeguard_p.lvalue, err);
        return;
      }
    };

    let resolved = follow_type::follow(type_fun.r#type());
    // `self.builtin_types` 按 Handle 契约（NonNull 编码非空）构造注入，比
    // checker 长寿；此处 get() 物化只读借用拷贝 `extern_type` 这一个 TypeId 值。
    let extern_type_builtin = self.builtin_types.get().extern_type;

    // You cannot refine to the top class type.
    if resolved == extern_type_builtin {
      let err = self.error_recovery_type_scope_ptr(&scope);
      add_refinement(refis, &typeguard_p.lvalue, err);
      return;
    }

    // We're only interested in the root type of any extern type.
    let is_root_extern_type = get_type::get::<ExternType>(resolved).is_some_and(|etv| {
      etv.parent == Some(extern_type_builtin) || has_tag_type_id(resolved, K_TYPEOF_ROOT_TAG)
    });
    if !is_root_extern_type {
      let err = self.error_recovery_type_scope_ptr(&scope);
      add_refinement(refis, &typeguard_p.lvalue, err);
      return;
    }

    // Until type filtering functions are broken out, we rewrite this to be the same as using IsA.
    self.resolve_is_a_predicate_refinement_map_scope_ptr_bool(
      &IsAPredicate {
        lvalue: typeguard_p.lvalue.clone(),
        location: typeguard_p.location,
        ty: resolved,
      },
      refis,
      scope,
      sense,
    );
  }

  pub fn resolve_eq_predicate_refinement_map_scope_ptr_bool(
    &mut self,
    eq_p: &EqPredicate,
    refis: &mut RefinementMap,
    scope: ScopePtr,
    sense: bool,
  ) {
    let followed = follow_type::follow(eq_p.ty);
    let rhs = match get_type::get::<UnionType>(followed) {
      Some(union) => union.options.clone(),
      None => alloc::vec![followed],
    };

    if sense && rhs.iter().copied().any(is_undecidable) {
      return;
    }

    let eq_ty = eq_p.ty;
    let location = eq_p.location;
    let scope_for_predicate = scope.clone();

    // 上游 lambda 捕获 `this` 并回调 `canUnify`；这里把 TypeChecker 作为入参传
    // 进来，避免把 `self` 裸指针塞进闭包（与 TypeIdPredicate 的签名一致）。
    let mut predicate = move |tc: &mut TypeChecker, option: TypeId| -> Option<TypeId> {
      if !sense && is_nil(eq_ty) {
        return if is_undecidable(option) || !is_nil(option) {
          Some(option)
        } else {
          None
        };
      }

      if maybe_singleton(eq_ty) {
        let option_is_subtype = tc
          .can_unify_type_id_type_id_scope_ptr_location(
            option,
            eq_ty,
            &scope_for_predicate,
            &location,
          )
          .is_empty();
        let target_is_subtype = tc
          .can_unify_type_id_type_id_scope_ptr_location(
            eq_ty,
            option,
            &scope_for_predicate,
            &location,
          )
          .is_empty();

        if sense {
          if option_is_subtype && !target_is_subtype {
            return Some(option);
          } else if !option_is_subtype && target_is_subtype {
            return Some(follow_type::follow(eq_ty));
          } else if !option_is_subtype && !target_is_subtype {
            return None;
          } else if option_is_subtype && target_is_subtype {
            return Some(follow_type::follow(eq_ty));
          }
        } else {
          let is_option_singleton = get_type::get::<SingletonType>(option).is_some();
          if !is_option_singleton {
            return Some(option);
          } else if option_is_subtype && target_is_subtype {
            return None;
          }
        }
      }

      Some(option)
    };

    self.refine_l_value(&eq_p.lvalue, refis, scope, &mut predicate);
  }
}
