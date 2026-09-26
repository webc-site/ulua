//! `Compiler` 模块面：`__EXP` 导出表、import 链判定与 GETIMPORT/常量发射辅助
//! （对照 cpp `Compiler.cpp` 的 ensureExportTable/compileExportTable/emit* 段）。
use ulua_ast::records::{
  ast_expr_global::AstExprGlobal, ast_local::AstLocal, ast_node::AstNode, location::Location,
};
use ulua_bytecode::{
  methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash,
  records::bytecode_builder::BytecodeBuilder,
};
use ulua_common::{enums::luau_opcode::LuauOpcode, fflag, macros::luau_assert::LUAU_ASSERT};

use crate::{
  enums::global::{Global, Global::Written},
  functions::{
    ast_slot_ref::ast_slot_ref, get_global_state::get_global_state, sref_compiler::sref_ast_name,
  },
  records::{
    compile_error::{CompileError, ERR_EXCEEDED_CONSTANT_LIMIT},
    compiler::{Compiler, K_DEFAULT_ALLOC_PC, K_MAX_AD_INDEX},
    node::Node,
  },
};
impl Compiler {
  /// 对应 cpp `canImport`（cpp/Compiler/src/Compiler.cpp:2910）：全局名在
  /// `globals` 状态表中仍为可导入（非 `Written`）时返回 true；`expr` 为调用方
  /// 持有的 arena 存活节点，仅读取 `name` 字段。
  pub(crate) fn can_import(&self, expr: &AstExprGlobal) -> bool {
    if self.options.optimization_level < 1 {
      return false;
    }

    let name = expr.name;
    get_global_state(&self.globals, name) != Written
  }

  /// 对应 cpp `Compiler::canImportChain(AstExprGlobal*)`：入参收敛为
  /// `&AstExprGlobal`（非空由类型证明），cpp 的 null 哨兵早退随签名变化
  /// 退役——可空性判定留在调用方的 `Option` 折叠里。
  pub(crate) fn can_import_chain(&self, expr: &AstExprGlobal) -> bool {
    if self.options.optimization_level < 1 {
      return false;
    }
    get_global_state(&self.globals, expr.name) == Global::Default
  }

  /// 对应 cpp `checkExportedLocal`（cpp/Compiler/src/Compiler.cpp:190）：`export`
  /// 修饰的 local 仅允许出现在顶层，否则抛类型化 `CompileError`；并把该 local 的
  /// 地址句柄记入 `exported_locals`（当前仅作 `exports_is_empty` 的非空判据）。
  /// `local` 为调用方持有的 arena 存活节点（parser/编译器构造期接线）。
  pub(crate) fn check_exported_local(&mut self, local: &mut AstLocal, location: &Location) {
    if local.is_exported {
      if !self.at_top_level() {
        // C++ `CompileError::raise(...)`：抛类型化 CompileError，而非 String。
        CompileError::raise(
          location,
          format_args!("'export' may only be applied to top-level statements"),
        );
      }

      // push 的裸指针只作地址记录（解引用链未移植），借用交出即终止。
      self.exported_locals.push(Node::from_mut(local));
    }
  }

