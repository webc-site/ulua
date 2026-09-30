//! Source: `Analysis/include/Luau/Constraint.h:318-341` (hand-ported)
use crate::{
  macros::variant_enum,
  records::{
    assign_index_constraint::AssignIndexConstraint, assign_prop_constraint::AssignPropConstraint,
    equality_constraint::EqualityConstraint, function_call_constraint::FunctionCallConstraint,
    function_check_constraint::FunctionCheckConstraint,
    generalization_constraint::GeneralizationConstraint,
    has_indexer_constraint::HasIndexerConstraint, has_prop_constraint::HasPropConstraint,
    iterable_constraint::IterableConstraint, name_constraint::NameConstraint,
    pack_subtype_constraint::PackSubtypeConstraint,
    primitive_type_constraint::PrimitiveTypeConstraint,
    push_function_type_constraint::PushFunctionTypeConstraint,
    push_type_constraint::PushTypeConstraint, reduce_constraint::ReduceConstraint,
    reduce_pack_constraint::ReducePackConstraint, simplify_constraint::SimplifyConstraint,
    subtype_constraint::SubtypeConstraint,
    type_alias_expansion_constraint::TypeAliasExpansionConstraint,
    type_instantiation_constraint::TypeInstantiationConstraint,
    unpack_constraint::UnpackConstraint,
  },
};

// 21 members exceed Variant7 -> custom enum (TypeVariant precedent).
variant_enum! {
  #[derive(Debug, Clone)]
  pub enum ConstraintV {
    Subtype => SubtypeConstraint,
    PackSubtype => PackSubtypeConstraint,
    Generalization => GeneralizationConstraint,
    Iterable => IterableConstraint,
    Name => NameConstraint,
    TypeAliasExpansion => TypeAliasExpansionConstraint,
    FunctionCall => FunctionCallConstraint,
    FunctionCheck => FunctionCheckConstraint,
    PrimitiveType => PrimitiveTypeConstraint,
    HasProp => HasPropConstraint,
    HasIndexer => HasIndexerConstraint,
    AssignProp => AssignPropConstraint,
    AssignIndex => AssignIndexConstraint,
    Unpack => UnpackConstraint,
    Reduce => ReduceConstraint,
    ReducePack => ReducePackConstraint,
    Equality => EqualityConstraint,
    Simplify => SimplifyConstraint,
    PushFunctionType => PushFunctionTypeConstraint,
    PushType => PushTypeConstraint,
    TypeInstantiation => TypeInstantiationConstraint,
  }

  /// `get_if<T>(&v)` — the Rust shape of C++ overload-on-T over this variant.
  pub trait ConstraintVMember;
}
