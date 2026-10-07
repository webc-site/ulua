use alloc::vec::Vec;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_array::AstArray, ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_name::AstName,
    ast_visitor::AstVisitor,
  },
  visit::ast_stat_visit_ref,
};
use ulua_config::enums::code::Code;

use crate::{
  functions::{emit_warning::emit_warning, is_string::is_string},
  macros::lint_stat_process,
  records::{
    arena_handle::alias_opt, lint_context::LintContext, lint_context_handle::LintContextHandle,
  },
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
/// `string.pack` 无长度说明符集，`X` 后禁止项（原局部绑定 `unsized_opts`）；
/// 含 `=`——`X=` 一样缺尺寸（cpp Linter.cpp:1548 `"<>=!zX "`）。
const K_PACK_UNSIZED_OPTS: CharSet = CharSet::of(b"<>=!zX ");

impl<'ctx> AstVisitor for LintFormatString<'ctx> {
  fn visit_expr_call(&mut self, node: &mut AstExprCall) -> bool {
    self.match_call(node);
    true
  }
}

// —— 原 methods/lint_format_string_check_date_format.rs ——
impl<'ctx> LintFormatString<'ctx> {
  pub fn check_date_format(&self, mut data: &[u8]) -> Option<&'static str> {
    while let Some((&first, rest)) = data.split_first() {
      match first {
        b'%' => match rest.split_first() {
          None => return Some("unfinished replacement"),
          Some((&next, tail)) => {
            if next != b'%' && !K_DATE_OPTIONS.has(next) {
              return Some(
                "unexpected replacement character; must be a date format specifier or %",
              );
            }
            data = tail;
          }
        },
        0 => return Some("date format can not contain null characters"),
        _ => data = rest,
      }
    }
    None
  }
}

// —— 原 methods/lint_format_string_check_string_format.rs ——
impl<'ctx> LintFormatString<'ctx> {
  pub fn check_string_format(&self, mut data: &[u8]) -> Option<&'static str> {
    while let Some((&first, mut rest)) = data.split_first() {
      if first != b'%' {
        data = rest;
        continue;
      }
      // Escaped % doesn't allow for flags/etc.
      if let Some(tail) = rest.strip_prefix(b"%") {
        data = tail;
        continue;
      }
      // Skip flags
      while let Some((&flag, tail)) = rest.split_first() {
        if !K_FORMAT_FLAGS.has(flag) {
          break;
        }
        rest = tail;
      }
      // Skip width (up to two digits)
      if let Some((&d1, tail)) = rest.split_first()
        && is_digit(d1)
      {
        rest = tail;
        if let Some((&d2, tail2)) = rest.split_first()
          && is_digit(d2)
        {
          rest = tail2;
        }
      }
      // Skip precision ('.' followed by up to two digits)
      if let Some(after_dot) = rest.strip_prefix(b".") {
        rest = after_dot;
        if let Some((&d1, tail)) = rest.split_first()
          && is_digit(d1)
        {
          rest = tail;
          if let Some((&d2, tail2)) = rest.split_first()
            && is_digit(d2)
          {
            rest = tail2;
          }
        }
      }
      match rest.split_first() {
        None => return Some("unfinished format specifier"),
        Some((&opt, tail)) => {
          if !K_FORMAT_OPTIONS.has(opt) {
            return Some("invalid format specifier: must be a string format specifier or %");
          }
          data = tail;
        }
      }
    }
    None
  }
}

