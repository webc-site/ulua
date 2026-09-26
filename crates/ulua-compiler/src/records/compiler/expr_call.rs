//! `Compiler` 调用与聚合表达式编译族：调用/fastcall/表构造/插值串/闭包构造等
//! 大体量表达式节点（对照 cpp `Compiler.cpp` 的 compileExprCall/Table/Function 段）。
use alloc::vec::Vec;
use core::mem::take;

use ulua_ast::{
  enums::ast_expr_ref::AstExprRef,
  records::{
    ast_array::AstArray,
    ast_expr_call::AstExprCall,
    ast_expr_function::AstExprFunction,
    ast_expr_interp_string::AstExprInterpString,
    ast_expr_table::{
      AstExprTable, Item, ItemKind,
      ItemKind::{List, Record},
    },
    ast_name::AstName,
  },
};
use ulua_bytecode::{
  methods::bytecode_builder_get_string_hash::bytecode_builder_get_string_hash,
  records::table_shape::TableShape,
};
use ulua_common::{
  enums::{
    luau_builtin_function::LuauBuiltinFunction, luau_capture_type::LuauCaptureType,
    luau_feedback_type::LuauFeedbackType, luau_opcode::LuauOpcode,
  },
  fflag,
  fint::{LuauCompileInlineDepth, LuauCompileInlineThreshold, LuauCompileInlineThresholdMaxBoost},
  macros::luau_assert::LUAU_ASSERT,
  records::insertion_ordered_map::InsertionOrderedMap,
};

use crate::{
  functions::{
    ast_slot_ref::ast_slot_ref,
    escape_and_append::escape_and_append,
    get_builtin::get_builtin,
    get_builtin_info::get_builtin_info,
    sref_compiler::{sref_ast_array_u8, sref_ast_name},
  },
  records::{
    builtin_info::BuiltinInfo,
    capture::Capture,
    compile_error::{CompileError, ERR_EXCEEDED_JUMP_DISTANCE_LIMIT, REMARK_INLINE_RECURSIVE},
    compiler::{Compiler, K_MAX_AD_INDEX, K_MAX_TARGET_COUNT},
    constant::Constant,
    node::Node,
  },
};
/// bit32 字段域总宽：`bit32.extract(v, f, w)` 折叠为 EXTRACTK 时须满足 `f + w <= 32`。
const K_BIT32_FIELD_WIDTH: i64 = 32;
/// EXTRACTK 常量打包位移：低 5 位存起始位 `f`，其上 5 位存 `w - 1`。
const K_BIT32_FIELD_PACK_SHIFT: u32 = 5;

