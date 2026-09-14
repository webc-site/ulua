//! Node: `cxx:Method:Luau.Analysis:Analysis/src/ToString.cpp:712:type_stringifier_operator_call`
//! Source: `Analysis/src/ToString.cpp:712-853` (hand-ported)

use alloc::string::ToString;
use core::ffi::c_void;

use crate::{
  enums::table_state::TableState,
  functions::{can_use_type_name_in_scope::can_use_type_name_in_scope, is_number::is_number},
  records::{table_type::TableType, type_stringifier::TypeStringifier},
  type_aliases::type_id::TypeId,
};

impl TypeStringifier {
  /// C++ `void operator()(TypeId ty, const TableType& ttv)`.
  pub fn operator_call_7(&mut self, ty: TypeId, ttv: &TableType) {
    unsafe {
      if let Some(bound_to) = ttv.bound_to {
        return self.stringify_type_id(bound_to);
      }

      // if hide table alias expansions are enabled and there is a name found for the table, use it
      let show_name =
        !(*self.state).exhaustive || (*(*self.state).opts).hide_table_alias_expansions;

      if show_name && let Some(name) = &ttv.name {
        // If scope if provided, add module name and check visibility
        if let Some(scope) = (*(*self.state).opts).scope.clone() {
          let (success, module_name) = can_use_type_name_in_scope(scope, name);

          if !success {
            (*(*self.state).result).invalid = true;
          }

          if let Some(module_name) = module_name {
            (*self.state).emit(module_name.as_str());
            (*self.state).emit(".");
          }
        }

        (*self.state).emit_and_record_span(name, ty);
        self.stringify_vector_type_id_vector_type_pack_id(
          &ttv.instantiated_type_params,
          &ttv.instantiated_type_pack_params,
        );
        return;
      }

      if !(*self.state).exhaustive
        && !(*self.state).ignore_synthetic_name
        && let Some(synthetic_name) = &ttv.synthetic_name
      {
        (*(*self.state).result).invalid = true;
        (*self.state).emit_and_record_span(synthetic_name, ty);
        self.stringify_vector_type_id_vector_type_pack_id(
          &ttv.instantiated_type_params,
          &ttv.instantiated_type_pack_params,
        );
        return;
      }

      if (*self.state).has_seen(ttv as *const TableType as *const c_void) {
        (*(*self.state).result).cycle = true;
        (*self.state).emit("*CYCLE*");
        return;
      }

      let effective_state = if (*(*self.state).opts).hide_table_kind {
        TableState::Sealed
      } else {
        ttv.state
      };
      let (openbrace, closedbrace) = match effective_state {
        TableState::Sealed => ("{", "}"),
        TableState::Unsealed => {
          (*(*self.state).result).invalid = true;
          ("{|", "|}")
        }
        TableState::Free => {
          (*(*self.state).result).invalid = true;
          ("{-", "-}")
        }
        TableState::Generic => {
          (*(*self.state).result).invalid = true;
          ("{+", "+}")
        }
      };

      // If this appears to be an array, we want to stringify it using the {T} syntax.
      if let Some(indexer) = &ttv.indexer
        && ttv.props.is_empty()
        && is_number(indexer.index_type)
      {
        (*self.state).emit("{");
        if indexer.is_read_only {
          (*self.state).emit("read ");
        }
        self.stringify_type_id(indexer.index_result_type);
        (*self.state).emit("}");

        (*self.state).unsee(ttv as *const TableType as *const c_void);
        return;
      }

      (*self.state).emit(openbrace);
      (*self.state).indent();

      let mut comma = false;
      if let Some(indexer) = &ttv.indexer {
        (*self.state).newline();
        if indexer.is_read_only {
          (*self.state).emit("read ");
        }
        (*self.state).emit("[");
        self.stringify_type_id(indexer.index_type);
        (*self.state).emit("]: ");
        self.stringify_type_id(indexer.index_result_type);
        comma = true;
      }

      let old_length = (&*(*self.state).result).name.len();
      for (index, (name, prop)) in ttv.props.iter().enumerate() {
        if comma {
          (*self.state).emit(",");
          (*self.state).newline();
        } else {
          (*self.state).newline();
        }

        let length = (&*(*self.state).result).name.len() - old_length;

        let max_table_length = (*(*self.state).opts).max_table_length;
        if max_table_length > 0 && (length - 2 * index) >= max_table_length {
          (*self.state).emit("... ");
          (*self.state).emit((ttv.props.len() - index).to_string().as_str());
          (*self.state).emit(" more ...");
          break;
        }

        self.stringify_string_property(name, prop);

        comma = true;
      }

      (*self.state).dedent();
      if comma {
        (*self.state).newline();
      } else {
        (*self.state).emit("  ");
      }
      (*self.state).emit(closedbrace);

      (*self.state).unsee(ttv as *const TableType as *const c_void);
    }
  }
}
