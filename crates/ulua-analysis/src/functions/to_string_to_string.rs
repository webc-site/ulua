use alloc::{format, string::String};

use ulua_ast::records::location::Location;

use crate::{
  functions::{
    dump_to_string::{dump_vector_type_id, dump_vector_type_pack_id},
    to_string_detailed_to_string::{
      to_string_detailed, to_string_detailed_type_pack_id_to_string_options,
    },
    to_string_to_string::{
      to_string_type_id_to_string_options as tos_ty,
      to_string_type_pack_id_to_string_options as tos_tp,
    },
    to_string_vector_to_string::to_string_vector_vector_type_id_to_string_options as to_string_vector_ty,
  },
  records::{
    constraint::Constraint, to_string_options::ToStringOptions, r#type::Type,
    type_pack_var::TypePackVar,
  },
  type_aliases::{
    constraint_v::ConstraintV,
    type_id::TypeId,
    type_or_pack::{TypeOrPack, TypeOrPackMember},
    type_pack_id::TypePackId,
  },
};

pub fn to_string_type_pack_id_to_string_options_mut(
  ty: TypePackId,
  mut opts: ToStringOptions,
) -> String {
  to_string_type_pack_id_to_string_options(ty, &mut opts)
}

pub fn to_string_type_id_to_string_options_mut(ty: TypeId, mut opts: ToStringOptions) -> String {
  to_string_type_id_to_string_options(ty, &mut opts)
}

pub fn to_string_type_id(ty: TypeId) -> String {
  let mut opts = ToStringOptions::default();
  to_string_type_id_to_string_options(ty, &mut opts)
}

pub fn to_string_type_pack_id(ty: TypePackId) -> String {
  let mut opts = ToStringOptions::default();
  to_string_type_pack_id_to_string_options(ty, &mut opts)
}

pub fn to_string_type_item(tv: &Type) -> String {
  let mut opts = ToStringOptions::default();
  to_string_type_item_to_string_options(tv, &mut opts)
}

pub fn to_string_type_pack_var(tp: &TypePackVar) -> String {
  let mut opts = ToStringOptions::default();
  to_string_type_pack_var_to_string_options(tp, &mut opts)
}

pub fn to_string_type_or_pack(ty_or_tp: &TypeOrPack) -> String {
  let mut opts = ToStringOptions::default();
  to_string_type_or_pack_to_string_options(ty_or_tp, &mut opts)
}

/// C++ `std::string to_string(TypeId ty, ToStringOptions& opts)`.
pub fn to_string_type_id_to_string_options(ty: TypeId, opts: &mut ToStringOptions) -> String {
  to_string_detailed(ty, opts).name
}

/// C++ `std::string to_string(TypePackId ty, ToStringOptions& opts)`.
pub fn to_string_type_pack_id_to_string_options(
  ty: TypePackId,
  opts: &mut ToStringOptions,
) -> String {
  to_string_detailed_type_pack_id_to_string_options(ty, opts).name
}

pub fn to_string_type_item_to_string_options(tv: &Type, opts: &mut ToStringOptions) -> String {
  to_string_type_id_to_string_options(tv as *const Type as TypeId, opts)
}

pub fn to_string_type_pack_var_to_string_options(
  tp: &TypePackVar,
  opts: &mut ToStringOptions,
) -> String {
  to_string_type_pack_id_to_string_options(tp as *const TypePackVar as TypePackId, opts)
}

