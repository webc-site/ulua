//! VM 帧支撑族：`VmFrame` 访问器生成宏与 call-fallback 产出码常量
//! （r7-macros98 合并票：逐字保真自原一文件一宏碎片）。
//!
//! 原文件内的 `vm_patch_c`/`vm_protect_pc` 两份 ulua-vm 运行时原子的同义克隆已
//! 单源化删除：请直接消费 `ulua_vm::macros::vm_patch_c::vm_patch_c` 与
//! `ulua_vm::macros::vm_protect_pc::vm_protect_pc`（r7-vmarch 仲裁票，勿再复制）。

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

pub const CALL_FALLBACK_YIELD: i32 = 1;
