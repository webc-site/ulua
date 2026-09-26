// `unifierState.cachedUnifyError.clear()` (a `DenseHashMap<.., TypeErrorData>`)
// needs the value type to be default-constructible for empty slots, modelled in
// the Rust port by `DenseDefault`. The default sentinel is never read as a real
// error.
use alloc::{sync::Arc, vec::Vec};

use ulua_ast::{enums::mode::Mode, records::location::Location};
use ulua_common::{fflag, fint, records::dense_hash_table::DenseDefault};

use crate::{
  enums::solver_mode::SolverMode,
  functions::{
    arc_as_mut::arc_as_mut, follow_type_pack, freeze::freeze, get_type_pack,
    synthesize_export_return::synthesize_export_return,
  },
  records::{
    arena_handle::Handle,
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
    TypeErrorData::CodeTooComplex(CodeTooComplex::default())
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
      root: module.root,
      ..Default::default()
    };

    let current_module: ModulePtr = Arc::new(new_module);
    self.current_module = Some(current_module.clone());

    // currentModule->internalTypes.owningModule = currentModule.get();
    // currentModule->interfaceTypes.owningModule = currentModule.get();
    // Safety: module_mut 是按 arc_as_mut 契约从 self.current_module 取的写穿句柄
    // （对应 cpp 里用裸 this 直写 owningModule）。该 Arc 由 self.current_module
    // 持有直到函数末尾 take()，指针所指 Module 在此期间恒存活；两条写入各在语句
    // 内完成并立即结束借用，此刻不存在任何其他并存 Module 借用，且分析单线程、
    // 经 &mut self 独占驱动，无非别名冲突。
    unsafe {
      let module_mut = arc_as_mut(self.expect_current_module());
      (*module_mut).internal_types.owning_module = module_mut;
      (*module_mut).interface_types.owning_module = module_mut;
    }

    // Safety: self.ice_handler 是 C++ 引用成员 InternalErrorReporter& 的裸指针化，
    // 所指对象由 Frontend 持有、生命周期覆盖整个 check，写 module_name 无并存借用。
    // arena 取的是 Arc::as_ptr(current_module) 所指 Module 的 internal_types 字段
    // 地址：Arc 分配块内的对象不搬运，self.current_module 持有该 Arc 直至末尾
    // take()；checker 此后才开始使用该 TypeArena，此前无其他活跃借用，故经
    // Handle::from_mut 固化为句柄持久化到 normalizer.arena 成立（对应 cpp
    // normalizer.arena 指针成员）。
    unsafe {
      (*self.ice_handler).module_name = module.name.to_string();
      self.normalizer.arena = Some(Handle::from_mut(
        &mut (*(Arc::as_ptr(self.expect_current_module()) as *mut Module)).internal_types,
      ));
    }

    self.unifier_state.counters.recursion_limit = fint::LuauTypeInferRecursionLimit.get();
    self.unifier_state.counters.iteration_limit = match self.unifier_iteration_limit {
      Some(limit) => limit,
      None => fint::LuauTypeInferIterationLimit.get(),
    };

    // Safety: global_scope 是 C++ `const ScopePtr&` 引用成员的裸指针化（*const
    // ScopePtr），所指 ScopePtr 由 TypeChecker 的构造方持有、生命周期长于本
    // checker；此处只读解引用并 clone 内层 Arc 的引用计数，不产生任何写入。
    let parent_scope = environment_scope.unwrap_or_else(|| unsafe { (*self.global_scope).clone() });
    let module_scope: ScopePtr = Arc::new(Scope::new(&parent_scope, 0));
    let module_scope_id = register_scope(&module_scope);

    // fresh_type_pack 只读取 scope.level，不保留 Arc 引用；创建点写回经注册表
    // 写出口 resolve_scope_mut（注册表持 Arc 强引用、地址稳定，句柄必可解析，
    // 借用半径止于下面两条字段写，与 arc_as_mut 写穿同一纪律）。
    let fresh_return = self.fresh_type_pack_scope_ptr(&module_scope);
    let fresh_vararg = self.fresh_type_pack_scope_ptr(&module_scope);
    let module_scope_mut =
      resolve_scope_mut(module_scope_id).expect("module_scope_id 刚由 register_scope 发放");
    module_scope_mut.return_type = fresh_return;
    module_scope_mut.vararg_pack = Some(fresh_vararg);

    // Safety: module_mut 同上按 arc_as_mut 写穿契约取值，Arc 存活且无并存借用，
    // 单线程独占；(*module.root) 是 SourceModule 随 allocator 持有的解析树根，
    // 分析期存活且此处只读取 location 字段。push 之后 module_scope 的 Arc 引用
    // 计数升为 2（本地 + scopes），本语句内写入 scopes/mode 后即结束借用。
    unsafe {
      let module_mut = arc_as_mut(self.expect_current_module());
      (*module_mut)
        .scopes
        .push(((*module.root).base.base.location, module_scope.clone()));
      (*module_mut).mode = mode;
    }

    if let Some(prepare_module_scope) = &self.prepare_module_scope {
      let module_name = self.expect_current_module().name.clone();
      prepare_module_scope(&module_name, &module_scope);
    }

    // Safety: module.root 指向 allocator 块内的解析树根节点，check 期间解析树
    // 不释放也不改写；这里建立只读借用交 check_block 遍历，checker 的写入全部
    // 进 type arena 与 Scope，不触及 AST 内存，无读写冲突。
    self.check_block(&module_scope, unsafe { &*module.root });

    if fflag::LuauExportValueSyntax.get()
      && fflag::LuauExportValueTypecheck.get()
      && !self.expect_current_module().timeout
      && !self.expect_current_module().cancelled
    {
      {
        // Safety: 满足 synthesize_export_return 的 # Safety——builtin_types 为
        // self.builtin_types（Handle：NonNull 编码非空，Frontend 持有、
        // 覆盖本调用）；module_mut 为 current_module 的 arc_as_mut 写穿句柄，该
        // Arc 存活且被调方以独占 &mut 视角使用 Module，调用点处 scopes.push 等
        // 先前借用皆已结束，本函数体内无第二读写路径。
        unsafe {
          let module_mut = arc_as_mut(self.expect_current_module());
          synthesize_export_return(self.builtin_types, module_mut)
        };
      }
    }

    let module_return_type = module_scope.return_type;
    if get_type_pack::get::<FreeTypePack>(follow_type_pack::follow(module_return_type)).is_some() {
      let empty_pack = self.add_type_pack_type_pack(TypePack::empty());
      // Safety: module_scope 的 Arc 已被 clone 进 current_module.scopes（计数
      // ≥2），照 arc_as_mut 契约取写穿句柄对应 cpp `moduleScope->returnType =
      // emptyPack` 裸写。empty_pack 在上一步已算出，此刻无任何 Scope 借用存活；
      // 写入在语句内完成，单线程分析保证无并存读者。
      unsafe {
        let module_scope_mut = arc_as_mut(&module_scope);
        (*module_scope_mut).return_type = empty_pack;
      }
    } else {
      let anyified = self.anyify_type_pack_id_location(module_return_type, Location::default());
      // Safety: 写句柄来源同上（本地 Arc 与 scopes 中 clone 指向同一 Scope）；
      // anyify 调用已返回、其内部经 clone 派生的借用全部失效，写 return_type 时
      // 无存续借用，借用随语句结束。
      unsafe {
        let module_scope_mut = arc_as_mut(&module_scope);
        (*module_scope_mut).return_type = anyified;
      }
    }

    let anyified_generics = self.anyify_module_return_type_pack_generics(module_scope.return_type);
    // Safety: 第三处对同一 Scope 的 return_type 写穿，句柄经 arc_as_mut 取自本地
    // module_scope Arc；anyified_generics 已求值完毕（TypeId 为 Copy），写入瞬间
    // 仅本语句借用该字段，后续读取均发生在此之后，不构成交替借用冲突。
    unsafe {
      let module_scope_mut = arc_as_mut(&module_scope);
      (*module_scope_mut).return_type = anyified_generics;
    }

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
      // Safety: keys 事先收集为 owned Vec，避免迭代期间借用 map；ty 是 Copy
      // TypeId 读取且 anyify 返回后借用已结束。写穿经 Arc::as_ptr 句柄（本地与
      // scopes 的 Arc 同指该 Scope），get_mut 槽位借用于赋值语句末结束，此刻
      // 循环体内无其他对该 Scope 的存活借用。
      unsafe {
        let module_scope_mut = arc_as_mut(&module_scope);
        module_scope_mut
          .as_mut()
          .expect("arc_as_mut 写穿句柄按契约非空，as_mut 必为 Some")
          .exported_type_bindings
          .get_mut(&key)
          .expect("keys 刚从该 map 的 keys() 收集，循环内不删改键，get_mut 必命中")
          .r#type = anyified;
      }
    }

    // Safety: errors 指针链为 current_module 的 arc_as_mut 句柄取 &mut 字段地址，
    // 中转 *mut ErrorVec 只是解除 self 借用链与 Module 借用的 borrowck 关联（对应
    // cpp 里 `this->prepareErrorsForDisplay(module->errors)` 两参数并存）。调用
    // 期间 checker 对该 ErrorVec 无第二访问路径，prepare 只改 errors 内容，返回
    // 即借用结束。
    unsafe {
      let errors = &mut (*(arc_as_mut(self.expect_current_module()))).errors as *mut ErrorVec;
      self.prepare_errors_for_display(&mut *errors);
    }

    // Clear the normalizer caches, since they contain types from the internal type surface
    self.normalizer.clear_caches();
    // null 哨兵恢复为 Option::None（模块外归一化上报路径语义不变）。
    self.normalizer.arena = None;

    // Safety: 三枚裸指针来源分明——module_mut 是仍被 self.current_module 持有的
    // Arc 写穿句柄（函数未结束，take 在最后一行）；ice 借用自 ice_handler（C++
    // 引用成员裸指针化，对象由 Frontend 持有、寿命覆盖本调用）；builtin_types
    // 同前。clone_public_interface 的 # Safety 由此满足，且 ice 与 Module 内存
    // 不相交；freeze 对两个 TypeArena 的可变借用由句柄字段地址派生，在调用处
    // 结束——此刻 normalizer.arena 已置 null，无任何并发借用。
    unsafe {
      let module_mut = arc_as_mut(self.expect_current_module());
      let ice = &mut *self.ice_handler;
      (*module_mut).clone_public_interface(self.builtin_types, ice, SolverMode::Old);

      freeze(&mut (*module_mut).internal_types);
      freeze(&mut (*module_mut).interface_types);
    }

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