  /// cpp `compileExportTable()`（Compiler.cpp:377-460）。
  ///
  /// 未移植（两处均在 `FFlag::LuauOptimizeExportTable` / `LuauExportedTypesParticipateInScc`
  /// 门控下，本移植未定义这两个旗标，等价于上游库默认值 false 时的行为）：
  /// - cpp:406-419：遍历 `exports.exportedFunctions` 写 `export function` 到 `__EXP`。
  ///   该集合由 ValueTracking.cpp:121-125 收集；旗标关闭时集合恒空，
  ///   `export function` 由 `compile_stat` 的 SETTABLEKS 路径写入（上游 cpp:4872-4889）。
  /// - cpp:421-430：仅有类型导出（`exports.hasTypeExports`）时新建一个空表返回。
  ///   `hasTypeExports` 未跟踪（上游 cpp:4905-4908），且旗标关闭时上游该分支不可达，
  ///   调用点 `compile_function` 的 `!exports_is_empty()` 亦保证此处非空。
  pub(crate) fn compile_export_table(&mut self) {
    // cpp:382 `if (!exports.isEmpty())`
    LUAU_ASSERT!(!self.exports_is_empty());
    LUAU_ASSERT!(self.current_function.is_some());

    // 契约：调用点 compile_function 在进入函数体前已置
    // `current_function = Some(func)`（指向 parser arena 内存活的 AstExprFunction，比 self 长寿），
    // 出口才复位 None；repr(C) 前缀字段基址重合，取 &base.base 仅读 location/class 元数据，
    // 编译链不回写该节点，独占无冲突。此借用整体提到函数头一次取得，供下方各报错定位复用。
    let loc_ast = match self.current_function {
      Some(node) => &node.borrow().base.base,
      None => return,
    };

    // 出现在模块仅导出 class 的情形
    self.ensure_export_table(loc_ast);

    let export_local = &raw mut self.export_table_local;
    let table_reg = self.get_local_reg(export_local);
    LUAU_ASSERT!(table_reg >= 0);
    let table_reg = table_reg as u8;

    if fflag::DebugLuauUserDefinedClasses.get() {
      let exported_classes = self.exported_classes.clone();
      for (class_local, class_reg) in exported_classes {
        // 门面解引用：类局部 AstLocal 由解析器分配于 arena，编译期全程存活（同
        // push_local 的借用契约）。
        let class_name_ref = sref_ast_name(
          ast_slot_ref(class_local)
            .expect("exported_classes 项为 arena 存活 AstLocal")
            .name,
        );
        let class_name_cid = self.bc_mut().add_constant_string(class_name_ref.clone());
        self.check_constant(class_name_cid, &loc_ast.location);

        self.bc_mut().emit_abc(
          LuauOpcode::LOP_SETTABLEKS,
          class_reg,
          table_reg,
          bytecode_builder_get_string_hash(class_name_ref) as u8,
        );
        self.bc_mut().emit_aux(class_name_cid as u32);
      }
    }

    let freeze_reg = self.alloc_reg(loc_ast, 2);
    // 参数为静态 "freeze"（&str，无 NUL 语义），get_or_add_str 只读该串并写入名表。
    let freeze_name = self.names_mut().get_or_add_str("freeze");
    let freeze_cid = self
      .bc_mut()
      .add_constant_string(sref_ast_name(freeze_name));
    self.check_constant(freeze_cid, &loc_ast.location);

    // "table" 为静态合法名，经 names_mut() 访问器写名表。
    let table_name = self.names_mut().get_or_add_str("table");
    let table_cid = self.bc_mut().add_constant_string(sref_ast_name(table_name));
    self.check_constant(table_cid, &loc_ast.location);

    let iid = BytecodeBuilder::get_import_id2(table_cid, freeze_cid);
    let cid = self.bc_mut().add_import(iid);

    if (0..K_MAX_AD_INDEX).contains(&cid) {
      self.emit_ad_aux(LuauOpcode::LOP_GETIMPORT, freeze_reg, cid as i16, iid);
    } else {
      CompileError::raise(
        &loc_ast.location,
        format_args!("{ERR_EXCEEDED_CONSTANT_LIMIT}"),
      );
    }

    self
      .bc_mut()
      .emit_abc(LuauOpcode::LOP_MOVE, freeze_reg + 1, table_reg, 0);
    self
      .bc_mut()
      .emit_abc(LuauOpcode::LOP_CALL, freeze_reg, 2, 2);

    self.close_locals(0);
    self
      .bc_mut()
      .emit_abc(LuauOpcode::LOP_RETURN, freeze_reg, 2, 0);
  }

  /// cpp `ensureExportTable()`（Compiler.cpp:201-222）。
  ///
  /// 未移植：cpp:211-214 在 `FFlag::LuauOptimizeExportTable` 下改发 `LOP_DUPTABLE`
  /// （形状由 `buildExportTableShape()`（cpp:235-278）预先写入常量池）。本移植未定义
  /// 该旗标，等价于上游库默认值 false，故恒发 `NEWTABLE` 且 hashSize 恒为 0（cpp:217）。
  pub(crate) fn ensure_export_table(&mut self, node: &AstNode) {
    // cpp:203：一旦有语句需要导出表就置位，参与 `exports_is_empty()` 判定
    self.has_exports = true;

    let export_local = Node::new(&raw mut self.export_table_local);
    if self.locals.contains(&export_local) {
      return;
    }

    LUAU_ASSERT!(self.at_top_level());

    let table_reg = self.alloc_reg(node, 1);
    self.bc_mut().emit_abc(
      LuauOpcode::LOP_NEWTABLE,
      table_reg,
      Compiler::encode_hash_size(0),
      0,
    );
    self.bc_mut().emit_aux(0);

    // export_local 指向 Compiler 内嵌的 export_table_local 字段（在 Compiler::new 中
    // 由 AstLocal::new 完整初始化，非空且对齐，与 self 同生命周期；Compiler 在整个
    // 编译期内不被移动，地址稳定——与 cpp 版把 &exportTableLocal 存入
    // locals/local_stack 的设计一致）。push_local 以该地址句柄为键读写自身映射表，
    // 写入字段与 export_table_local 不相交（存活契约见 `Node::borrow`）。
    self.push_local(export_local.borrow(), table_reg, K_DEFAULT_ALLOC_PC);
  }

