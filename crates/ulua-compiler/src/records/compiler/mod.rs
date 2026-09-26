use alloc::vec::Vec;
use core::{
  fmt::Arguments,
  ptr::{NonNull, null_mut},
};

use ulua_ast::records::{
  ast_expr::AstExpr, ast_expr_call::AstExprCall, ast_expr_function::AstExprFunction,
  ast_expr_table::AstExprTable, ast_local::AstLocal, ast_name::AstName,
  ast_name_table::AstNameTable, location::Location,
};
use ulua_bytecode::records::bytecode_builder::BytecodeBuilder;
use ulua_common::{
  enums::luau_bytecode_type::LuauBytecodeType, records::dense_hash_map::DenseHashMap,
};

use crate::{
  enums::{global::Global, table_constant_kind::TableConstantKind},
  records::{
    builtin_ast_types::BuiltinAstTypes, capture::Capture, compile_options::CompileOptions,
    constant::Constant, expr_constant_change::ExprConstantChange, function::Function,
    inline_frame::InlineFrame, local::Local, local_constant_change::LocalConstantChange,
    r#loop::Loop, loop_jump::LoopJump, node::Node, table_shape::TableShape, variable::Variable,
  },
};
/// AD 指令 16 位 D 域能编码的最大常量索引（C++ `0x8000`，超限走慢路径）
pub(crate) const K_MAX_AD_INDEX: i32 = 0x8000;

/// K 系指令（ANDK/ORK/ADDK…）u8 操作数可编码的常量索引上界（C++ `cid < 256`）
pub(crate) const K_MAX_K_CONST_INDEX: i32 = 0x100;

/// 目标寄存器数量上界哨兵（C++ `targetCount == 255` 报 "Exceeded result count limit"，
/// 各处 `LUAU_ASSERT(targetCount < 255)` 同源）
pub(crate) const K_MAX_TARGET_COUNT: u8 = 255;

/// import id 10 位域上限（C++ `1024`）
pub(crate) const K_MAX_IMPORT_ID: i32 = 1024;

/// GETIMPORT 辅助字的高位标志（C++ `0x80000000`，GETIMPORT flip）
pub(crate) const K_GETIMPORT_FLAG: u32 = 0x80000000;

/// 无效寄存器哨兵（C++ `kInvalidReg`，与 K_MAX_TARGET_COUNT 同源）
pub(crate) const K_INVALID_REG: u8 = 255;

/// push_local 的 allocpc 哨兵（C++ `kDefaultAllocPc`）：表示"取当前 debugpc"
pub(crate) const K_DEFAULT_ALLOC_PC: u32 = !0u32;

/// 寄存器总量上界（C++ `allocReg` 的 `kMaxRegisterCount`）
pub(crate) const K_MAX_REGISTER_COUNT: u32 = 255;

