use alloc::vec::Vec;

use ulua_ast::{
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall,
    ast_expr_constant_bool::AstExprConstantBool, ast_expr_constant_string::AstExprConstantString,
    ast_expr_global::AstExprGlobal, ast_expr_group::AstExprGroup,
    ast_expr_index_name::AstExprIndexName, ast_name::AstName, ast_visitor::AstVisitor,
  },
  rtti,
  rtti::AstNodePtr,
  visit::ast_stat_visit,
};
use ulua_common::LUAU_ASSERT;
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, is_string::is_string},
  macros::lint_stat_process,
  records::{lint_context::LintContext, lint_context_handle::LintContextHandle},
};

#[derive(Debug, Clone)]
pub struct LintFormatString<'ctx> {
  pub(crate) context: LintContextHandle<'ctx>,
}

/// 256 位字节集位图门面：本文件的字符集全部为编译期固定字面量，经 `const fn`
/// 在编译期构建一次并提为 `const`，把原先每次调用都线性扫描 `&[u8] contains(&b)`
/// （每字符 O(集大小)）的运行时开销降为 O(1) 位测试。风格参照 ulua-ast
/// `char_classifier` 的编译期分类表先例；位图化仅替换成员判定，`|b' '` 大小写
/// 折叠臂语义逐字保留。
#[derive(Debug, Clone, Copy)]
pub struct CharSet([u64; 4]);

impl CharSet {
  /// 由字节字面量集构建位图（const 求值的 while 循环，重复元素幂等无害，
  /// 产物落 rodata，无运行时初始化）。
  const fn of(bytes: &[u8]) -> Self {
    let mut words = [0u64; 4];
    let mut i = 0;
    while i < bytes.len() {
      let b = bytes[i];
      words[(b >> 6) as usize] |= 1u64 << (b & 63);
      i += 1;
    }
    Self(words)
  }

  /// 成员判定：按字节高 6 位选字、低 6 位定位，单次位测试。
  #[inline]
  const fn has(self, b: u8) -> bool {
    (self.0[(b >> 6) as usize] >> (b & 63)) & 1 != 0
  }
}

/// `check_date_format` 合法日期格式符集（原局部绑定 `options`）。
const K_DATE_OPTIONS: CharSet = CharSet::of(b"aAbBcdHIjmMpSUwWxXyYzZ");
/// `check_string_format` 标志位集（原局部绑定 `flags`）。
const K_FORMAT_FLAGS: CharSet = CharSet::of(b"-+ #0");
/// `check_string_format` 转换符集（原局部绑定 `options`）。
const K_FORMAT_OPTIONS: CharSet = CharSet::of(b"cdiouxXeEfgGqs*");
/// `string.match` 模式 `%` 转义魔字符集（原局部绑定 `magic`）。
const K_MAGIC_CHARS: CharSet = CharSet::of(b"^$()%.[]*+-?)");
/// `string.match` 字符类小写集；上位类经 `|b' '` 折叠到小写后查本集
/// （原局部绑定 `classes`）。
const K_CLASSES: CharSet = CharSet::of(b"acdglpsuwxz");
/// `string.pack` 合法说明符+空格集（原局部绑定 `options`）。
const K_PACK_OPTIONS: CharSet = CharSet::of(b"<>!=bBhHlLjJTiIfdnczsxX ");
/// `string.pack` 无长度说明符集，`X` 后禁止项（原局部绑定 `unsized_opts`）。
const K_PACK_UNSIZED_OPTS: CharSet = CharSet::of(b"<>!zX ");

impl<'ctx> AstVisitor for LintFormatString<'ctx> {
  fn visit_expr_call(&mut self, node: &mut AstExprCall) -> bool {
    self.match_call(node);
    true
  }
}

