//! C++ `GlobalTypes::GlobalTypes(NotNull<BuiltinTypes>, SolverMode)`
//! (`Analysis/src/GlobalTypes.cpp:11`). Builds the shared global scope and the
//! global type-function scope, registers the builtin type bindings, and wires
//! up the string metatable.
use alloc::{sync::Arc, vec::Vec};
use core::ptr::{NonNull, null_mut};

use ulua_common::fflag;

use crate::{
  enums::solver_mode::SolverMode,
  functions::{
    as_mutable_type::as_mutable_type_id, freeze::freeze,
    make_string_metatable::make_string_metatable, persist_type::persist, unfreeze::unfreeze,
  },
  records::{
    builtin_types::BuiltinTypes, free_type_pack::FreeTypePack, global_types::GlobalTypes,
    primitive_type::PrimitiveType, scope::Scope, scope_registry::register_scope,
    source_module::SourceModule, type_arena::TypeArena, type_fun::TypeFun, type_level::TypeLevel,
  },
  type_aliases::type_variant::TypeVariant,
};
impl GlobalTypes {
  /// C++ `GlobalTypes(NotNull<BuiltinTypes>, SolverMode)`。内建单例入参以
  /// `&mut BuiltinTypes` 受检引用承载（对应 C++ `NotNull` 的「非空、存活、
  /// 单一」语义），构造期内部即时转铸为 `NonNull` 句柄供 arena chokepoint 使用，
  /// 调用点免构造 `NonNull`。
  pub fn new(builtin_types: &mut BuiltinTypes, mode: SolverMode) -> Self {
    // `&mut` 引用天然非空且对齐，转铸免 unsafe；本函数体内 arena 改写
    // （`arena_handle`）与只读快照（`builtin_types_of`）一律复用此句柄。
    let builtin_types = NonNull::from(builtin_types);
    let mut global_types = TypeArena::default();

    // globalScope = std::make_shared<Scope>(globalTypes.addTypePack(TypePackVar{FreeTypePack{TypeLevel{}}}));
    let mut free_pack = FreeTypePack {
      index: 0,
      level: TypeLevel::default(),
      scope: null_mut(),
      polarity: Default::default(),
    };
    free_pack.free_type_pack_type_level(TypeLevel::default());
    let global_scope_ret = global_types.add_type_pack_t(free_pack);

    let mut free_pack_fn = FreeTypePack {
      index: 0,
      level: TypeLevel::default(),
      scope: null_mut(),
      polarity: Default::default(),
    };
    free_pack_fn.free_type_pack_type_level(TypeLevel::default());
    let global_type_function_scope_ret = global_types.add_type_pack_t(free_pack_fn);

    // Build the scope locally so we can register the builtin bindings before
    // sharing it via Arc (C++ mutates the freshly-constructed shared scope).
    let mut global_scope = Scope::scope_type_pack_id(global_scope_ret);
    let global_type_function_scope = Scope::scope_type_pack_id(global_type_function_scope_ret);

    // Snapshot the builtin TypeIds (raw `*const` copies) so the later
    // mutable borrows of `builtinTypes->arena` don't conflict.
    let (
      any_type,
      nil_type,
      number_type,
      integer_type,
      string_type,
      boolean_type,
      thread_type,
      buffer_type,
      unknown_type,
      never_type,
      object_type,
      class_type,
    ) = {
      // 解引用集中于 `GlobalTypes::builtin_types_of` chokepoint：入参 NonNull
      // 按 C++ `NotNull<BuiltinTypes>` 契约由构造方保证非空且指向比本次
      // GlobalTypes 构建长寿的单例；此处只读常量 TypeId 快照（均为既有 arena
      // 节点指针的拷贝）。
      let builtins = unsafe { Self::builtin_types_of(builtin_types) };
      (
        builtins.any_type,
        builtins.nil_type,
        builtins.number_type,
        builtins.integer_type,
        builtins.string_type,
        builtins.boolean_type,
        builtins.thread_type,
        builtins.buffer_type,
        builtins.unknown_type,
        builtins.never_type,
        builtins.object_type,
        builtins.class_type,
      )
    };

    global_scope
      .add_builtin_type_binding("any", &TypeFun::type_fun_type_id(any_type));
    global_scope
      .add_builtin_type_binding("nil", &TypeFun::type_fun_type_id(nil_type));
    global_scope.add_builtin_type_binding(
      "number",
      &TypeFun::type_fun_type_id(number_type),
    );
    if fflag::LuauIntegerType2.get() {
      global_scope.add_builtin_type_binding(
        "integer",
        &TypeFun::type_fun_type_id(integer_type),
      );
    }
    global_scope.add_builtin_type_binding(
      "string",
      &TypeFun::type_fun_type_id(string_type),
    );
    global_scope.add_builtin_type_binding(
      "boolean",
      &TypeFun::type_fun_type_id(boolean_type),
    );
    global_scope.add_builtin_type_binding(
      "thread",
      &TypeFun::type_fun_type_id(thread_type),
    );
    global_scope.add_builtin_type_binding(
      "buffer",
      &TypeFun::type_fun_type_id(buffer_type),
    );
    global_scope.add_builtin_type_binding(
      "unknown",
      &TypeFun::type_fun_type_id(unknown_type),
    );
    global_scope.add_builtin_type_binding(
      "never",
      &TypeFun::type_fun_type_id(never_type),
    );
    if fflag::DebugLuauUserDefinedClasses.get() {
      global_scope.add_builtin_type_binding(
        "object",
        &TypeFun::type_fun_type_id(object_type),
      );
      global_scope.add_builtin_type_binding(
        "class",
        &TypeFun::type_fun_type_id(class_type),
      );
    }

    let global_scope: Arc<Scope> = Arc::new(global_scope);
    register_scope(&global_scope);
    let global_type_function_scope: Arc<Scope> = Arc::new(global_type_function_scope);
    register_scope(&global_type_function_scope);

    // unfreeze(*builtinTypes->arena);
    // Safety: builtin_types 为 NotNull 契约接线的存活单例，其 arena 字段是 Box 独占
    // 堆分配的 TypeArena（唯一对象、地址稳定）；构造序列单线程执行，unfreeze 期间
    // 无其他借用指向该 arena，满足 `BuiltinTypes::arena_handle` 的 `# Safety` 契约。
    unfreeze(unsafe { BuiltinTypes::arena_handle(builtin_types) }.get_mut());

    let string_metatable_ty = make_string_metatable(builtin_types, mode);

    // asMutable(builtinTypes->string_type)->ty.emplace<PrimitiveType>(PrimitiveType::STRING, stringMetatableTy);
    // Safety: string_type 取自上方 builtins 快照，是 builtin arena（bump 块，地址不
    // 移动）中存活 Type 节点的拷贝指针，as_mutable_type_id 仅恒等指针转换故非空；
    // 上一步已 unfreeze、下一步即 freeze，此窗口内构造序列是唯一写者，覆写 ty 与
    // C++ emplace 同语义（附 new 的 string 元表，节点自此持久化）。
    unsafe {
      (*as_mutable_type_id(string_type)).ty = TypeVariant::Primitive(PrimitiveType {
        r#type: PrimitiveType::STRING,
        metatable: Some(string_metatable_ty),
      });
    }

    persist(string_metatable_ty);

    // freeze(*builtinTypes->arena);
    // Safety: 与上方 unfreeze 分支同一论证——Box 独占的 arena 对象、构造序列
    // 单线程且此刻无并发借用，句柄物化不引入别名。
    freeze(unsafe { BuiltinTypes::arena_handle(builtin_types) }.get_mut());

    Self {
      builtin_types: Some(builtin_types),
      global_types,
      global_names: SourceModule::new(),
      global_scope,
      global_type_function_scope,
      mode,
      retained_modules: Vec::new(),
    }
  }
}