impl Compiler {
  /// 调用表达式编译（对应 cpp/Compiler/src/Compiler.cpp:1484-1556）：
  /// `FFlag::LuauCompileFastpcall` 开启且优化级别 ≥1 时，对可导入的全局
  /// `pcall`/`xpcall` 直发 `LOP_FASTPCALL`；否则走 FASTCALL/CALL 常规路径。
  /// `expr` 为分发器判型后的存活 `AstExprCall`（args 访问受 `args.size` 界约束）；
  /// `target` 起 `target_count` 个寄存器须已在当前窗口分配，`target_top`/`mult_ret`
  /// 须与调用方（多返回值展开协议）一致——不匹配时 FASTCALL 原位写会越出调用方
  /// 预留的寄存器区间。
  pub(crate) fn compile_expr_call(
    &mut self,
    expr: impl Into<Node<AstExprCall>>,
    target: u8,
    target_count: u8,
    target_top: bool,
    mult_ret: bool,
  ) {
    let expr = expr.into();
    let expr_ref = expr.borrow();
    LUAU_ASSERT!(target_count < K_MAX_TARGET_COUNT);
    LUAU_ASSERT!(!target_top || (target as u32 + target_count as u32) == self.reg_top);

    self.set_debug_line_ast_node(&expr_ref.base.base);

    if self.options.optimization_level >= 2 && !expr_ref.self_ {
      // get_function_expr 以 Option 表达“未解析出函数表达式”（取代 cpp null 哨兵）
      let func = self.get_function_expr(expr_ref.func);
      let fi = func.and_then(|func| self.functions.find(&func));
      // 先求值：后面 try_compile_inlined_call 需要 &mut self
      let can_inline = fi.is_some_and(|f| f.can_inline);
      let has_fi = fi.is_some();

      // can_inline 由 fi 导出，fi 命中蕴含 func 命中 Some，下方解引用安全
      if let Some(func) = func {
        // 调用前置契约：func 来自 get_function_expr 的 Option 命中（存活 AstExprFunction），
        // Some 分支内只读；can_inline 前置门保持——try_compile_inlined_call 契约
        // 要求 functions.find 命中且 can_inline 为真（cpp Compiler.cpp:1360 同判据）。
        if can_inline
          && self.try_compile_inlined_call(
            expr_ref,
            func.borrow(),
            target,
            target_count,
            mult_ret,
            LuauCompileInlineThreshold.get(),
            LuauCompileInlineThresholdMaxBoost.get(),
            LuauCompileInlineDepth.get(),
          )
        {
          return;
        }

        // 为未尝试内联的函数补调试备注（cpp Compiler.cpp:1374-1384）
        if !can_inline && self.bc().needs_debug_remarks() {
          let func_ref = func.borrow();
          if func_ref.vararg {
            self
              .bc_mut()
              .add_debug_remark(format_args!("inlining failed: function is variadic"));
          } else if !has_fi {
            self
              .bc_mut()
              .add_debug_remark(format_args!("{REMARK_INLINE_RECURSIVE}"));
          } else if self.getfenv_used || self.setfenv_used {
            self
              .bc_mut()
              .add_debug_remark(format_args!("inlining failed: module uses getfenv/setfenv"));
          }
        }
      }
    }

    let _rs = self.reg_scope();
    let reg_count = (1 + (expr_ref.self_ as usize) + expr_ref.args.size).max(target_count as usize);
    let regs = if target_top {
      self.alloc_reg(
        &expr_ref.base.base,
        (reg_count - target_count as usize) as u32,
      ) - target_count
    } else {
      self.alloc_reg(&expr_ref.base.base, reg_count as u32)
    };

    let mut selfreg = 0u8;
    let mut bfid = -1;

    if self.options.optimization_level >= 1 && !expr_ref.self_ {
      // 对应 C++ `id && *id != LBF_NONE`：builtins 表中的 LBF_NONE (0) 意为
      // "非内建"，绝不可启用 FASTCALL。内建 apply/restore（及 operator[] 查找）
      // 会留下真实的 LBF_NONE 条目，故只测 `!= -1` 会误置 bfid = 0 并以
      // builtin 0 发出 FASTCALL。
      if let Some(id) = self.builtins.find(&expr)
        && *id != LuauBuiltinFunction::LBF_NONE as i32
      {
        bfid = *id;
      }
    }

    if bfid >= 0 && self.bc().needs_debug_remarks() {
      let builtin = get_builtin(expr_ref.func.into(), &self.globals, &self.variables);
      let last_mult = expr_ref
        .args
        .iter()
        .next_back()
        .is_some_and(|&last| self.is_expr_mult_ret(last));
      if !builtin.empty() {
        // C++ `addDebugRemark("builtin %s.%s/%d%s", object, method, args.size, lastMult?"+":"")`
        // （带对象形态），无对象时为 `"builtin %s/%d%s"`；
        // object 与 method 均为空时不发备注（与 cpp 分支次序一致）。
        let argc = expr_ref.args.size as i32;
        let suffix = if last_mult { "+" } else { "" };
        let method = builtin.method.as_str_or_empty();
        if !builtin.object.is_null() {
          let object = builtin.object.as_str_or_empty();
          self.bc_mut().add_debug_remark(format_args!(
            "builtin {}.{}/{}{}",
            object, method, argc, suffix
          ));
        } else if !method.is_empty() {
          self
            .bc_mut()
            .add_debug_remark(format_args!("builtin {}/{}{}", method, argc, suffix));
        }
      }
    }

    if bfid == LuauBuiltinFunction::LBF_SELECT_VARARG as i32 {
      // 优化：把 select(_, ...) 编成 FASTCALL1；仅限单返回值表达式，
      // 否则回退为普通调用（bfid = -1）。
      if !mult_ret && target_count == 1 {
        return self.compile_expr_select_vararg(
          expr_ref,
          target,
          target_count,
          target_top,
          mult_ret,
          regs,
        );
      } else {
        bfid = -1;
      }
    }

    if bfid == LuauBuiltinFunction::LBF_BIT32_EXTRACT as i32
      && expr_ref.args.size == 3
      && self.is_constant(expr_ref.args.as_slice()[1])
      && self.is_constant(expr_ref.args.as_slice()[2])
    {
      let fc = self.get_constant(expr_ref.args.as_slice()[1]);
      let wc = self.get_constant(expr_ref.args.as_slice()[2]);
      // 折叠后的字段位置/宽度常量：非 Number 一律 -1（下方守卫即不成立）
      let [fi, wi] = [fc, wc].map(|c| match c {
        Constant::Number(n) => n as i32,
        _ => -1,
      });
      // 用加法加宽：`fi`/`wi` 是折叠后的用户常量，超大字段时 `fi + wi`
      // 会溢出 `int`（C++ 里是 UB；开 overflow-checks 则 panic）。
      if fi >= 0 && wi > 0 && fi as i64 + wi as i64 <= K_BIT32_FIELD_WIDTH {
        let fwp = fi | ((wi - 1) << K_BIT32_FIELD_PACK_SHIFT);
        let cid = self.bc_mut().add_constant_number(fwp as f64);
        self.check_constant(cid, &expr_ref.base.base.location);
        return self.compile_expr_fastcall_n(
          expr_ref,
          target,
          target_count,
          target_top,
          mult_ret,
          regs,
          LuauBuiltinFunction::LBF_BIT32_EXTRACTK as i32,
          cid,
        );
      }
    }

    let mut max_fastcall_args = 2;
    if bfid >= 0
      && expr_ref.args.size == 3
      && expr_ref
        .args
        .iter()
        .any(|&arg| self.get_expr_local_reg(arg) >= 0)
    {
      max_fastcall_args = 3;
    }

    if bfid >= 0 && expr_ref.args.size >= 1 && expr_ref.args.size <= max_fastcall_args {
      // 尾实参经 as_slice().last() 取回（size>=1 守卫保证命中）
      let last_mult = expr_ref
        .args
        .as_slice()
        .last()
        .is_some_and(|&arg| self.is_expr_mult_ret(arg));
      // 两分支发射同一条 FASTCALL：尾实参非 multret 直接走；multret 仅在 opt≥2
      // 且内建实参数精确匹配并标记 NONE_SAFE 时才保留（判定收口为一次闭包调用）。
      let mult_ret_none_safe = || {
        let info = get_builtin_info(bfid);
        self.options.optimization_level >= 2
          && expr_ref.args.size as i32 == info.params
          && (info.flags & BuiltinInfo::FLAG_NONE_SAFE) != 0
      };

      if !last_mult || mult_ret_none_safe() {
        return self.compile_expr_fastcall_n(
          expr_ref,
          target,
          target_count,
          target_top,
          mult_ret,
          regs,
          bfid,
          -1,
        );
      }
    }

    // 优化：pcall/xpcall 有专用 fastcall 指令（cpp Compiler.cpp:1484-1494）
    let mut fast_pcall_id = -1i32;
    if fflag::LuauCompileFastpcall.get()
      && self.options.optimization_level >= 1
      && !expr_ref.self_
      // 判型+下转：非 AstExprGlobal 折叠为 None（cpp 只读判型形态）。
      && let AstExprRef::Global(g) = Node::from(expr_ref.func).as_expr_ref()
      && self.can_import(g)
    {
      let name = g.name;
      if name == "pcall" && expr_ref.args.size >= 1 {
        fast_pcall_id = 0;
      } else if name == "xpcall" && expr_ref.args.size >= 2 {
        fast_pcall_id = 1;
      }
    }

    // self_ 调用的 func 必为 AstExprIndexName（parser 保证）：checked 共享下转 +
    // 断言收口为闭包两处复用；全为只读访问，调用点现场重建借用、半径限于单条语句。
    let index_name = || match Node::from(expr_ref.func).as_expr_ref() {
      AstExprRef::IndexName(fi) => fi,
      _ => {
        LUAU_ASSERT!(false);
        panic!("self_ 调用的 func 必为 AstExprIndexName（parser 保证，cpp 静态断言后裸解引用）");
      }
    };

    if expr_ref.self_ {
      let fi = index_name();
      let reg = self.get_expr_local_reg(fi.expr);
      if reg >= 0 {
        selfreg = reg as u8;
      } else {
        selfreg = regs;
        // Safety: fi.expr 已句柄化恒非空，裸出口经 as_ptr 桥接；&mut 写穿仅落在该节点
        // 的编译期临时字段，调用方独占本编译器、AST arena 无并发访问。
        self.compile_expr_temp_top(unsafe { &mut *fi.expr.as_ptr() }, selfreg);
      }
    } else if bfid < 0 && fast_pcall_id < 0 {
      // Safety: expr_ref.func 为调用节点的存活函数表达式子指针（parser 保证非空）；
      // &mut 写穿限于该节点编译期临时字段，独占由本编译器持有。
      self.compile_expr_temp_top(unsafe { &mut *expr_ref.func }, regs);
    }

    let mut mult_call = false;
    for (i, &arg) in expr_ref.args.iter().enumerate() {
      if i + 1 == expr_ref.args.size {
        // Safety: arg 为 args 数组第 i 项，parser 登记的存活 AstExpr 子指针；
        // &mut 写穿限于该节点编译期临时字段。
        mult_call = self.compile_expr_temp_mult_ret(
          unsafe { &mut *arg },
          regs + 1 + (expr_ref.self_ as u8) + i as u8,
        );
      } else {
        // Safety: 同上，args 各项均为存活 AstExpr 子指针。
        self.compile_expr_temp_top(
          unsafe { &mut *arg },
          regs + 1 + (expr_ref.self_ as u8) + i as u8,
        );
      }
    }

    // 门面解引用后仅读 func 基类 location（cpp 同处直接解引用，非空由 parser 接线保证）。
    self.set_debug_line_end(
      &ast_slot_ref(expr_ref.func)
        .expect("expr_ref.func 为 parser 接线的存活 AstExpr 子指针")
        .base,
    );

    if expr_ref.self_ {
      let fi = index_name();
      self.set_debug_line_location(&fi.index_location);
      let iname = sref_ast_name(fi.index);
      let cid = self.bc_mut().add_constant_string(iname.clone());
      self.check_constant(cid, &fi.base.base.location);
      self.bc_mut().emit_abc(
        LuauOpcode::LOP_NAMECALL,
        regs,
        selfreg,
        bytecode_builder_get_string_hash(iname) as u8,
      );
      self.bc_mut().emit_aux(cid as u32);
      // expr 已句柄化恒非空；hint_table_reg 为既有裸指针 API，经 as_ptr 桥接。
      self.hint_table_reg(fi.expr.as_ptr(), selfreg, 2);
    } else if bfid >= 0 || fast_pcall_id >= 0 {
      let fastcall_label = self.bc().emit_label();
      if fast_pcall_id >= 0 {
        let explicit_args = (expr_ref.args.size - usize::from(mult_call)) as u8;
        self.bc_mut().emit_abc(
          LuauOpcode::LOP_FASTPCALL,
          fast_pcall_id as u8,
          explicit_args,
          0,
        );
      } else {
        self
          .bc_mut()
          .emit_abc(LuauOpcode::LOP_FASTCALL, bfid as u8, 0, 0);
      }
      // Safety: expr_ref.func 为存活 AstExpr 子指针；&mut 写穿限于该节点编译期临时字段。
      self.compile_expr(unsafe { &mut *expr_ref.func }, regs, true);
      let call_label = self.bc().emit_label();
      // cpp 校验 patchSkipC 返回值，失败时报错（Compiler.cpp:1562-1565）
      if !self.bc_mut().patch_skip_c(fastcall_label, call_label) {
        // 门面解引用后仅读存活 func 节点的 location 用于报错。
        let location = ast_slot_ref(expr_ref.func)
          .expect("expr_ref.func 为 parser 接线的存活 AstExpr 子指针")
          .base
          .location;
        CompileError::raise(
          &location,
          format_args!("{ERR_EXCEEDED_JUMP_DISTANCE_LIMIT}"),
        );
      }
    }

    // CALL/CALLFB 两分支共用的实参窗口与结果数（纯算式，提出不改变行为）
    let nargs = if mult_call {
      0
    } else {
      (expr_ref.self_ as u8) + expr_ref.args.size as u8 + 1
    };
    let aresults = if mult_ret { 0 } else { target_count + 1 };

    // C++ `canInline = currentFunction->functionDepth != 0 && !multCall && !multRet`
    // （Compiler.cpp:1394）：调用反馈只为*嵌套*函数中的调用发射，不含 depth-0 主块。
    // 此前移植版误用一个判空替代了深度测试。
    if fflag::LuauEmitCallFeedback.get()
      && bfid < 0
      && fast_pcall_id < 0
      // current_function 为 Some 时必是 compile_function 进入函数体置入的
      // 存活句柄，仅读 function_depth。
      && self.current_function.is_some_and(|f| f.borrow().function_depth != 0)
      && !mult_call
      && !mult_ret
    {
      let fb_slot = self.bc_mut().add_fb_slot(LuauFeedbackType::LFT_CALLTARGET);
      self.emit_abc_aux(LuauOpcode::LOP_CALLFB, regs, nargs, aresults, fb_slot);
    } else {
      self
        .bc_mut()
        .emit_abc(LuauOpcode::LOP_CALL, regs, nargs, aresults);
    }

    if !target_top {
      self.fill_regs_move(target, regs, target_count);
    }
  }

