use crate::common::functions::get_or_create_atom::direct_slot_for_atom;

pub(crate) fn update_direct_slot(atom: i32, cachedslot: *mut u16) {
  if let Some(slot) = direct_slot_for_atom(atom) {
    // Safety: 测试并行运行下本资源由本用例独占、无共享与并发访问；`cachedslot` 在本用例作用域内取得/构造（&mut 再借用、Box::into_raw 或 as_ptr 布线），至本行使用前不释放，故满足被调 unsafe 例程与 C ABI 的前置条件。
    unsafe {
      *cachedslot = slot as u16;
    }
  }
}
