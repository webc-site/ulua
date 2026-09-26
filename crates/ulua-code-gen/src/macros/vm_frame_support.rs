//! VM 帧支撑族：`VmFrame` 访问器生成宏、pc 保护/补丁转发与 call-fallback
//! 产出码常量（r7-macros98 合并票：逐字保真自原一文件一宏碎片）。

use ulua_vm::{
  macros::vm_protect_pc::vm_protect_pc as VM_PROTECT_PC_VM, records::lua_state::LuaState,
};

use crate::type_aliases::instruction_ir_builder::Instruction;

/// 收口 `VmFrame` 的「严格同形单行访问器」族：签名为 `&self` + 单个逐位类型化形参、
/// 函数体恰为一条 `unsafe { $body }`（纯字段读或谓词/取值宏转发）、成员间只差字段名/
/// 返回类型的同形壳。`#[inline]` 与 `pub(crate)` 由宏体统一携带，展开为裸 fn 条目、须在
/// `impl VmFrame` 块内调用（同 `shared_ptr_accessors!` 先例），语义与手写体逐位一致。
/// 卫生沿用 bytecode 侧 `define_input_getter!`（abs-r122）先例：`$arg`/`$argty`/`$ret`/
/// `$body` 全取调用点拼写，同处调用点 hygiene 上下文，宏体不引任何路径；文档经属性槽
/// 承接、写在宏块内（规避 unused_doc_comments），`// Safety` 注释逐条
/// 保留于条目上方。可空指针读壳（null 哨兵折叠为 `Option`，review.md §2）由 `opt` 臂
/// 同形收口；其余带判空逻辑/类型转换/指针算术/惰性初始化或含 `let l = self.l` 前戏的
/// 非同形臂一律手写保留。`lget`/`lset`/`ciget`/`ciset` 臂收口 `L`（及 `L->ci`）上普通
/// 字段的零参读/单参写壳（`self.l` 由宏体硬编码，规避调用点传 `self` 的卫生歧义）。
macro_rules! define_vm_frame_accessor {
  (
    $(
      $(#[$attr:meta])*
      lget $field:ident as $name:ident -> $ret:ty;
    )+
  ) => {
    $(
      $(#[$attr])*
      #[inline]
      pub(crate) fn $name(&self) -> $ret {
        unsafe { (*self.l).$field }
      }
    )+
  };
  (
    $(
      $(#[$attr:meta])*
      opt $name:ident($arg:ident : $argty:ty) -> $ret:ty = $body:expr;
    )+
  ) => {
    $(
      $(#[$attr])*
      #[inline]
      pub(crate) fn $name(&self, $arg: $argty) -> Option<$ret> {
        // 空哨兵→None 的折叠契约由本臂统一携带；条目 `// Safety` 论证 $body 存活性。
        let p = unsafe { $body };
        (!p.is_null()).then_some(p)
      }
    )+
  };
  (
    $(
      $(#[$attr:meta])*
      lset $field:ident as $name:ident($arg:ident : $argty:ty);
    )+
  ) => {
    $(
      $(#[$attr])*
      #[inline]
      pub(crate) fn $name(&self, $arg: $argty) {
        unsafe { (*self.l).$field = $arg };
      }
    )+
  };
  (
    $(
      $(#[$attr:meta])*
      ciget $field:ident as $name:ident -> $ret:ty;
    )+
  ) => {
    $(
      $(#[$attr])*
      #[inline]
      pub(crate) fn $name(&self) -> $ret {
        unsafe { (*(*self.l).ci).$field }
      }
    )+
  };
  (
    $(
      $(#[$attr:meta])*
      ciset $field:ident as $name:ident($arg:ident : $argty:ty);
    )+
  ) => {
    $(
      $(#[$attr])*
      #[inline]
      pub(crate) fn $name(&self, $arg: $argty) {
        unsafe { (*(*self.l).ci).$field = $arg };
      }
    )+
  };
  (
    $(
      $(#[$attr:meta])*
      $name:ident($arg:ident : $argty:ty) -> $ret:ty = $body:expr;
    )+
  ) => {
    $(
      $(#[$attr])*
      #[inline]
      pub(crate) fn $name(&self, $arg: $argty) -> $ret {
        unsafe { $body }
      }
    )+
  };
}

pub(crate) use define_vm_frame_accessor;

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn vm_patch_c(pc: *const Instruction, slot: i32) {
  // Safety: 契约保证 pc 指向字节码缓冲内存活的合法 Instruction 槽，转 *mut 仅就地
  // 改写当前槽；读写同为 Instruction(u32) 对齐一致，单线程串行 patch 无别名冲突。
  unsafe {
    *(pc as *mut Instruction) = ((slot as u8 as u32) << 24) | (0x00ffffffu32 & *pc);
  }
}


/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn vm_protect_pc(l: *mut LuaState, pc: *const u32) {
  // Safety: 本 unsafe fn 的 `# Safety` 契约保证 l 为存活 LuaState*、pc 为界内指令指针，
  // 与被转发的 VM_PROTECT_PC_VM 前置条件完全一致，转发既未收紧也未放宽该契约。
  unsafe {
    VM_PROTECT_PC_VM(l, pc);
  }
}

pub const CALL_FALLBACK_YIELD: i32 = 1;