  /// 对应 cpp `compileExprFastcallN`（cpp/Compiler/src/Compiler.cpp:811）：内建
  /// 调用的 FASTCALL1/2/2K/3 序列原位发射。`expr` 为调用方持有的存活 `AstExprCall`
  /// （`bfid`/`bf_k` 由调用方从 `builtins` 表对该节点匹配所得，二者须对应同一
  /// 调用点）；`target` 起 `regs`/`target_count` 个寄存器须已分配且与
  /// `target_top`/`mult_ret` 展开协议一致——编号失配会踩踏活跃寄存器。
  pub(crate) fn compile_expr_fastcall_n(
    &mut self,
    expr: &AstExprCall,
    target: u8,
    target_count: u8,
    target_top: bool,
    mult_ret: bool,
    regs: u8,
    bfid: i32,
    bf_k: i32,
  ) {
    // expr 为 parser arena 内存活节点（编译期 AST 只读、arena 比本调用长寿）。
    let expr_ref = expr;
    LUAU_ASSERT!(!expr_ref.self_);
    LUAU_ASSERT!(expr_ref.args.size >= 1);
    LUAU_ASSERT!(expr_ref.args.size <= 3);
    LUAU_ASSERT!(if bfid == LuauBuiltinFunction::LBF_BIT32_EXTRACTK as i32 {
      bf_k >= 0
    } else {
      bf_k < 0
    });
    LUAU_ASSERT!(target_count < K_MAX_TARGET_COUNT);

    let opc = if expr_ref.args.size == 1 {
      LuauOpcode::LOP_FASTCALL1
    } else if bf_k >= 0
      // size==2 短路保证第二槽存在：as_slice 界内索引取代裸 add(1)
      || (expr_ref.args.size == 2 && self.is_constant(expr_ref.args.as_slice()[1]))
    {
      LuauOpcode::LOP_FASTCALL2K
    } else if expr_ref.args.size == 2 {
      LuauOpcode::LOP_FASTCALL2
    } else {
      LuauOpcode::LOP_FASTCALL3
    };

    let mut args = [0u32; 3];
    // size <= 3 已断言，固定 [u32; 3] 足够
    // as_slice 即 data/size 的受界切片（内部已守 null），取代手写 from_raw_parts
    let arg_ptrs = expr_ref.args.as_slice();
    for (i, &arg_expr) in arg_ptrs.iter().enumerate() {
      if i > 0 && opc == LuauOpcode::LOP_FASTCALL2K {
        // 门面解引用：arg_expr 取自 args 切片，即数组内非空存活表达式指针。
        let arg = ast_slot_ref(arg_expr).expect("args 槽位由 parser 保证存活非空表达式指针");
        let cid = self.get_constant_index(arg);
        self.check_constant(cid, &arg.base.location);
        args[i] = cid as u32;
      } else {
        let reg = self.get_expr_local_reg(arg_expr);
        if reg >= 0 {
          args[i] = reg as u32;
        } else {
          args[i] = (regs as u32) + 1 + (i as u32);
          // Safety: arg_expr 为存活子表达式指针；args[i] 由 regs+1+i 推得，
          // regs 为本调用已分配的实参基寄存器，编号在帧内有效。
          self.compile_expr_temp_top(unsafe { &mut *arg_expr }, args[i] as u8);
        }
      }
    }

    // 原局部 &mut 别名横跨 self.emit_load_k / self.compile_expr_temp 的再入借用，
    // 改经 bc()/bc_mut() 逐点取用，借用半径压到单条语句。
    let fastcall_label = self.bc().emit_label();

    self.bc_mut().emit_abc(opc, bfid as u8, args[0] as u8, 0);

    if opc == LuauOpcode::LOP_FASTCALL3 {
      LUAU_ASSERT!(bf_k < 0);
      self.bc_mut().emit_aux(args[1] | (args[2] << 8));
    } else if opc != LuauOpcode::LOP_FASTCALL1 {
      self
        .bc_mut()
        .emit_aux(if bf_k >= 0 { bf_k as u32 } else { args[1] });
    }

    for (i, &arg) in args.iter().enumerate().take(expr_ref.args.size) {
      if i > 0 && opc == LuauOpcode::LOP_FASTCALL2K {
        self.emit_load_k(regs + 1 + i as u8, arg as i32);
      } else if arg != (regs as u32) + 1 + (i as u32) {
        self
          .bc_mut()
          .emit_abc(LuauOpcode::LOP_MOVE, regs + 1 + i as u8, arg as u8, 0);
      }
    }

    // Safety: expr_ref.func 为 parser 保证非空的被调表达式指针（arena 内存活），
    // regs 是本帧为其分配的基寄存器。
    self.compile_expr(unsafe { &mut *expr_ref.func }, regs, true);

    let call_label = self.bc().emit_label();

    if !self.bc_mut().patch_skip_c(fastcall_label, call_label) {
      // 门面解引用：func 同上存活，读 location 仅用于报错。
      let location = ast_slot_ref(expr_ref.func)
        .expect("expr_ref.func 为 parser 接线的存活被调表达式指针")
        .base
        .location;
      CompileError::raise(
        &location,
        core::format_args!("{ERR_EXCEEDED_JUMP_DISTANCE_LIMIT}"),
      );
    }

    self.bc_mut().emit_abc(
      LuauOpcode::LOP_CALL,
      regs,
      (expr_ref.args.size + 1) as u8,
      if mult_ret { 0 } else { target_count + 1 },
    );

    if !target_top {
      self.fill_regs_move(target, regs, target_count);
    }
  }

