//! 替换器宿主「入口对」的收口单点。
//!
//! cpp 里每个 `Substitution` 子类（Replacer/Anyification/Instantiation/…）的
//! 公有入口都是同一句式：先 `installSubstitutionVtable()`（把 `this` 与本类的
//! 回调组写进基类虚表），再调基类 `substitute` 主流程。Rust 直译后这层「装表 →
//! 委托」在 8 个宿主的 `impl` 里各抄一份逐字相同的双入口（15 个方法），收口为
//! 本宏：在宿主 `impl` 块内展开同形方法，行为（安装时序、委托目标）与手写版
//! 逐字等价。
//!
//! `(id)` 短臂只生成 `substitute_type_id`（供无 pack 入口的宿主，如
//! `RefineTypeScrubber`），避免生成无人调用的死入口。

/// 见模块文档。展开点需将 `TypeId`（pack 臂另需 `TypePackId`）置于作用域，
/// 且宿主自身已有 `install_substitution_vtable` 方法与 `base: Substitution` 字段。
macro_rules! substitution_entry {
  (id, pack) => {
    pub fn substitute_type_id(&mut self, ty: TypeId) -> Option<TypeId> {
      self.install_substitution_vtable();
      self.base.substitute_type_id(ty)
    }

    pub fn substitute_type_pack_id(&mut self, tp: TypePackId) -> Option<TypePackId> {
      self.install_substitution_vtable();
      self.base.substitute_type_pack_id(tp)
    }
  };
  (id) => {
    pub fn substitute_type_id(&mut self, ty: TypeId) -> Option<TypeId> {
      self.install_substitution_vtable();
      self.base.substitute_type_id(ty)
    }
  };
}

pub(crate) use substitution_entry;