/// 编译期可变状态载体（对照 cpp `Compiler.h` 的 `class Compiler`）。
///
/// 本结构不跨出 ulua-compiler（对外仅经 `compile`/`compile_or_throw_*` 门面
/// 间接构造），故全部字段收为 `pub(crate)`，最小化 crate 表面。
#[derive(Debug)]
pub struct Compiler {
  /// cpp `Compiler.h` `BytecodeBuilder& bytecode` 的句柄化：契约是「builder 活得
  /// 比 `Compiler` 久」，由唯一构造点（`Compiler::new`）的 `&mut` 借用兑现。
  /// `'static` 只是让收口点签名接受 arena 视图（`sref_*` 构造的 `StringRef`，
  /// 生命周期由调用方选择），与上游 `string_view` 进 `BytecodeBuilder` 的约定
  /// 一致。解引用全经 `bc()`/`bc_mut()` 两个收口点，业务代码不触碰句柄本体。
  pub(crate) bytecode: NonNull<BytecodeBuilder<'static>>,
  pub(crate) options: CompileOptions,
  // 以下指针键 DenseHashMap 以 `DenseHashMap::default()` 构造——ulua-common
  // 指针键门面（`Default` 经 `DenseDefault` 取 null 占位，等价 cpp
  // `DenseHashMap<K*,V>(nullptr)` 惯用法的 Rust 镜像）。占用与否由位图判定，
  // null 占位键亦可存取（"哨兵可存取"契约见 `dense_hash_table` 模块文档）；
  // AST 节点地址恒非空，业务上 null 从不作为有效键查询。键型统一为
  // [`Node`] 地址句柄（`NonNull` 内核，无 null 哨兵可能）。
  //
  pub(crate) functions: DenseHashMap<Node<AstExprFunction>, Function>,
  pub(crate) locals: DenseHashMap<Node<AstLocal>, Local>,
  pub(crate) globals: DenseHashMap<AstName, Global>,
  pub(crate) variables: DenseHashMap<Node<AstLocal>, Variable>,
  pub(crate) constants: DenseHashMap<Node<AstExpr>, Constant>,
  pub(crate) locstants: DenseHashMap<Node<AstLocal>, Constant>,
  pub(crate) table_constants: DenseHashMap<Node<AstLocal>, TableConstantKind>,
  pub(crate) table_shapes: DenseHashMap<Node<AstExprTable>, TableShape>,
  pub(crate) builtins: DenseHashMap<Node<AstExprCall>, i32>,
  pub(crate) userdata_types: DenseHashMap<AstName, u8>,
  pub(crate) function_types: DenseHashMap<Node<AstExprFunction>, Vec<u8>>,
  pub(crate) local_types: DenseHashMap<Node<AstLocal>, LuauBytecodeType>,
  pub(crate) expr_types: DenseHashMap<Node<AstExpr>, LuauBytecodeType>,
  pub(crate) inline_builtins: DenseHashMap<Node<AstExprCall>, i32>,
  pub(crate) inline_builtins_backup: DenseHashMap<Node<AstExprCall>, i32>,
  pub(crate) expr_changes: Vec<ExprConstantChange>,
  pub(crate) local_changes: Vec<LocalConstantChange>,
  pub(crate) builtin_types: BuiltinAstTypes,
  /// cpp `Compiler.h` `AstNameTable& names` 引用成员的句柄化，契约与
  /// `bytecode` 同款：表由编译驱动方（`compile_or_throw_*` 的调用栈）持有
  /// 且活得比 `Compiler` 久，唯一构造点 `new` 由 `&mut` 借用提升。存句柄
  /// 而非引用的原因是借用拆分——编译期字符串驻留要单独 `&mut` 本表，
  /// 同时调用点还持有 `self` 其它字段的可变借用，借用检查器无法证明二者
  /// 不相交。解引用收口在 `names()`/`names_mut()` 两个访问器。
  pub(crate) names: NonNull<AstNameTable>,
  pub(crate) export_table_local: AstLocal,
  /// cpp `builtinsFold` 指针的 Rust 化：不存自指裸指针（会被后续 `&mut self.builtins`
  /// 使 provenance 失效），只记录折叠门控是否开启，`fold_constants` 时现取 `&self.builtins`。
  pub(crate) builtins_fold: bool,
  pub(crate) builtins_fold_library_k: bool,
  pub(crate) reg_top: u32,
  pub(crate) stack_size: u32,
  pub(crate) arg_count: usize,
  pub(crate) has_loops: bool,
  /// cpp `hasMultiRet`（Compiler.cpp:5530）：本函数是否发出过 multret RETURN。
  /// 由 `compile_stat_return` 置位、`compile_function` 收尾复位，
  /// 用于 `LPF_INLINABLE` 判定（cpp:613）。
  pub(crate) has_multi_ret: bool,
  /// cpp `AstExprFunction* currentFunction`：当前正在编译的函数体节点；
  /// 未处于任何函数体内时为 `None`（cpp null 哨兵 → Option）。
  pub(crate) current_function: Option<Node<AstExprFunction>>,
  pub(crate) block_depth: usize,
  pub(crate) getfenv_used: bool,
  pub(crate) setfenv_used: bool,
  // 作用域栈元素即「哪些节点在当前作用域」的有序句柄集合。
  pub(crate) local_stack: Vec<Node<AstLocal>>,
  pub(crate) upvals: Vec<Node<AstLocal>>,
  pub(crate) loop_jumps: Vec<LoopJump>,
  pub(crate) loops: Vec<Loop>,
  pub(crate) inline_frames: Vec<InlineFrame>,
  pub(crate) captures: Vec<Capture>,
  pub(crate) exported_locals: Vec<Node<AstLocal>>,
  // cpp `DenseHashMap<AstLocal*, uint8_t> exportedClasses`（Compiler.cpp:5606）：
  // 以类局部 AstLocal 指针为键（同名遮蔽各为独立键，不去重），dest 为类寄存器
  pub(crate) exported_classes: Vec<(*mut AstLocal, u8)>,
  /// cpp `Exports::hasExports`（Compiler.cpp:5553）：`ensure_export_table` 一旦被
  /// 调用即置位，参与 `exports_is_empty()` 判定。
  pub(crate) has_exports: bool,
  pub(crate) class_locals: DenseHashMap<AstName, Node<AstLocal>>,
}

mod expr;
mod expr_call;
mod fold;
mod function;
mod module;
mod scope;
mod stat;