  /// 对应 cpp `Compiler::compileExprInterpString`。原 `pub(crate) unsafe fn` 已
  /// safe 化：`expr` 由 `compile_expr` 分发器经 RTTI 门面命中后 `from_mut` 交还
  /// （arena 存活、RTTI 确为 `AstExprInterpString`），其 `parts`/`expressions`
  /// 两个 `AstArray` 的 `size` 与实际元素数一致（越界读会扫描 arena 脏内存）；
  /// `target` 为已分配寄存器编号（CONCAT 结果槽）。残余窄 `unsafe` 仅解引用
  /// 上述 arena 指针与 names 表句柄。
  pub(crate) fn compile_expr_interp_string(
    &mut self,
    expr: &AstExprInterpString,
    target: u8,
    target_temp: bool,
  ) {
    let expr_ref = expr;
    let mut format_capacity = 0;
    for string in expr_ref.strings.iter() {
      format_capacity += string.size + (*string).iter().filter(|&&c| c == b'%').count();
    }

    // 单次遍历同时完成：构建格式串 + 记录各子表达式是否为 String 常量。
    // 该标记供尾部发射循环复用，原先"预扫描 + 构建 + 发射"三轮各查一次
    // 常量表（3N 次哈希），现合并为一轮（N 次）。
    let mut sub_is_string_const = Vec::with_capacity(expr_ref.expressions.size);
    let mut skipped_sub_expr = 0;
    // 容量下界：字符串段 + 每个子表达式至多 2 字节占位（"%*"），字符串常量
    // 追加时超出部分由 Vec 摊销扩容
    let mut format_string = Vec::with_capacity(format_capacity + 2 * expr_ref.expressions.size);
    let expressions = expr_ref.expressions.as_slice();
    for (i, string) in expr_ref.strings.iter().enumerate() {
      escape_and_append(&mut format_string, string.as_bytes());
      if let Some(&sub_expr) = expressions.get(i) {
        if let Some(c) = self.constants.find(&sub_expr.into())
          && matches!(c, Constant::Str(_))
        {
          escape_and_append(&mut format_string, c.get_string_bytes());
          sub_is_string_const.push(true);
          skipped_sub_expr += 1;
        } else {
          format_string.extend_from_slice(b"%*");
          sub_is_string_const.push(false);
        }
      }
    }

    let format_string_index = if format_string.is_empty() {
      // `self.names()` 访问器兑现构造期接线契约（名表比 self 长寿）；
      // get_or_add_str 只插入不移动已有项。
      let interned = self.names_mut().get_or_add_str("");
      self.bc_mut().add_constant_string(sref_ast_name(interned))
    } else {
      // 同上，get_or_add_slice 只在其内插入；返回的 interned.value 是合法
      // NUL 结尾串指针，仅作只读 sref 消费。
      let interned = self.names_mut().get_or_add_slice(&format_string);
      // *const→*mut 仅用于填充 cpp 形制的 AstArray 字段；该指针随后只经
      // sref_ast_array_u8 作只读 sref 消费，不经由它写入。
      let format_string_array = AstArray {
        data: interned.value as *mut u8,
        size: format_string.len(),
      };
      self
        .bc_mut()
        .add_constant_string(sref_ast_array_u8(format_string_array))
    };

    self.check_constant(format_string_index, &expr_ref.base.base.location);

    let _rs = self.reg_scope();
    let reg_count = 2 + expr_ref.expressions.size - skipped_sub_expr;
    let target_top = fflag::LuauCompileStringInterpTargetTop.get()
        && target_temp
        // `reg_top != 0` 对齐 cpp 的无符号回绕语义（regTop==0 时比较恒为假）
        && self.reg_top != 0
        && target as u32 == self.reg_top - 1;
    let base_reg = if target_top {
      self.alloc_reg(&expr_ref.base.base, (reg_count - 1) as u32) - 1
    } else {
      self.alloc_reg(&expr_ref.base.base, reg_count as u32)
    };

    self.emit_load_k(base_reg, format_string_index);

    let mut skipped = 0;
    for (i, (&sub_expr, &is_string_const)) in expr_ref
      .expressions
      .iter()
      .zip(sub_is_string_const.iter())
      .enumerate()
    {
      if is_string_const {
        skipped += 1;
      } else {
        // Safety: sub_expr 为 expressions 数组记录的存活 AstExpr 子指针；&mut 写穿
        // 限于该节点编译期临时字段，AST 与 &mut self 各字段无别名交集。
        self.compile_expr_temp_top(
          unsafe { &mut *sub_expr },
          base_reg + 2 + i as u8 - skipped as u8,
        );
      }
    }

    let format_method = sref_ast_name(AstName::from_static(b"format"));
    let format_method_index = self.bc_mut().add_constant_string(format_method.clone());
    self.check_constant(format_method_index, &expr_ref.base.base.location);

    self.bc_mut().emit_abc(
      LuauOpcode::LOP_NAMECALL,
      base_reg,
      base_reg,
      bytecode_builder_get_string_hash(format_method) as u8,
    );
    self.bc_mut().emit_aux(format_method_index as u32);
    self.bc_mut().emit_abc(
      LuauOpcode::LOP_CALL,
      base_reg,
      (expr_ref.expressions.size + 2 - skipped_sub_expr) as u8,
      2,
    );
    if target != base_reg {
      self
        .bc_mut()
        .emit_abc(LuauOpcode::LOP_MOVE, target, base_reg, 0);
    }
  }

