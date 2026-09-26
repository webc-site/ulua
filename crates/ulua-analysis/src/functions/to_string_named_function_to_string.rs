use alloc::{format, string::String};
use core::ptr::null_mut;

use crate::{
  functions::{
    begin_type_pack::begin, end_type_pack::end, finite::finite, follow_type_pack,
    get_type_pack::get, size_type_pack::size,
  },
  records::{
    function_type::FunctionType, stringifier_state::StringifierState,
    to_string_options::ToStringOptions, to_string_result::ToStringResult, type_pack::TypePack,
    type_stringifier::TypeStringifier, variadic_type_pack::VariadicTypePack,
  },
};

pub fn to_string_named_function_string_function_type(
  func_name: &str,
  ftv: &FunctionType,
) -> String {
  let mut opts = ToStringOptions::default();
  to_string_named_function_string_function_type_to_string_options(func_name, ftv, &mut opts)
}

pub fn to_string_named_function_string_function_type_to_string_options(
  func_name: &str,
  ftv: &FunctionType,
  opts: &mut ToStringOptions,
) -> String {
  let mut result = ToStringResult::default();
  // Safety: 构造器按 C++ `StringifierState(ToStringOptions&, ToStringResult&)`
  // 契约把两个入参裸化为存储指针——`opts` 借自本函数参数（caller 保证整个
  // 调用期存活），`result` 是紧邻上一行声明的局部，按逆序析构必然比 `state`
  // 与后续 `tvs` 活得久；构造体内只共享读 `*opts` 拷贝标志/名字表，无别名写。
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
  while arg_pack_iter != end_iter {
    // ftv takes a self parameter as the first argument, skip it if specified in option
    if idx == 0 && ftv.has_self && opts.hide_function_self_argument {
      arg_pack_iter.advance();
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
    } else if let Some(Some(arg_name)) = ftv.arg_names.get(idx) {
      // 双写合一：`idx < len && arg_names[idx].is_some()` 判定+取值并为
      // 一次 `get` 双层模式匹配，Some 直接绑定。
      state.emit(format!("{}: ", arg_name.name).as_str());
    } else {
      state.emit("_: ");
    }
    // Safety: `stringify_type_id` 的隐式前提是 `tvs.state` 及其内部 opts/result
    // 裸指针有效——三者分别锚定本函数存活的 `state`、入参 `opts` 与局部
    // `result`（逆序析构保证晚于本行）；被调已降 safe，句柄前提在其窄块内证成。
    tvs.stringify_type_id(*arg_pack_iter.current());

    arg_pack_iter.advance();
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
        // 被调已降 safe；vtp.ty 为 arena 存活句柄，前提在窄块内证成。
        tvs.stringify_type_id(vtp.ty);
      } else {
        tvs.stringify_type_pack_id(tail);
      }
    }
  }

  state.emit("): ");

  // Safety: `size` 的 `log` 入参显式判 null——传 null_mut() 即走无日志的
  // `follow_type_pack_id`，对应 C++ 默认 `TxnLog* log = nullptr`；`tp` 入参
  // `ftv.ret_types` 是入参 FunctionType 携带的 arena 驻留 TypePack 句柄
  // （NotNull 语义），函数仅沿 tail 链只读遍历，本次调用期节点存活。
  let ret_size = size(ftv.ret_types, None);
  // Safety: 与上一行 size 同一论证——`finite` 内部判 log null 走无日志
  // follow，且只沿 `ftv.ret_types` 的 tail 链只读探测变体类型。
  let has_tail = !unsafe { finite(ftv.ret_types, null_mut()) };
  let wrap = get::<TypePack>(follow_type_pack::follow(ftv.ret_types)).is_some()
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
