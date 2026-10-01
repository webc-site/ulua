use core::{
  mem::size_of,
  ops::{Deref, DerefMut, Index, IndexMut},
  ptr::{copy_nonoverlapping, null_mut},
  slice::{Iter, IterMut, SliceIndex},
  str::{Utf8Error, from_utf8},
};

use ulua_common::functions::c_slice::{c_slice, c_slice_mut};

use crate::records::{allocator::Allocator, ast_name::AstName};

/// cpp `AstArray<T>{ T* data; size_t size; }` 的逐位镜像：arena 定长视图。
///
/// 不变量（由 [`AstArrayBuilder`]（arena 槽填充）、`AstArray::EMPTY`、
/// `AstArray::from_slice`（借用视图）与 `Parser::copy_*`（C 拷贝内核）四个唯一
/// 写入源兑现）：**`data == null` 恒配 `size == 0`**；`data` 非空时
/// `[data, data + size)` 是 arena 内连续 `size` 个 `T` 的合法区域（`size` 可为 0，
/// 例如指向空字符串名缓冲的 `{非空, 0}`）。bump arena 从不移动已分配块，故区域
/// 在持有本视图的节点存活期内稳定。
///
/// 为什么这里保留裸指针而不收 `Option<NonNull<T>>`（DELIBERATE DEVIATION / 保留理由，
/// 数字为本仓实测，非估测）：`data` 的空与非空不是独立的可空语义，而是与 `size` 成对
/// 的「区域描述」——`Option<NonNull>` 只表达指针一边，`size` 仍需另立「null 恒配 0」
/// 不变式，收口后 `as_slice` 处照样要 `// Safety:` 论证区间可读。
///
/// 换型落点实测（把 `data` 改为 `Option<NonNull<T>>` 后 `cargo check -p ulua-ast
/// -p ulua-compiler -p ulua-analysis -p ulua-unit-test -p ulua-code-gen
/// --all-targets`）：除本文件内 `from_slice`/`finish`/`finish_with`/`from_ast_name`
/// 四个构造点外，报错落在清单外 6 个文件 8 处——`methods::parser_copy_parser::
/// copy_t_usize`（arena 分配结果直填 `data`，1 处）、`ulua-ast/tests/ast_array.rs`
/// （字面量构造，1 处）、跨 crate 的 `ulua-compiler` 四处：
/// `functions::sref_compiler`（`data.data.is_null()` 与 `from_raw_parts(data.data,
/// ..)`，2 处）、`records::constant`（2 处）、`records::constant_visitor`（1 处）、
/// `records::compiler::expr_call`（1 处）。ulua-analysis、ulua-unit-test、
/// ulua-code-gen 全部只经 `as_slice`/`iter`/`Deref` 门面读取，零命中。
///
/// 结论：收益只是形态（读端本就没有散点解引用），代价却要同时改动本轮清单外的
/// ulua-ast parser 与 ulua-compiler 消费面，故维持 `*mut T` 的 `#[repr(C)]` ABI
/// 镜像，`data` 保持 `pub` 仅为该镜像服务。构造端已全部定形：ast / analysis 侧
/// 不再手写 `{data, size}` 字面量，一律经 [`AstArrayBuilder`] / [`AstArray::EMPTY`] /
/// [`AstArray::from_slice`]；读取端一律走切片门面，唯一的裸指针出口是 `as_ptr`
/// （AstName 以指针为 identity，需逐位保真原址）。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AstArray<T> {
  pub data: *mut T,
  pub size: usize,
}

impl<T> AstArray<T> {
  /// 空数组（`{null, 0}`，cpp `AstArray<T>{}`）的唯一构造点：所有「无元素」形态都必须
  /// 由此产生，杜绝手写字面量时把 `null` 与 `size > 0` 配成自相矛盾的区域。
  /// 这里是全类型唯一一处 `null_mut()`：`const` 上下文里它不能由 `NonNull` 造出
  /// （null 不是 `NonNull` 的合法值），`Option<NonNull<T>>` 形态下等价写法只能是
  /// `None`，与本常量承担的角色一致——见类型文档的保留理由。
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

