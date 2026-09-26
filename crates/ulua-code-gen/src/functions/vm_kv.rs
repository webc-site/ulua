use ulua_vm::{records::closure::Closure, type_aliases::t_value::TValue};

/// 读取 Closure 常量表第 `i` 个 kv 常量（cpp vmKV）。
///
/// 由 execute 系列共享：SetTableKS / NameCall / GetTableKS 三处指令实现同体复制。
/// # Safety
/// `cl`/`k` 必须有效且指向存活对象，`i` 为字节码 AUX 常量下标且界内。
#[inline]
pub(crate) unsafe fn vm_kv(i: u32, cl: *mut Closure, k: *mut TValue) -> *mut TValue {
  // Safety: 契约保证 cl 为存活 Closure，其 inner.l.p 在构造时接线为非空 Proto*
  // （Lua 闭包不变量，比持有者长寿），故 `(*cl).inner.l.p` 解引用安全；LUAU_ASSERT
  // 保证 i<sizek，k 为 execute 调用点传入的该 proto 常量表基址，TValue 对齐一致，
  // 故 k.add(i) 落在界内常量槽上。
  unsafe {
    let p = {
      let l = &(*cl).inner.l;
      l.p
    };
    ulua_common::LUAU_ASSERT!(i < (*p).sizek as u32);
    k.add(i as usize)
  }
}
