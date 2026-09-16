//! C++ `GlobalTypes::GlobalTypes(NotNull<BuiltinTypes>, SolverMode)`
//! (`Analysis/src/GlobalTypes.cpp:11`). Builds the shared global scope and the
//! global type-function scope, registers the builtin type bindings, and wires
//! up the string metatable.
use alloc::{string::String, sync::Arc, vec::Vec};
use core::ptr::{NonNull, null_mut};

use ulua_common::FFlag;

use crate::{
  enums::solver_mode::SolverMode,
  functions::{
    as_mutable_type::as_mutable_type_id, freeze::freeze,
    make_string_metatable::make_string_metatable, persist_type::persist, unfreeze::unfreeze,
  },
  records::{
    builtin_types::BuiltinTypes, free_type_pack::FreeTypePack, global_types::GlobalTypes,
    primitive_type::PrimitiveType, scope::Scope, source_module::SourceModule,
    type_arena::TypeArena, type_fun::TypeFun, type_level::TypeLevel,
  },
  type_aliases::type_variant::TypeVariant,
};
impl GlobalTypes {
  pub fn new(mut builtin_types: NonNull<BuiltinTypes>, mode: SolverMode) -> Self {
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
      let builtins = unsafe { builtin_types.as_ref() };
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
      .add_builtin_type_binding(&String::from("any"), &TypeFun::type_fun_type_id(any_type));
    global_scope
      .add_builtin_type_binding(&String::from("nil"), &TypeFun::type_fun_type_id(nil_type));
    global_scope.add_builtin_type_binding(
      &String::from("number"),
      &TypeFun::type_fun_type_id(number_type),
    );
    if FFlag::LuauIntegerType2.get() {
      global_scope.add_builtin_type_binding(
        &String::from("integer"),
        &TypeFun::type_fun_type_id(integer_type),
      );
    }
    global_scope.add_builtin_type_binding(
      &String::from("string"),
      &TypeFun::type_fun_type_id(string_type),
    );
    global_scope.add_builtin_type_binding(
      &String::from("boolean"),
      &TypeFun::type_fun_type_id(boolean_type),
    );
    global_scope.add_builtin_type_binding(
      &String::from("thread"),
      &TypeFun::type_fun_type_id(thread_type),
    );
    global_scope.add_builtin_type_binding(
      &String::from("buffer"),
      &TypeFun::type_fun_type_id(buffer_type),
    );
    global_scope.add_builtin_type_binding(
      &String::from("unknown"),
      &TypeFun::type_fun_type_id(unknown_type),
    );
    global_scope.add_builtin_type_binding(
      &String::from("never"),
      &TypeFun::type_fun_type_id(never_type),
    );
    if FFlag::DebugLuauUserDefinedClasses.get() {
      global_scope.add_builtin_type_binding(
        &String::from("object"),
        &TypeFun::type_fun_type_id(object_type),
      );
      global_scope.add_builtin_type_binding(
        &String::from("class"),
        &TypeFun::type_fun_type_id(class_type),
      );
    }

    let global_scope: Arc<Scope> = Arc::new(global_scope);
    let global_type_function_scope: Arc<Scope> = Arc::new(global_type_function_scope);

    // unfreeze(*builtinTypes->arena);
    unsafe {
      let arena: &mut TypeArena = &mut *(&mut *builtin_types.as_mut().arena as *mut TypeArena);
      unfreeze(arena);
    }

    let string_metatable_ty = make_string_metatable(builtin_types, mode);

    // asMutable(builtinTypes->string_type)->ty.emplace<PrimitiveType>(PrimitiveType::STRING, stringMetatableTy);
    unsafe {
      (*as_mutable_type_id(string_type)).ty = TypeVariant::Primitive(PrimitiveType {
        r#type: PrimitiveType::STRING,
        metatable: Some(string_metatable_ty),
      });
    }

    persist(string_metatable_ty);

    // freeze(*builtinTypes->arena);
    unsafe {
      let arena: &mut TypeArena = &mut *(&mut *builtin_types.as_mut().arena as *mut TypeArena);
      freeze(arena);
    }

    Self {
      builtin_types,
      global_types,
      global_names: SourceModule::new(),
      global_scope,
      global_type_function_scope,
      mode,
      retained_modules: Vec::new(),
    }
  }
}