// —— 原 methods/lint_format_string_check_date_format.rs ——
impl<'ctx> LintFormatString<'ctx> {
  pub fn check_date_format(&self, data: &[u8]) -> Option<&'static str> {
    let size = data.len();
    let mut i = 0;
    while i < size {
      let ch = data[i];
      if ch == b'%' {
        i += 1;
        if i == size {
          return Some("unfinished replacement");
        }
        let next_ch = data[i];
        if next_ch != b'%' && !K_DATE_OPTIONS.has(next_ch) {
          return Some("unexpected replacement character; must be a date format specifier or %");
        }
      }
      if data[i] == 0 {
        return Some("date format can not contain null characters");
      }
      i += 1;
    }
    None
  }
}

// —— 原 methods/lint_format_string_check_string_format.rs ——
impl<'ctx> LintFormatString<'ctx> {
  pub fn check_string_format(&self, data: &[u8]) -> Option<&'static str> {
    let size = data.len();
    let mut i = 0;
    while i < size {
      let ch = data[i];
      if ch == b'%' {
        i += 1;
        if i < size && data[i] == b'%' {
          i += 1;
          continue;
        }
        while i < size && K_FORMAT_FLAGS.has(data[i]) {
          i += 1;
        }
        if i < size && self.is_digit(data[i]) {
          i += 1;
        }
        if i < size && self.is_digit(data[i]) {
          i += 1;
        }
        if i < size && data[i] == b'.' {
          i += 1;
          if i < size && self.is_digit(data[i]) {
            i += 1;
          }
          if i < size && self.is_digit(data[i]) {
            i += 1;
          }
        }
        if i == size {
          return Some("unfinished format specifier");
        }
        if !K_FORMAT_OPTIONS.has(data[i]) {
          return Some("invalid format specifier: must be a string format specifier or %");
        }
      }
      i += 1;
    }
    None
  }
}

// —— 原 methods/lint_format_string_check_string_match.rs ——
impl<'ctx> LintFormatString<'ctx> {
  #[inline]
  pub fn check_string_match(&self, data: &[u8]) -> Result<i32, &'static str> {
    let size = data.len();
    let mut open_captures: Vec<i32> = Vec::new();
    let mut total_captures: i32 = 0;
    let mut i: usize = 0;
    while i < size {
      if data[i] == b'%' {
        i += 1;
        if i == size {
          return Err("unfinished character class");
        }
        let ch = data[i];
        if self.is_digit(ch) {
          if ch == b'0' {
            return Err("invalid capture reference, must be 1-9");
          }
          let capture_index = (ch - b'0') as i32;
          if capture_index > total_captures {
            return Err("invalid capture reference, must refer to a valid capture");
          }
          for &open in &open_captures {
            if open == capture_index {
              return Err("invalid capture reference, must refer to a closed capture");
            }
          }
        } else if self.is_alpha(ch) {
          if ch == b'b' {
            if i + 2 >= size {
              return Err("missing brace characters for balanced match");
            }
            i += 2;
          } else if ch == b'f' {
            if i + 1 >= size || data[i + 1] != b'[' {
              return Err("missing set after a frontier pattern");
            }
            // we can parse the set with the regular logic
          } else {
            // lower case lookup - upper case for every character class is defined as its inverse
            if !K_CLASSES.has(ch | b' ') {
              return Err("invalid character class, must refer to a defined class or its inverse");
            }
          }
        } else {
          // technically % can escape any non-alphanumeric character but this is error-prone
          if !K_MAGIC_CHARS.has(ch) {
            return Err("expected a magic character after %");
          }
        }
      } else if data[i] == b'[' {
        let mut j = i + 1;
        // empty patterns don't exist as per grammar rules, so we skip leading ^ and ]
        if j < size && data[j] == b'^' {
          j += 1;
        }
        if j < size && data[j] == b']' {
          j += 1;
        }
        // scan for the end of the pattern
        while j < size && data[j] != b']' {
          // % escapes the next character
          if j + 1 < size && data[j] == b'%' {
            j += 1;
          }
          j += 1;
        }
        if j == size {
          return Err("expected ] at the end of the string to close a set");
        }
        if let Some(error) = self.check_string_match_set(&data[i + 1..j], K_MAGIC_CHARS, K_CLASSES)
        {
          return Err(error);
        }
        debug_assert!(data[j] == b']');
        i = j;
      } else if data[i] == b'(' {
        total_captures += 1;
        open_captures.push(total_captures);
      } else if data[i] == b')' {
        if open_captures.is_empty() {
          return Err("unexpected ) without a matching (");
        }
        open_captures.pop();
      }
      i += 1;
    }
    if !open_captures.is_empty() {
      return Err("expected ) at the end of the string to close a capture");
    }
    Ok(total_captures)
  }
}

