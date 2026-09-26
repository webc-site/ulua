use core::{mem::size_of, ptr::copy_nonoverlapping};

use crate::records::{
  allocator::Allocator, ast_array::AstArray, node_handle::Nodes, parser::Parser,
  temp_vector::TempVector,
};

impl Parser {
  pub(crate) fn copy_t_usize<T: Clone>(&mut self, data: *const T, size: usize) -> AstArray<T> {
    // 空区间即 cpp `AstArray{}`：恒 `{null, 0}` 形态（`AstArray::EMPTY` 单源）。源指针为
    // null 而 size>0 不是合法入参（`as_slice` 会用 null 造非空切片），故一并折叠成 EMPTY，
    // 不再产出「null 配非零 size」这种自相矛盾的数组。
    if size == 0 || data.is_null() {
      return AstArray::EMPTY;
    }

    // Safety: &mut *self.allocator：allocator 是 Parser 构造时给出的非空长寿 arena 指针；allocate 返回恒非空（失败 handle_alloc_error 中止）且 8 对齐、容量 size_of::<T>()*size 精确（本函数实例化 T 均为指针/Position 等 align≤8 的 POD 元素）；入口已挡 size==0 与空源指针，data/size 按 cpp Parser::copy 契约 size>0 时源可读；新鲜目标与源必不重叠，位拷贝与逐元素 clone+write 等价；arena 块永不移动，result.data 存活。
    let storage = unsafe {
      let storage = Allocator::allocate(&mut *self.allocator, size_of::<T>() * size).cast::<T>();

      // Allocator 返回的存储是新鲜分配，与源区间必不重叠：整段 memcpy
      // 一次到位，省掉逐元素 `write(add(i), clone())` 的循环与逐元素
      // Clone 分发。位拷贝语义与逐个 clone 后再 write 等价（此处 T 均为
      // 指针/POD 数组元素，无自引用不变式）。
      copy_nonoverlapping(data, storage, size);

      storage
    };

    AstArray {
      data: storage,
      size,
    }
  }
}

impl Parser {
  pub fn copy_temp_vector_t<'a, T: Clone>(&mut self, data: &TempVector<'a, T>) -> AstArray<T> {
    // scratch 窗口的读取统一经 `TempVector::as_slice`（区间 `[offset, offset+size_)`
    // 的判界与不变式论证已收口在该门面），此处只把切片交给初始化列表内核：
    // 空窗口（size_ == 0）与空切片同为 `AstArray::EMPTY`，与旧「先判 size_ 再
    // 裸偏移取指针」逐位等价，本调用点不再开 unsafe。
    self.copy_initializer_list_t(data.as_slice())
  }
}

impl Parser {
  pub fn copy_initializer_list_t<T: Clone>(&mut self, data: &[T]) -> AstArray<T> {
    // 空切片的 `as_ptr()` 是齐址悬挂指针（非 null），但 `copy_t_usize` 先按 size==0 早退
    // 并直接回 `AstArray::EMPTY`，从不解引用，故无需在此再造一个 null 分支。
    self.copy_t_usize(data.as_ptr(), data.len())
  }
}

impl Parser {
  /// scratch（TempVector 里的 arena 节点槽）→ 类型化子节点数组句柄。
  /// 主干 records 引用化（`records::node_handle`）的构造端单点：数组本体改堆
  /// 持有，元素仍是非空 `Node` 句柄，arena 存活契约不变。
  /// 实现收口到 [`Nodes::from_temp_vector`]（cpp `copyTempVector` 句柄化形态）。
  pub(crate) fn copy_temp_vector_nodes<T>(&mut self, data: &TempVector<'_, *mut T>) -> Nodes<T> {
    Nodes::from_temp_vector(data)
  }
}
