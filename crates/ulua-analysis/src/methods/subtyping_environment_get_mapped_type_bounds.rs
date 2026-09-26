use ulua_common::LUAU_ASSERT;

use crate::{
  functions::follow_type::follow,
  methods::subtyping_bind_generic::dense_hash_map_find_mut_no_default,
  records::{
    generic_bounds::GenericBounds, internal_error_reporter::InternalErrorReporter,
    subtyping_environment::SubtypingEnvironment,
  },
  type_aliases::type_id::TypeId,
};
impl SubtypingEnvironment {
  /// # Safety
  /// - `self.parent` 及其父链为构造期接线的 `*mut SubtypingEnvironment` 环境层级裸指针：
  ///   可空（空则不递归），非空者指向比本环境长寿的外层作用域环境。
  /// - `ice_reporter` 必须非 null 且指向存活的 `InternalErrorReporter`——对应 C++
  ///   `getMappedTypeBounds(TypeId, InternalErrorReporter&)` 的引用形参，调用方从不传 null。
  /// - `ty` 经 `follow` 解引用，须指向类型 arena 中存活节点。
  pub unsafe fn get_mapped_type_bounds(
    &mut self,
    ty: TypeId,
    ice_reporter: *mut InternalErrorReporter,
  ) -> &mut GenericBounds {
    let ty = follow(ty);
    if let Some(bounds) = dense_hash_map_find_mut_no_default(&mut self.mapped_generics, &ty)
      && !bounds.is_empty()
    {
      // 链上 `!bounds.is_empty()` 蕴含 last_mut() 命中 Some。
      return bounds.last_mut().expect("链上 !bounds.is_empty() 蕴含非空");
    }

    if !self.parent.is_null() {
      // Safety: 上方 `!self.parent.is_null()` 已保证父环境句柄非空；它是构造期接线的
      // 环境父链裸指针，指向比 `&mut self` 长寿的外层环境，故递归返回的 `&mut GenericBounds`
      // 在此存活有效。递归调用的 unsafe fn 契约（父链/`ice_reporter`/`ty`）与原调用一致。
      return unsafe { (*self.parent).get_mapped_type_bounds(ty, ice_reporter) };
    }

    LUAU_ASSERT!(false);
    // Safety: `ice_reporter` 依函数级契约非 null 且指向存活 `InternalErrorReporter`（C++
    // 侧为引用形参，恒非空）；`ice_string` 只上报一条诊断消息并 panic 发散，不产生并存别名。
    unsafe {
      (*ice_reporter).ice_string("Trying to access bounds for a type with no in-scope bounds");
    }
    unreachable!()
  }
}
