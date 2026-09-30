//! `constraint_generator` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{sync::Arc, vec, vec::Vec};
use core::{
  mem::take,
  ptr::{NonNull, from_ref, null_mut},
};

use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr_function::AstExprFunction, ast_local::AstLocal,
    ast_node::AstNode, ast_stat_block::AstStatBlock, ast_type_or_pack::AstTypeOrPack,
    location::Location,
  },
  visit::AstVisitable,
};
use ulua_common::{
  dfint,
  macros::luau_assert::LUAU_ASSERT,
  records::{dense_hash_set::DenseHashSet, dense_hash_table::DenseDefault},
};

use super::constraint_generator_prototype_type_definitions::make_binding;
use crate::{
  enums::{control_flow::ControlFlow, polarity::Polarity},
  functions::{
    first::first, follow_type, follow_type::follow, fresh_type::fresh_type, get_mutable_type,
    get_type, is_prim::is_nil, shared_mut::shared_mut, simplify_union::simplify_union,
  },
  records::{
    arena_handle::{alias, alias_ref},
    arena_id::ArenaId,
    blocked_type::BlockedType,
    code_too_complex::CodeTooComplex,
    constraint::Constraint,
    constraint_generator::{ConstraintGenerator, InferredBinding},
    constraint_set::ConstraintSet,
    free_type_pack::FreeTypePack,
    global_prepopulator::GlobalPrepopulator,
    inference::Inference,
    inference_pack::InferencePack,
    never_type::NeverType,
    pack_subtype_constraint::PackSubtypeConstraint,
    recursion_counter::RecursionCounter,
    reduce_constraint::ReduceConstraint,
    scope::Scope,
    scope_registry::register_scope,
    symbol::Symbol,
    type_error::TypeError,
    type_function::TypeFunction,
    type_ids::TypeIds,
    type_level::TypeLevel,
    type_pack::TypePack,
    type_pack_var::TypePackVar,
    union_builder::UnionBuilder,
    union_type::UnionType,
    unpack_constraint::UnpackConstraint,
  },
  type_aliases::{
    constraint_v::ConstraintV, def_id_def::DefId, l_value::LValue, scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant,
  },
};

impl ConstraintGenerator {
  pub fn add_constraint_scope_ptr_location_constraint_v(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    cv: ConstraintV,
  ) -> *mut Constraint {
    let c = Box::new(Constraint {
      scope: shared_mut(scope),
      location,
      c: cv,
      deprecated_dependencies: Vec::new(),
    });
    let c_ptr = Box::into_raw(c);
    self.constraints.push(c_ptr);
    c_ptr
  }
}

impl ConstraintGenerator {
  pub fn add_type_pack(&mut self, head: Vec<TypeId>, tail: Option<TypePackId>) -> TypePackId {
    if head.is_empty() {
      if let Some(tail) = tail {
        tail
      } else {
        // Safety: self.builtin_types.as_ptr() 为构造期接线的会话级 *mut BuiltinTypes（非空、比本
        // generator 长寿，指向内建单例），此处仅按值只读其 empty_type_pack 句柄字段。
        self.builtin_types.get().empty_type_pack
      }
    } else {
      let pack = TypePack::new(head, tail);
      let pack_var = TypePackVar {
        ty: TypePackVariant::TypePack(pack),
        persistent: false,
        owning_arena: ArenaId::NONE,
      };

      // Safety: self.arena.as_ptr() 是构造期注入的非空 *mut TypeArena 且比本次检查长寿；
      // add_type_pack_t 是 arena 独占追加（&mut self），pack_var 为按值移交的新节点，
      // 返回 arena 分配的存活 TypePackId。
      self.arena.get_mut().add_type_pack_t(pack_var)
    }
  }
}

impl ConstraintGenerator {
  pub fn check_function_body(&mut self, scope: &ScopePtr, fn_expr: &AstExprFunction) {
    let cf = self.visit_block_without_child_scope(scope, fn_expr.body.get());

    if cf == ControlFlow::None {
      // Safety: `self.builtin_types.as_ptr()` 是 ConstraintGenerator 构造期接线的 `*mut BuiltinTypes`，
      // 指向比本 generator 长寿的内置类型单例，全程非空存活；此处重建共享借用仅读
      // `empty_type_pack`（Copy 的 TypePackId），`scope.return_type` 亦只读，单线程无别名。
      let builtin_types = self.builtin_types.get();
      let sub_pack = builtin_types.empty_type_pack;
      let super_pack = scope.return_type;

      let constraint = PackSubtypeConstraint {
        sub_pack,
        super_pack,
        returns: true,
      };

      self.add_constraint_scope_ptr_location_constraint_v(
        scope,
        fn_expr.base.base.location,
        ConstraintV::PackSubtype(constraint),
      );
    }
  }
}

