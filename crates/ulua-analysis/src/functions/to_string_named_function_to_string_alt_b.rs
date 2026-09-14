//! Node: `cxx:Function:Luau.Analysis:Analysis/src/ToString.cpp:1742:to_string_named_function`
//! Source: `Analysis/src/ToString.cpp:1742-1824` (hand-ported)

use alloc::{format, string::String};
/// C++ `std::string toStringNamedFunction(const std::string& funcName, const FunctionType& ftv, ToStringOptions& opts)`.
use core::ptr::null_mut;

use crate::{
  functions::{
    begin_type_pack::begin, end_type_pack::end, finite::finite,
    follow_type_pack::follow_type_pack_id, get_type_pack::get, size_type_pack::size,
  },
  records::{
    function_type::FunctionType, stringifier_state::StringifierState,
    to_string_options::ToStringOptions, to_string_result::ToStringResult, type_pack::TypePack,
    type_stringifier::TypeStringifier, variadic_type_pack::VariadicTypePack,
  },
};
pub fn to_string_named_function_string_function_type_to_string_options(
  func_name: &str,
  ftv: &FunctionType,
  opts: &mut ToStringOptions,
) -> String {
  let mut result = ToStringResult::default();
  let mut state = unsafe {
    StringifierState::stringifier_state_stringifier_state(
      opts as *mut ToStringOptions,
      &mut result as *mut ToStringResult,
    )
  };
  let mut tvs = TypeStringifier {
    state: &mut state as *mut StringifierState,
  };

  state.emit(func_name);

  if !opts.hide_named_function_type_parameters {
    tvs.stringify_vector_type_id_vector_type_pack_id(&ftv.generics, &ftv.generic_packs);
  }

  state.emit("(");

  let mut arg_pack_iter = begin(ftv.arg_types);
  let end_iter = end(ftv.arg_types);

  let mut first = true;
  let mut idx: usize = 0;
  while arg_pack_iter.operator_ne(&end_iter) {
    // ftv takes a self parameter as the first argument, skip it if specified in option
    if idx == 0 && ftv.has_self && opts.hide_function_self_argument {
      arg_pack_iter.operator_inc();
      idx += 1;
      continue;
    }

    if !first {
      state.emit(", ");
    }
    first = false;

    // We don't respect opts.function_type_arguments
    if idx < opts.named_function_override_arg_names.len() {
      state.emit(format!("{}: ", opts.named_function_override_arg_names[idx]).as_str());
    } else if idx < ftv.arg_names.len() && ftv.arg_names[idx].is_some() {
      state.emit(format!("{}: ", ftv.arg_names[idx].as_ref().unwrap().name).as_str());
    } else {
      state.emit("_: ");
    }
    unsafe { tvs.stringify_type_id(*arg_pack_iter.operator_deref()) };

    arg_pack_iter.operator_inc();
    idx += 1;
  }

  if let Some(tail) = arg_pack_iter.tail() {
    let vtp = get::<VariadicTypePack>(tail);
    let hidden = vtp.is_some_and(|v| v.hidden);
    if !hidden {
      if !first {
        state.emit(", ");
      }

      state.emit("...: ");

      if let Some(vtp) = vtp {
        unsafe { tvs.stringify_type_id(vtp.ty) };
      } else {
        tvs.stringify_type_pack_id(tail);
      }
    }
  }

  state.emit("): ");

  let ret_size = unsafe { size(ftv.ret_types, null_mut()) };
  let has_tail = !unsafe { finite(ftv.ret_types, null_mut()) };
  let wrap = !get::<TypePack>(unsafe { follow_type_pack_id(ftv.ret_types) }).is_none()
    && (if has_tail {
      ret_size != 0
    } else {
      ret_size > 1
    });

  if wrap {
    state.emit("(");
  }

  tvs.stringify_type_pack_id(ftv.ret_types);

  if wrap {
    state.emit(")");
  }

  result.name
}
