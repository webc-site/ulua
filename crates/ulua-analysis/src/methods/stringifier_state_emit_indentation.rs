use core::iter::repeat_n;

use crate::records::stringifier_state::StringifierState;
impl StringifierState {
  /// C++ `emitIndentation()`:直接向结果追加缩进空格,
  /// 复用 `emit` 的截断守卫,免去临时 `String` 分配。
  pub fn emit_indentation(&mut self) {
    // Safety: `self.opts` 由构造期入口 `to_string*` 以 `opts as *mut ToStringOptions`
    // 注入，指向调用栈上存活的 `ToStringOptions`，其生命期覆盖整个 stringify 调用；
    // 此处仅只读两个标量字段，单线程无并发写。
    let use_line_breaks = unsafe { (*self.opts).use_line_breaks };
    if !use_line_breaks {
      return;
    }

    // Safety: 同一条构造期注入不变量——`self.opts` 指向存活只读的 `ToStringOptions`。
    let max_type_length = unsafe { (*self.opts).max_type_length };
    if max_type_length > 0 {
      // Safety: `self.result` 由构造期以 `&mut result as *mut ToStringResult` 注入，
      // 指向入口函数存活的局部；此处仅取只读借用查长度。
      let result_name = unsafe { &(*self.result).name };
      if result_name.len() > max_type_length {
        return;
      }
    }

    // Safety: `self.result` 指向构造期注入的存活局部，本函数独占且单线程使用，
    // 重建的可变借用与上面的只读借用不重叠（只读借用已在 if 块结束）。
    let result = unsafe { &mut *self.result };
    // 计数循环改迭代器：repeat_n 按 size_hint 一次性预留并追加空格，免手工下标。
    result.name.extend(repeat_n(' ', self.indentation));
  }
}
