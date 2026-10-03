// `unifierState.cachedUnifyError.clear()` (a `DenseHashMap<.., TypeErrorData>`)
// needs the value type to be default-constructible for empty slots, modelled in
// the Rust port by `DenseDefault`. The default sentinel is never read as a real
// error.
use alloc::{sync::Arc, vec::Vec};
use core::ptr::NonNull;

use ulua_ast::{enums::mode::Mode, records::location::Location};
use ulua_common::{fflag, fint, records::dense_hash_table::DenseDefault};

use crate::{
  enums::solver_mode::SolverMode,
  functions::{
    follow_type_pack, freeze::freeze, get_type_pack, shared_mut::shared_mut,
    synthesize_export_return::synthesize_export_return,
  },
  records::{
    arena_handle::{Handle, alias_ref},
    code_too_complex::CodeTooComplex,
    free_type_pack::FreeTypePack,
    module::Module,
    scope::Scope,
    scope_registry::{register_scope, resolve_scope_mut},
    source_module::SourceModule,
    type_checker::TypeChecker,
    type_pack::TypePack,
  },
  type_aliases::{
    error_vec::ErrorVec, module_ptr_module::ModulePtr, name_type::Name, scope_ptr_type::ScopePtr,
    type_error_data::TypeErrorData,
  },
};
impl DenseDefault for TypeErrorData {
  fn dense_default() -> Self {
    TypeErrorData::CodeTooComplex(CodeTooComplex)
  }
}

