//! `stringifier_state` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::{
  format,
  string::{String, ToString},
};
use core::{
  iter::repeat_n,
  ptr::{from_mut, from_ref},
};

use ulua_common::{
  fflag, fint,
  records::{
    dense_hash_map::DenseHashMap, dense_hash_set::DenseHashSet, dense_hash_table::DenseDefault,
  },
};

use crate::{
  enums::polarity::Polarity,
  functions::{
    follow_type, follow_type_pack,
    generate_name::{MAX_GENERATED_NAME_ATTEMPTS, generate_name},
    get_type, get_type_pack,
  },
  records::{
    generic_type::GenericType, generic_type_pack::GenericTypePack, scope::Scope,
    scope_registry::resolve_scope, set::Set, stringifier_state::StringifierState,
    to_string_options::ToStringOptions, to_string_result::ToStringResult,
    to_string_span::ToStringSpan, type_level::TypeLevel, visit_key::VisitKey,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl StringifierState {
  pub fn dedent(&mut self) {
    self.indentation -= 4;
  }
}

// Source: `Analysis/src/ToString.cpp:305-313` (hand-ported)

impl StringifierState {
  /// C++ `void emitAndRecordSpan(const std::string& s, TypeId ty)`.
  pub fn emit_and_record_span(&mut self, s: &str, ty: TypeId) {
    let start_pos = self.result_mut().name.len();
    self.emit(s);
    let end_pos = self.result().name.len();

    if end_pos > start_pos {
      self.result_mut().type_spans.push(ToStringSpan {
        start_pos,
        end_pos,
        r#type: ty,
      });
    }
  }
}

impl StringifierState {
  /// C++ `emitIndentation()`:直接向结果追加缩进空格,
  /// 复用 `emit` 的截断守卫,免去临时 `String` 分配。
  pub fn emit_indentation(&mut self) {
    if !self.opts().use_line_breaks {
      return;
    }

    let max_type_length = self.opts().max_type_length;
    if max_type_length > 0 && self.result().name.len() > max_type_length {
      return;
    }

    // 计数循环改迭代器：repeat_n 按 size_hint 一次性预留并追加空格，免手工下标。
    self
      .result_mut()
      .name
      .extend(repeat_n(' ', self.indentation));
  }
}

// Source: `Analysis/src/ToString.cpp:262-278` (hand-ported)

impl StringifierState {
  /// C++ `void emitLevel(Scope* scope)`：可空 `Scope*` 形参以 `Option<&Scope>`
  /// 承载（cpp nullptr 语义），函数体内沿 `parent` 句柄链只读上溯。
  pub fn emit_level(&mut self, scope: Option<&Scope>) {
    let mut count: usize = 0;
    // 句柄化上溯：仅计数深度，走只读 resolve_scope 出口。
    let mut s = scope;
    while let Some(scope_ref) = s {
      count += 1;
      s = scope_ref.parent.and_then(resolve_scope);
    }

    self.emit(&count);

    if fint::DebugLuauVerboseTypeNames.get() >= 3 {
      self.emit("-");
      // snprintf(Buffer, 16, "0x%x", uint32_t(intptr_t(scope) & 0xFFFFFF))
      let v = (scope.map_or(0, |s| from_ref(s) as usize) as u32) & 0xFFFFFF;
      let buffer = format!("0x{:x}", v);
      self.emit(buffer.as_str());
    }
  }
}

// Source: `Analysis/src/ToString.cpp:254-305` (hand-ported)
// The C++ overloaded `emit(...)` family as a trait + generic method
// (the AstJsonEncoder::write precedent).

pub trait EmitText {
  fn emit_text(&self, state: &mut StringifierState);
}
impl StringifierState {
  /// Generic entry mirroring C++ overload resolution: `state.emit(x)`.
  pub fn emit<T: EmitText + ?Sized>(&mut self, value: &T) {
    value.emit_text(self);
  }

  fn emit_str_raw(&mut self, s: &str) {
    // if (opts.maxTypeLength > 0 && result.name.length() > opts.maxTypeLength) return;
    let opts = self.opts();
    if opts.max_type_length > 0 && self.result().name.len() > opts.max_type_length {
      return;
    }
    self.result_mut().name.push_str(s);
  }
}
impl EmitText for str {
  fn emit_text(&self, state: &mut StringifierState) {
    state.emit_str_raw(self);
  }
}
impl EmitText for String {
  fn emit_text(&self, state: &mut StringifierState) {
    state.emit_str_raw(self);
  }
}
impl EmitText for TypeLevel {
  fn emit_text(&self, state: &mut StringifierState) {
    state.emit_str_raw(&self.level.to_string());
    state.emit_str_raw("-");
    state.emit_str_raw(&self.sub_level.to_string());
  }
}
macro_rules! emit_int {
    ($($t:ty),*) => {$(
        impl EmitText for $t {
            fn emit_text(&self, state: &mut StringifierState) {
                state.emit_str_raw(&self.to_string());
            }
        }
    )*};
}
emit_int!(i32, i64, u32, u64, usize, isize);

impl StringifierState {
  pub fn emit_string(&mut self, s: &str) {
    if self.opts.is_null() {
      return;
    }

    let max_type_length = self.opts().max_type_length;
    if max_type_length > 0 && self.result().name.len() > max_type_length {
      return;
    }

    self.result_mut().name.push_str(s);
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

impl StringifierState {
  /// C++ `std::string getName(TypeId ty)`。
  pub fn get_name_type_id(&mut self, ty: TypeId) -> String {
    let opts = self.opts_mut();
    let s = opts.name_map.types.size();
    // std::string& n = opts.nameMap.types[ty]; (default-constructs)
    {
      let n = opts.name_map.types.get_or_insert(ty);
      if !n.is_empty() {
        return n.clone();
      }
    }

    // C++ `FFlag::LuauBetterInferredGenericNames ? nullptr != get<GenericType>(follow(ty)) : false`。
    let is_for_generic = fflag::LuauBetterInferredGenericNames.get()
      && get_type::get::<GenericType>(follow_type::follow(ty)).is_some();

    // 试名改迭代器链：map 产候选、find 短路命中，与原 early-return 循环逐位等价。
    if let Some(candidate) = (0..MAX_GENERATED_NAME_ATTEMPTS)
      .map(|count| generate_name(self.used_names.size() + count, is_for_generic))
      .find(|candidate| !self.used_names.contains_str(candidate.as_str()))
    {
      self.used_names.insert(candidate.clone());
      *opts.name_map.types.get_or_insert(ty) = candidate.clone();
      return candidate;
    }

    generate_name(s, is_for_generic)
  }

  /// C++ `std::string getName(TypePackId ty)`。
  pub fn get_name_type_pack_id(&mut self, ty: TypePackId) -> String {
    let opts = self.opts_mut();
    let s = opts.name_map.type_packs.size();
    {
      let n = opts.name_map.type_packs.get_or_insert(ty);
      if !n.is_empty() {
        return n.clone();
      }
    }

    // C++ `FFlag::LuauBetterInferredGenericNames ? nullptr != get<GenericTypePack>(follow(ty)) : false`。
    let is_for_generic = fflag::LuauBetterInferredGenericNames.get()
      && get_type_pack::get::<GenericTypePack>(follow_type_pack::follow(ty)).is_some();

    // 试名改迭代器链；count 随候选一并带出，用于回写 previous_name_index 水位。
    let base = self.previous_name_index as usize;
    if let Some((count, candidate)) = (0..MAX_GENERATED_NAME_ATTEMPTS)
      .map(|count| (count, generate_name(base + count, is_for_generic)))
      .find(|(_, candidate)| !self.used_names.contains_str(candidate.as_str()))
    {
      self.previous_name_index += count as i32;
      self.used_names.insert(candidate.clone());
      *opts.name_map.type_packs.get_or_insert(ty) = candidate.clone();
      return candidate;
    }

    generate_name(s, is_for_generic)
  }
}

// Source: `Analysis/src/ToString.cpp:191-199` (hand-ported)

impl StringifierState {
  /// C++ `bool hasSeen(const void* tv)`. §2：身份键入参收为共享引用，
  /// 地址转换是本函数唯一指针操作（只作身份，从不解引用该指针）。
  pub fn has_seen<T>(&mut self, tv: &T) -> bool {
    let key = VisitKey::from_ptr(from_ref(tv));
    if self.seen.contains(&key) {
      return true;
    }

    self.seen.insert(&key);
    false
  }
}

impl StringifierState {
  pub fn indent(&mut self) {
    self.indentation += 4;
  }
}

// Source: `Analysis/src/ToString.cpp:347-354` (hand-ported)

impl StringifierState {
  /// C++ `void newline()` — `if (!opts.useLineBreaks) return emit(" ");`
  /// (an earlier translation dropped the space, gluing separators).
  pub fn newline(&mut self) {
    if !self.opts().use_line_breaks {
      return self.emit_string(" ");
    }

    self.emit_string("\n");
    self.emit_indentation();
  }
}

// Source: `Analysis/src/ToString.cpp:178-190` (hand-ported)

impl StringifierState {
  /// C++ `StringifierState(ToStringOptions& opts, ToStringResult& result)`.
  /// The C++ reference members are raw pointers in the record; callers keep
  /// `opts`/`result` alive for the state's lifetime (as in C++).
  ///
  /// 降 safe：两个形参均为受检引用（C++ 引用形参的直译），裸指针只在本构造体
  /// 内落字段（记录布局不改），解引用统一走记录侧 `opts_mut`/`result_mut` 收口。
  pub fn stringifier_state_stringifier_state(
    opts: &mut ToStringOptions,
    result: &mut ToStringResult,
  ) -> Self {
    let mut state = StringifierState {
      opts: from_mut(opts),
      result: from_mut(result),
      cycle_names: DenseHashMap::default(),
      cycle_tp_names: DenseHashMap::default(),
      seen: Set::new(VisitKey::dense_default()),
      // `$$$` is the usedNames tombstone: not a valid name syntactically
      // and short for string comparison reasons.
      used_names: DenseHashSet::new(String::from("$$$")),
      indentation: 0,
      exhaustive: opts.exhaustive,
      ignore_synthetic_name: opts.ignore_synthetic_name,
      previous_name_index: 0,
    };

    // 裸指针落字段后 `opts` 的借用随之结束：此处为对 `*opts` 的普通共享读。
    for (_k, v) in opts.name_map.types.iter() {
      state.used_names.insert(v.clone());
    }
    for (_k, v) in opts.name_map.type_packs.iter() {
      state.used_names.insert(v.clone());
    }

    state
  }
}

// Source: `Analysis/src/ToString.cpp:201-207` (hand-ported)
//
// C++ `void unsee(const void* tv)` — really erases (Luau::Set supports
// erase). An earlier translation wired this to the DenseHashSet no-op
// `unsee`, which made every repeated sibling type print `*CYCLE*`.

impl StringifierState {
  /// §2：身份键入参收为共享引用，地址转换只作身份、不解引用。
  pub fn unsee<T>(&mut self, tv: &T) {
    let key = VisitKey::from_ptr(from_ref(tv));

    if self.seen.contains(&key) {
      self.seen.erase(&key);
    }
  }
}