// —— 原 methods/lint_format_string_check_string_match_set.rs ——
impl<'ctx> LintFormatString<'ctx> {
  #[inline]
  pub fn check_string_match_set(
    &self,
    data: &[u8],
    magic: CharSet,
    classes: CharSet,
  ) -> Option<&'static str> {
    let size = data.len();
    let mut i = 0;
    while i < size {
      let ch = data[i];
      if ch == b'%' {
        i += 1;
        if i == size {
          return Some("unfinished character class");
        }
        let next_ch = data[i];
        if self.is_digit(next_ch) {
          return Some("sets can not contain capture references");
        } else if self.is_alpha(next_ch) {
          // lower case lookup - upper case for every character class is defined as its inverse
          if !classes.has(next_ch | b' ') {
            return Some("invalid character class, must refer to a defined class or its inverse");
          }
        } else {
          // technically % can escape any non-alphanumeric character but this is error-prone
          if !magic.has(next_ch) {
            return Some("expected a magic character after %");
          }
        }
        if i + 1 < size && data[i + 1] == b'-' {
          return Some("character range can't include character sets");
        }
      } else if ch == b'-' && i + 1 < size && data[i + 1] == b'%' {
        return Some("character range can't include character sets");
      }
      i += 1;
    }
    None
  }
}

// —— 原 methods/lint_format_string_check_string_pack.rs ——
impl<'ctx> LintFormatString<'ctx> {
  #[inline]
  pub fn check_string_pack(&self, data: &[u8], fixed: bool) -> Option<&'static str> {
    let size = data.len();
    let mut i = 0;
    while i < size {
      let ch = data[i];
      if !K_PACK_OPTIONS.has(ch) {
        return Some("unexpected character; must be a pack specifier or space");
      }
      if ch == b'c' && (i + 1 == size || !self.is_digit(data[i + 1])) {
        return Some("fixed-sized string format must specify the size");
      }
      if ch == b'X' && (i + 1 == size || K_PACK_UNSIZED_OPTS.has(data[i + 1])) {
        return Some("X must be followed by a size specifier");
      }
      if fixed && matches!(ch, b'z' | b's') {
        return Some("pack specifier must be fixed-size");
      }
      if matches!(ch, b'!' | b'i' | b'I' | b'c' | b's')
        && i + 1 < size
        && self.is_digit(data[i + 1])
      {
        let isc = ch == b'c';
        let mut v: u32 = 0;
        while i + 1 < size && self.is_digit(data[i + 1]) && v <= (i32::MAX as u32 - 9) / 10 {
          let digit_ch = data[i + 1];
          v = v * 10 + (digit_ch - b'0') as u32;
          i += 1;
        }
        if i + 1 < size && self.is_digit(data[i + 1]) {
          return Some("size specifier is too large");
        }
        if !isc && (v == 0 || v > 16) {
          return Some("integer size must be in range [1,16]");
        }
      }
      i += 1;
    }
    None
  }
}

