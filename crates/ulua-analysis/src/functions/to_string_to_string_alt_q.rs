//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1958:to_string`
//! Source: `Analysis/src/ToString.cpp:1958-2058` (hand-ported)

use alloc::{format, string::String};

/// C++ `std::string to_string(const Constraint& constraint, ToStringOptions& opts)`.
use crate::functions::dump_to_string_alt_g::dump_vector_type_id;
use crate::{
  functions::{
    dump_to_string_alt_h::dump_vector_type_pack_id,
    to_string_to_string_alt_m::to_string_type_id_to_string_options as tos_ty,
    to_string_to_string_alt_n::to_string_type_pack_id_to_string_options as tos_tp,
    to_string_vector_to_string::to_string_vector_vector_type_id_to_string_options as to_string_vector_ty,
  },
  records::{constraint::Constraint, to_string_options::ToStringOptions},
  type_aliases::constraint_v::ConstraintV,
};
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