impl ConstraintGenerator {
  /// C++ `ScopePtr ConstraintGenerator::childScope(AstNode* node, const ScopePtr& parent)`
  /// (`cpp/Analysis/src/ConstraintGenerator.cpp:522`)。
  ///
  /// `node` 为遍历方交出的存活 arena 节点共享借用（cpp 裸指针形参的 Rust 对应），
  /// 本函数只读取其 `location` 并以其地址作 `ast_scopes` 映射键。
  pub fn child_scope(&mut self, node: &AstNode, parent: &ScopePtr) -> ScopePtr {
    let scope: ScopePtr = Arc::new(Scope::new(parent, 0));
    // 注册发放句柄：父 children 存句柄；`scope_raw` 仍供 `ast_scopes` 裸指针映射。
    let scope_id = register_scope(&scope);
    let scope_raw = shared_mut(&scope);
    self.scopes.push((node.location, scope.clone()));

    let scope_mut = alias(scope_raw);
    scope_mut.location = node.location;
    scope_mut.return_type = parent.return_type;
    scope_mut.vararg_pack = parent.vararg_pack;

    alias(shared_mut(parent)).children.push(scope_id);

    if let Some(module) = &self.module {
      let module_ptr = shared_mut(module);
      *alias(module_ptr).ast_scopes.get_or_insert(&raw const *node) = scope_raw;
    }

    scope
  }
}

impl ConstraintGenerator {
  pub fn create_type_function_instance(
    &mut self,
    function: &TypeFunction,
    type_arguments: Vec<TypeId>,
    pack_arguments: Vec<TypePackId>,
    scope: &ScopePtr,
    location: Location,
  ) -> TypeId {
    let result = {
      self
        .arena
        .get_mut()
        .add_type_function_type_function_vector_type_id_vector_type_pack_id(
          function,
          type_arguments,
          pack_arguments,
        )
    };

    self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      location,
      ConstraintV::Reduce(ReduceConstraint { ty: result }),
    );

    result
  }
}

impl ConstraintGenerator {
  /// C++ `fillInferredBindings(const ScopePtr& globalScope, AstStatBlock* block)`：
  /// `_block` 仅保留 cpp 形参形态（函数体不使用）；`InferredBinding.scope` 记录
  /// 字段保持裸指针布局（记录布局不改），写 bindings 经 alias 收口。
  pub fn fill_in_inferred_bindings(&mut self, _global_scope: &ScopePtr, _block: &AstStatBlock) {
    let inferred_bindings: Vec<(Symbol, *mut Scope, Location, Vec<TypeId>)> = self
      .inferred_bindings
      .iter()
      .map(|(symbol, p)| (symbol.clone(), p.scope, p.location, p.types.order.clone()))
      .collect();

    for (symbol, scope, location, tys) in inferred_bindings {
      let ty = match tys.as_slice() {
        [only] => *only,
        _ => self.make_union_vector_type_id(tys),
      };

      alias(scope)
        .bindings
        .insert(symbol, make_binding(ty, location));
    }
  }
}

impl ConstraintGenerator {
  pub fn flatten_pack(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    pack: InferencePack,
  ) -> Inference {
    let tp = pack.tp;
    let refinements = pack.refinements;

    let refinement = refinements.first().copied();

    if let Some(f) = first(tp, true) {
      return Inference::inference_type_id_refinement_id(f, refinement.unwrap_or(null_mut()));
    }

    let type_result = self.arena.get_mut().add_type(BlockedType::default());

    let unpack_constraint = UnpackConstraint {
      result_pack: vec![type_result],
      source_pack: tp,
    };
    let constraint_ptr = self.add_constraint_scope_ptr_location_constraint_v(
      scope,
      location,
      ConstraintV::Unpack(unpack_constraint),
    );

    // type_result 刚由 add_type(BlockedType) 分配，必命中；对照 C++:5030
    // `getMutable<BlockedType>(typeResult)->setOwner(c)`
    let blocked = get_mutable_type::get_mutable::<BlockedType>(type_result)
      .expect("type_result 刚由 add_type(BlockedType) 分配，必命中（cpp:5030）");
    blocked.set_owner(constraint_ptr as *const _);

    Inference::inference_type_id_refinement_id(type_result, refinement.unwrap_or(null_mut()))
  }
}