  /// DUPTABLE 双路循环头部同构六行样板的单点收口：断言 Record 形态 → 判型
  /// 下转 key 为 AstExprConstantString（parser 保证）→ 登记字符串常量 →
  /// 溢出守卫。发射调用序列与诊断文案和原两路逐位一致。
  #[inline]
  fn record_item_key_cid(&mut self, item: &Item) -> i32 {
    LUAU_ASSERT!(item.kind == ItemKind::Record);
    // 门面判型+下转：Record 项的 key 由 parser 保证为存活 AstExprConstantString。
    let AstExprRef::ConstantString(ckey) = Node::from(item.key).as_expr_ref() else {
      LUAU_ASSERT!(false);
      panic!("Record 项的 key 由 parser 保证为 AstExprConstantString");
    };
    let key_cid = self
      .bc_mut()
      .add_constant_string(sref_ast_array_u8(ckey.value));
    self.check_constant(key_cid, &ckey.base.base.location);
    key_cid
  }

  /// `target` 须为已分配寄存器编号（NEWTABLE 结果槽）；`items` 子指针为 parser
  /// 接线、编译期存活的节点，`item.key`/`item.value` 判空后才解引用。
  pub(crate) fn compile_expr_table(
    &mut self,
    expr_ref: &AstExprTable,
    target: u8,
    target_temp: bool,
  ) {
    if expr_ref.items.is_empty() {
      // C++ `TableShape shape = tableShapes[expr];` 的 `operator[]` 在表没有
      // 预测形状时（空 `{}` 且后续无字段写入）默认构造零值形状
      // （hash_size=0、array_size=0）。此前模型把未命中译成了 panic。
      let shape = self
        .table_shapes
        .find(&Node::from_ref(expr_ref))
        .copied()
        .unwrap_or_default();
      self
        .bc_mut()
        .add_debug_remark(format_args!("allocation: table hash {}", shape.hash_size));
      self.bc_mut().emit_abc(
        LuauOpcode::LOP_NEWTABLE,
        target,
        Self::encode_hash_size(shape.hash_size),
        0,
      );
      self.bc_mut().emit_aux(shape.array_size);
      return;
    }

    let mut array_size = 0;
    let mut hash_size = 0;
    let mut record_size = 0;
    for item in expr_ref.items.iter() {
      array_size += (item.kind == List) as u32;
      hash_size += (item.kind != List) as u32;
      record_size += (item.kind == Record) as u32;
    }

    let mut index_size = 0;
    if array_size == 0 && hash_size > 0 {
      for item in expr_ref.items.iter() {
        LUAU_ASSERT!(!item.key.is_null());
        if let Some(Constant::Number(val)) = self.constants.find(&item.key.into())
          && *val == (index_size + 1) as f64
        {
          index_size += 1;
        }
      }
      if hash_size == record_size + index_size {
        hash_size = record_size;
      } else {
        index_size = 0;
      }
    }

    let encoded_hash_size = Self::encode_hash_size(hash_size);
    let _rs = self.reg_scope();
    // 优化：若 target 是临时寄存器，直接把结果算进它本身。
    let reg = if target_temp {
      target
    } else {
      self.alloc_reg(&expr_ref.base.base, 1)
    };

    // record 字段（模板表路径）的拍平映射表。
    let mut last_key_val: InsertionOrderedMap<i32, i32> = InsertionOrderedMap::new();

    // 优化：当所有项都是 record 字段时，走模板表（DUPTABLE）。
    if array_size == 0
      && index_size == 0
      && hash_size == record_size
      && (1..=TableShape::K_MAX_LENGTH).contains(&record_size)
    {
      let mut shape = TableShape::default();

      if fflag::LuauCompileDuptableConstantPack2.get() {
        for item in expr_ref.items.iter() {
          let key_cid = self.record_item_key_cid(item);
          // 门面解引用：item.value 为 parser 接线存活的表项值节点。
          let value_cid = self.get_constant_index(
            ast_slot_ref(item.value).expect("Record 项的 value 由 parser 保证存活非空"),
          );
          if let Some(existing) = last_key_val.get(&key_cid)
            && *existing == -1
          {
            continue;
          }
          // C++ `lastKeyVal[keyCid] = valueCid` 的 operator[] 会覆盖已有条目。
          // 而 InsertionOrderedMap::insert 在键已存在时是空操作，于是当重复键
          // 的值后来变成 -1（非常量，如 Closure）时降级失败，表被错误地打包
          // 了常量。get_or_default 才是 operator[] 的等价写法。
          *last_key_val.get_or_default(key_cid) = value_cid;
        }

        for (key_cid, value_cid) in last_key_val.iter() {
          LUAU_ASSERT!(shape.length < TableShape::K_MAX_LENGTH);
          let idx = shape.length as usize;
          shape.keys[idx] = *key_cid;
          shape.constants[idx] = *value_cid;
          if *value_cid >= 0 {
            shape.has_constants = true;
          }
          shape.length += 1;
        }
      } else {
        for item in expr_ref.items.iter() {
          let cid = self.record_item_key_cid(item);
          LUAU_ASSERT!(shape.length < TableShape::K_MAX_LENGTH);
          shape.keys[shape.length as usize] = cid;
          shape.length += 1;
        }
      }

      let tid = self.bc_mut().add_constant_table(&shape);
      self.check_constant(tid, &expr_ref.base.base.location);
      self
        .bc_mut()
        .add_debug_remark(format_args!("allocation: table template {}", hash_size));

      if tid < K_MAX_AD_INDEX {
        self
          .bc_mut()
          .emit_ad(LuauOpcode::LOP_DUPTABLE, reg, tid as i16);
      } else {
        // 此处必须停用 duptable 常量打包优化，因为已退回普通新建表路径
        if fflag::LuauCompileDuptableConstantPack2.get() {
          last_key_val.clear();
        }
        self.emit_abc_aux(LuauOpcode::LOP_NEWTABLE, reg, encoded_hash_size, 0, 0);
      }
    } else {
      // 优化：末元素是 `...` 时，把存储分配交给 SETLIST 处理。
      // items 非空（size==0 已提前返回），iter().last() 必命中
      // 谓词是「value 是 Varargs」（cpp `last.value->is<AstExprVarargs>()`），
      // matches! 即该判定本身，替代旧 ast_slot_is。
      let trailing_varargs = expr_ref.items.iter().last().is_some_and(|last| {
        last.kind == ItemKind::List
          && matches!(Node::from(last.value).as_expr_ref(), AstExprRef::Varargs(_))
      });
      LUAU_ASSERT!(!trailing_varargs || array_size > 0);

      let array_allocation = array_size - (trailing_varargs as u32) + index_size;

      if hash_size == 0 {
        self
          .bc_mut()
          .add_debug_remark(format_args!("allocation: table array {}", array_allocation));
      } else if array_allocation == 0 {
        self
          .bc_mut()
          .add_debug_remark(format_args!("allocation: table hash {}", hash_size));
      } else {
        self.bc_mut().add_debug_remark(format_args!(
          "allocation: table hash {} array {}",
          hash_size, array_allocation
        ));
      }

      self
        .bc_mut()
        .emit_abc(LuauOpcode::LOP_NEWTABLE, reg, encoded_hash_size, 0);
      self.bc_mut().emit_aux(array_allocation);
    }

    // 数组项分块写 SETLIST 的块宽上限（C++ `std::min(16u, arraySize)`）
    const K_ARRAY_CHUNK_SIZE: u32 = 16;
    let array_chunk_size = K_ARRAY_CHUNK_SIZE.min(array_size);
    let array_chunk_reg = self.alloc_reg(&expr_ref.base.base, array_chunk_size);
    let mut array_chunk_current: u32 = 0;
    let mut array_index: u32 = 1;
    let mut mult_ret = false;

    for (i, item) in expr_ref.items.iter().enumerate() {
      let key = item.key;
      let value = item.value;

      if fflag::LuauCompileDuptableConstantPack2.get()
        && last_key_val.size() > 0
        // 门面判型+下转：带键项的 key 命中 AstExprConstantString 才读 value。
        && let AstExprRef::ConstantString(ckey) = Node::from(key).as_expr_ref()
      {
        let key_cid = self
          .bc_mut()
          .add_constant_string(sref_ast_array_u8(ckey.value));
        if let Some(value_cid) = last_key_val.get(&key_cid) {
          // 常量不再另行生成赋值指令
          if *value_cid >= 0 {
            continue;
          }
        }
      }

      // 部分键值对不需要编译表达式，行号信息在此统一设置
      // 门面解引用：每个 item.value 均为 parser 保证非空存活的表达式节点。
      self.set_debug_line_ast_node(
        &ast_slot_ref(value)
          .expect("item.value 为 parser 接线的存活表项节点")
          .base,
      );

      if self.options.coverage_level >= 2 {
        self.bc_mut().emit_abc(LuauOpcode::LOP_COVERAGE, 0, 0, 0);
      }

      // 数组块溢出时、或写到哈希键之前先冲刷，以保持插入顺序
      if array_chunk_current > 0 && (!key.is_null() || array_chunk_current == array_chunk_size) {
        self.bc_mut().emit_abc(
          LuauOpcode::LOP_SETLIST,
          reg,
          array_chunk_reg,
          (array_chunk_current + 1) as u8,
        );
        self.bc_mut().emit_aux(array_index);
        array_index += array_chunk_current;
        array_chunk_current = 0;
      }

      if !key.is_null() {
        // 带键的项经 SETTABLE/SETTABLEKS/SETTABLEN 赋值
        let mut rsi = self.reg_scope();
        // key 为已判非空的存活表键节点，value 为存活表达式节点，
        // compile_l_value_index/compile_expr_auto 只读消费。
        let lv = self.compile_l_value_index(reg, key, &mut rsi);
        let rv = self.compile_expr_auto(value, &mut rsi);
        self.compile_assign(&lv, rv, None);
      } else {
        // 无键的项经 SETLIST 批量写入，快速初始化大数组
        let temp = (array_chunk_reg as u32 + array_chunk_current) as u8;
        if i + 1 == expr_ref.items.size {
          // Safety: value 为存活表项节点，&mut 借用只覆盖该次调用。
          mult_ret = self.compile_expr_temp_mult_ret(unsafe { &mut *value }, temp);
        } else {
          // Safety: value 为存活表项节点，temp 是本块连续数组槽。
          self.compile_expr_temp_top(unsafe { &mut *value }, temp);
        }
        array_chunk_current += 1;
      }
    }

    // 冲刷最后一个数组块；若末表达式是多返回值需走 multret 形态
    if array_chunk_current != 0 {
      self.bc_mut().emit_abc(
        LuauOpcode::LOP_SETLIST,
        reg,
        array_chunk_reg,
        if mult_ret {
          0
        } else {
          (array_chunk_current + 1) as u8
        },
      );
      self.bc_mut().emit_aux(array_index);
    }

    if target != reg {
      self.bc_mut().emit_abc(LuauOpcode::LOP_MOVE, target, reg, 0);
    }
  }