impl TypeChecker {
  pub fn check_without_recursion_check(
    &mut self,
    module: &SourceModule,
    mode: Mode,
    environment_scope: Option<ScopePtr>,
  ) -> ModulePtr {
    let new_module = Module {
      name: module.name.clone(),
      human_readable_name: module.human_readable_name.clone(),
      r#type: module.r#type,
      allocator: Some(module.allocator.clone()),
      names: Some(module.names.clone()),
      // `Module.root` 已句柄化：直传 `Option<Handle>`，`None` ≡ cpp nullptr。
      root: module.root,
      ..Default::default()
    };

    let current_module: ModulePtr = Arc::new(new_module);
    self.current_module = Some(current_module.clone());

    // currentModule->internalTypes.owningModule = currentModule.get();
    // currentModule->interfaceTypes.owningModule = currentModule.get();
    let module_mut = shared_mut(self.expect_current_module());
    let owning = Some(NonNull::from(&mut *module_mut));
    module_mut.internal_types.owning_module = owning;
    module_mut.interface_types.owning_module = owning;

    // Safety: self.ice_handler 为 Handle（构造期由 Frontend 持有的
    // InternalErrorReporter 接线，非空由类型编码），所指对象寿命覆盖整个
    // check，写 module_name 无并存借用。arena 取的是 Arc::as_ptr(current_module)
    // 所指 Module 的 internal_types 字段地址：Arc 分配块内的对象不搬运，
    // self.current_module 持有该 Arc 直至末尾 take()；checker 此后才开始使用该
    // TypeArena，此前无其他活跃借用，故经 Handle::from_mut 固化为句柄持久化到
    // normalizer.arena 成立（对应 cpp normalizer.arena 指针成员）。
    unsafe {
      self.ice_handler.get_mut().module_name = module.name.to_string();
      self.normalizer.arena = Some(Handle::from_mut(
        &mut (*(Arc::as_ptr(self.expect_current_module()) as *mut Module)).internal_types,
      ));
    }

    self.unifier_state.counters.recursion_limit = fint::LuauTypeInferRecursionLimit.get();
    self.unifier_state.counters.iteration_limit = match self.unifier_iteration_limit {
      Some(limit) => limit,
      None => fint::LuauTypeInferIterationLimit.get(),
    };

    let parent_scope = environment_scope.unwrap_or_else(|| alias_ref(self.global_scope).clone());
    let module_scope: ScopePtr = Arc::new(Scope::new(&parent_scope, 0));
    let module_scope_id = register_scope(&module_scope);

    // fresh_type_pack 只读取 scope.level，不保留 Arc 引用；创建点写回经注册表
    // 写出口 resolve_scope_mut（注册表持 Arc 强引用、地址稳定，句柄必可解析，
    // 借用半径止于下面两条字段写，与 shared_mut 写穿同一纪律）。
    let fresh_return = self.fresh_type_pack_scope_ptr(&module_scope);
    let fresh_vararg = self.fresh_type_pack_scope_ptr(&module_scope);
    let module_scope_mut =
      resolve_scope_mut(module_scope_id).expect("module_scope_id 刚由 register_scope 发放");
    module_scope_mut.return_type = fresh_return;
    module_scope_mut.vararg_pack = Some(fresh_vararg);

    {
      let module_mut = shared_mut(self.expect_current_module());
      // cpp `module->scopes.push_back({sourceModule.root->location, ...})`：
      // 根块在场为检查契约，句柄物化只读借用。
      let root = module
        .root
        .expect("checkWithoutRecursionCheck: 根块应在场（cpp 直取 sourceModule.root）");
      let root_location = root.get().base.base.location;
      module_mut
        .scopes
        .push((root_location, module_scope.clone()));
      module_mut.mode = mode;
    }

    if let Some(prepare_module_scope) = &self.prepare_module_scope {
      let module_name = self.expect_current_module().name.clone();
      prepare_module_scope(&module_name, &module_scope);
    }

    self.check_block(
      &module_scope,
      module
        .root
        .expect("checkWithoutRecursionCheck: 根块应在场（cpp 直取 sourceModule.root）")
        .get(),
    );

    if fflag::LuauExportValueSyntax.get()
      && fflag::LuauExportValueTypecheck.get()
      && !self.expect_current_module().timeout
      && !self.expect_current_module().cancelled
    {
      // builtin_types 为 Handle（NonNull 编码非空，Frontend 持有、覆盖本调用）；
      // module 借自 current_module 的写穿句柄，先前借用皆已结束。
      let module_mut = shared_mut(self.expect_current_module());
      synthesize_export_return(self.builtin_types, module_mut);
    }

    let module_return_type = module_scope.return_type;
    if get_type_pack::get::<FreeTypePack>(follow_type_pack::follow(module_return_type)).is_some() {
      let empty_pack = self.add_type_pack_type_pack(TypePack::empty());
      shared_mut(&module_scope).return_type = empty_pack;
    } else {
      let anyified = self.anyify_type_pack_id_location(module_return_type, Location::default());
      shared_mut(&module_scope).return_type = anyified;
    }

    let anyified_generics = self.anyify_module_return_type_pack_generics(module_scope.return_type);
    shared_mut(&module_scope).return_type = anyified_generics;

    let keys: Vec<Name> = module_scope
      .exported_type_bindings
      .keys()
      .cloned()
      .collect();
    for key in keys {
      let ty = module_scope
        .exported_type_bindings
        .get(&key)
        .expect("keys 刚从同一 map 的 keys() 收集，循环内不删改键，get 必命中")
        .r#type;
      let anyified = self.anyify_type_id_location(ty, Location::default());
      // keys 事先收集为 owned Vec，避免迭代期间借用 map；写止于本语句。
      shared_mut(&module_scope)
        .exported_type_bindings
        .get_mut(&key)
        .expect("keys 刚从该 map 的 keys() 收集，循环内不删改键，get_mut 必命中")
        .r#type = anyified;
    }

    // Safety: errors 指针链为 current_module 的 shared_mut 句柄取 &mut 字段地址，
    // 中转 *mut ErrorVec 只是解除 self 借用链与 Module 借用的 borrowck 关联（对应
    // cpp 里 `this->prepareErrorsForDisplay(module->errors)` 两参数并存）。调用
    // 期间 checker 对该 ErrorVec 无第二访问路径，prepare 只改 errors 内容，返回
    // 即借用结束。
    unsafe {
      let errors = &mut (shared_mut(self.expect_current_module())).errors as *mut ErrorVec;
      self.prepare_errors_for_display(&mut *errors);
    }

    // Clear the normalizer caches, since they contain types from the internal type surface
    self.normalizer.clear_caches();
    // null 哨兵恢复为 Option::None（模块外归一化上报路径语义不变）。
    self.normalizer.arena = None;

    // 说明: 三处接线来源分明——module_mut 是仍被 self.current_module 持有的
    // Arc 写穿句柄（函数未结束，take 在最后一行）；ice 经 self.ice_handler 句柄
    // get_mut 物化（对象由 Frontend 持有、寿命覆盖本调用，句柄期无并存别名）；
    // builtin_types 同前。ice 与 Module 内存不相交；freeze 对两个 TypeArena 的
    // 可变借用由句柄字段地址派生，在调用处结束——此刻 normalizer.arena 已置
    // null，无任何并发借用。
    let module_mut = shared_mut(self.expect_current_module());
    let ice = self.ice_handler.get_mut();
    (*module_mut).clone_public_interface(self.builtin_types, ice, SolverMode::Old);

    freeze(&mut module_mut.internal_types);
    freeze(&mut module_mut.interface_types);

    // Clear unifier cache since it's keyed off internal types that get deallocated
    self.unifier_state.cached_unify.clear();
    self.unifier_state.cached_unify_error.clear();
    self.unifier_state.skip_cache_for_type.clear();

    self.duplicate_type_aliases.clear();
    self.incorrect_extern_type_definitions.clear();

    self
      .current_module
      .take()
      .expect("本函数开头置入 Some 且全程未 take，末尾必命中")
  }
}