impl ConstraintGenerator {
  pub fn fresh_type(&mut self, scope: &ScopePtr, polarity: Polarity) -> TypeId {
    let ft = {
      fresh_type(
        self.arena.get_mut(),
        self.builtin_types.get(),
        // `scope` 为 Arc<Scope> 共享持有，直接换共享引用（原 arc_as_mut 裸指针
        // 仅因旧签名需要裸指针而存在）。
        Some(scope.as_ref()),
        polarity,
      )
    };

    if let Some(interior_free_types) = self.interior_free_types.last_mut() {
      interior_free_types.types.push(ft);
    }

    self.free_types.insert_type_id(ft);
    ft
  }
}

impl ConstraintGenerator {
  pub fn fresh_type_pack(&mut self, scope: &ScopePtr, polarity: Polarity) -> TypePackId {
    // FreeTypePack f{scope.get(), polarity};
    let mut free = FreeTypePack {
      index: 0,
      level: TypeLevel::default(),
      scope: null_mut(),
      polarity: Polarity::None,
    };
    free.free_type_pack_scope_polarity(shared_mut(scope), polarity);

    // arena->addTypePack(TypePackVar{std::move(f)})
    let result = self.arena.get_mut().add_type_pack_t(free);

    // interiorFreeTypes.back().typePacks.push_back(result)
    if let Some(interior) = self.interior_free_types.last_mut() {
      interior.type_packs.push(result);
    }

    result
  }
}

impl ConstraintGenerator {
  pub fn is_shared_refinement_assignment_type(&self, ty: TypeId) -> bool {
    let is_nil_assignment = |ty: TypeId| {
      let ty = follow_type::follow(ty);
      is_nil(ty)
    };

    if is_nil_assignment(ty) {
      return true;
    }

    self
      .local_types
      .find(&ty)
      .is_some_and(|types| types.begin().into_iter().any(is_nil_assignment))
  }
}

impl ConstraintGenerator {
  pub fn make_intersect(
    &mut self,
    scope: &ScopePtr,
    location: Location,
    lhs: TypeId,
    rhs: TypeId,
  ) -> TypeId {
    let builtin_types = self.builtin_types.get();
    let intersect_func = &builtin_types.type_functions.intersect_func;

    self.create_type_function_instance(
      intersect_func,
      alloc::vec![lhs, rhs],
      alloc::vec![],
      scope,
      location,
    )
  }
}

impl ConstraintGenerator {
  pub(crate) fn make_union_scope_ptr_location_type_id_type_id(
    &mut self,
    _scope: &ScopePtr,
    _location: Location,
    lhs: TypeId,
    rhs: TypeId,
  ) -> TypeId {
    // 对照 C++ makeUnion：`if (get<NeverType>(follow(lhs))) return rhs;`
    if get_type::get::<NeverType>(follow(lhs)).is_some() {
      return rhs;
    }

    if get_type::get::<NeverType>(follow(rhs)).is_some() {
      return lhs;
    }

    let result = simplify_union(self.builtin_types, self.arena, lhs, rhs).result;

    if get_type::get::<UnionType>(follow(result)).is_some() {
      self.unions_to_simplify.push(result);
    }

    result
  }

  pub fn make_union_vector_type_id(&mut self, options: Vec<TypeId>) -> TypeId {
    let mut ub = UnionBuilder::new(self.arena, self.builtin_types);
    ub.reserve(options.len());

    for option in options {
      ub.add(option);
    }

    let union_ty = ub.build();

    if get_type::get::<UnionType>(union_ty).is_some() {
      self.unions_to_simplify.push(union_ty);
    }

    union_ty
  }
}