  /// 对应 cpp `compileExprFunction`（cpp/Compiler/src/Compiler.cpp:1632）：发射
  /// CLOSURE/DUPCLOSURE 与 upvalue 捕获序列。`expr` 为 parser arena 存活、RTTI 确为
  /// `AstExprFunction` 的闭包字面量节点，且已在 `functions` 表登记（缺条目以类型化
  /// 编译错误上抛）；`target` 须为已分配寄存器编号——CLOSURE 指令写该槽。常量捕获
  /// 借内部 RegScope 临时占寄存器，守卫析构回卷水位。
  pub(crate) fn compile_expr_function(
    &mut self,
    expr: impl Into<Node<AstExprFunction>>,
    target: u8,
  ) {
    // cpp:1633 `RegScope rs(this);`：常量捕获会临时占用寄存器，作用域结束必须回退
    // regTop，否则同一函数内后续表达式的寄存器分配整体后移。
    let _rs = self.reg_scope();
    // expr 系调用方（分发器/stat.func 字段/门面借用）交出的 parser arena
    // 存活节点句柄（compile_or_throw 持有 parse arena 至编译结束），编译链对其
    // 只读；借用止于本函数末尾且不与 self 字段重叠（契约见 `Node::borrow`）。
    let expr = expr.into();
    let func_ref = expr.borrow();

    // cpp:1634-1635 `LUAU_ASSERT(f)`：functions 表缺条目属于编译器内部状态被破坏，
    // 以类型化编译错误上抛，不用 unwrap/panic 掩盖。
    let (fid, upvals) = match self.functions.find(&expr) {
      Some(f) => (f.id, f.upvals.clone()),
      None => CompileError::raise(
        &func_ref.base.base.location,
        format_args!("Internal error: no compiled function for closure expression"),
      ),
    };

    // 闭包带 upvalue 时，运行时靠这条记录创建闭包
    // 闭包无 upvalue 时用常量闭包，技术上并不依赖子函数列表
    // 但仍必须登记子函数——调试器设置断点时依赖函数层级结构
    let pid = self.bc_mut().add_child_function(fid);
    if pid < 0 {
      CompileError::raise(
        &func_ref.base.base.location,
        format_args!("Exceeded closure limit; simplify the code to compile"),
      );
    }

    // cpp:1645-1646：captures 复用同一 scratch 缓冲区（本函数非重入）
    self.captures.clear();
    self.captures.reserve(upvals.len());
    for uv in upvals {
      // cpp:1651 `LUAU_ASSERT(uv->functionDepth < expr->functionDepth)`
      // uv 来自 compile_function 阶段登记的 upvalue 列表（arena 存活 AstLocal
      // 句柄，比编译器长寿），此处仅读 function_depth。
      LUAU_ASSERT!(uv.borrow().function_depth < func_ref.function_depth);
      let reg = self.get_local_reg(uv);
      if reg >= 0 {
        // 注：无法判定 uv 是否为当前帧的 upvalue——内联会把 upvalue 迁移为 local
        let immutable = self.variables.find(&uv).is_none_or(|ul| !ul.written);
        self.captures.push(Capture {
          r#type: if immutable {
            LuauCaptureType::LCT_VAL
          } else {
            LuauCaptureType::LCT_REF
          },
          data: reg as u8,
        });
      } else if let Some(uc) = self
        .locstants
        .find(&uv)
        // cpp:1661 `uc && uc->type != Constant::Type_Unknown`：locstants 允许存在
        // Unknown 占位（内联还原、未折叠的参数），此时不能按常量编译，
        // 必须回落到 else 分支从父帧取 upvalue。
        .filter(|uc| !uc.is_unknown())
        .copied()
      {
        // 内联可能把常量变成 upvalue 捕获，此时不经临时寄存器无法完成捕获
        let reg = self.alloc_reg(&func_ref.base.base, 1);
        // expr 存活（门面借用），compile_expr_constant 只读其 location、把常量
        // 写入 reg（alloc_reg 刚分配的寄存器）。
        self.compile_expr_constant(&func_ref.base, &uc, reg);
        self.captures.push(Capture {
          r#type: LuauCaptureType::LCT_VAL,
          data: reg,
        });
      } else {
        // cpp:1671；wrapping_sub 与上游无符号减法一致（此处 function_depth >= 1 恒成立）
        // uv 同上为存活 AstLocal 句柄，仅读取。
        LUAU_ASSERT!(uv.borrow().function_depth < func_ref.function_depth.wrapping_sub(1));

        // 从父帧取 upvalue
        // 注：如有必要，这会把 uv 加入当前 upvalue 列表
        // uv 为存活 AstLocal 句柄；get_upval 仅将其作 upvalues map 键并按
        // cpp 逻辑登记到当前函数，不解引用写入。
        let uid = self.get_upval(uv.borrow());
        self.captures.push(Capture {
          r#type: LuauCaptureType::LCT_UPVAL,
          data: uid,
        });
      }
    }
    let mut shared = -1i16;
    if self.options.optimization_level >= 1 && self.should_share_closure(expr) && !self.setfenv_used
    {
      let cid = self.bc_mut().add_constant_closure(fid);
      if (0..K_MAX_AD_INDEX).contains(&cid) {
        shared = cid as i16;
      }
    }
    // cpp:shared<0 时备注闭包上值数（仅 Dump_Remarks 生效）
    if shared < 0 {
      let count = self.captures.len();
      self
        .bc_mut()
        .add_debug_remark(format_args!("allocation: closure with {count} upvalues"));
    }
    if shared >= 0 {
      self
        .bc_mut()
        .emit_ad(LuauOpcode::LOP_DUPCLOSURE, target, shared);
    } else {
      self
        .bc_mut()
        .emit_ad(LuauOpcode::LOP_NEWCLOSURE, target, pid);
    }
    // take 出走借用按值迭代（Capture 为 Copy），使 bc_mut 的 &mut self
    // 与 captures 读取不重叠；收尾归位保留 scratch 缓冲区容量供下次复用。
    let captures = take(&mut self.captures);
    for c in &captures {
      self
        .bc_mut()
        .emit_abc(LuauOpcode::LOP_CAPTURE, c.r#type as u8, c.data, 0);
    }
    self.captures = captures;
  }

