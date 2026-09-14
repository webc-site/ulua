use alloc::{string::String, sync::Arc, vec::Vec};
use core::ptr::null;

use ulua_analysis::{
  functions::try_get_global_binding::try_get_global_binding,
  records::{
    blocked_type::BlockedType, function_type::FunctionType, generic_type::GenericType,
    pending_expansion_type::PendingExpansionType, scope::Scope, singleton_type::SingletonType,
    string_singleton::StringSingleton, to_string_options::ToStringOptions, type_arena::TypeArena,
  },
  type_aliases::singleton_variant::SingletonVariant,
};
use ulua_ast::records::ast_name::AstName;
use ulua_common::FFlag;

use crate::{
  functions::create_some_extern_types::create_some_extern_types,
  records::{fixture::Fixture, simplify_fixture::SimplifyFixture},
  type_aliases::scoped_fast_flag::ScopedFastFlag,
};
impl SimplifyFixture {
  pub fn new() -> Self {
    let null_type = null();
    let null_pack = null();

    let mut fixture = Self {
      base: Fixture::fixture_bool(false),
      arena: TypeArena::default(),
      opts: ToStringOptions::default(),
      scope: Arc::new(Scope::scope_type_pack_id(null_pack)),
      any_ty: null_type,
      unknown_ty: null_type,
      never_ty: null_type,
      error_ty: null_type,
      function_ty: null_type,
      table_ty: null_type,
      number_ty: null_type,
      string_ty: null_type,
      boolean_ty: null_type,
      nil_ty: null_type,
      class_ty: null_type,
      true_ty: null_type,
      false_ty: null_type,
      truthy_ty: null_type,
      falsy_ty: null_type,
      free_ty: null_type,
      generic_ty: null_type,
      blocked_ty: null_type,
      pending_ty: null_type,
      hello_ty: null_type,
      world_ty: null_type,
      empty_type_pack: null_pack,
      fn1_ty: null_type,
      fn2_ty: null_type,
      parent_class_ty: null_type,
      child_class_ty: null_type,
      another_child_class_ty: null_type,
      unrelated_class_ty: null_type,
      sff_debug_luau_force_old_solver: ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false),
    };

    {
      let frontend = fixture.base.get_frontend();
      create_some_extern_types(frontend);
    }

    let (parent_class_ty, child_class_ty, another_child_class_ty, unrelated_class_ty) = {
      let frontend = fixture.base.get_frontend();
      let globals = &mut frontend.globals;

      (
        try_get_global_binding(globals, "Parent")
          .expect("Parent global binding")
          .type_id,
        try_get_global_binding(globals, "Child")
          .expect("Child global binding")
          .type_id,
        try_get_global_binding(globals, "AnotherChild")
          .expect("AnotherChild global binding")
          .type_id,
        try_get_global_binding(globals, "Unrelated")
          .expect("Unrelated global binding")
          .type_id,
      )
    };

    let builtins_ptr = fixture.base.builtin_types;
    let builtins = unsafe { &mut *builtins_ptr };
    let scope = Arc::new(Scope::scope_type_pack_id(builtins.any_type_pack));
    let scope_ptr = Arc::as_ptr(&scope) as *mut Scope;

    let free_ty = fixture
      .arena
      .fresh_type_not_null_builtin_types_scope(builtins, scope_ptr);
    let generic_ty = fixture.arena.add_type(GenericType::default());
    let blocked_ty = fixture.arena.add_type(BlockedType::default());
    let pending_ty = fixture.arena.add_type(PendingExpansionType {
      prefix: None,
      name: AstName::default(),
      type_arguments: Vec::new(),
      pack_arguments: Vec::new(),
      index: PendingExpansionType::fresh_index(),
    });
    let hello_ty = fixture
      .arena
      .add_type(SingletonType::new(SingletonVariant::V1(
        StringSingleton::new(String::from("hello")),
      )));
    let world_ty = fixture
      .arena
      .add_type(SingletonType::new(SingletonVariant::V1(
        StringSingleton::new(String::from("world")),
      )));

    let empty_type_pack = fixture.arena.add_type_pack_initializer_list_type_id(&[]);
    let fn1_ty = fixture.arena.add_type(FunctionType::function_type_new(
      empty_type_pack,
      empty_type_pack,
      None,
      false,
    ));
    let fn2_ty = fixture.arena.add_type(FunctionType::function_type_new(
      builtins.any_type_pack,
      empty_type_pack,
      None,
      false,
    ));

    fixture.scope = scope;
    fixture.any_ty = builtins.any_type;
    fixture.unknown_ty = builtins.unknown_type;
    fixture.never_ty = builtins.never_type;
    fixture.error_ty = builtins.error_type;
    fixture.function_ty = builtins.function_type;
    fixture.table_ty = builtins.table_type;
    fixture.number_ty = builtins.number_type;
    fixture.string_ty = builtins.string_type;
    fixture.boolean_ty = builtins.boolean_type;
    fixture.nil_ty = builtins.nil_type;
    fixture.class_ty = builtins.extern_type;
    fixture.true_ty = builtins.true_type;
    fixture.false_ty = builtins.false_type;
    fixture.truthy_ty = builtins.truthy_type;
    fixture.falsy_ty = builtins.falsy_type;
    fixture.free_ty = free_ty;
    fixture.generic_ty = generic_ty;
    fixture.blocked_ty = blocked_ty;
    fixture.pending_ty = pending_ty;
    fixture.hello_ty = hello_ty;
    fixture.world_ty = world_ty;
    fixture.empty_type_pack = empty_type_pack;
    fixture.fn1_ty = fn1_ty;
    fixture.fn2_ty = fn2_ty;
    fixture.parent_class_ty = parent_class_ty;
    fixture.child_class_ty = child_class_ty;
    fixture.another_child_class_ty = another_child_class_ty;
    fixture.unrelated_class_ty = unrelated_class_ty;

    fixture
  }

  pub fn simplify_fixture(&mut self) {
    *self = Self::new();
  }
}