  /// 区间首址（cpp `arr.data` 的读址门面，只取址不 bespoke 判空）。
  ///
  /// 与 `Deref` 链上的 [`slice::as_ptr`] 的差别仅在空区间：本方法返回存储
  /// 原指针（如 `copy_bytes("")` 的非空 NUL 缓冲址、EMPTY 的 null），不折算
  /// 成空切片的悬挂地址——`AstName` 这类以指针为 identity 的消费者需要
  /// 逐位保真的原址。
  #[inline(always)]
  pub fn as_ptr(&self) -> *const T {
    self.data
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

/// Arena 槽数组构造期缓冲：cpp「`allocate<T>(cap)` 先取容量槽 → 逐槽写入 →
/// `arr.size = n` 按实际写入数定形」三步的唯一收口。
///
/// 「向未初始化 arena 槽写入」的全部 unsafe 内聚在 [`AstArrayBuilder::push`] /
/// [`AstArrayBuilder::push_slice`]（契约见各自注释；容量界由 `assert` 兑现），
/// 调用点保持纯安全代码——review.md §2「把 unsafe 关进一个有契约的最小边界，
/// 不得让 unsafe 渗透到业务逻辑」的构造端落地。`new` 把 arena 借用折算成裸块
/// 地址后即归还，不持有 `&mut Allocator`，调用方可在填充期间继续向同一 arena
/// 分配（rehydration 递归 visit 的既有形态）。
///
/// 终态以实际写入数定形（[`AstArrayBuilder::finish`]），`finish_with(size)`
/// 允许把已初始化但逻辑上不含的尾槽（如 NUL 终止符）留在区间外；从此不存在
/// 「size 覆盖未初始化槽」的数组。
pub struct AstArrayBuilder<T> {
  /// `new` 时从 arena 申请、可容纳 `capacity` 个 `T` 的连续块首址（bump 分配
  /// 恒非空、块永不移动，存活期覆盖整个 AST）。
  data: *mut T,
  /// 申请的槽容量；`push` 次数不得越过（越界即 `assert` 拦截，对应 cpp 侧
  /// 「写入次数恒 ≤ allocate 容量」的手写记账）。
  capacity: usize,
  /// 已完成初始化的槽数。
  len: usize,
}

impl<T> AstArrayBuilder<T> {
  /// 向 `allocator` 申请 `capacity` 个 `T` 的连续槽（即使 `capacity == 0` 也
  /// 走一次 `allocate`，与 cpp `allocate(0)` 的占位形态逐位一致）。
  #[inline]
  pub fn new(allocator: &mut Allocator, capacity: usize) -> Self {
    Self {
      data: allocator.allocate(size_of::<T>() * capacity).cast::<T>(),
      capacity,
      len: 0,
    }
  }

  /// 追加一槽。
  ///
  /// 契约（由既有调用点的不变式兑现，越界即 panic 拦截而非静默写穿 arena）：
  /// `push` 总次数 ≤ `new` 的 `capacity`——调用点按「同一集合决定容量与写入
  /// 次数」的 cpp 记账推导（如 props 每项恰占一槽、generics 每项至多占一槽）。
  #[inline]
  pub fn push(&mut self, value: T) {
    assert!(
      self.len < self.capacity,
      "AstArray 槽越界：push 次数越过容量"
    );
    // Safety: 上一行 assert 保证 data.add(len) 在申请的 capacity 槽块内；
    // 槽位来自 arena 的新鲜分配、从未初始化，write 恰一次即完成初始化，
    // 不读旧值、无 double-drop。
    unsafe { self.data.add(self.len).write(value) };
    self.len += 1;
  }

  /// 追加一段（`copy_from_slice` 语义：整段 memcpy、单次界检）。
  pub fn push_slice(&mut self, values: &[T])
  where
    T: Copy,
  {
    assert!(
      self.len + values.len() <= self.capacity,
      "AstArray 槽越界：push_slice 越过容量"
    );
    // Safety: 上一行 assert 保证 [len, len+values.len()) 在申请的容量块内；
    // 目标是 arena 新鲜分配、与任何既有切片不重叠，Copy 元素按位拷贝即完成
    // 初始化。
    unsafe { copy_nonoverlapping(values.as_ptr(), self.data.add(self.len), values.len()) };
    self.len += values.len();
  }

  /// 已初始化槽数（= [`AstArrayBuilder::finish`] 将写入 `size` 的值）。
  #[inline]
  pub fn len(&self) -> usize {
    self.len
  }

  /// 已初始化槽是否为空。
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.len == 0
  }

  /// 以全部已初始化槽定形。
  #[inline]
  pub fn finish(self) -> AstArray<T> {
    let Self { data, len, .. } = self;
    AstArray { data, size: len }
  }

  /// 以前 `size` 个已初始化槽定形（`size ≤ len`；余量已初始化但落在区间外，
  /// 供 `Parser::copy_bytes` 的「NUL 驻留块尾、逻辑长度不含 NUL」形态）。
  #[inline]
  pub fn finish_with(self, size: usize) -> AstArray<T> {
    assert!(size <= self.len, "AstArray 定形越界：size 超过已初始化槽数");
    AstArray {
      data: self.data,
      size,
    }
  }
}

/// 字符串字节数组（批 2 存储面）：cpp `AstArray<char>`（Ast.h:411 等）→ `AstArray<u8>`，
/// 同一字节域；`as_bytes`/`as_str`/`contains_null` 门面签名与语义零变化。
impl AstArray<u8> {
  /// 从 [`AstName`] 的字节域构造借用视图（cpp
  /// `AstArray<char>(name.name.data, name.name.size)`，parser 表字段名直切片）。
  /// 字节域归名表 intern 缓冲所有（NUL 结尾契约见 `AstName`），`data` 保留
  /// name 指针原值——AstName 以指针为 identity，取址经身份桥透传、不折算。
  #[inline]
  pub fn from_ast_name(name: &AstName) -> Self {
    Self {
      data: name.as_ptr().cast_mut(),
      size: name.len(),
    }
  }

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
  /// `strchr(..., 0)` 前置判定），原 parser 两处手动循环合并于此；NUL 扫描交
  /// memchr（SIMD）。
  #[inline]
  pub fn contains_null(&self) -> bool {
    memchr::memchr(0, self.as_bytes()).is_some()
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