// —— 原 methods/lint_format_string_check_string_match.rs ——
impl<'ctx> LintFormatString<'ctx> {
  #[inline]
  pub fn check_string_match(&self, mut data: &[u8]) -> Result<i32, &'static str> {
    let mut open_captures: Vec<i32> = Vec::new();
    let mut total_captures: i32 = 0;
    while let Some((&first, rest)) = data.split_first() {
      match first {
        b'%' => match rest.split_first() {
          None => return Err("unfinished character class"),
          Some((&ch, tail)) => {
            if is_digit(ch) {
              if ch == b'0' {
                return Err("invalid capture reference, must be 1-9");
              }
              let capture_index = i32::from(ch - b'0');
              if capture_index > total_captures {
                return Err("invalid capture reference, must refer to a valid capture");
              }
              if open_captures.contains(&capture_index) {
                return Err("invalid capture reference, must refer to a closed capture");
              }
              data = tail;
            } else if self.is_alpha(ch) {
              if ch == b'b' {
                let Some(after_braces) = tail.get(2..) else {
                  return Err("missing brace characters for balanced match");
                };
                data = after_braces;
              } else if ch == b'f' {
                if !tail.starts_with(b"[") {
                  return Err("missing set after a frontier pattern");
                }
                // we can parse the set with the regular logic
                data = tail;
              } else {
                // lower case lookup - upper case for every character class is defined as its inverse
                if !K_CLASSES.has(ch | b' ') {
                  return Err(
                    "invalid character class, must refer to a defined class or its inverse",
                  );
                }
                data = tail;
              }
            } else {
              // technically % can escape any non-alphanumeric character but this is error-prone
              if !K_MAGIC_CHARS.has(ch) {
                return Err("expected a magic character after %");
              }
              data = tail;
            }
          }
        },
        b'[' => {
          let mut scan = rest;
          // empty patterns don't exist as per grammar rules, so we skip leading ^ and ]
          if let Some(after_hat) = scan.strip_prefix(b"^") {
            scan = after_hat;
          }
          if let Some(after_bracket) = scan.strip_prefix(b"]") {
            scan = after_bracket;
          }
          let mut closed = false;
          while let Some((&ch, tail)) = scan.split_first() {
            match ch {
              b']' => {
                let len = rest.len() - scan.len();
                let set_content = &rest[..len];
                if let Some(error) =
                  self.check_string_match_set(set_content, K_MAGIC_CHARS, K_CLASSES)
                {
                  return Err(error);
                }
                data = tail;
                closed = true;
                break;
              }
              b'%' => {
                // % escapes the next character
                scan = match tail.split_first() {
                  Some((_, after_escaped)) => after_escaped,
                  None => tail,
                };
              }
              _ => scan = tail,
            }
          }
          if !closed {
            return Err("expected ] at the end of the string to close a set");
          }
        }
        b'(' => {
          total_captures += 1;
          open_captures.push(total_captures);
          data = rest;
        }
        b')' => {
          if open_captures.is_empty() {
            return Err("unexpected ) without a matching (");
          }
          open_captures.pop();
          data = rest;
        }
        _ => data = rest,
      }
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
    mut data: &[u8],
    magic: CharSet,
    classes: CharSet,
  ) -> Option<&'static str> {
    while let Some((&first, rest)) = data.split_first() {
      match first {
        b'%' => match rest.split_first() {
          None => return Some("unfinished character class"),
          Some((&next_ch, tail)) => {
            if is_digit(next_ch) {
              return Some("sets can not contain capture references");
            } else if self.is_alpha(next_ch) {
              // lower case lookup - upper case for every character class is defined as its inverse
              if !classes.has(next_ch | b' ') {
                return Some(
                  "invalid character class, must refer to a defined class or its inverse",
                );
              }
            } else if !magic.has(next_ch) {
              // technically % can escape any non-alphanumeric character but this is error-prone
              return Some("expected a magic character after %");
            }
            if tail.starts_with(b"-") {
              return Some("character range can't include character sets");
            }
            data = tail;
          }
        },
        b'-' => {
          if rest.starts_with(b"%") {
            return Some("character range can't include character sets");
          }
          data = rest;
        }
        _ => data = rest,
      }
    }
    None
  }
}

// —— 原 methods/lint_format_string_check_string_pack.rs ——
impl<'ctx> LintFormatString<'ctx> {
  #[inline]
  pub fn check_string_pack(&self, mut data: &[u8], fixed: bool) -> Option<&'static str> {
    while let Some((&ch, mut rest)) = data.split_first() {
      if !K_PACK_OPTIONS.has(ch) {
        return Some("unexpected character; must be a pack specifier or space");
      }
      let first_is_digit = rest.first().is_some_and(|&d| is_digit(d));
      if ch == b'c' && !first_is_digit {
        return Some("fixed-sized string format must specify the size");
      }
      let next_unsized = rest
        .first()
        .is_none_or(|&next| K_PACK_UNSIZED_OPTS.has(next));
      if ch == b'X' && next_unsized {
        return Some("X must be followed by a size specifier");
      }
      if fixed && matches!(ch, b'z' | b's') {
        return Some("pack specifier must be fixed-size");
      }
      if matches!(ch, b'!' | b'i' | b'I' | b'c' | b's') && first_is_digit {
        let (digits, tail) = match rest.iter().position(|&d| !is_digit(d)) {
          Some(pos) => rest.split_at(pos),
          None => (rest, &[][..]),
        };
        let v = match digits.iter().try_fold(0u32, |acc, &d| {
          if acc <= (i32::MAX as u32 - 9) / 10 {
            Ok(acc * 10 + u32::from(d - b'0'))
          } else {
            Err(())
          }
        }) {
          Ok(v) => v,
          Err(()) => return Some("size specifier is too large"),
        };
        if ch != b'c' && (v == 0 || v > 16) {
          return Some("integer size must be in range [1,16]");
        }
        rest = tail;
      }
      data = rest;
    }
    None
  }
}

