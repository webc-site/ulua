use crate::{enums::polarity::Polarity, records::stringifier_state::StringifierState};

impl StringifierState {
  pub fn emit_string(&mut self, s: &str) {
    if self.opts.is_null() {
      return;
    }

    let max_type_length =
      // Safety: 上一行 `self.opts.is_null()` 守卫已确保 `opts` 非 null；它是构造函数
      // `stringifier_state_stringifier_state` 从 C++ `ToStringOptions&` 引用直译而来、由调用方
      // 持有并贯穿本次 stringification 的会话级裸指针。此处只读取 `max_type_length`（Copy）。
      unsafe { (*self.opts).max_type_length };
    if max_type_length > 0 {
      // Safety: `self.result` 由构造函数按 C++ `ToStringResult&` 引用（NotNull 语境）接线，
      // 全程非空且指向调用方持有的存活结果缓冲；这里重建共享借用只为读取 `name` 的长度。
      let result_name = unsafe { &(*self.result).name };
      if result_name.len() > max_type_length {
        return;
      }
    }

    // Safety: 同上——`self.result` 为构造期接线的非空存活 `ToStringResult` 句柄；本次在
    // `&mut self` 独占下向 `name` 追加，与上方读取 `name` 的共享借用互不重叠（后者已结束）。
    unsafe { (*self.result).name.push_str(s) };
  }

  pub fn emit_polarity(&mut self, p: Polarity) {
    let s = match p {
      Polarity::None => "  ",
      Polarity::Negative => " -",
      Polarity::Positive => "+ ",
      Polarity::Mixed => "+-",
      _ => "!!",
    };
    self.emit_string(s);
  }
}