impl ConstraintGenerator {
  /// 直译 cpp `ConstraintGenerator::prepopulateGlobalScope(globalScope, program)`。
  /// 形参链已引用化（原 `# Safety` 契约由签名承担）：`global_scope` 为存活
  /// `Arc<Scope>` 共享借用；`program` 为分析会话 arena 内存活的 `AstStatBlock`
  /// 共享借用，visit 期间无人并发改写该 AST（本 pass 由驱动者独占 arena）；
  /// `GlobalPrepopulator` 的 NonNull 三字段由存活句柄经 `NonNull::from` 构造
  /// （记录布局不改），遍历所需的 `&mut AstStatBlock` 由 alias 门面即时物化。
  pub fn prepopulate_global_scope(&mut self, global_scope: &ScopePtr, program: &AstStatBlock) {
    let mut gp = GlobalPrepopulator {
      global_scope: NonNull::from(&**global_scope),
      arena: NonNull::from(self.arena.get_mut()),
      dfg: NonNull::from(self.dfg_ref()),
      uninitialized_globals: DenseHashSet::default(),
    };

    if let Some(module) = &self.module {
      (self.prepare_module_scope)(&module.name, global_scope);
    }

    // visit 遍历 API 要求 `&mut AstStatBlock`（GlobalPrepopulator 仅读不改 AST）；
    // alias 门面从本函数参数引用即时物化、语句内即用即弃。
    alias(from_ref(program).cast_mut()).visit(&mut gp);

    for name in gp.uninitialized_globals.iter() {
      self.uninitialized_globals.insert(name);
    }

    // 尾段与 `prepopulate_global_scope_for_fragment_typecheck` 主体同体
    // （cpp 同名函数共享的 type function 环境预填充），收敛到单点实现。
    self.prepopulate_type_function_globals(program);
  }
}

impl DenseDefault for InferredBinding {
  fn dense_default() -> Self {
    Self {
      scope: null_mut(),
      location: Location::default(),
      types: TypeIds::new(),
    }
  }
}
impl ConstraintGenerator {
  pub fn record_inferred_binding(&mut self, local: *mut AstLocal, ty: TypeId) {
    if let Some(ib) = self.inferred_bindings.find_mut(&Symbol::from_local(local)) {
      ib.types.insert_type_id(ty);
    }
  }
}

impl ConstraintGenerator {
  pub fn report_code_too_complex(&mut self, location: Location) {
    // cpp:5129-5136 的前两句与 `reportError` 完全同体（emplace + logger
    // capture），此处直接复用；差异仅在其后追加的 `recursionLimitMet` 置位。
    self.report_error(
      location,
      TypeErrorData::CodeTooComplex(CodeTooComplex { _unused: None }),
    );
    self.recursion_limit_met = true;
  }
}

impl ConstraintGenerator {
  pub fn report_error(&mut self, location: Location, err: TypeErrorData) {
    self.errors.push(TypeError {
      location,
      // 构造期接线不变式：module 恒 Some。
      module_name: self
        .module
        .as_ref()
        .expect("ConstraintGenerator 构造期以 ModulePtr 接线并 LUAU_ASSERT(is_some)，恒为 Some")
        .name
        .clone(),
      data: err.clone(),
    });
    if !self.logger.is_null() {
      // 先经 is_null 短路；logger 为构造期注入、与会话同寿的 DcrLogger 裸句柄
      // （记录布局不改），解引用经 alias 收口；紧邻 push 之后取尾，必非空。
      alias(self.logger).capture_generation_error(
        self
          .errors
          .last()
          .expect("紧邻上方 push 刚入队，last 必命中（cpp errors.back() 同位）"),
      );
    }
  }
}

impl ConstraintGenerator {
  /// C++ `resolveTypeArguments(const ScopePtr& scope, const AstArray<AstTypeOrPack>&)`：
  /// `scope` 为调用方持有的 `ScopePtr` 共享借用，数组元素载荷为 parser arena
  /// 存活节点的长寿共享引用（`AstTypeOrPack` 枚举）。
  pub fn resolve_type_arguments(
    &mut self,
    scope: &ScopePtr,
    type_arguments: AstArray<AstTypeOrPack>,
  ) -> (Vec<TypeId>, Vec<TypePackId>) {
    let mut resolved_type_arguments = Vec::new();
    let mut resolved_type_pack_arguments = Vec::new();

    for &type_or_pack in type_arguments.iter() {
      match type_or_pack {
        AstTypeOrPack::Type(ty) => {
          resolved_type_arguments.push(self.resolve_type(
            scope,
            ty,
            false,
            false,
            Polarity::Unknown,
          ));
        }
        AstTypeOrPack::Pack(pack) => {
          resolved_type_pack_arguments.push(
            self.resolve_type_pack_scope_ptr_ast_type_pack_bool_bool_polarity(
              scope,
              pack,
              false,
              false,
              Polarity::Unknown,
            ),
          );
        }
        // cpp 走 else 臂先 `LUAU_ASSERT(tp.typePack)` 再把 null 传进 resolveTypePack
        // （ConstraintGenerator.cpp:5714）；变体载荷恒非空，null 侧只剩断言上报，
        // 故此处只保留断言失败路径，不再向下游透传空槽。
        AstTypeOrPack::Error => LUAU_ASSERT!(false),
      }
    }

    (resolved_type_arguments, resolved_type_pack_arguments)
  }
}