// —— 原 methods/lint_format_string_check_string_replace.rs ——
impl<'ctx> LintFormatString<'ctx> {
  #[inline]
  pub fn check_string_replace(&self, data: &[u8], captures: i32) -> Option<&'static str> {
    let size = data.len();
    let mut i = 0;
    while i < size {
      if data[i] == b'%' {
        i += 1;
        if i == size {
          return Some("unfinished replacement");
        }
        let next_ch = data[i];
        if next_ch != b'%' && !self.is_digit(next_ch) {
          return Some("unexpected replacement character; must be a digit or %");
        }
        if self.is_digit(next_ch) && captures >= 0 && (next_ch - b'0') as i32 > captures {
          return Some("invalid capture index, must refer to pattern capture");
        }
      }
      i += 1;
    }
    None
  }
}

// —— 原 methods/lint_format_string_fuzz.rs ——
impl<'ctx> LintFormatString<'ctx> {
  /// 纯模式串自检入口（fuzz 用）：`check_string_*`/`check_date_format` 均为
  /// `&self` 的无副作用校验，不触碰宿主 context，故无需接线即可独立调用。
  pub fn fuzz(&self, data: &[u8]) {
    self.check_string_format(data);
    self.check_string_pack(data, false);
    let _ = self.check_string_match(data);
    self.check_string_replace(data, -1);
    self.check_date_format(data);
  }
}

// —— 原 methods/lint_format_string_is_alpha.rs ——
impl<'ctx> LintFormatString<'ctx> {
  #[inline]
  pub fn is_alpha(&self, ch: u8) -> bool {
    ((ch | b' ').wrapping_sub(b'a')) < 26
  }
}

// —— 原 methods/lint_format_string_is_digit.rs ——
impl<'ctx> LintFormatString<'ctx> {
  #[inline]
  pub fn is_digit(&self, ch: u8) -> bool {
    // use unsigned comparison to do range check for performance
    let _ = self;
    LUAU_ASSERT!(true);
    ch.wrapping_sub(b'0') < 10
  }
}

// —— 原 methods/lint_format_string_match_call.rs ——
impl<'ctx> LintFormatString<'ctx> {
  /// cpp `matchCall(AstExprCall*)`：`node` 为分析期存活、由 arena 持有的调用节点
  /// 共享借用（cpp 裸指针形参的 Rust 对应），本方法对其只读；宿主 context 经
  /// `self.context` 写句柄瞬时借用。
  pub fn match_call(&mut self, node: &AstExprCall) {
    // Safety: parser 为每个调用节点无条件填非空 `func`，指向存活表达式 arena
    // 节点；`*mut AstExpr` 经 `as_ast_node` 仅做基址不变的类型视图转换
    // （repr(C) 基类在偏移 0），判型读 `class_index`，命中引用即该节点基址，
    // 存活至本函数结束，期间 AST 无人写入。
    let Some(func) = (unsafe { rtti::ast_node_try_as_ptr::<AstExprIndexName>(node.func) }) else {
      return;
    };
    if node.self_ {
      // Safety: `func.expr` 同为 parser 写入 `AstExprIndexName` 的非空存活
      // arena 表达式指针；判型只读基类 `class_index`，未命中返回 None 时回退
      // 到 `func.expr` 本身（拷贝指针值，不新增借用），与 cpp `group ?
      // group->expr : func->expr` 逐步对应。
      let group = unsafe { rtti::ast_node_try_as_ptr::<AstExprGroup>(func.expr) };
      let self_expr: *mut AstExpr = if let Some(group) = group {
        // group.expr 已句柄化恒非空；行走链沿用裸指针 API，经 as_ptr 桥接。
        group.expr.as_ptr()
      } else {
        // func.expr 已句柄化恒非空；行走链沿用裸指针 API，经 as_ptr 桥接。
        func.expr.as_ptr()
      };
      if unsafe { rtti::ast_node_is_ptr::<AstExprConstantString>(self_expr) } {
        // Safety: `func.index` 是上方存活 `&AstExprIndexName` 的 `AstName`
        // 字段拷贝；`self_expr` 指向同一存活调用子树内的 arena 节点，callee
        // （`match_string_call`）只对它判型与只读。
        self.match_string_call(func.index, self_expr, node.args);
      } else if let Some(type_id) = self.context.get().get_type(self_expr)
        && is_string(type_id)
      {
        self.match_string_call(func.index, self_expr, node.args);
      }
      return;
    }
    // Safety: `func.expr` 非空且指向存活 arena 节点（同 self_ 分支）；判型命中
    // `AstExprGlobal` 后仅拷贝其 `name` 字段（`AstName` 为 Copy），借用止于下
    // 方分支条件求值。
    let Some(lib) = (unsafe { rtti::ast_node_try_as_ptr::<AstExprGlobal>(func.expr) }) else {
      return;
    };
    let lib_name = lib.name;
    if lib_name == "string" {
      if let Some(&first_arg) = node.args.first() {
        let rest = AstArray::from_slice(&node.args[1..]);
        // Safety: `first_arg` 是存活 `args` 数组首槽位的非空 arena 表达式
        // 指针（与 `rest` 同源），`func.index` 为上方存活的 `&AstExprIndexName`
        // 的字段拷贝；callee 对二者只读。
        self.match_string_call(func.index, first_arg, rest);
      }
    } else if lib_name == "os"
      && func.index == "date"
      && let Some(&arg0) = node.args.first()
    {
      // Safety: `arg0` 是存活 `args` 数组的首槽位指针（`Some` 已保证存在），
      // 指向 parser arena 中不可变存活的表达式节点；判型命中后共享借用只读
      // `value`/`location`，止于本分支。
      if let Some(fmt) = unsafe { rtti::ast_node_try_as_ptr::<AstExprConstantString>(arg0) }
        && let Some(error) = self.check_date_format(fmt.value.as_bytes())
      {
        emit_warning(
          self.context.get(),
          Code::FormatString,
          fmt.base.base.location,
          format_args!("Invalid date format: {}", error),
        );
      }
    }
  }
}

