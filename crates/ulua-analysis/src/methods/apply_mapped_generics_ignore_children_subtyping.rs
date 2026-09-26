use crate::{
  functions::{
    clone_clone::{pack_is_persistent, type_is_persistent},
    follow_type, get_type,
  },
  methods::subtyping_bind_generic::dense_hash_map_find_no_default,
  records::{
    apply_mapped_generics::ApplyMappedGenerics, extern_type::ExternType,
    function_type::FunctionType,
  },
  type_aliases::{type_id::TypeId, type_pack_id::TypePackId},
};

impl ApplyMappedGenerics {
  /// 泛型实参带 mapped-generics 界（或 extern/持久）的类型不向下遍历子节点。
  /// 对应 cpp `ApplyMappedGenerics`（`Analysis/src/Subtyping.cpp:395` 子类，
  /// `ignoreChildrenVisit` 语义即 `Tarjan::ignoreChildrenVisit`
  /// `Substitution.cpp:561` 的定制覆写）。
  ///
  /// 降 safe 说明：`ty` 为 arena `TypeId` 句柄（同 `get_type_id` 门面纪律），
  /// 对 `self.env`（构造期 `NotNull<SubtypingEnvironment>` 接线、比本对象长寿）
  /// 与 `(*ty).persistent` 的解引用收进窄 `unsafe` 块。
  pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
    // Safety: self.env 构造期以 NotNull{&env} 接线，非空且比本次替换长寿；
    // 只读查询 mapped_generics 帧，无并存可变借用。
    let env = unsafe { &*self.env };
    if get_type::get::<ExternType>(ty).is_some() {
      return true;
    }
    if let Some(f) = get_type::get::<FunctionType>(ty) {
      for &g in &f.generics {
        let g = follow_type::follow(g);
        if let Some(bounds) = dense_hash_map_find_no_default(&env.mapped_generics, &g)
          && !bounds.is_empty()
        {
          return true;
        }
      }
    }
    // type_is_persistent 探针（clone_clone）内部窄块已证成 arena 句柄前提。
    type_is_persistent(ty)
  }

  /// 类型包孪生版；降 safe 理由同上。cpp 锚点 `Substitution.cpp:566`。
  pub fn ignore_children_type_pack_id(&mut self, ty: TypePackId) -> bool {
    // 同上：pack_is_persistent 探针收口。
    pack_is_persistent(ty)
  }
}
