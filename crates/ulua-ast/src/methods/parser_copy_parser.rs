use crate::records::{
  ast_array::{AstArray, AstArrayBuilder},
  node_handle::Nodes,
  parser::Parser,
  temp_vector::TempVector,
};

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
  /// cpp `copy(const T* value, size_t size)` 的切片形态内核：定长拷贝 arena 数组
  /// 的唯一构造点（原 `copy_t_usize` 的「指针 + 计数」入参已随唯一调用链折叠为
  /// `&[T]`，指针与长度不再可漂移）。
  ///
  /// 空切片即 cpp `AstArray{}`：恒 `{null, 0}` 形态（[`AstArray::EMPTY`] 单源），
  /// 不为零长区间向 arena 申请槽块。槽位申请与写入收口在 [`AstArrayBuilder`]
  /// 契约边界（容量恒等于元素数，逐槽 clone+write 与旧 `copy_nonoverlapping`
  /// 位拷贝等价——本内核实例化 T 均为指针/Position/POD 元素），本调用点无 unsafe。
  pub fn copy_initializer_list_t<T: Clone>(&mut self, data: &[T]) -> AstArray<T> {
    if data.is_empty() {
      return AstArray::EMPTY;
    }
    let mut slots = AstArrayBuilder::new(self.arena(), data.len());
    for value in data {
      slots.push(value.clone());
    }
    slots.finish()
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
