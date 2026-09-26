use crate::{
  functions::{
    follow_type_pack, get_type_pack::get, is_empty::is_empty,
    to_string_detailed_to_string::visit_type_arms,
  },
  records::{
    function_argument::FunctionArgument, property_type::Property, type_pack::TypePack,
    type_pack_stringifier::TypePackStringifier, type_stringifier::TypeStringifier,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl TypeStringifier {
  /// 对应 C++ `void TypeStringifier::stringify(TypeId tv)`（`Analysis/src/ToString.cpp`
  /// TypeStringifier 段）。`tv` 为 arena `TypeId` 句柄（同 `get_type_id` 门面
  /// 纪律）；`state/opts/result` 裸句柄经 `st`/`opts_mut`/`result_mut` 单点恢复
  /// 为普通引用（存活契约锚定入口 `to_string*` 会话），解引用不再外渗到本函数，
  /// 与文件内 `stringify_vector_*`/`stringify_type_pack_id` 同一接线契约。
  pub fn stringify_type_id(&mut self, tv: TypeId) {
    // state/opts/result 的裸句柄经 `st`/`opts_mut`/`result_mut` 单点恢复为
    // 普通引用；`tv` 是传入的存活 `TypeId`。
    let state = self.st();
    let opts = state.opts_mut();
    let result = state.result_mut();
    if opts.max_type_length > 0 && result.name.len() > opts.max_type_length {
      return;
    }

    if let Some(p) = state.cycle_names.find(&tv) {
      let name = p.clone();
      state.emit(name.as_str());
      return;
    }

    visit_type_arms(self, tv);
  }

  pub fn stringify_string_property(&mut self, name: &str, prop: &Property) {
    let mut comma = false;

    if prop.is_shared() {
      self.emit_key(name);
      if let Some(read_ty) = prop.read_ty {
        self.stringify_type_id(read_ty);
      }
      return;
    }

    if let Some(read_ty) = prop.read_ty {
      self.st().emit("read ");
      self.emit_key(name);
      self.stringify_type_id(read_ty);
      comma = true;
    }

    if let Some(write_ty) = prop.write_ty {
      if comma {
        let state = self.st();
        state.emit(",");
        state.newline();
      }

      self.st().emit("write ");
      self.emit_key(name);
      self.stringify_type_id(write_ty);
    }
  }

  /// C++ `void stringify(const std::vector<TypeId>& types, const std::vector<TypePackId>& typePacks)`.
  pub fn stringify_vector_type_id_vector_type_pack_id(
    &mut self,
    types: &[TypeId],
    type_packs: &[TypePackId],
  ) {
    // Safety: `self.state` 为非空存活的 `StringifierState`。每次 `self.st().emit(..)`
    // 是即时结束的 `&mut` 再借用，与随后 `stringify_type_id`/`stringify_type_pack_id`（各自
    // 重新借用 state）在单线程内顺序执行、互不重叠；`get::<TypePack>(..)` 仅返回共享只读引用。
    if types.is_empty() && type_packs.is_empty() {
      return;
    }

    if !types.is_empty() || !type_packs.is_empty() {
      self.st().emit("<");
    }

    let mut first = true;

    for &ty in types.iter() {
      if !first {
        self.st().emit(", ");
      }
      first = false;

      self.stringify_type_id(ty);
    }

    let single_tp = type_packs.len() == 1;

    for &tp in type_packs.iter() {
      if is_empty(tp) && single_tp {
        continue;
      }

      if !first {
        self.st().emit(", ");
      } else {
        first = false;
      }

      let mut wrap = !single_tp && get::<TypePack>(follow_type_pack::follow(tp)).is_some();

      wrap &= !is_empty(tp);

      if wrap {
        self.st().emit("(");
      }

      self.stringify_type_pack_id(tp);

      if wrap {
        self.st().emit(")");
      }
    }

    if !types.is_empty() || !type_packs.is_empty() {
      self.st().emit(">");
    }
  }

  /// C++ `void TypeStringifier::stringify(TypePackId tp)`.
  pub(crate) fn stringify_type_pack_id(&mut self, tp: TypePackId) {
    let mut tps = TypePackStringifier::type_pack_stringifier_stringifier_state(self.state);
    tps.stringify_type_pack_id(tp);
  }

  /// C++ `void TypeStringifier::stringify(TypePackId tpid, const std::vector<std::optional<FunctionArgument>>& names)`
  /// (cpp `Analysis/src/ToString.cpp:1453`).
  ///
  /// 降 safe：本函数只把构造期接线的 `self.state` 裸句柄与 `tpid` arena 句柄转交
  /// `TypePackStringifier`（与同文件已 safe 的 `stringify_type_pack_id` 完全同形），
  /// 自身无任何解引用；`names` 为普通共享切片。
  pub fn stringify_type_pack_id_vector_optional_function_argument(
    &mut self,
    tpid: TypePackId,
    names: &[Option<FunctionArgument>],
  ) {
    let mut tps = TypePackStringifier::type_pack_stringifier_stringifier_state_vector_optional_function_argument(
            self.state, names,
        );
    tps.stringify_type_pack_id(tpid);
  }
}
