//! Source: `Analysis/src/ToString.cpp:1220-1380` (hand-ported)

use ulua_common::fint;

use crate::{
  functions::{follow_type_pack, get_type_pack::get, is_empty::is_empty},
  records::{
    blocked_type_pack::BlockedTypePack, free_type_pack::FreeTypePack,
    generic_type_pack::GenericTypePack,
    type_function_instance_type_pack::TypeFunctionInstanceTypePack, type_pack::TypePack,
    type_pack_stringifier::TypePackStringifier, variadic_type_pack::VariadicTypePack,
  },
  type_aliases::{
    bound_type_pack::BoundTypePack, error_type_pack::ErrorTypePack, type_pack_id::TypePackId,
  },
};

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
      // # Safety: `pack.scope` 为 arena 节点接线、存活跨 stringify 会话的
      // `Scope` 句柄（可为空）；emit_level 内仅 `as_ref` 只读上溯。
      unsafe { self.st().emit_level(pack.scope) };
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
      // # Safety: `pack.scope` 同上——构造期接线的存活 `Scope` 句柄。
      unsafe { self.st().emit_level(pack.scope) };
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
    // # Safety: `tfitp.function` 是写入实例节点时记录的 `*const TypePackFunction`，
    // 指向类型函数表中存活记录，此处仅只读其 `name`。
    let fname = unsafe { &(*tfitp.function).name };
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