  /// `target_count` 必须为 1（入口断言），`target` 起 `regs` 个寄存器须已在当前
  /// 窗口分配且与 `target_top`/`mult_ret` 协议一致；`expr` 为 select-vararg
  /// 优化模式匹配后传入的存活调用节点（本函数只读其子树）。
  pub(crate) fn compile_expr_select_vararg(
    &mut self,
    expr: &AstExprCall,
    target: u8,
    target_count: u8,
    target_top: bool,
    mult_ret: bool,
    regs: u8,
  ) {
    LUAU_ASSERT!(target_count == 1);
    let expr_ref = expr;
    LUAU_ASSERT!(!expr_ref.self_);
    LUAU_ASSERT!(expr_ref.args.size == 2);
    // size==2 断言下 as_slice 界内取两槽，取代裸 data.add(i)：
    // 槽元素为 parser 保证非空的表达式指针（越界仅剩可诊断的 panic，不再是 UB 读）
    let arg = expr_ref.args.as_slice()[0];
    let arg_varargs = expr_ref.args.as_slice()[1];
    // 门面判型：arg_varargs 由上方 select 特判保证为 arena 存活 AstExpr 指针。
    LUAU_ASSERT!(matches!(
      Node::from(arg_varargs).as_expr_ref(),
      AstExprRef::Varargs(_)
    ));

    let argreg: u8;
    let reg = self.get_expr_local_reg(arg);
    if reg >= 0 {
      argreg = reg as u8;
    } else {
      argreg = regs + 1;
      // Safety: arg 为存活索引参数表达式；regs+1 是为本调用预留的实参槽。
      self.compile_expr_temp_top(unsafe { &mut *arg }, argreg);
    }

    let fastcall_label = self.bc().emit_label();

    self.bc_mut().emit_abc(
      LuauOpcode::LOP_FASTCALL1,
      LuauBuiltinFunction::LBF_SELECT_VARARG as u8,
      argreg,
      0,
    );

    // Safety: expr_ref.func 为 parser 保证非空存活的被调表达式（此处为 `select`），
    // regs 为其基寄存器。
    self.compile_expr(unsafe { &mut *expr_ref.func }, regs, true);

    if argreg != regs + 1 {
      self
        .bc_mut()
        .emit_abc(LuauOpcode::LOP_MOVE, regs + 1, argreg, 0);
    }

    self
      .bc_mut()
      .emit_abc(LuauOpcode::LOP_GETVARARGS, regs + 2, 0, 0);

    let call_label = self.bc().emit_label();
    if !self.bc_mut().patch_skip_c(fastcall_label, call_label) {
      CompileError::raise(
        // 门面解引用：func 同上存活，仅读 location 用于报错。
        &ast_slot_ref(expr_ref.func)
          .expect("expr_ref.func 为 parser 接线的存活被调表达式指针")
          .base
          .location,
        core::format_args!("{ERR_EXCEEDED_JUMP_DISTANCE_LIMIT}"),
      );
    }

    self.bc_mut().emit_abc(
      LuauOpcode::LOP_CALL,
      regs,
      0,
      if mult_ret { 0 } else { target_count + 1 },
    );

    if !target_top {
      self.fill_regs_move(target, regs, target_count);
    }
  }
}
