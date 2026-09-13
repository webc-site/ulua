//! C++ `BuiltinTypes::BuiltinTypes()` (Type.cpp:843-880). Allocates the builtin
//! arena, mints every persistent builtin type/singleton/pack, then freezes the
//! arena. `type_functions` is `make_unique<BuiltinTypeFunctions>()`.
use alloc::{boxed::Box, string::ToString, vec};
use core::ptr::null_mut;

use ulua_common::FFlag;

use crate::{
  enums::table_state::TableState,
  functions::freeze::freeze,
  records::{
    any_type::AnyType, boolean_singleton::BooleanSingleton,
    builtin_type_functions::BuiltinTypeFunctions, builtin_types::BuiltinTypes,
    extern_type::ExternType, negation_type::NegationType, never_type::NeverType,
    no_refine_type::NoRefineType, primitive_type::PrimitiveType, singleton_type::SingletonType,
    table_type::TableType, r#type::Type, type_arena::TypeArena, type_level::TypeLevel,
    type_pack::TypePack, type_pack_var::TypePackVar, unifiable::Error, union_type::UnionType,
    unknown_type::UnknownType, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    singleton_variant::SingletonVariant, type_id::TypeId, type_pack_id::TypePackId,
    type_pack_variant::TypePackVariant, type_variant::TypeVariant,
  },
};
impl BuiltinTypes {
  /// C++ `BuiltinTypes::BuiltinTypes()` — the real default ctor.
  pub fn new() -> Self {
    let mut arena: Box<TypeArena> = Box::default();
    let debug_freeze_arena = FFlag::DebugLuauFreezeArena.get();

    // Persistent type/pack minting helpers (mirror `arena->addType(Type{v,
    // /*persistent*/ true})` / `addTypePack(TypePackVar{v, true})`).
    fn add_persistent_type(arena: &mut TypeArena, v: TypeVariant) -> TypeId {
      arena.add_tv(Type::new_with_persistence(v, true))
    }
    fn add_persistent_pack(arena: &mut TypeArena, v: TypePackVariant) -> TypePackId {
      arena.add_type_pack_type_pack_var(TypePackVar {
        ty: v,
        persistent: true,
        owning_arena: null_mut(),
      })
    }

    let arena_ref = &mut *arena;

    let nil_type = add_persistent_type(
      arena_ref,
      TypeVariant::Primitive(PrimitiveType {
        r#type: PrimitiveType::NIL_TYPE,
        metatable: None,
      }),
    );
    let number_type = add_persistent_type(
      arena_ref,
      TypeVariant::Primitive(PrimitiveType {
        r#type: PrimitiveType::NUMBER,
        metatable: None,
      }),
    );
    let integer_type = add_persistent_type(
      arena_ref,
      TypeVariant::Primitive(PrimitiveType {
        r#type: PrimitiveType::INTEGER,
        metatable: None,
      }),
    );
    let string_type = add_persistent_type(
      arena_ref,
      TypeVariant::Primitive(PrimitiveType {
        r#type: PrimitiveType::STRING,
        metatable: None,
      }),
    );
    let boolean_type = add_persistent_type(
      arena_ref,
      TypeVariant::Primitive(PrimitiveType {
        r#type: PrimitiveType::BOOLEAN,
        metatable: None,
      }),
    );
    let thread_type = add_persistent_type(
      arena_ref,
      TypeVariant::Primitive(PrimitiveType {
        r#type: PrimitiveType::THREAD,
        metatable: None,
      }),
    );
    let buffer_type = add_persistent_type(
      arena_ref,
      TypeVariant::Primitive(PrimitiveType {
        r#type: PrimitiveType::BUFFER,
        metatable: None,
      }),
    );
    let function_type = add_persistent_type(
      arena_ref,
      TypeVariant::Primitive(PrimitiveType {
        r#type: PrimitiveType::FUNCTION,
        metatable: None,
      }),
    );

    // ExternType{"userdata"|"object"|"class", {}, nullopt, nullopt, {}, {}, {}, {}}
    let make_extern = |name: &str| ExternType {
      name: name.to_string(),
      props: Default::default(),
      parent: None,
      metatable: None,
      tags: Default::default(),
      user_data: None,
      definition_module_name: Default::default(),
      definition_location: None,
      indexer: None,
      relation: None,
    };
    let extern_type = add_persistent_type(arena_ref, TypeVariant::Extern(make_extern("userdata")));
    let object_type = add_persistent_type(arena_ref, TypeVariant::Extern(make_extern("object")));
    let class_type = add_persistent_type(arena_ref, TypeVariant::Extern(make_extern("class")));

    let table_type_id = add_persistent_type(
      arena_ref,
      TypeVariant::Primitive(PrimitiveType {
        r#type: PrimitiveType::TABLE,
        metatable: None,
      }),
    );

    // TableType{TableState::Sealed, TypeLevel{}, nullptr}
    let empty_table_type = add_persistent_type(
      arena_ref,
      TypeVariant::Table(TableType::table_type_table_state_type_level_scope(
        TableState::Sealed,
        TypeLevel::default(),
        null_mut(),
      )),
    );

    let true_type = add_persistent_type(
      arena_ref,
      TypeVariant::Singleton(SingletonType::new(SingletonVariant::V0(
        BooleanSingleton::new(true),
      ))),
    );
    let false_type = add_persistent_type(
      arena_ref,
      TypeVariant::Singleton(SingletonType::new(SingletonVariant::V0(
        BooleanSingleton::new(false),
      ))),
    );

    let any_type = add_persistent_type(arena_ref, TypeVariant::Any(AnyType::default()));
    let unknown_type = add_persistent_type(arena_ref, TypeVariant::Unknown(UnknownType::default()));
    let never_type = add_persistent_type(arena_ref, TypeVariant::Never(NeverType::default()));
    let error_type = add_persistent_type(arena_ref, TypeVariant::Error(Error::new()));
    let no_refine_type =
      add_persistent_type(arena_ref, TypeVariant::NoRefine(NoRefineType::default()));

    let falsy_type = add_persistent_type(
      arena_ref,
      TypeVariant::Union(UnionType {
        options: vec![false_type, nil_type],
      }),
    );
    let truthy_type = add_persistent_type(
      arena_ref,
      TypeVariant::Negation(NegationType { ty: falsy_type }),
    );
    let not_nil_type = add_persistent_type(
      arena_ref,
      TypeVariant::Negation(NegationType { ty: nil_type }),
    );
    let optional_number_type = add_persistent_type(
      arena_ref,
      TypeVariant::Union(UnionType {
        options: vec![number_type, nil_type],
      }),
    );
    let optional_string_type = add_persistent_type(
      arena_ref,
      TypeVariant::Union(UnionType {
        options: vec![string_type, nil_type],
      }),
    );

    let empty_type_pack = add_persistent_pack(
      arena_ref,
      TypePackVariant::TypePack(TypePack {
        head: vec![],
        tail: None,
      }),
    );
    let any_type_pack = add_persistent_pack(
      arena_ref,
      TypePackVariant::Variadic(VariadicTypePack {
        ty: any_type,
        hidden: false,
      }),
    );
    let unknown_type_pack = add_persistent_pack(
      arena_ref,
      TypePackVariant::Variadic(VariadicTypePack {
        ty: unknown_type,
        hidden: false,
      }),
    );
    let never_type_pack = add_persistent_pack(
      arena_ref,
      TypePackVariant::Variadic(VariadicTypePack {
        ty: never_type,
        hidden: false,
      }),
    );
    let uninhabitable_type_pack = add_persistent_pack(
      arena_ref,
      TypePackVariant::TypePack(TypePack {
        head: vec![never_type],
        tail: Some(never_type_pack),
      }),
    );
    let error_type_pack = add_persistent_pack(arena_ref, TypePackVariant::Error(Error::new()));

    freeze(&mut arena);

    BuiltinTypes {
      arena,
      debug_freeze_arena,
      type_functions: Box::new(BuiltinTypeFunctions::new()),
      nil_type,
      number_type,
      integer_type,
      string_type,
      boolean_type,
      thread_type,
      buffer_type,
      function_type,
      extern_type,
      object_type,
      class_type,
      table_type: table_type_id,
      empty_table_type,
      true_type,
      false_type,
      any_type,
      unknown_type,
      never_type,
      error_type,
      no_refine_type,
      falsy_type,
      truthy_type,
      not_nil_type,
      optional_number_type,
      optional_string_type,
      empty_type_pack,
      any_type_pack,
      unknown_type_pack,
      never_type_pack,
      uninhabitable_type_pack,
      error_type_pack,
    }
  }
}

impl Default for BuiltinTypes {
  fn default() -> Self {
    Self::new()
  }
}
