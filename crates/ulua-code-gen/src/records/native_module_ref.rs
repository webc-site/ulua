use core::{
  mem::swap,
  ptr::{NonNull, null},
};

use crate::records::native_module::NativeModule;

#[derive(Debug, Default)]
pub struct NativeModuleRef {
  pub(crate) native_module: Option<NonNull<NativeModule>>,
}

impl NativeModuleRef {
  /// 以裸地址构造引用并对所指模块加计数（`native_module` 为空即空 ref，等价 cpp 拷贝构造）。
  ///
  /// # Safety
  /// 非空时 `native_module` 必须指向存活 `NativeModule`；`add_ref` 只以 `&self` 做原子
  /// fetch_add，不产生可变借用。
  pub unsafe fn native_module_ref_native_module(native_module: *const NativeModule) -> Self {
    let native_module = NonNull::new(native_module.cast_mut());
    if let Some(native_module) = native_module {
      // Safety: 依本函数契约，非空 Some 分支的指针指向存活 NativeModule。
      unsafe { native_module.as_ref() }.native_module_add_ref();
    }

    Self { native_module }
  }

  /// 移动构造（cpp move-ctor）：掏空 `other` 的引用、不重复计数。
  pub fn native_module_ref_native_module_ref_mut(other: &mut NativeModuleRef) -> Self {
    Self {
      native_module: other.native_module.take(),
    }
  }

  pub fn native_module_ref_empty(&self) -> bool {
    self.native_module.is_none()
  }

  /// 所指模块的裸地址（空 ref 为 `null`）。供指针相等性比较与 VM 侧地址消费。
  pub fn native_module_ref_get(&self) -> *const NativeModule {
    self.native_module.map_or_else(null, |p| p.as_ptr())
  }

  /// 空判定与只读访问的安全收口：非空 ref 恒指向计数托管的存活模块。
  pub fn as_ref_option(&self) -> Option<&NativeModule> {
    // Safety: 构造不变量保证 Some 分支指针指向本 ref 引用计数托管的存活 NativeModule，
    // 共享借用与 release 的 &self 原子递减语义一致，无双轨可变别名。
    self.native_module.map(|p| unsafe { p.as_ref() })
  }

  pub fn native_module_ref_operator_assign(
    &mut self,
    mut other: NativeModuleRef,
  ) -> &mut NativeModuleRef {
    self.native_module_ref_swap(&mut other);
    self
  }

  pub fn native_module_ref_reset(&mut self) {
    if let Some(native_module) = self.native_module.take() {
      // Safety: 不变量保证 Some 分支指针指向仍在世的 NativeModule（本 ref 持有其一份引用），
      // take 前计数尚在，release() 以 &self 递减原子引用计数，无别名冲突。
      unsafe { native_module.as_ref() }.release();
    }
  }

  pub fn native_module_ref_swap(&mut self, other: &mut NativeModuleRef) {
    swap(&mut self.native_module, &mut other.native_module);
  }
}

impl Clone for NativeModuleRef {
  fn clone(&self) -> Self {
    if let Some(native_module) = self.native_module {
      // Safety: 构造不变量保证 Some 分支指针恒指向本 ref 引用计数托管的存活 NativeModule；
      // add_ref 只以 &self 做原子加计数，无双轨可变借用。
      unsafe { native_module.as_ref() }.native_module_add_ref();
    }

    Self {
      native_module: self.native_module,
    }
  }
}

impl Drop for NativeModuleRef {
  fn drop(&mut self) {
    self.native_module_ref_reset();
  }
}
