use crate::records::arena_handle::Handle;

/// RAII 递归深度守卫：构造即 `++`、`drop` 即 `--`。计数器以 [`Handle`]
/// 别名持有（目标为宿主的 `i32` 字段，存活期覆盖守卫生命周期，单线程串行），
/// 因此守卫不占用借用系统资源，可在 `&mut self` 方法调用之间自由存续——
/// 这正是 C++ `int*` 成员模型的借用语义。
#[derive(Debug)]
pub struct RecursionCounter {
  pub(crate) count: Handle<i32>,
}
