//! `type_pack_stringifier` 方法汇总：原先按 cpp 符号逐方法拆分的同前缀小文件合并至此，行为逐字保留。

use alloc::vec::Vec;
use core::ptr::from_mut;

use ulua_common::fint;

use crate::{
  functions::{
    follow_type_pack, get_type_pack::get, is_empty::is_empty,
    to_string_detailed_to_string::visit_pack_arms,
  },
  records::{
    arena_handle::alias_opt, blocked_type_pack::BlockedTypePack, free_type_pack::FreeTypePack,
    function_argument::FunctionArgument, generic_type_pack::GenericTypePack,
    stringifier_state::StringifierState,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    type_pack_stringifier::TypePackStringifier, type_stringifier::TypeStringifier,
    variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack, type_id::TypeId,
    type_pack_id::TypePackId,
  },
};

impl TypePackStringifier {
  /// C++ `void stringify(TypeId tv)`. 转发给 `TypeStringifier::stringify_type_id`；
  /// `tvs.state` 与 `self.state` 指向同一 `StringifierState`，转发期间不再触碰
  /// `self.state`，借用串行无重叠。
  pub fn stringify_type_id(&mut self, tv: TypeId) {
    let mut tvs = TypeStringifier { state: self.state };
    tvs.stringify_type_id(tv);
  }

  /// C++ `void stringify(TypePackId tp)` — the `Luau::visit` dispatch.
  pub(crate) fn stringify_type_pack_id(&mut self, tp: TypePackId) {
    let state = self.st();
    let opts = state.opts_mut();
    let result = state.result_mut();
    if opts.max_type_length > 0 && result.name.len() > opts.max_type_length {
      return;
    }

    if let Some(p) = state.cycle_tp_names.find(&tp) {
      let name = p.clone();
      state.emit(name.as_str());
      return;
    }

    visit_pack_arms(self, tp);
  }
}

impl TypePackStringifier {
  /// C++ `explicit TypePackStringifier(StringifierState& state, const std::vector<std::optional<FunctionArgument>>& elemNames)`：
  /// 形参降为受检 `&mut`（C++ 引用直译），仅在构造体把字段落为裸指针（记录布局不改）。
  pub fn type_pack_stringifier_stringifier_state_vector_optional_function_argument(
    state: &mut StringifierState,
    elem_names: &[Option<FunctionArgument>],
  ) -> Self {
    Self {
      state: from_mut(state),
      elem_names: elem_names.to_vec(),
      elem_index: 0,
    }
  }

  /// C++ `explicit TypePackStringifier(StringifierState& state)` — uses the
  /// empty `dummyElemNames`.
  pub fn type_pack_stringifier_stringifier_state(state: &mut StringifierState) -> Self {
    Self {
      state: from_mut(state),
      elem_names: Vec::new(),
      elem_index: 0,
    }
  }
}

// Source: `Analysis/src/ToString.cpp:1220-1380` (hand-ported)

impl TypePackStringifier {
  /// C++ `void operator()(TypePackId, const TypePack& tp)`.
  pub fn stringify_type_pack(&mut self, _id: TypePackId, tp: &TypePack) {
    // state/opts/result 经 `st`/`opts_mut`/`result_mut` 单点收口；`tp` 借用自
    // 正在匹配的 arena TypePack 节点，仅作指针身份做 seen 集去重/回溯。

    if self.st().has_seen(tp) {
      self.st().result_mut().cycle = true;
      self.st().emit("*CYCLETP*");
      return;
    }

    // 双写合一：`tail.is_none() || is_empty(tail.unwrap())` 收为 `is_none_or`。
    if tp.head.is_empty() && tp.tail.is_none_or(is_empty) {
      self.st().emit("()");
      self.st().unsee(tp);
      return;
    }

    let mut first = true;

    for &type_id in tp.head.iter() {
      if first {
        first = false;
      } else {
        self.st().emit(", ");
      }

      // Do not respect opts.namedFunctionOverrideArgNames here
      let idx = self.elem_index as usize;
      if idx < self.elem_names.len()
        && let Some(elem_name) = self.elem_names[idx].as_ref()
      {
        let name = elem_name.name.clone();
        self.st().emit(name.as_str());
        self.st().emit(": ");
      }

      self.elem_index += 1;

      self.stringify_type_id(type_id);
    }

    if let Some(tp_tail) = tp.tail
      && !is_empty(tp_tail)
    {
      let tail = follow_type_pack::follow(tp_tail);
      let vtp = get::<VariadicTypePack>(tail);
      let should_emit = match vtp {
        None => true,
        Some(v) => fint::DebugLuauVerboseTypeNames.get() < 1 && !v.hidden,
      };
      if should_emit {
        if first {
          // first = false; (C++ writes it; nothing reads it after)
        } else {
          self.st().emit(", ");
        }

        self.stringify_type_pack_id(tail);
      }
    }

    self.st().unsee(tp);
  }