// —— 原 methods/lint_format_string_check_string_replace.rs ——
impl<'ctx> LintFormatString<'ctx> {
  #[inline]
  pub fn check_string_replace(&self, mut data: &[u8], captures: i32) -> Option<&'static str> {
    while let Some((&first, rest)) = data.split_first() {
      if first == b'%' {
        match rest.split_first() {
          None => return Some("unfinished replacement"),
          Some((&next_ch, tail)) => {
            if next_ch != b'%' && !is_digit(next_ch) {
              return Some("unexpected replacement character; must be a digit or %");
            }
            if is_digit(next_ch) && captures >= 0 && i32::from(next_ch - b'0') > captures {
              return Some("invalid capture index, must refer to pattern capture");
            }
            data = tail;
          }
        }
      } else {
        data = rest;
      }
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
/// cpp `Linter.cpp` 的静态自由函数 `isDigit(char)`（不依赖任何状态），Rust 对应
/// 模块级自由函数；无符号比较完成 '0'..='9' 范围判定。
#[inline]
fn is_digit(ch: u8) -> bool {
  // use unsigned comparison to do range check for performance
  ch.wrapping_sub(b'0') < 10
}

// —— 原 methods/lint_format_string_match_call.rs ——
impl<'ctx> LintFormatString<'ctx> {
  /// cpp `matchCall(AstExprCall*)`：`node` 为分析期存活、由 arena 持有的调用节点
  /// 共享借用（cpp 裸指针形参的 Rust 对应），本方法对其只读；宿主 context 经
  /// `self.context` 写句柄瞬时借用。
  pub fn match_call(&mut self, node: &AstExprCall) {
    let Some(func_ref) = alias_opt(node.func) else {
      return;
    };
    let AstExprRef::IndexName(func) = func_ref.as_expr_ref() else {
      return;
    };
    if node.self_ {
      let self_expr: *mut AstExpr = if let AstExprRef::Group(group) = func.expr.as_expr_ref() {
        // group.expr 已句柄化恒非空；行走链沿用裸指针 API，经 as_ptr 桥接。
        group.expr.as_ptr()
      } else {
        // func.expr 已句柄化恒非空；行走链沿用裸指针 API，经 as_ptr 桥接。
        func.expr.as_ptr()
      };
      let self_ref = alias_opt(self_expr);
      if matches!(
        self_ref.map(|e| e.as_expr_ref()),
        Some(AstExprRef::ConstantString(_))
      ) {
        self.match_string_call(func.index, self_expr, node.args);
      } else if let Some(type_id) = self.context.get().get_type(self_expr)
        && is_string(type_id)
      {
        self.match_string_call(func.index, self_expr, node.args);
      }
      return;
    }
    let AstExprRef::Global(lib) = func.expr.as_expr_ref() else {
      return;
    };
    let lib_name = lib.name;
    if lib_name == "string" {
      if let [first_arg, rest_args @ ..] = node.args.as_slice() {
        let rest = AstArray::from_slice(rest_args);
        self.match_string_call(func.index, *first_arg, rest);
      }
    } else if lib_name == "os"
      && func.index == "date"
      && let Some(&arg0) = node.args.first()
      && let Some(arg0_ref) = alias_opt(arg0)
      && let AstExprRef::ConstantString(fmt) = arg0_ref.as_expr_ref()
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

// —— 原 methods/lint_format_string_match_string_call.rs ——
impl<'ctx> LintFormatString<'ctx> {
  pub(crate) fn match_string_call(
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
    let self_ref = alias_opt(self_expr);
    if is_format {
      if let Some(self_ref) = self_ref
        && let AstExprRef::ConstantString(fmt) = self_ref.as_expr_ref()
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
      if let Some(self_ref) = self_ref
        && let AstExprRef::ConstantString(fmt) = self_ref.as_expr_ref()
      {
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
      if let Some(first_ref) = alias_opt(first_arg)
        && let AstExprRef::ConstantString(pat) = first_ref.as_expr_ref()
        && let Err(error) = self.check_string_match(pat.value.as_bytes())
      {
        emit_warning(
          context,
          Code::FormatString,
          pat.base.base.location,
          format_args!("Invalid match pattern: {}", error),
        );
      }
    } else if is_find {
      match args_slice {
        [first, ..] if args_slice.len() <= 2 => {
          if let Some(first_ref) = alias_opt(*first)
            && let AstExprRef::ConstantString(pat) = first_ref.as_expr_ref()
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
        [first, _, plain, ..] => {
          if let Some(plain_ref) = alias_opt(*plain)
            && let AstExprRef::ConstantBool(mode_val) = plain_ref.as_expr_ref()
            && !mode_val.value
            && let Some(first_ref) = alias_opt(*first)
            && let AstExprRef::ConstantString(pat) = first_ref.as_expr_ref()
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
        _ => {}
      }
    } else if is_gsub && let [first, repl, ..] = args_slice {
      let mut captures = -1;
      if let Some(first_ref) = alias_opt(*first)
        && let AstExprRef::ConstantString(pat) = first_ref.as_expr_ref()
      {
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
      if let Some(repl_ref) = alias_opt(*repl)
        && let AstExprRef::ConstantString(rep) = repl_ref.as_expr_ref()
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