// —— 原 methods/lint_format_string_match_string_call.rs ——
impl<'ctx> LintFormatString<'ctx> {
  pub fn match_string_call(
    &mut self,
    name: AstName,
    self_expr: *mut AstExpr,
    args: AstArray<*mut AstExpr>,
  ) {
    let args_slice = args.as_slice();
    let is_format = name == "format";
    let is_pack_packsize_unpack = name == "pack" || name == "packsize" || name == "unpack";
    let is_match_gmatch = name == "match" || name == "gmatch";
    let is_find = name == "find";
    let is_gsub = name == "gsub";
    let mut handle = self.context;
    let context = handle.get();
    // repr(C) 基类在偏移 0，`as *mut AstNode` 是基址不变的类型视图转换；
    // self_expr 由调用方 match_call 从存活 AstExprCall 节点取出，非空。
    let self_node = self_expr.as_ast_node();
    if is_format {
      // Safety: `self_node` 指向调用方 `match_call` 正持有的存活 `AstExprCall`
      // 的 self 表达式（parser arena 分配，非空、遍历期间存活且无人改写）；
      // try_as_ptr 读偏移 0 判型，命中返回的只读引用即该节点基址，存活期覆盖
      // 本分支，且 linter 的可变状态与 AST arena 不相交。
      if let Some(fmt) = unsafe { rtti::ast_node_try_as_ptr::<AstExprConstantString>(self_node) }
        && let Some(error) = self.check_string_format(fmt.value.as_bytes())
      {
        emit_warning(
          context,
          Code::FormatString,
          fmt.base.base.location,
          format_args!("Invalid format string: {}", error),
        );
      }
    } else if is_pack_packsize_unpack {
      // Safety: 与 format 分支同源——同一 `self_node` 指针，判型为
      // AstExprConstantString 后只读其 value/location；引用生命周期止于本分支。
      if let Some(fmt) = unsafe { rtti::ast_node_try_as_ptr::<AstExprConstantString>(self_node) } {
        let is_packsize = name == "packsize";
        if let Some(error) = self.check_string_pack(fmt.value.as_bytes(), is_packsize) {
          emit_warning(
            context,
            Code::FormatString,
            fmt.base.base.location,
            format_args!("Invalid pack format: {}", error),
          );
        }
      }
    } else if is_match_gmatch && let Some(&first_arg) = args_slice.first() {
      // Safety: `first_arg` 取自 `args`（即存活 `AstExprCall::args`，parser 成对
      // 写入 data/size 的 AstArray），槽位都是 arena 内存活的非空表达式指针；
      // 判型后只读模式串内容，无并发可变别名。
      if let Some(pat) = unsafe { rtti::ast_node_try_as_ptr::<AstExprConstantString>(first_arg) }
        && let Err(error) = self.check_string_match(pat.value.as_bytes())
      {
        emit_warning(
          context,
          Code::FormatString,
          pat.base.base.location,
          format_args!("Invalid match pattern: {}", error),
        );
      }
    } else if is_find && !args_slice.is_empty() && args_slice.len() <= 2 {
      let first = args_slice[0];
      // Safety: `first` 与上一分支的 first_arg 同源，是存活调用节点的
      // 第一实参槽位；判型命中后引用借用同一 arena 节点，只读到分支结束。
      if let Some(pat) = unsafe { rtti::ast_node_try_as_ptr::<AstExprConstantString>(first) }
        && let Err(error) = self.check_string_match(pat.value.as_bytes())
      {
        emit_warning(
          context,
          Code::FormatString,
          pat.base.base.location,
          format_args!("Invalid match pattern: {}", error),
        );
      }
    } else if is_find && args_slice.len() >= 3 {
      let plain = args_slice[2];
      let first = args_slice[0];
      // Safety: `plain` 是 find 第三实参（plain 标志）所在的存活 arena 节点
      // 指针，判型只读 class_index、`value` 是 bool 字段拷贝。
      if let Some(mode_val) = unsafe { rtti::ast_node_try_as_ptr::<AstExprConstantBool>(plain) }
        && !mode_val.value
      {
        // Safety: 对第一实参（模式串）的下转与 is_find 短参分支完全同源
        // （同一 args 数组、同一存活期），只读引用仅在本分支作用域内使用。
        if let Some(pat) = unsafe { rtti::ast_node_try_as_ptr::<AstExprConstantString>(first) }
          && let Err(error) = self.check_string_match(pat.value.as_bytes())
        {
          emit_warning(
            context,
            Code::FormatString,
            pat.base.base.location,
            format_args!("Invalid match pattern: {}", error),
          );
        }
      }
    } else if is_gsub && args_slice.len() > 1 {
      let mut captures = -1;
      let first = args_slice[0];
      // Safety: `first` 为 gsub 模式串实参槽位，指向存活 arena 节点；match
      // 两个 arm 对引用的使用都在判型后的同一作用域内。
      if let Some(pat) = unsafe { rtti::ast_node_try_as_ptr::<AstExprConstantString>(first) } {
        match self.check_string_match(pat.value.as_bytes()) {
          Ok(c) => {
            captures = c;
          }
          Err(error) => {
            emit_warning(
              context,
              Code::FormatString,
              pat.base.base.location,
              format_args!("Invalid match pattern: {}", error),
            );
          }
        }
      }
      let repl = args_slice[1];
      // Safety: `repl` 是同一存活 `AstExprCall::args` 数组的替换串槽位，与
      // 上一块同源；只读取字符串字段与 location。
      if let Some(rep) = unsafe { rtti::ast_node_try_as_ptr::<AstExprConstantString>(repl) }
        && let Some(error) = self.check_string_replace(rep.value.as_bytes(), captures)
      {
        emit_warning(
          context,
          Code::FormatString,
          rep.base.base.location,
          format_args!("Invalid match replacement: {}", error),
        );
      }
    }
  }
}

// —— 原 methods/lint_format_string_process.rs ——
impl<'ctx> LintFormatString<'ctx> {
  lint_stat_process!(LintFormatString);
}