  /// cpp `Exports::isEmpty()`（Compiler.cpp:5561-5564）。
  ///
  /// 未移植：`exports.exportedFunctions`。该集合仅在 `LuauOptimizeExportTable` 下由
  /// `getValueTrackingInfo` 收集（ValueTracking.cpp:121-125），唯一消费者是同旗标门控的
  /// `compileExportTable` 尾部写入（Compiler.cpp:406-419）。本移植未定义该旗标，
  /// 等价于上游库默认值 false：此时上游集合恒为空，且 `export function` 的表项由
  /// `compile_stat` 的 SETTABLEKS 路径写出（对应上游 cpp:4872-4889 的
  /// `!FFlag::LuauOptimizeExportTable` 分支），故不追踪该集合不改变可观察行为。
  /// 注意上游 CLI 的 `setLuauFlagsDefault`（CLI/Flags.cpp:40-45）会把所有 `Luau*`
  /// 旗标打开，届时该分支才需要一并移植。
  pub(crate) fn exports_is_empty(&self) -> bool {
    !self.has_exports && self.exported_classes.is_empty() && self.exported_locals.is_empty()
  }

  pub(crate) fn get_export_table_reg(&mut self, node: &AstNode) -> u8 {
    let local_ptr = &raw mut self.export_table_local;
    let reg = self.get_local_reg(local_ptr);
    if reg >= 0 {
      return reg as u8;
    }

    // local_ptr 是 Compiler 内嵌 export_table_local 字段的地址
    // （&raw mut self.export_table_local），构造时由 AstLocal::new 完整初始化，
    // 非空且对齐；Compiler 在编译期内不移动、该字段地址稳定。get_upval 只把地址
    // 作为 upvals/variables/locals 映射的键并短借读取，可变写入均落在其他字段，
    // 与该共享借用不相交。
    let upval = self.get_upval(ast_slot_ref(local_ptr).expect("export_table_local 字段地址恒有效"));
    let reg = self.alloc_reg(node, 1);
    self
      .bc_mut()
      .emit_abc(LuauOpcode::LOP_GETUPVAL, reg, upval, 0);
    reg
  }

  pub(crate) fn emit_abc_aux(&mut self, op: LuauOpcode, a: u8, b: u8, c: u8, aux: u32) {
    self.bc_mut().emit_abc(op, a, b, c);
    self.bc_mut().emit_aux(aux);
  }

  pub(crate) fn emit_ad_aux(&mut self, op: LuauOpcode, a: u8, d: i16, aux: u32) {
    self.bc_mut().emit_ad(op, a, d);
    self.bc_mut().emit_aux(aux);
  }

  pub(crate) fn emit_load_k(&mut self, target: u8, cid: i32) {
    LUAU_ASSERT!(cid >= 0);

    if cid < K_MAX_AD_INDEX {
      self
        .bc_mut()
        .emit_ad(LuauOpcode::LOP_LOADK, target, cid as i16);
    } else {
      self.emit_ad_aux(LuauOpcode::LOP_LOADKX, target, 0, cid as u32);
    }
  }

  /// 哈希域大小编码：0 保持 0；否则为 `ceil(log2(hash_size)) + 1`。
  /// `next_power_of_two().trailing_zeros()` 即原「移位递增至 2^k >= hash_size」
  /// 循环的闭合形式（hash_size>0 时 `next_power_of_two` 不溢出，哈希域上界远小于 2^31）。
  pub fn encode_hash_size(hash_size: u32) -> u8 {
    if hash_size == 0 {
      return 0;
    }

    (hash_size.next_power_of_two().trailing_zeros() + 1) as u8
  }
}