  /// C++ `void operator()(TypePackId, const ErrorTypePack& error)`.
  pub fn stringify_error_pack(&mut self, _id: TypePackId, error: &ErrorTypePack) {
    // Safety: `self.state` 与 `result` 与 stringify_type_pack 同源——构造期注入的
    // 入口局部 `StringifierState`/`ToStringResult` 裸化句柄，本次调用内独占；
    // `error.synthetic` 是从被匹配 arena 节点读出的另一 TypePackId，满足
    // `stringify_type_pack_id` 的存活入参契约。

    self.st().result_mut().error = true;

    if let Some(synthetic) = error.synthetic {
      self.st().emit("*");
      self.stringify_type_pack_id(synthetic);
      self.st().emit("*");
    } else {
      self.st().emit("*error-type*");
    }
  }

  /// C++ `void operator()(TypePackId, const VariadicTypePack& pack)`.
  pub fn stringify_variadic_pack(&mut self, _id: TypePackId, pack: &VariadicTypePack) {
    // Safety: 仅向 `self.state` emit——它指向入口函数活借用后裸化的
    // `StringifierState`，本次序列化期间为唯一访问路径；`pack.ty` 借用自被匹配
    // arena 节点，满足 `stringify_type_id` 契约。

    self.st().emit("...");
    if fint::DebugLuauVerboseTypeNames.get() >= 1 && pack.hidden {
      self.st().emit("*hidden*");
    }
    self.stringify_type_id(pack.ty);
  }

  /// C++ `void operator()(TypePackId tp, const GenericTypePack& pack)`.
  pub fn stringify_generic_pack(&mut self, tp: TypePackId, pack: &GenericTypePack) {
    if fint::DebugLuauVerboseTypeNames.get() >= 1 {
      self.st().emit("gen-");
    }

    if pack.explicit_name {
      self.st().used_names.insert(pack.name.clone());
      *self.st().opts_mut().name_map.type_packs.get_or_insert(tp) = pack.name.clone();
      self.st().emit(pack.name.as_str());
    } else {
      let name = self.st().get_name_type_pack_id(tp);
      self.st().emit(name.as_str());
    }

    if fint::DebugLuauVerboseTypeNames.get() >= 1 {
      self.st().emit_polarity(pack.polarity);
    }

    if fint::DebugLuauVerboseTypeNames.get() >= 2 {
      self.st().emit("-");
      // `pack.scope` 为 arena 节点接线、存活跨 stringify 会话的 `Scope` 句柄
      // （可为空）；经 alias_opt 收口为 Option<&Scope> 后只读上溯。
      self.st().emit_level(alias_opt(pack.scope));
    }

    self.st().emit("...");
  }

  /// C++ `void operator()(TypePackId tp, const FreeTypePack& pack)`.
  pub fn stringify_free_pack(&mut self, tp: TypePackId, pack: &FreeTypePack) {
    self.st().result_mut().invalid = true;
    if fint::DebugLuauVerboseTypeNames.get() >= 1 {
      self.st().emit("free-");
    }
    let name = self.st().get_name_type_pack_id(tp);
    self.st().emit(name.as_str());

    if fint::DebugLuauVerboseTypeNames.get() >= 1 {
      self.st().emit_polarity(pack.polarity);
    }

    if fint::DebugLuauVerboseTypeNames.get() >= 2 {
      self.st().emit("-");
      // `pack.scope` 同上——构造期接线的存活 `Scope` 句柄，alias_opt 收口。
      self.st().emit_level(alias_opt(pack.scope));
    }

    self.st().emit("...");
  }

  /// C++ `void operator()(TypePackId, const BoundTypePack& btv)`.
  pub fn stringify_bound_pack(&mut self, _id: TypePackId, btv: &BoundTypePack) {
    self.stringify_type_pack_id(btv.bound_to);
  }

  /// C++ `void operator()(TypePackId, const BlockedTypePack& btp)`.
  pub fn stringify_blocked_pack(&mut self, _id: TypePackId, btp: &BlockedTypePack) {
    self.st().emit("*blocked-tp-");
    self.st().emit(&btp.index);
    self.st().emit("*");
  }

  /// C++ `void operator()(TypePackId, const TypeFunctionInstanceTypePack& tfitp)`.
  pub fn stringify_type_function_instance_pack(
    &mut self,
    _id: TypePackId,
    tfitp: &TypeFunctionInstanceTypePack,
  ) {
    // `tfitp.function()` 经契约访问器共享借用：指向类型函数表中存活记录。
    let fname = &tfitp.function().name;
    self.st().emit(fname.as_str());
    self.st().emit("<");

    let mut comma = false;
    for &p in tfitp.type_arguments.iter() {
      if comma {
        self.st().emit(", ");
      }

      comma = true;
      self.stringify_type_id(p);
    }

    for &p in tfitp.pack_arguments.iter() {
      if comma {
        self.st().emit(", ");
      }

      comma = true;
      self.stringify_type_pack_id(p);
    }

    self.st().emit(">");
  }
}