impl ConstraintGenerator {
  /// C++ `ConstraintSet ConstraintGenerator::run(AstStatBlock* block)`：`block`
  /// 为会话 arena 根块的裸指针入口（记录字段直传边界），本函数经 alias_ref
  /// 收口为共享引用后全程只读。
  pub(crate) fn run(&mut self, block: *mut AstStatBlock) -> ConstraintSet {
    self.visit_module_root(alias_ref(block));

    ConstraintSet {
      // `visit_module_root` 已置位 root_scope（C++ `NotNull` 语义），向下沉为
      // 写句柄交给 ConstraintSet/求解器（其字段 Option 化属后续波段）。
      root_scope: shared_mut(self.root()),
      constraints: take(&mut self.constraints),
      free_types: take(&mut self.free_types),
      // take 与原 `replace(.., DenseHashMap::default())` 逐位等价：
      // default 门面哨兵即 null 键。
      scope_to_function: take(&mut self.scope_to_function),
      errors: take(&mut self.errors),
    }
  }
}

impl ConstraintGenerator {
  pub(crate) fn simplify_union(
    &mut self,
    _scope: ScopePtr,
    _location: Location,
    left: TypeId,
    right: TypeId,
  ) -> TypeId {
    simplify_union(self.builtin_types, self.arena, left, right).result
  }
}

impl ConstraintGenerator {
  // ConstraintGenerator::updateRValueRefinements(const ScopePtr&, DefId, TypeId) const
  // (ConstraintGenerator.cpp:5139).
  /// 形参链已引用化（原 `# Safety` 契约由签名承担）：`scope` 为调用方持有的
  /// 存活 `Arc<Scope>` 共享借用，写入经 alias 门面即时物化（单线程独占写、
  /// 借用窗口止于各语句），`dfg` 为构造期注入的只读句柄。
  pub(crate) fn update_r_value_refinements_scope_ptr_def_id_type_id(
    &self,
    scope: &ScopePtr,
    def: DefId,
    ty: TypeId,
  ) {
    *alias(shared_mut(scope))
      .rvalue_refinements
      .get_or_insert(def) = ty;

    // C++ (ConstraintGenerator.cpp:5142-5143):
    //     if (auto sym = dfg->getSymbolFromDef(def))
    //         scope->refinements[*sym] = ty;
    // Only locals/globals are mapped in `defToSymbol`; for any other def
    // (e.g. a property-access def from `checkIndexName`) `getSymbolFromDef`
    // returns nullopt and the refinement write is skipped. Do NOT fall back
    // to `def->name` — that would associate an index result with the base
    // symbol's `refinements` entry, which later corrupts the fragment clone.
    if let Some(sym) = self.dfg_ref().get_symbol_from_def(def) {
      alias(shared_mut(scope))
        .refinements
        .insert(LValue::Symbol(sym), ty);
    }
  }
}

impl ConstraintGenerator {
  /// C++ `visitBlockWithoutChildScope(const ScopePtr& scope, AstStatBlock* block)`：
  /// `scope` 为调用方持有的存活 `ScopePtr` 共享借用（cpp `const ScopePtr&` 的
  /// Rust 对应，不再经裸指针重建 Arc）；`block` 为解析器 arena 存活的
  /// `AstStatBlock` 共享借用。
  pub fn visit_block_without_child_scope(
    &mut self,
    scope: &ScopePtr,
    block: &AstStatBlock,
  ) -> ControlFlow {
    let _counter = RecursionCounter::recursion_counter_i32(&mut self.recursion_count);

    if self.recursion_count >= dfint::LuauConstraintGeneratorRecursionLimit.get() {
      self.report_code_too_complex(block.base.base.location);
      return ControlFlow::None;
    }

    self.prototype_type_definitions(scope, block);

    let mut first_control_flow: Option<ControlFlow> = None;
    for stat in block.body.iter() {
      let cf = self.visit_stat(scope, stat);
      if cf != ControlFlow::None && first_control_flow.is_none() {
        first_control_flow = Some(cf);
      }
    }

    first_control_flow.unwrap_or(ControlFlow::None)
  }
}
