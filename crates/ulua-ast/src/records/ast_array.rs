use core::{
  ops::{Deref, DerefMut, Index, IndexMut},
  ptr::null_mut,
  slice::{Iter, IterMut, SliceIndex},
  str::{Utf8Error, from_utf8},
};

use ulua_common::functions::c_slice::{c_slice, c_slice_mut};

/// cpp `AstArray<T>{ T* data; size_t size; }` 的逐位镜像：arena 定长视图。
///
/// 不变量（由 `Parser::copy_*` / `AstArray::EMPTY` 两个唯一写入源兑现）：
/// **`data == null` 恒配 `size == 0`**；`data` 非空时 `[data, data + size)` 是 arena 内
/// 连续 `size` 个 `T` 的合法区域（`size` 可为 0，例如指向空字符串名缓冲的 `{非空, 0}`）。
/// bump arena 从不移动已分配块，故区域在持有本视图的节点存活期内稳定。
///
/// 为什么这里保留裸指针而不收 `Option<NonNull<T>>`：`data` 的空与非空不是独立的可空
/// 语义，而是与 `size` 成对的「区域描述」——`Option<NonNull>` 只能表达指针一边的信息，
/// `size` 仍需另立不变式，收口后仍要 `as_slice` 处的 `// Safety:`；而 `#[repr(C)]` 布局
/// 与 `data` 字段读点散布 analysis / compiler / unit-test（远超 10 文件），改动牵连面
/// 大于收益。故本轮只把**构造端**收敛到 [`AstArray::EMPTY`] 单源，并把上式不变量写成
/// 可核对的契约。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstArray<T> {
  pub data: *mut T,
  pub size: usize,
}

impl<T> AstArray<T> {
  /// 空数组（`{null, 0}`，cpp `AstArray<T>{}`）的唯一构造点：所有「无元素」形态都必须
  /// 由此产生，杜绝手写字面量时把 `null` 与 `size > 0` 配成自相矛盾的区域。
  pub const EMPTY: Self = Self {
    data: null_mut(),
    size: 0,
  };

  /// 从切片构造 AstArray 视图（仅借用，若切片为空则返回 [`AstArray::EMPTY`]）。
  #[inline(always)]
  pub fn from_slice(slice: &[T]) -> Self {
    if slice.is_empty() {
      Self::EMPTY
    } else {
      Self {
        data: slice.as_ptr() as *mut T,
        size: slice.len(),
      }
    }
  }

  /// 元素切片，对应 cpp `AstArray` 在 `[data, data + size)` 上的 `begin()/end()`。
  ///
  /// 前置条件即类型不变量：`data`/`size` 由 `Parser::copy_*`（arena `allocate` 成对
  /// 写入，块永不移动）或 [`AstArray::EMPTY`]（null 恒配 0）给出；构造端不再存在只写
  /// 单边字段的路径，故 `c_slice` 的「非空 + 对齐 + 区间可读」三条件成立。
  #[inline(always)]
  pub fn as_slice(&self) -> &[T] {
    // Safety: 见函数级前置条件——区域由 arena 成对写入，或为 EMPTY 的 null 配 0。
    unsafe { c_slice(self.data, self.size) }
  }

  /// 元素可变切片，对应 cpp `AstArray` 在 `[data, data + size)` 上的可变切片借用。
  #[inline(always)]
  pub fn as_mut_slice(&mut self) -> &mut [T] {
    // Safety: 见类型不变量——区域由 arena 成对写入，或为 EMPTY 的 null 配 0；
    // self 是独占引用 &mut self，且 arena 内节点在 AST 生命周期内有效。
    unsafe { c_slice_mut(self.data, self.size) }
  }

  #[inline(always)]
  pub fn iter(&self) -> Iter<'_, T> {
    self.as_slice().iter()
  }

  #[inline(always)]
  pub fn iter_mut(&mut self) -> IterMut<'_, T> {
    self.as_mut_slice().iter_mut()
  }

  #[inline(always)]
  pub fn len(&self) -> usize {
    self.size
  }

  #[inline(always)]
  pub fn is_empty(&self) -> bool {
    self.size == 0
  }
}

impl<T> AstArray<*mut T> {
  /// 节点指针数组的**只读**遍历：元素裸指针逐个解引用为 `&T`
  /// （cpp `for (auto o : a->generics)` 读取形态的安全对应）。
  ///
  /// 这里刻意不产出 `&mut T`：`&self` 的共享借用下造可变引用是 noalias UB。
  /// 真正需要写穿节点的调用点请直接用元素指针（`iter()` 给出 `&*mut T`），
  /// 在自带的 `// Safety:` 注释下写入，或走 `rtti::ast_node_try_as_mut`。
  #[inline]
  pub fn iter_nodes(&self) -> impl Iterator<Item = &T> {
    self.as_slice().iter().map(|&p|
      // Safety: 元素由 arena 写入，指向存活节点；只取共享引用，不写。
      unsafe { &*p })
  }
}

/// 字符串字节数组（批 2 存储面）：cpp `AstArray<char>`（Ast.h:411 等）→ `AstArray<u8>`，
/// 同一字节域；`as_bytes`/`as_str`/`contains_null` 门面签名与语义零变化。
impl AstArray<u8> {
  /// Returns the backing byte buffer as a `&[u8]` slice.
  #[inline]
  pub fn as_bytes(&self) -> &[u8] {
    self.as_slice()
  }

  /// Tries to convert the backing byte buffer to a UTF-8 `&str`.
  #[inline]
  pub fn as_str(&self) -> Result<&str, Utf8Error> {
    from_utf8(self.as_bytes())
  }

  /// 是否含 NUL 字节：AstName 承载的 `char*` 不能指向 NUL（C++
  /// `strchr(..., 0)` 前置判定），原 parser 两处手动循环合并于此。
  #[inline]
  pub fn contains_null(&self) -> bool {
    self.as_bytes().contains(&0)
  }
}

impl<T> Deref for AstArray<T> {
  type Target = [T];

  #[inline(always)]
  fn deref(&self) -> &Self::Target {
    self.as_slice()
  }
}

impl<T> DerefMut for AstArray<T> {
  #[inline(always)]
  fn deref_mut(&mut self) -> &mut Self::Target {
    self.as_mut_slice()
  }
}

impl<T, I> Index<I> for AstArray<T>
where
  I: SliceIndex<[T]>,
{
  type Output = I::Output;

  #[inline(always)]
  fn index(&self, index: I) -> &Self::Output {
    &self.as_slice()[index]
  }
}

impl<T, I> IndexMut<I> for AstArray<T>
where
  I: SliceIndex<[T]>,
{
  #[inline(always)]
  fn index_mut(&mut self, index: I) -> &mut Self::Output {
    &mut self.as_mut_slice()[index]
  }
}

impl<'a, T> From<&'a [T]> for AstArray<T> {
  #[inline(always)]
  fn from(slice: &'a [T]) -> Self {
    Self::from_slice(slice)
  }
}

impl<'a, T> IntoIterator for &'a AstArray<T> {
  type Item = &'a T;
  type IntoIter = Iter<'a, T>;

  #[inline(always)]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T> IntoIterator for &'a mut AstArray<T> {
  type Item = &'a mut T;
  type IntoIter = IterMut<'a, T>;

  #[inline(always)]
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

// An empty array (`{nullptr, 0}`) for every `T`, mirroring C++ `AstArray<T>{}`.
// Manual (not derived) so it does not require `T: Default`.
impl<T> Default for AstArray<T> {
  fn default() -> Self {
    AstArray::EMPTY
  }
}
