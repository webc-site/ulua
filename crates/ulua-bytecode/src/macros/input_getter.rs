/// 定义 `BytecodeGraphSerializer` 的「VM 常量输入」同形 getter：
/// 经 `get_vm_const_input_raw` 取常量 id 绑定为 `$val` → 命中 `$over` 越界条件时置
/// `self.error` → 返回 `$conv` 的换算结果。`$val` 由调用点命名，使 `$over`/`$conv`
/// 能引用同一局部绑定（规避 `macro_rules` 对宏体内局部名的卫生隔离）。
/// 用 `$crate::` 全限定路径引用类型，令展开结果不依赖调用点的 `use`。
macro_rules! define_input_getter {
  (
    $(#[$attr:meta])*
    $name:ident -> $ret:ty {
      $val:ident if $over:expr;
      $conv:expr
    }
  ) => {
    impl<'a, 'b, 'f>
      $crate::records::bytecode_graph_serializer::BytecodeGraphSerializer<'a, 'b, 'f>
    {
      $(#[$attr])*
      pub(crate) fn $name(
        &mut self,
        insn_op: $crate::records::bc_op::BcOp,
        index: u8,
      ) -> $ret {
        let $val: u32 = self.get_vm_const_input_raw(insn_op, index);
        if $over {
          self.error = true;
        }
        $conv
      }
    }
  };
}

pub(crate) use define_input_getter;