impl Compiler {
  pub fn new(
    bytecode: &mut BytecodeBuilder,
    options: &CompileOptions,
    names: &mut AstNameTable,
  ) -> Compiler {
    // `names` 为调用方持有的活 `&mut AstNameTable`，本次调用期间无其它借用与之冲突。
    let export_name = names.get_or_add_str("__EXP");

    let mut compiler = Compiler {
      // 契约：builder 必须活得比返回的 `Compiler` 久（cpp 侧为引用成员 `BytecodeBuilder&`）。
      // `'static` 只是类型位，让收口点签名接受 arena 视图（`sref_*`）；安全性由契约
      // 与 `bc()`/`bc_mut()` 两处的 unsafe 解引用兑现。
      bytecode: NonNull::from(bytecode).cast::<BytecodeBuilder<'static>>(),
      options: *options,
      functions: DenseHashMap::default(),
      locals: DenseHashMap::default(),
      globals: DenseHashMap::default(),
      variables: DenseHashMap::default(),
      constants: DenseHashMap::default(),
      locstants: DenseHashMap::default(),
      table_constants: DenseHashMap::default(),
      table_shapes: DenseHashMap::default(),
      builtins: DenseHashMap::default(),
      userdata_types: DenseHashMap::default(),
      function_types: DenseHashMap::default(),
      local_types: DenseHashMap::default(),
      expr_types: DenseHashMap::default(),
      inline_builtins: DenseHashMap::default(),
      inline_builtins_backup: DenseHashMap::default(),
      expr_changes: Vec::new(),
      local_changes: Vec::new(),
      builtin_types: BuiltinAstTypes::new(options.vector_type.cast()),
      names: NonNull::from(names),
      // cpp 形制 AstLocal 的合成初值：parent/Location 之外的两个 null_mut 槽
      // 对应 `__EXP` 无父作用域、无类型标注，从不被解引用（仅作 map 键与注册记录）。
      export_table_local: AstLocal::new(
        export_name,
        Location::default(),
        null_mut(),
        0,
        0,
        null_mut(),
        true,
      ),
      builtins_fold: false,
      builtins_fold_library_k: false,
      reg_top: 0,
      stack_size: 0,
      arg_count: 0,
      has_loops: false,
      has_multi_ret: false,
      current_function: None,
      block_depth: 0,
      getfenv_used: false,
      setfenv_used: false,
      local_stack: Vec::new(),
      upvals: Vec::new(),
      loop_jumps: Vec::new(),
      loops: Vec::new(),
      inline_frames: Vec::new(),
      captures: Vec::new(),
      exported_locals: Vec::new(),
      exported_classes: Vec::new(),
      has_exports: false,
      class_locals: DenseHashMap::default(),
    };

    // 作用域栈初版预留（cpp Compiler 构造中的 reserve 值），避免小函数反复扩容
    const SCOPE_STACK_INITIAL_CAPACITY: usize = 16;
    compiler.local_stack.reserve(SCOPE_STACK_INITIAL_CAPACITY);
    compiler.upvals.reserve(SCOPE_STACK_INITIAL_CAPACITY);
    compiler
  }

  /// `bytecode` 句柄解引用的唯一安全收口点（只读视图）。字段契约见
  /// `Compiler::bytecode`：builder 由唯一构造点 `new` 的 `&mut` 借用提升而来，
  /// 且活得比 `Compiler` 久，故经指针造引用与直接持有引用语义一致。
  #[inline]
  pub(crate) fn bc(&self) -> &BytecodeBuilder<'static> {
    // Safety: 字段契约保证句柄非空且指向存活 BytecodeBuilder（`NonNull::as_ref`
    // 的半径由调用点借用决定，同原裸指针解引用）。
    unsafe { self.bytecode.as_ref() }
  }

  /// [`bc`](Self::bc) 的可变形态：独占借用半径即本次调用生命周期，与原先
  /// 散点 `(*self.bytecode).emit_x(..)` 解引用所在的语句范围一致。
  #[inline]
  pub(crate) fn bc_mut(&mut self) -> &mut BytecodeBuilder<'static> {
    // Safety: 同 bc()；调用处 self 的 &mut 借用保证无并发别名。
    unsafe { self.bytecode.as_mut() }
  }

  /// `names` 句柄解引用的唯一安全收口点（只读视图），契约同 [`bc`](Self::bc)。
  #[inline]
  pub(crate) fn names(&self) -> &AstNameTable {
    // Safety: 字段契约保证句柄非空且指向存活 AstNameTable。
    unsafe { self.names.as_ref() }
  }

  /// [`names`](Self::names) 的可变形态（字符串驻留点使用），契约同 [`bc_mut`](Self::bc_mut)。
  #[inline]
  pub(crate) fn names_mut(&mut self) -> &mut AstNameTable {
    // Safety: 同 names()；调用处 self 的 &mut 借用保证无并发别名。
    unsafe { self.names.as_mut() }
  }

  /// `try_compile_*` 的守卫样板单源：记录放弃原因 remark 后返回 `false`
  /// （cpp 各 `if (...) { addRemark(...); return false; }` 同构分支）。
  #[inline]
  pub(crate) fn reject_with_remark(&mut self, args: Arguments<'_>) -> bool {
    self.bc_mut().add_debug_remark(args);
    false
  }
}

/// 百分比基准（cpp 成本模型 `costPercent` 的 100）：for 展开与内联调用判定共用。
pub(crate) const K_COST_PERCENT_SCALE: i32 = 100;
