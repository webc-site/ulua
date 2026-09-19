use core::{ffi::c_char, mem::size_of, ptr::read_unaligned};

/// 定宽读取（cpp `lvmload.cpp:126-132` `read<T>`）：读不出 `size_of::<T>()` 字节
/// 即失败。
///
/// cpp 只有 `LUAU_ASSERT(size >= offset + sizeof(T))`，release 编译掉后损坏流就是
/// 越界读；Rust 侧不能以 panic 收口（`luau_load` 会把 panic 误判成成功计数），
/// 故返回 `Option`：越界（含 `offset + sizeof(T)` 自身回绕）时返回 `None` 且
/// **不推进** `offset`，由 `loadsafe` 统一转成「损坏字节码」错误。
///
/// # Safety
/// `data`/`size` 必须描述一段可读字节缓冲；`size == 0` 时 `data` 可为 null，
/// 因为越界判定先于任何解引用完成，null 永不被解引用。
pub(crate) unsafe fn read<T: Copy>(
  data: *const c_char,
  size: usize,
  offset: &mut usize,
) -> Option<T> {
  // checked_add 兼顾 offset 自身回绕，否则 ptr::add 会算出界外地址
  let end = offset.checked_add(size_of::<T>())?;

  if end > size {
    return None;
  }

  // SAFETY：上面已确认 `[offset, end)` 完整落在缓冲内；read_unaligned 不要求对齐。
  let result = unsafe { read_unaligned(data.add(*offset) as *const T) };
  *offset = end;

  Some(result)
}