pub fn to_string_constraint_to_string_options(
  constraint: &Constraint,
  opts: &mut ToStringOptions,
) -> String {
  match &constraint.c {
    ConstraintV::Subtype(c) => {
      let sub_str = tos_ty(c.sub_type, opts);
      let super_str = tos_ty(c.super_type, opts);
      format!("{} <: {}", sub_str, super_str)
    }
    ConstraintV::PackSubtype(c) => {
      let sub_str = tos_tp(c.sub_pack, opts);
      let super_str = tos_tp(c.super_pack, opts);
      format!("{} <...: {}", sub_str, super_str)
    }
    ConstraintV::Generalization(c) => {
      let sub_str = tos_ty(c.generalized_type, opts);
      let super_str = tos_ty(c.source_type, opts);
      format!("{} ~ gen {}", sub_str, super_str)
    }
    ConstraintV::Iterable(c) => {
      let iterator_str = tos_tp(c.iterator, opts);
      let variable_str = to_string_vector_ty(&c.variables, opts);

      format!("{} ~ iterate {}", variable_str, iterator_str)
    }
    ConstraintV::Name(c) => {
      let named_str = tos_ty(c.named_type, opts);
      format!("@name({}) = {}", named_str, c.name)
    }
    ConstraintV::TypeAliasExpansion(c) => {
      let target_str = tos_ty(c.target, opts);
      format!("expand {}", target_str)
    }
    ConstraintV::FunctionCall(c) => {
      format!(
        "call {}( {} ) with {{ result = {} }}",
        tos_ty(c.fn_type, opts),
        tos_tp(c.args_pack, opts),
        tos_tp(c.result, opts)
      )
    }
    ConstraintV::FunctionCheck(c) => {
      format!(
        "function_check {} {}",
        tos_ty(c.fn_type, opts),
        tos_tp(c.args_pack, opts)
      )
    }
    ConstraintV::PrimitiveType(c) => {
      if let Some(expected_type) = c.expected_type {
        format!(
          "prim {}[expected: {}] as {}",
          tos_ty(c.free_type, opts),
          tos_ty(expected_type, opts),
          tos_ty(c.primitive_type, opts)
        )
      } else {
        format!(
          "prim {} as {}",
          tos_ty(c.free_type, opts),
          tos_ty(c.primitive_type, opts)
        )
      }
    }
    ConstraintV::HasProp(c) => {
      let mut s = format!(
        "{} ~ hasProp {}, \"{}\" ctx={}",
        tos_ty(c.result_type, opts),
        tos_ty(c.subject_type, opts),
        c.prop,
        c.context as i32
      );
      if c.in_conditional {
        s.push_str(" (inConditional)");
      }
      s
    }
    ConstraintV::HasIndexer(c) => {
      format!(
        "{} ~ hasIndexer {} {}",
        tos_ty(c.result_type, opts),
        tos_ty(c.subject_type, opts),
        tos_ty(c.index_type, opts)
      )
    }
    ConstraintV::AssignProp(c) => {
      format!(
        "{} ~ assignProp {} {} {}",
        tos_ty(c.prop_type, opts),
        tos_ty(c.lhs_type, opts),
        c.prop_name,
        tos_ty(c.rhs_type, opts)
      )
    }
    ConstraintV::AssignIndex(c) => {
      format!(
        "assignIndex {} {} {}",
        tos_ty(c.lhs_type, opts),
        tos_ty(c.index_type, opts),
        tos_ty(c.rhs_type, opts)
      )
    }
    ConstraintV::Unpack(c) => {
      format!(
        "{} ~ ...unpack {}",
        to_string_vector_ty(&c.result_pack, opts),
        tos_tp(c.source_pack, opts)
      )
    }
    ConstraintV::Reduce(c) => format!("reduce {}", tos_ty(c.ty, opts)),
    ConstraintV::ReducePack(c) => {
      format!("reduce {}", tos_tp(c.tp, opts))
    }
    ConstraintV::Equality(c) => {
      format!(
        "equality: {} ~ {}",
        tos_ty(c.result_type, opts),
        tos_ty(c.assignment_type, opts)
      )
    }
    ConstraintV::Simplify(c) => format!("simplify {}", tos_ty(c.ty, opts)),
    ConstraintV::PushFunctionType(c) => {
      format!(
        "push_function_type {} => {}",
        tos_ty(c.expected_function_type, opts),
        tos_ty(c.function_type, opts)
      )
    }
    ConstraintV::TypeInstantiation(c) => {
      format!(
        "explicitly_specified_constraints {} (type_arguments = {}), (typePackArguments = {})",
        tos_ty(c.function_type, opts),
        dump_vector_type_id(&c.type_arguments),
        dump_vector_type_pack_id(&c.type_pack_arguments)
      )
    }
    ConstraintV::PushType(c) => {
      format!(
        "push_type {} => {}",
        tos_ty(c.expected_type, opts),
        tos_ty(c.target_type, opts)
      )
    }
  }
}

/// C++ `std::string to_string(const Location& location, int offset = 0, bool useBegin = true)`.
/// NOTE: C++ ignores `useBegin` in the body (it prints both ends); preserved.
pub fn to_string_location_i32_bool(location: &Location, offset: i32, _use_begin: bool) -> String {
  format!(
    "({}, {}) - ({}, {})",
    location.begin.line as i64 + offset as i64,
    location.begin.column as i64 + offset as i64,
    location.end.line as i64 + offset as i64,
    location.end.column as i64 + offset as i64,
  )
}

pub fn to_string_type_or_pack_to_string_options(
  ty_or_tp: &TypeOrPack,
  opts: &mut ToStringOptions,
) -> String {
  if let Some(ty) = TypeId::get_if(ty_or_tp) {
    to_string_type_id_to_string_options(*ty, opts)
  } else if let Some(tp) = TypePackId::get_if(ty_or_tp) {
    to_string_type_pack_id_to_string_options(*tp, opts)
  } else {
    unreachable!("LUAU_UNREACHABLE: TypeOrPack has exactly two members")
  }
}
