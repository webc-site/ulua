//! Source: `VM/src/lvmexecute.cpp:228-3722` (hand-port in progress; design
//! card at `translation/design-cards/lvmexecute.md`)
//!
//! ALL 89 `VM_CASE` opcode arms live HERE as match arms (the per-arm
//! `vm_case_lvmexecute*.rs` node files are macro-extraction artifacts and
//! become doc-pointers as their arms land). Control-flow mapping:
//! `VM_NEXT()` -> `continue 'dispatch`; `VM_CONTINUE(op)` -> set
//! `continue_op` + `continue 'dispatch`; `goto exit` -> `return`.
//!
//! 位段宽度审计（`LUAU_INSN_*` 定义于 ulua-common/src/macros/）：A/B/C/OP
//! 位段 8 位（值 ≤255，`as i32`/`as usize`/`as u8` 无损）；D/E 由
//! `(insn as i32) >> 16` / `>> 8` 有符号移位得到 i32（`as isize` 符号扩展
//! 无损，`as f64` 在 16/24 位值域内精确）；enum 判别值 <256，`as u8` 无损。
//! 本文件中位段相关 `as` 转换均无静默截断。

use core::ptr::{addr_of, eq};
/// luaG_typeerrorL 的固定错误消息（cpp: "iterate over"）
const ERR_ITERATE_OVER: &str = "iterate over";

use core::{ffi::c_void, ptr::null};

use ulua_common::{
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  fflag,
  macros::{
    luau_assert::LUAU_ASSERT,
    luau_insn_ops::{
      LUAU_INSN_FBSLOT_SEALED, luau_insn_a, luau_insn_aux_a, luau_insn_aux_b, luau_insn_aux_kb,
      luau_insn_aux_kv, luau_insn_aux_kv16, luau_insn_aux_not, luau_insn_aux_slot, luau_insn_b,
      luau_insn_c, luau_insn_d, luau_insn_e, luau_insn_op,
    },
  },
};

use crate::{
  enums::{lua_type::LuaType, tms::TMS, value_view::ValueView},
  functions::{
    copy_results_pop_frame::pop_frame_copy_results, lua_d_call::lua_d_call,
    lua_d_check_cstack::lua_d_check_cstack, lua_d_performcally::lua_d_performcally,
    lua_f_close::lua_f_close, lua_f_findupval::lua_f_findupval,
    lua_f_new_lclosure::lua_f_new_lclosure, lua_f_recordhit::lua_f_recordhit,
    lua_g_methoderror::lua_g_methoderror, lua_g_missingmembererror::lua_g_missingmembererror,
    lua_g_typeerror_l::lua_g_typeerror_l, lua_h_clone::lua_h_clone, lua_h_getn::lua_h_getn,
    lua_h_getstr::lua_h_getstr, lua_h_new::lua_h_new, lua_h_resizearray::lua_h_resizearray,
    lua_h_setstr::lua_h_setstr, lua_o_rawequal_obj::lua_o_rawequal_obj,
    lua_r_addclassmember::lua_r_addclassmember, lua_r_cloneclass::lua_r_cloneclass,
    lua_r_inheritclass::lua_r_inheritclass, lua_t_gettmbyobj::lua_t_gettmbyobj,
    lua_v_call_tm::lua_v_call_tm, lua_v_concat::lua_v_concat, lua_v_doarithimpl::lua_v_doarithimpl,
    lua_v_dolen::lua_v_dolen, lua_v_equalval::lua_v_equalval, lua_v_getimport::lua_v_getimport,
    lua_v_gettable::lua_v_gettable, lua_v_lessequal::lua_v_lessequal,
    lua_v_lessthan::lua_v_lessthan, lua_v_prepare_forn::lua_v_prepare_forn,
    lua_v_settable::lua_v_settable, lua_v_strcmp::lua_v_strcmp, lua_v_tryfunc_tm::lua_v_tryfunc_tm,
    luai_numidiv::luai_numidiv, luai_nummod::luai_nummod, luai_veceq::luai_veceq,
    luau_callhook::luau_callhook, luau_setupcci::luau_setupcci, luau_skipstep::luau_skipstep,
    set_iterator_done::set_iterator_done, set_iterator_index::set_iterator_index,
  },
  macros::{
    classvalue::classvalue, fastnotm::fastnotm, gcvalue::gcvalue, getnodekey::getnodekey,
    gkey::gval, gnext::gnext, gval_2_slot::gval2slot, incr_ci::incr_ci, is_lua::isLua,
    lightuserdatatag::lightuserdatatag, lu_tag_iterator::LU_TAG_ITERATOR,
    lua_c_barrier::lua_c_barrier, lua_c_barrierfast::lua_c_barrierfast,
    lua_c_barriert::luaC_barriert, lua_c_check_gc::lua_c_check_gc,
    lua_callinfo_native::LUA_CALLINFO_NATIVE, lua_callinfo_return::LUA_CALLINFO_RETURN,
    lua_d_checkstack::luaD_checkstack, lua_d_checkstackfornewci::lua_d_checkstackfornewci,
    lua_g_typeerror::luaG_typeerror, lua_multret::LUA_MULTRET, lua_o_nilobject::LUA_O_NILOBJECT,
    lua_r_lookupmemberatoffset::luaR_lookupmemberatoffset, luai_maxccalls::LUAI_MAXCCALLS,
    luau_f_table::LUAU_F_TABLE, lvalue::lvalue, objectvalue::objectvalue, pvalue::pvalue,
    setbvalue::setbvalue, setclassvalue::setclassvalue, setclvalue::setclvalue,
    sethvalue::sethvalue, setnilvalue::setnilvalue, setnvalue::setnvalue, setobj::setobj,
    setobj_2_s::setobj_2_s, setobj_2_t::setobj2t, setupvalue::setupvalue, setvvalue::setvvalue,
    sizenode::sizenode, ttype::ttype, upvalue::upvalue, vm_check_gc::VM_CHECK_GC,
    vm_interrupt::VM_INTERRUPT, vm_kv::VM_KV, vm_patch_aux::vm_patch_aux,
    vm_patch_aux_slot::vm_patch_aux_slot, vm_patch_c::vm_patch_c, vm_patch_e::vm_patch_e,
    vm_patch_op::vm_patch_op, vm_protect::vm_protect, vm_reg::VM_REG, vm_uv::VM_UV,
  },
  records::{
    closure::{Closure, LClosure},
    lua_node::LuaNode,
    lua_state::LuaState,
    vm_frame::VmFrame,
  },
  type_aliases::{
    instruction::Instruction, lua_userdata_direct_field_get::from_ptr, stk_id::StkId,
    t_value::TValue,
  },
};

/// cpp `cl->l.p` 的固定三连收敛：取闭包 `cl` 的 Proto 指针。
/// 执行循环里 30+ 个 opcode 臂与断言都要读它，统一走这一个入口。
///
/// Safety（由调用侧 unsafe 上下文承担）：`cl` 须指向存活的 `Closure`；
/// `p` 在闭包存续期内不失效。
macro_rules! cl_proto {
  ($cl:expr) => {{
    let l = &(*$cl).inner.l;
    l.p
  }};
}

/// cpp/VM/src/lvmexecute.cpp 里 `pc += cond ? LUAU_INSN_D(insn) : 1;` +
/// `VM_ASSERT_PC(pc);` + `VM_NEXT();` 的固定三连（lvmexecute.cpp:86）：JUMPIF*
/// 系列 opcode 臂算出跳转条件后，按条件推进 pc、断言 pc 仍落在当前 proto 的
/// code 范围内，再回到 dispatch 循环取指。
///
/// 6 个 JUMPIF* 臂共用这一份定义；`pc`/`cl`/`insn` 与 dispatch 循环标签全部
/// 显式传参，不依赖声明宏自由标识符的解析，避免宏体与臂内局部变量的 hygiene
/// 歧义（任一处的 pc 断言修正只需改这里一次）。
macro_rules! jump_and_next {
  ($pc:ident, $cl:expr, $insn:expr, $label:lifetime, $cond:expr) => {{
    $pc = $pc.offset(if $cond { luau_insn_d($insn) as isize } else { 1 });
    let p = cl_proto!($cl);
    LUAU_ASSERT!(($pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
    continue $label;
  }};
}

macro_rules! jump_if_false_and_next {
  ($pc:ident, $cl:expr, $insn:expr, $label:lifetime, $cond:expr) => {{
    $pc = $pc.offset(if $cond { 1 } else { luau_insn_d($insn) as isize });
    let p = cl_proto!($cl);
    LUAU_ASSERT!(($pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
    continue $label;
  }};
}

/// cpp FASTCALL* 系列 opcode 臂的通用快速调用分派骨架（lvmexecute.cpp:3183-3350
/// 查表、safeenv 校验、savedpc 保护、LUA_MULTRET 处理与 pc 跳步）。
macro_rules! dispatch_fastcall {
  ($l:ident, $cl:ident, $pc:ident, $base:ident, $bfid:expr, $skip:expr, $arg1:expr, $args_opt:expr, $nparams:expr) => {{
    {
      let p = cl_proto!($cl);
      LUAU_ASSERT!((($pc.offset_from((*p).code) as i32 + $skip) as u32) < (*p).sizecode as u32);
    }
    let call: Instruction = *$pc.add($skip as usize);
    LUAU_ASSERT!(luau_insn_op(call) == LuauOpcode::LOP_CALL as u32);
    let ra = VM_REG!(luau_insn_a(call), $l, $base);
    let nresults = luau_insn_c(call) as i32 - 1;
    if let Some(f) = LUAU_F_TABLE[$bfid as usize]
      && (*(*$cl).env).safeenv != 0
    {
      (*(*$l).ci).savedpc = $pc;
      let n = f($l, ra, $arg1, nresults, $args_opt, $nparams);
      if n >= 0 {
        if nresults == LUA_MULTRET {
          (*$l).top = ra.add(n as usize);
        }
        $pc = $pc.add(($skip + 1) as usize);
        let p = cl_proto!($cl);
        LUAU_ASSERT!(($pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
      }
    }
  }};
}

/// GETUDATAKS/SETUDATAKS 快路径单源（lvmexecute.cpp:3441-3512/3516-3587）：两臂
/// 仅差 udatadirect 字段对（index/indextm ↔ newindex/newindextm）、是否追压新值
/// 实参、setupcci 结果数、deopt 回查 opcode 与是否回写结果；tag 取值、压栈、
/// C 栈计数、cachedslot 回补、帧弹出与 base 刷新逐句相同，收敛于此。`$val` 为
/// SET 追压的写值槽（GET 传 `None::<*const TValue>`，死分支被折叠消除），
/// `$grab_result` 仅 GET 为真。生成码与原两臂手写逐语句一致。
macro_rules! udata_direct_fast {
  ($l:ident, $pc:ident, $base:ident, $insn:expr, $rb:expr, $aux:expr, $kidx:expr, $kv:expr,
   $fn_field:ident, $tm_field:ident, $val:expr, $nres:expr, $deopt_op:ident, $grab_result:expr, $label:lifetime) => {{
    'udata_fast: {
      // §11 pass B: rb 的 userdata tag 判定收敛为 ValueView::Userdata 匹配
      // （uvalue! 同为 *const Udata，payload 语义不变）
      if let ValueView::Userdata(u) = ValueView::from_tvalue(&*$rb) {
        // cpp: `int utag = uvalue(rb)->tag;`（lvmexecute.cpp:3445/3519/3588）
        let utag = (*u).tag as usize;
        let udatadirect = &mut (*(*$l).global).udatadirect[utag];
        let onudata = udatadirect.$fn_field;
        let tm = &mut udatadirect.$tm_field as *mut TValue;

        if let Some(onudata) = onudata
          && !matches!(ValueView::from_tvalue(&*tm), ValueView::Nil)
        {
          // cpp: `void* udata = uvalue(rb)->data;`（lvmexecute.cpp:3452/3526/3595）
          let udata = (*u).data.as_ptr() as *mut c_void;

          // note: it's safe to push arguments past top for
          // complicated reasons (see top of the file)
          let nargs = 3 + usize::from($val.is_some());
          LUAU_ASSERT!((*$l).top.add(nargs) < (*$l).stack.add((*$l).stacksize as usize));
          let top = (*$l).top;
          setobj_2_s!($l, top.add(0), tm);
          setobj_2_s!($l, top.add(1), $rb);
          setobj_2_s!($l, top.add(2), $kv);
          if let Some(v) = $val {
            setobj_2_s!($l, top.add(3), v);
          }
          (*$l).top = top.add(nargs);

          (*(*$l).ci).savedpc = $pc;

          (*$l).n_ccalls += 1;

          if ((*$l).n_ccalls as i32) >= LUAI_MAXCCALLS {
            lua_d_check_cstack($l);
          }

          luau_setupcci($l, $nres, top);

          let mut cachedslot: u16 = luau_insn_aux_slot($aux) as u16;
          onudata(
            $l,
            udata,
            (*(*$kv).as_string_ptr()).atom as i32,
            &mut cachedslot,
            utag as i32,
          );

          // update cached slot if instruction didn't deoptimize
          if cachedslot as u32 != luau_insn_aux_slot($aux)
            && luau_insn_op(*$pc.sub(2)) == LuauOpcode::$deopt_op as u32
          {
            vm_patch_aux_slot($pc.sub(1), $kidx, cachedslot as i32);
          }

          // ci is our callinfo, cip is our parent
          let ci = (*$l).ci;
          let cip = ci.sub(1);

          (*$l).ci = cip;
          (*$l).base = (*cip).base;
          if !$grab_result {
            (*$l).top = (*cip).top;
          }
          (*$l).n_ccalls -= 1;

          // stack may have been reallocated, so we need to refresh base ptr
          $base = (*$l).base;

          if $grab_result {
            let ra = VM_REG!(luau_insn_a($insn), $l, $base);

            // grab result while l->top is still pointed to the
            // previous function frame
            setobj_2_s!($l, ra, (*$l).top.sub(1));

            // then update top
            (*$l).top = (*cip).top;
          }

          continue $label;
        }
      }
      break 'udata_fast;
    }
  }};
}

/// LOP_CALL/LOP_CALLFB 单源（lvmexecute.cpp:1038/1145）：两臂除 CALLFB 的
/// feedback slot 读取、SEALED 回补与命中记录外逐行同构。`$fb` 为反馈槽快照：
/// CALL 传 `None::<Instruction>`（三处反馈桩被 LLVM 常量折叠消除，与无反馈版
/// 逐字节一致），CALLFB 传 `Some(*pc)`（此刻 pc 尚未越过 aux 槽）。生成码与
/// 原两臂手写逐语句一致，判定顺序、GC/重入点一律不动。
macro_rules! call_arm {
  ($l:ident, $pc:ident, $base:ident, $cl:ident, $k:ident, $fb:expr, $label:lifetime) => {{
    VM_INTERRUPT!($l, $pc, $base);
    let insn = *$pc;
    $pc = $pc.add(1);
    let feedback_slot: Option<Instruction> = $fb;
    if feedback_slot.is_some() {
      $pc = $pc.add(1);
    }
    let ra = VM_REG!(luau_insn_a(insn), $l, $base);

    let nparams = luau_insn_b(insn) as i32 - 1;
    let nresults = luau_insn_c(insn) as i32 - 1;

    let mut argtop = (*$l).top;
    argtop = if nparams == LUA_MULTRET {
      argtop
    } else {
      ra.add(1 + nparams as usize)
    };

    // slow-path: not a function call
    if !(*ra).is_function() {
      if let Some(fb) = feedback_slot
        && fb != LUAU_INSN_FBSLOT_SEALED
      {
        vm_patch_aux($pc.sub(1), LUAU_INSN_FBSLOT_SEALED as i32);
      }

      (*(*$l).ci).savedpc = $pc; // vm_protect_pc(): luaV_tryfuncTM may fail

      lua_v_tryfunc_tm($l, ra);
      argtop = argtop.add(1); // __call adds an extra self
    }

    let ccl = (*ra).as_closure_ptr();
    (*(*$l).ci).savedpc = $pc;

    incr_ci!($l);
    let ci = (*$l).ci;
    (*ci).func = ra;
    (*ci).base = ra.add(1);
    // note: technically UB since we haven't reallocated the stack yet
    (*ci).top = argtop.add((*ccl).stacksize as usize);
    (*ci).savedpc = null();
    (*ci).flags = 0;
    (*ci).nresults = nresults;

    (*$l).base = (*ci).base;
    (*$l).top = argtop;

    // note: this reallocs stack, but we don't need to VM_PROTECT this
    // this is because we're going to modify base/savedpc manually anyhow
    // crucially, we can't use ra/argtop after this line
    lua_d_checkstackfornewci($l, (*ccl).stacksize as i32);

    LUAU_ASSERT!((*ci).top <= (*$l).stack_last);

    if (*ccl).is_c == 0 {
      let p = cl_proto!(ccl);

      if let Some(fb) = feedback_slot
        && fb != LUAU_INSN_FBSLOT_SEALED
        && !lua_f_recordhit($l, $cl, ccl, fb)
      {
        vm_patch_aux($pc.sub(1), LUAU_INSN_FBSLOT_SEALED as i32);
      }

      // fill unused parameters with nil
      let mut argi = (*$l).top;
      let argend = (*$l).base.add((*p).numparams as usize);
      while argi < argend {
        setnilvalue!(argi);
        argi = argi.add(1);
      }
      (*$l).top = if (*p).is_vararg != 0 { argi } else { (*ci).top };

      // reentry
      // codeentry may point to NATIVECALL instruction when proto is
      // compiled to native code; execution continues in native code.
      // note that p->codeentry may point *outside* of
      // p->code..p->code+p->sizecode, but that pointer never gets
      // saved to savedpc.
      $pc = if SINGLE_STEP {
        (*p).code
      } else {
        (*p).codeentry
      };
      $cl = ccl;
      $base = (*$l).base;
      $k = (*p).k;
      continue $label;
    } else {
      if let Some(fb) = feedback_slot
        && fb != LUAU_INSN_FBSLOT_SEALED
      {
        vm_patch_aux($pc.sub(1), LUAU_INSN_FBSLOT_SEALED as i32);
      }

      let func = {
        let c = &(*ccl).inner.c;
        c.f
      };
      let n = match func {
        Some(f) => f($l),
        None => 0,
      };

      // yield
      if n < 0 {
        return; // goto exit
      }

      // 将返回值拷回父栈（最多 nresults 个），不足补 nil，并弹出本帧
      // （lvmexecute.cpp:1131-1142，见 pop_frame_copy_results）
      pop_frame_copy_results($l, (*$l).top.sub(n as usize), (*$l).top, nresults);

      // stack may have been reallocated, so we need to refresh base ptr
      $base = (*$l).base;
      continue $label;
    }
  }};
}

/// Vector×标量四分写臂尾：`frame.lanes` 一次取 lane 段、`op_fn` 按 lane0/1/2/3
/// 序写 `ra` 后回 dispatch。寄存器变体（标量取 rc 的 Number 载荷）与 K 常量变体
/// （标量取数值常量）共享本单源，`$scalar` 为纯读表达式（f64→f32 透传 cast），
/// 求值序与旧 `let vc` 提前绑定不可观察差异；判定顺序不动。
macro_rules! vec_scalar_op {
  ($frame:ident, $ra:expr, $vslot:expr, $scalar:expr, $op_fn:expr, $label:lifetime) => {{
    let vb = $frame.lanes($vslot);
    let op_fn = $op_fn;
    setvvalue!(
      $ra,
      op_fn(vb[0], $scalar),
      op_fn(vb[1], $scalar),
      op_fn(vb[2], $scalar),
      op_fn($frame.lane_at($vslot, 3), $scalar)
    );
    continue $label;
  }};
}

/// 算术 opcode 族（ADD/SUB/MUL/DIV/IDIV/MOD/POW/UNM/SUBRK/DIVRK 各臂及其
/// K 常量变体）slow-path 臂尾单源：`vm_protect!` 保护下调用 `lua_v_doarithimpl`
/// 回 dispatch，九处变体只差操作数槽位与 tm；判定顺序、快慢路划分一律不动。
macro_rules! arith_slow {
  ($l:ident, $pc:ident, $base:ident, $ra:expr, $rb:expr, $rc:expr, $tm:expr, $label:lifetime) => {{
    // slow-path, may invoke C/Lua via metamethods
    vm_protect!($l, $pc, $base, {
      lua_v_doarithimpl($l, $ra, $rb, $rc, $tm);
    });
    continue $label;
  }};
}

/// GETTABLE 慢路径（cpp `VM_PROTECT(luaV_gettable(L, rb, rc, ra))`，lvmexecute.cpp:795）：
/// 非表/非数字键、数组段越界、非整数键与 `__index` 元方法都在此收敛。
///
/// 独立成 `#[cold]` 外壳只为把布局意图交给 LLVM：cpp 靠 `LUAU_LIKELY` 给数组命中分支
/// 加权（lvmexecute.cpp:785），Rust 没有稳定的分支权重提示，若慢路径与快路径同处一个
/// 内联区，块摆放会把数组拷贝甩到函数尾部（表访问密集循环里每次命中多出两次远距离
/// 跳转）。返回值而非 `&mut base` 出参：入参地址一旦外泄，`base` 就退化成栈槽，热路径
/// 每条指令都得从 `[sp]` 重载（同理见 review.md §3「出参 → 返回值」）。
///
/// # Safety（内部 unsafe 块契约，签名安全：调用方无需 unsafe 上下文）
/// `l` 指向当前执行的存活 `lua_State` 且 `ci` 活动；`pc` 落在当前 proto code 数组内；
/// `ra` 为活栈槽，`rb`/`rc` 指向活栈槽——即 opcode 臂内的原语不变量。
/// 返回回调后新的活动帧基址（cpp `VM_PROTECT` 之后的 `base = L->base`）。
#[cold]
#[inline(never)]
fn gettable_slow(
  l: *mut LuaState,
  pc: *const Instruction,
  ra: StkId,
  rb: StkId,
  rc: StkId,
) -> StkId {
  // Safety: 契约同上；序列与臂内原地 `vm_protect!` 展开逐字一致。
  unsafe {
    (*(*l).ci).savedpc = pc;
    lua_v_gettable(l, rb, rc, ra);
    (*l).base
  }
}

/// SETTABLE 慢路径（cpp `VM_PROTECT(luaV_settable(L, rb, rc, ra))`，lvmexecute.cpp:826）：
/// 冷外壳与返回值理由同 [`gettable_slow`]。
#[cold]
#[inline(never)]
fn settable_slow(
  l: *mut LuaState,
  pc: *const Instruction,
  ra: StkId,
  rb: StkId,
  rc: StkId,
) -> StkId {
  // Safety: 契约同 [`gettable_slow`]。
  unsafe {
    (*(*l).ci).savedpc = pc;
    lua_v_settable(l, rb, rc, ra);
    (*l).base
  }
}

/// cpp 各 opcode 臂 C 元方法快速路径的固定样板（lvmexecute.cpp 多处
/// `VM_PROTECT_PC(); luaV_callTM(L, n, res)`）：把 `[tm, args..]` 压栈、
/// 记录 savedpc、调用 `luaV_callTM`，随后刷新 `base`。
/// `cachedslot` 回填 / `vm_patch_c` 等差异步骤留在调用点，语义与逐臂手展一致。
///
/// # Safety（内部 unsafe 块契约，签名安全：调用方无需 unsafe 上下文）
///
/// `l` 指向当前执行的存活 `LuaState` 且其 `ci` 活动；`tm` 非空且为 is_c 闭包（调用点已判）；
/// `args` 各项指向存活栈槽；`l->top + args.len() + 1` 仍在栈预留区内。
#[inline(always)]
fn call_c_tm(
  l: *mut LuaState,
  pc: *const Instruction,
  base: &mut StkId,
  tm: *const TValue,
  args: &[*const TValue],
  res: i32,
) {
  // SAFETY: 上述契约保证 l/tm/args 有效，且 top+nparams+1 在 stack..stack+stacksize 内（下方 LUAU_ASSERT 兜底）
  unsafe {
    let nparams = args.len();
    // note: it's safe to push arguments past top for
    // complicated reasons (see top of the file)
    LUAU_ASSERT!((*l).top.add(nparams + 1) < (*l).stack.add((*l).stacksize as usize));
    let top = (*l).top;
    setobj_2_s!(l, top, tm);
    for (i, &arg) in args.iter().enumerate() {
      setobj_2_s!(l, top.add(i + 1), arg);
    }
    (*l).top = top.add(nparams + 1);

    (*(*l).ci).savedpc = pc;
    lua_v_call_tm(l, nparams as i32, res);
    *base = (*l).base;
  }
}

/// cpp 哈希节点快速路径的公共判据：预期槽位上是与常量 `kv` 同一的字符串键，
/// 且值非 nil（`LOP_GETGLOBAL/SETGLOBAL/GETTABLEKS/SETTABLEKS/NAMECALL` 等臂
/// 共用的三连判定，lvmexecute.cpp:386/417/520/547/676/947/1010）。
///
/// 键轴两判据均收敛为视图 match（§11 路线图：B2a [`TKeyView`] + pass B 次波
/// [`ValueView`]）：节点键命中 `TKeyView::String`、常量槽命中 `ValueView::String`
/// 后作指针同一性比较，取代原先「假定 kv 为字符串」的 `(*kv).as_string_ptr()` 直读；kv 的
/// 字符串不变量仍由各调用臂顶部的 `LUAU_ASSERT!((*kv).is_string())` 在 debug 下校验
/// （字节码装载期已保证，故非字符串 kv 不可达）。ptr::eq 与原裸指针 `==` 同为地址同一性。
///
/// 纯读取比较，签名安全：`n`/`kv` 以引用传入，裸指针 → 引用的解引用边界由调用点
/// （[`luau_execute_impl`] 的 unsafe 上下文）承担，函数体内 unsafe 只剩经
/// `as_string_ptr` 读取 GC 载荷的一处。三判据均为无副作用纯读，`!is_nil` 提前于
/// 指针同一性比较不改变可观察行为，且短路语义保持（键非字符串时不读 kv 载荷）。
#[inline(always)]
fn gslot_hit(n: &LuaNode, kv: &TValue) -> bool {
  // 对照 cpp lvmexecute.cpp:408: ttisstring(gkey(n)) && tsvalue(gkey(n)) == tsvalue(kv) && !ttisnil(gval(n))
  if !n.key.is_string() || n.val.is_nil() {
    return false;
  }
  // SAFETY: 键 tag 已判 is_string（上方短路），kv 的字符串不变量由各调用臂顶部的
  // `LUAU_ASSERT!((*kv).is_string())` 校验（字节码装载期已保证，故非字符串 kv 不可达），
  // tag 收敛读取 GC 分支合法；此处仅读两侧 TString 指针作同一性比较，不解引用其内容
  unsafe { eq(n.key.as_string_ptr(), kv.as_string_ptr()) }
}

/// C++ `void luau_execute(LuaState* l)` (lvmexecute.cpp:3716) — dispatches
/// to the `template<bool SingleStep>` monomorphs.
///
/// # Safety（内部 unsafe 块契约，签名安全：调用方全部在 crate 内，无需 unsafe 上下文）
///
/// `l` 必须指向存活且 `isactive` 的 `LuaState`，其 `ci` 当前为 Lua 闭包帧（入口
/// `LUAU_ASSERT!(isLua!((*l).ci))` 兜底）、`base/cl/k/pc` 解释器状态已由 `luaD_precall`
/// 建立，且同一 VM 状态任一时刻仅单线程执行。
pub(crate) fn luau_execute(l: *mut LuaState) {
  // SAFETY: 契约保证 l 为就绪的 Lua 帧状态，impl 两分支仅模板参数不同
  unsafe {
    if (*l).singlestep {
      luau_execute_impl::<true>(l)
    } else {
      luau_execute_impl::<false>(l)
    }
  }
}

/// C++ `template<bool SingleStep> static void luau_execute(LuaState* l)`
/// (lvmexecute.cpp:228). The computed-goto dispatch table becomes the match
/// below (both blindly index by the opcode byte).
///
/// # Safety（内部 unsafe 块契约，签名安全：唯一调用方 [`luau_execute`] 已持契约）
///
/// `l` 必须指向存活且 `isactive` 的 `LuaState`，其当前 callinfo 为 Lua 闭包
/// （入口 `LUAU_ASSERT!(isLua!((*l).ci))` 兜底），调用栈与解释器局部量
/// （`base`/`cl`/`k`/`pc`）均已就绪，且同一 VM 状态在任一时刻仅由单线程解释执行。
fn luau_execute_impl<const SINGLE_STEP: bool>(l: *mut LuaState) {
  // SAFETY: 契约保证 `l` 满足与 luau_execute 相同前置（存活且 isactive、当前 ci 为 Lua 闭包、单线程执行），const 分支仅切换单步开关
  unsafe {
    // the critical interpreter state, stored in locals for performance
    let mut cl: *mut Closure;
    let mut base: StkId;
    let mut k: *mut TValue;
    let mut pc: *const Instruction;

    LUAU_ASSERT!(isLua!((*l).ci));
    LUAU_ASSERT!((*l).isactive);
    // C++ also asserts !isblack(obj2gco(l)) — active threads never turn black.

    // VM_HAS_NATIVE entry: execution may continue in native code.
    if ((*(*l).ci).flags as i32 & LUA_CALLINFO_NATIVE) != 0 && !SINGLE_STEP {
      let native_cl = (*(*(*l).ci).func).as_closure_ptr();
      let native_lcl = addr_of!((*native_cl).inner.l).cast::<LClosure>();
      let p = (*native_lcl).p;
      LUAU_ASSERT!(!(*p).execdata.is_null());
      if let Some(enter) = (*(*l).global).ecb.enter
        && enter(l, p) == 0
      {
        return;
      }
    }

    // C++ `reentry:` label (goto target from NATIVECALL/RETURN native paths).
    'reentry: loop {
      LUAU_ASSERT!(isLua!((*l).ci));

      pc = (*(*l).ci).savedpc;
      cl = (*(*(*l).ci).func).as_closure_ptr();
      base = (*l).base;
      k = {
        let l = &(*cl).inner.l;
        (*l.p).k
      };

      // C++ `VM_CONTINUE(op)` re-dispatches WITHOUT refetching `*pc`.
      // 覆写值以裸 `u8` 承载、`LOP_NOP`(0) 作「无覆写」哨兵：`Option<u8>` 无可用
      // niche，判别位与载荷被拆成两个独立 SSA 值，于是每条指令派发回到循环头时都要
      // 双 `mov` 重置 None（A64 派发块实测 `mov w9,#0; mov w24,#0`）。五个覆写点
      // （LOP_CALL 落穿 / LOP_BREAK 的 debuginsn 原码 / GETTABLEKS / SETTABLEKS /
      // NAMECALL 三处 vm_patch 回写）写入的 opcode 恒非 0，与哨兵无冲突。
      let mut continue_op: u8 = LuauOpcode::LOP_NOP as u8;

      // 栈槽视图门面：把 from_raw_parts / 分量索引类指针算术关进带 `# Safety`
      // 契约的方法（见 records/vm_frame.rs），臂内不再手写。
      let frame = VmFrame::new(l);

      // C++ `dispatch:` label; `VM_NEXT()` == `continue 'dispatch`.
      'dispatch: loop {
        // Note: in C++ this assert block is bypassed by computed goto
        // except in single-step mode; asserts only.
        if SINGLE_STEP
          && continue_op == LuauOpcode::LOP_NOP as u8
          && (*(*l).global).cb.debugstep.is_some()
          && !luau_skipstep(luau_insn_op(*pc) as u8)
        {
          let debugstep = (*(*l).global).cb.debugstep;
          vm_protect!(l, pc, base, {
            luau_callhook(l, debugstep, None);
          });
          // allow debugstep hook to put thread into error/yield state
          if (*l).status != 0 {
            return; // goto exit
          }
        }

        let op: u8 = if continue_op != LuauOpcode::LOP_NOP as u8 {
          continue_op
        } else {
          luau_insn_op(*pc) as u8
        };
        continue_op = LuauOpcode::LOP_NOP as u8;

        // C++ 跳转表盲目索引 opcode 字节，越界时靠 `LUAU_UNREACHABLE()` 兜底
        // （等价 UB）。此处用 `From<u8>`：合法 opcode（< LopCount）结果与
        // C++ 逐字节一致；越界字节（损坏的 bytecode，C++ 同为 UB，无已定义
        // 行为可保留）钳到 `LopNop`，避免 UB。
        match LuauOpcode::from(op) {
          LuauOpcode::LOP_NOP => {
            // lvmexecute.cpp:319
            let insn = *pc;
            pc = pc.add(1);
            LUAU_ASSERT!(insn == 0);
            continue 'dispatch;
          }

          LuauOpcode::LOP_LOADNIL => {
            // lvmexecute.cpp:326
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);

            setnilvalue!(ra);
            continue 'dispatch;
          }

          LuauOpcode::LOP_LOADB => {
            // lvmexecute.cpp:335
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);

            setbvalue!(ra, luau_insn_b(insn) as i32);

            pc = pc.add(luau_insn_c(insn) as usize);
            let p = cl_proto!(cl);
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }

          LuauOpcode::LOP_LOADN => {
            // lvmexecute.cpp:347
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);

            setnvalue!(ra, luau_insn_d(insn) as f64);
            continue 'dispatch;
          }

          LuauOpcode::LOP_LOADK => {
            // lvmexecute.cpp:356
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let kv = VM_KV!(luau_insn_d(insn), cl, k);

            setobj_2_s!(l, ra, kv);
            continue 'dispatch;
          }

          LuauOpcode::LOP_MOVE => {
            // lvmexecute.cpp:366
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);

            setobj_2_s!(l, ra, rb);
            continue 'dispatch;
          }

          LuauOpcode::LOP_GETGLOBAL => {
            // lvmexecute.cpp:376
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k);
            LUAU_ASSERT!((*kv).is_string());

            // fast-path: value is in expected slot
            let h = (*cl).env;
            let slot = (luau_insn_c(insn) as i32) & (*h).nodemask8 as i32;
            let n = (*h).node.add(slot as usize);

            if gslot_hit(&*n, &*kv) {
              setobj_2_s!(l, ra, gval!(n));
              continue 'dispatch;
            } else {
              // slow-path, may invoke Lua calls via __index metamethod
              let mut g = TValue::default();
              sethvalue!(l, &mut g, h);
              (*l).cachedslot = slot;
              vm_protect!(l, pc, base, {
                lua_v_gettable(l, &g, kv, ra);
              });
              // save cachedslot to accelerate future lookups; patches
              // currently executing instruction since pc-2 rolls back two pc++
              vm_patch_c(pc.sub(2), (*l).cachedslot);
              continue 'dispatch;
            }
          }

          LuauOpcode::LOP_SETGLOBAL => {
            // lvmexecute.cpp:407
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k);
            LUAU_ASSERT!((*kv).is_string());

            // fast-path: value is in expected slot
            let h = (*cl).env;
            let slot = (luau_insn_c(insn) as i32) & (*h).nodemask8 as i32;
            let n = (*h).node.add(slot as usize);

            if gslot_hit(&*n, &*kv) && (*h).readonly == 0 {
              setobj2t!(l, gval!(n), ra);
              luaC_barriert!(l, h, ra);
              continue 'dispatch;
            } else {
              // slow-path, may invoke Lua calls via __newindex metamethod
              let mut g = TValue::default();
              sethvalue!(l, &mut g, h);
              (*l).cachedslot = slot;
              vm_protect!(l, pc, base, {
                lua_v_settable(l, &g, kv, ra);
              });
              vm_patch_c(pc.sub(2), (*l).cachedslot);
              continue 'dispatch;
            }
          }

          LuauOpcode::LOP_GETUPVAL => {
            // lvmexecute.cpp:439
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let ur = VM_UV!(luau_insn_b(insn), cl);
            let v = if (*ur).is_upval() {
              (*upvalue!(ur)).v
            } else {
              ur
            };

            setobj_2_s!(l, ra, v);
            continue 'dispatch;
          }

          LuauOpcode::LOP_SETUPVAL => {
            // lvmexecute.cpp:450
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let ur = VM_UV!(luau_insn_b(insn), cl);
            // cpp: `UpVal* uv = upvalue(ur);`（lvmexecute.cpp:477）——裸指针形态，与上游一致。
            let uv = upvalue!(ur);

            setobj!(l, (*uv).v, ra);
            lua_c_barrier!(l, uv, ra);
            continue 'dispatch;
          }

          LuauOpcode::LOP_CLOSEUPVALS => {
            // lvmexecute.cpp:462
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);

            // §2 (b)：可空链表头以 Option<&UpVal>（null→None 由 as_ref 归一）判别，
            // 取代「is_null 哨兵比较 + 裸指针二次解引用」
            if let Some(uv) = (*l).openupval.as_ref()
              && uv.v >= ra
            {
              lua_f_close(l, ra);
            }
            continue 'dispatch;
          }

          LuauOpcode::LOP_GETIMPORT => {
            // lvmexecute.cpp:472
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let kv = VM_KV!(luau_insn_d(insn), cl, k);

            // fast-path: import resolution was successful and closure
            // environment is "safe" for import
            if !(*kv).is_nil() && (*(*cl).env).safeenv != 0 {
              setobj_2_s!(l, ra, kv);
              pc = pc.add(1); // skip over AUX
              continue 'dispatch;
            } else {
              let aux: u32 = *pc;
              pc = pc.add(1);

              vm_protect!(l, pc, base, {
                lua_v_getimport(l, (*cl).env, k, ra, aux, /* propagatenil= */ false);
              });
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_GETTABLEKS => {
            // lvmexecute.cpp:494
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k);
            LUAU_ASSERT!((*kv).is_string());

            // fast-path: built-in table
            if (*rb).is_table() {
              let h = (*rb).as_table_ptr();

              let slot = (luau_insn_c(insn) as i32) & (*h).nodemask8 as i32;
              let n = (*h).node.add(slot as usize);

              // fast-path: value is in expected slot
              if gslot_hit(&*n, &*kv) {
                setobj_2_s!(l, ra, gval!(n));
                continue 'dispatch;
              } else if (*h).metatable.is_null() {
                // fast-path: value is not in expected slot, but the table
                // lookup doesn't involve metatable
                let res = lua_h_getstr(h, (*kv).as_string_ptr());

                if res != LUA_O_NILOBJECT {
                  let cachedslot = gval2slot!(h, res);
                  // save cachedslot to accelerate future lookups; patches
                  // currently executing instruction since pc-2 rolls back two pc++
                  vm_patch_c(pc.sub(2), cachedslot);
                }

                setobj_2_s!(l, ra, res);
                continue 'dispatch;
              } else {
                // slow-path, may invoke Lua calls via __index metamethod
                (*l).cachedslot = slot;
                vm_protect!(l, pc, base, {
                  lua_v_gettable(l, rb, kv, ra);
                });
                vm_patch_c(pc.sub(2), (*l).cachedslot);
                continue 'dispatch;
              }
            } else {
              // fast-path: registered direct field handler
              if fflag::LuauDirectFieldGet.get() && (*rb).is_userdata() {
                let u = (*rb).as_userdata_ptr();
                let dispatch_t = {
                  let t = (*(*l).global).udatadirectfields[(*u).tag as usize];
                  (!t.is_null()).then_some(t)
                };
                if let Some(dispatch_t) = dispatch_t {
                  let slot = (luau_insn_c(insn) as i32) & (*dispatch_t).nodemask8 as i32;
                  let n = (*dispatch_t).node.add(slot as usize);

                  if gslot_hit(&*n, &*kv)
                    && let Some(f) = from_ptr(pvalue!(gval!(n)))
                  {
                    f((*u).data.as_ptr() as *mut c_void, ra as *mut c_void);
                    continue 'dispatch;
                  }

                  let fptr = lua_h_getstr(dispatch_t, (*kv).as_string_ptr());
                  if !(*fptr).is_nil()
                    && let Some(f) = from_ptr(pvalue!(fptr))
                  {
                    vm_patch_c(pc.sub(2), gval2slot!(dispatch_t, fptr));
                    f((*u).data.as_ptr() as *mut c_void, ra as *mut c_void);
                    continue 'dispatch;
                  }
                }
              }

              // fast-path: user data with C __index TM
              if let Some(fn_tm) = frame.c_udata_tm(rb, TMS::TmIndex) {
                (*l).cachedslot = luau_insn_c(insn) as i32;
                call_c_tm(l, pc, &mut base, fn_tm, &[rb, kv], luau_insn_a(insn) as i32);
                vm_patch_c(pc.sub(2), (*l).cachedslot);
                continue 'dispatch;
              } else if (*rb).is_vector() {
                // fast-path: quick case-insensitive comparison with "X"/"Y"/"Z"
                let name = frame.string_bytes((*kv).as_string_ptr());
                let ic = (name[0] | b' ') as i32 - b'x' as i32;

                if (ic as u32) < 3 && name[1] == 0 {
                  setnvalue!(ra, frame.lanes(rb)[ic as usize] as f64);
                  continue 'dispatch;
                }

                // vector 类型元表上的 C __index
                if let Some(fn_tm) = frame.type_metatable_c_tm(LuaType::Vector as u32, TMS::TmIndex)
                {
                  (*l).cachedslot = luau_insn_c(insn) as i32;
                  call_c_tm(l, pc, &mut base, fn_tm, &[rb, kv], luau_insn_a(insn) as i32);
                  vm_patch_c(pc.sub(2), (*l).cachedslot);
                  continue 'dispatch;
                }
              } else if fflag::DebugLuauUserDefinedClassesRuntime.get() && (*rb).is_object() {
                let inst = objectvalue!(rb);
                let slot = luau_insn_c(insn) as u8;
                if (slot as i32) < (*(*inst).lclass).numberofallmembers
                  && (*kv).as_string_ptr() == *(*(*inst).lclass).offsettomember.add(slot as usize)
                {
                  setobj_2_s!(l, ra, luaR_lookupmemberatoffset!(inst, slot as i32));
                  continue 'dispatch;
                } else {
                  let offset =
                    lua_h_getstr((*(*inst).lclass).memberstooffset, (*kv).as_string_ptr());
                  if (*offset).is_nil() {
                    lua_g_missingmembererror(l, rb, kv);
                  }
                  LUAU_ASSERT!((*offset).is_number());
                  let offsetnum = (*offset).as_number() as i32;
                  setobj_2_s!(l, ra, luaR_lookupmemberatoffset!(inst, offsetnum));
                  vm_patch_c(pc.sub(2), offsetnum);
                  continue 'dispatch;
                }
              }
            }

            // slow-path, may invoke Lua calls via __index metamethod
            vm_protect!(l, pc, base, {
              lua_v_gettable(l, rb, kv, ra);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_SETTABLEKS => {
            // lvmexecute.cpp:665
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k);
            LUAU_ASSERT!((*kv).is_string());

            // fast-path: built-in table
            if (*rb).is_table() {
              let h = (*rb).as_table_ptr();

              let slot = (luau_insn_c(insn) as i32) & (*h).nodemask8 as i32;
              let n = (*h).node.add(slot as usize);

              // fast-path: value is in expected slot
              if gslot_hit(&*n, &*kv) && (*h).readonly == 0 {
                setobj2t!(l, gval!(n), ra);
                luaC_barriert!(l, h, ra);
                continue 'dispatch;
              } else if fastnotm((*h).metatable, TMS::TmNewIndex) && (*h).readonly == 0 {
                (*(*l).ci).savedpc = pc; // vm_protect_pc(): set may fail

                let res = lua_h_setstr(l, h, (*kv).as_string_ptr());
                let cachedslot = gval2slot!(h, res);
                vm_patch_c(pc.sub(2), cachedslot);
                setobj2t!(l, res, ra);
                luaC_barriert!(l, h, ra);
                continue 'dispatch;
              } else {
                // slow-path, may invoke Lua calls via __newindex metamethod
                (*l).cachedslot = slot;
                vm_protect!(l, pc, base, {
                  lua_v_settable(l, rb, kv, ra);
                });
                vm_patch_c(pc.sub(2), (*l).cachedslot);
                continue 'dispatch;
              }
            } else {
              // fast-path: user data with C __newindex TM
              if let Some(fn_tm) = frame.c_udata_tm(rb, TMS::TmNewIndex) {
                (*l).cachedslot = luau_insn_c(insn) as i32;
                call_c_tm(l, pc, &mut base, fn_tm, &[rb, kv, ra], -1);
                vm_patch_c(pc.sub(2), (*l).cachedslot);
                continue 'dispatch;
              } else {
                // slow-path, may invoke Lua calls via __newindex metamethod
                vm_protect!(l, pc, base, {
                  lua_v_settable(l, rb, kv, ra);
                });
                continue 'dispatch;
              }
            }
          }
          LuauOpcode::LOP_GETTABLE => {
            // lvmexecute.cpp:741
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            // fast-path: array access
            if (*rb).is_table() && (*rc).is_number() {
              let h = (*rb).as_table_ptr();
              let indexd = (*rc).as_number();
              let index = indexd as i32;

              // index has to be an exact integer and in-bounds for the array portion
              if ((index as u32).wrapping_sub(1)) < (*h).sizearray as u32
                && (*h).metatable.is_null()
                && index as f64 == indexd
              {
                setobj_2_s!(l, ra, (*h).array.add((index - 1) as u32 as usize));
                continue 'dispatch;
              }
            }

            // slow-path
            base = gettable_slow(l, pc, ra, rb, rc);
            continue 'dispatch;
          }
          LuauOpcode::LOP_SETTABLE => {
            // lvmexecute.cpp:771
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            // fast-path: array access
            if (*rb).is_table() && (*rc).is_number() {
              let h = (*rb).as_table_ptr();
              let indexd = (*rc).as_number();
              let index = indexd as i32;

              // index has to be an exact integer and in-bounds for the array portion
              if ((index as u32).wrapping_sub(1)) < (*h).sizearray as u32
                && (*h).metatable.is_null()
                && (*h).readonly == 0
                && index as f64 == indexd
              {
                setobj2t!(l, (*h).array.add((index - 1) as u32 as usize), ra);
                luaC_barriert!(l, h, ra);
                continue 'dispatch;
              }
            }

            // slow-path
            base = settable_slow(l, pc, ra, rb, rc);
            continue 'dispatch;
          }
          LuauOpcode::LOP_GETTABLEN => {
            // lvmexecute.cpp:802
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let c = luau_insn_c(insn) as i32;

            // fast-path: array access
            if (*rb).is_table() {
              let h = (*rb).as_table_ptr();

              if (c as u32) < (*h).sizearray as u32 && (*h).metatable.is_null() {
                setobj_2_s!(l, ra, (*h).array.add(c as usize));
                continue 'dispatch;
              }
            }

            // slow-path: handles out of bounds array accesses
            let mut n = TValue::default();
            setnvalue!(&mut n, (c + 1) as f64);
            vm_protect!(l, pc, base, {
              lua_v_gettable(l, rb, &mut n, ra);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_SETTABLEN => {
            // lvmexecute.cpp:830
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let c = luau_insn_c(insn) as i32;

            // fast-path: array access
            if (*rb).is_table() {
              let h = (*rb).as_table_ptr();

              if (c as u32) < (*h).sizearray as u32
                && (*h).metatable.is_null()
                && (*h).readonly == 0
              {
                setobj2t!(l, (*h).array.add(c as usize), ra);
                luaC_barriert!(l, h, ra);
                continue 'dispatch;
              }
            }

            // slow-path: handles out of bounds array accesses
            let mut n = TValue::default();
            setnvalue!(&mut n, (c + 1) as f64);
            vm_protect!(l, pc, base, {
              lua_v_settable(l, rb, &n, ra);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_NEWCLOSURE => {
            // lvmexecute.cpp:859
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);

            let (pv, sizep) = {
              let l = &(*cl).inner.l;
              (*(*l.p).p.add(luau_insn_d(insn) as usize), (*l.p).sizep)
            };
            LUAU_ASSERT!((luau_insn_d(insn) as u32) < sizep as u32);

            (*(*l).ci).savedpc = pc; // vm_protect_pc(): lua_f_new_lclosure may fail due to OOM

            // note: we save closure to stack early in case the code below
            // wants to capture it by value
            let ncl = lua_f_new_lclosure(l, (*pv).nups as i32, (*cl).env, pv);
            setclvalue!(l, ra, ncl);

            // nups 条 LOP_CAPTURE 紧跟 NEWCLOSURE 排在代码段里（与 LOP_DUPCLOSURE 直接读
            // pc[ui] 同一约定），故把指令窗口定界成切片迭代，循环尾一次性推进 pc；
            // ui 保留为 uprefs 槽位号（下标本身即数据）
            let nups = (*pv).nups as usize;
            let captures = frame.insns(pc, nups);
            for (ui, &uinsn) in captures.iter().enumerate() {
              LUAU_ASSERT!(luau_insn_op(uinsn) == LuauOpcode::LOP_CAPTURE as u32);

              let uref = {
                let l = &mut (*ncl).inner.l;
                l.uprefs.as_mut_ptr().add(ui)
              };
              match luau_insn_a(uinsn) {
                x if x == LuauCaptureType::LCT_VAL as u32 => {
                  setobj!(l, uref, VM_REG!(luau_insn_b(uinsn), l, base));
                }
                x if x == LuauCaptureType::LCT_REF as u32 => {
                  setupvalue!(
                    l,
                    uref,
                    lua_f_findupval(l, VM_REG!(luau_insn_b(uinsn), l, base))
                  );
                }
                x if x == LuauCaptureType::LCT_UPVAL as u32 => {
                  setobj!(l, uref, VM_UV!(luau_insn_b(uinsn), cl));
                }
                _ => {
                  // LUAU_ASSERT(!"Unknown upvalue capture type")
                  // 中文说明：捕获类型三分支穷尽 LCT_NONE/LCT_VALUE/LCT_UPVAL，落空即损坏字节码（cpp: LUAU_UNREACHABLE）
                  LUAU_ASSERT!(false);
                  unreachable!() // LUAU_UNREACHABLE()
                }
              }
            }
            pc = pc.add(nups);

            vm_protect!(l, pc, base, {
              lua_c_check_gc!(l);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_NAMECALL => {
            // lvmexecute.cpp:902
            let insn = *pc;
            pc = pc.add(1);
            let mut ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k);
            LUAU_ASSERT!((*kv).is_string());

            // §11 pass B: rb tag 判定收敛为 ValueView::Table 匹配
            let vrb = ValueView::from_tvalue(&*rb);
            if let ValueView::Table(_) = vrb {
              let h = (*rb).as_table_ptr();
              // note: we can't use nodemask8 here because we need to query the
              // main position of the table, and 8-bit nodemask8 only works for
              // predictive lookups
              let n = (*h)
                .node
                .add(((*(*kv).as_string_ptr()).hash & (sizenode!(h) - 1) as u32) as usize);

              // fast-path: key is in the table in expected slot
              if gslot_hit(&*n, &*kv) {
                // §2 (a)：本臂 [ra, ra+2)（方法槽/self 槽）双槽窗口经
                // `VmFrame::slots_mut` 切片视图读写，取代 `ra.add(1)` 手写算术
                let w = frame.slots_mut(ra, 2);
                // note: order of copies allows rb to alias ra+1 or ra
                setobj_2_s!(l, &raw mut w[1], rb);
                setobj_2_s!(l, &raw mut w[0], gval!(n));
              } else {
                // fast-path: key is absent from the base, table has an
                // __index table, and it has the result in the expected slot
                let mut hit_mt_fast = false;
                if gnext!(n) == 0 {
                  // §11 pass B 次波：__index 元方法的 Table tag 判定与载荷读取合一
                  // 走 ValueView（原 `.filter(matches!(.. Table))` + `(*mt).as_table_ptr()`
                  // 两次 tag 读），视图不命中即保持 hit_mt_fast=false 落慢路径，
                  // 与收敛前的 `&&` 短路链逐位同形
                  if let Some(tm) = frame.fast_tm((*h).metatable.as_ref(), TMS::TmIndex)
                    && let ValueView::Table(index_tab) = ValueView::from_tvalue(&*tm)
                  {
                    let mtn = index_tab
                      .node
                      .add(((luau_insn_c(insn) as i32) & index_tab.nodemask8 as i32) as usize);
                    if gslot_hit(&*mtn, &*kv) {
                      let w = frame.slots_mut(ra, 2);
                      // note: order of copies allows rb to alias ra+1 or ra
                      setobj_2_s!(l, &raw mut w[1], rb);
                      setobj_2_s!(l, &raw mut w[0], gval!(mtn));
                      hit_mt_fast = true;
                    }
                  }
                }
                if !hit_mt_fast {
                  // slow-path: handles full table lookup
                  let w = frame.slots_mut(ra, 2);
                  setobj_2_s!(l, &raw mut w[1], rb);
                  (*l).cachedslot = luau_insn_c(insn) as i32;
                  vm_protect!(l, pc, base, {
                    lua_v_gettable(l, rb, kv, ra);
                  });
                  vm_patch_c(pc.sub(2), (*l).cachedslot);
                  // recompute ra since stack might have been reallocated
                  ra = VM_REG!(luau_insn_a(insn), l, base);
                  // 视图不跨重分配点存活：重算 ra 后另起 [ra, ra+2) 窗口
                  let w = frame.slots_mut(ra, 2);
                  if matches!(ValueView::from_tvalue(&w[0]), ValueView::Nil) {
                    lua_g_methoderror(l, &w[1], (*kv).as_string_ptr());
                  }
                }
              }
            } else {
              // cpp: `LuaTable* mt = ttisuserdata(rb) ? uvalue(rb)->metatable : L->global->mt[ttype(rb)];`
              // （lvmexecute.cpp:981）§11 pass B: userdata tag 判定收敛为
              // ValueView::Userdata 匹配（uvalue! 同为 *const Udata）；其余类型按
              // tag 值索引 global.mt（ttype! 为数组下标读数，非分支判等，保留）。
              let mt = if let ValueView::Userdata(u) = vrb {
                (*u).metatable.as_ref()
              } else {
                frame.type_metatable(ttype!(rb))
              };

              // fast-path: metatable with __namecall
              if let Some(fn_nc) = frame.fast_tm(mt, TMS::TmNameCall) {
                let w = frame.slots_mut(ra, 2);
                // note: order of copies allows rb to alias ra+1 or ra
                setobj_2_s!(l, &raw mut w[1], rb);
                setobj_2_s!(l, &raw mut w[0], fn_nc);

                (*l).namecall = (*kv).as_string_ptr();
              } else {
                // §11 pass B 次波：__index 元方法的 Table tag 判定与载荷读取合一走
                // ValueView（原 `.filter(matches!(.. Table))` + `(*tmi).as_table_ptr()` 两次
                // tag 读）；视图末次使用止于 `index_tab.node`，其后 lua_v_gettable
                // 的写路径经节点裸指针，与原 `(*(*tmi).as_table_ptr()).node` 同址同序
                if let Some(tm) = frame.fast_tm(mt, TMS::TmIndex)
                  && let ValueView::Table(index_tab) = ValueView::from_tvalue(&*tm)
                {
                  let slot = (luau_insn_c(insn) as i32) & index_tab.nodemask8 as i32;
                  let n = index_tab.node.add(slot as usize);

                  // fast-path: metatable with __index that has method in expected slot
                  if gslot_hit(&*n, &*kv) {
                    let w = frame.slots_mut(ra, 2);
                    // note: order of copies allows rb to alias ra+1 or ra
                    setobj_2_s!(l, &raw mut w[1], rb);
                    setobj_2_s!(l, &raw mut w[0], gval!(n));
                  } else {
                    // slow-path: handles slot mismatch
                    let w = frame.slots_mut(ra, 2);
                    setobj_2_s!(l, &raw mut w[1], rb);
                    (*l).cachedslot = slot;
                    vm_protect!(l, pc, base, {
                      lua_v_gettable(l, rb, kv, ra);
                    });
                    vm_patch_c(pc.sub(2), (*l).cachedslot);
                    // recompute ra since stack might have been reallocated
                    ra = VM_REG!(luau_insn_a(insn), l, base);
                    let w = frame.slots_mut(ra, 2);
                    if matches!(ValueView::from_tvalue(&w[0]), ValueView::Nil) {
                      lua_g_methoderror(l, &w[1], (*kv).as_string_ptr());
                    }
                  }
                } else if fflag::DebugLuauUserDefinedClassesRuntime.get()
                  && let ValueView::Object(inst) = vrb
                {
                  let slot = luau_insn_c(insn) as i32;
                  // §11 pass B 次波：与 GETTABLEKS 同型——rb 的 object tag+payload
                  // 收敛为 ValueView::Object 一级变体（视图载荷即 `objectvalue!`
                  // 同址的 `*const LuauObject`），本臂只读指针字段不改写实例。
                  if slot < (*(*inst).lclass).numberofallmembers
                    && (*kv).as_string_ptr() == *(*(*inst).lclass).offsettomember.add(slot as usize)
                  {
                    let w = frame.slots_mut(ra, 2);
                    // note: order of copies allows rb to alias ra+1 or ra
                    setobj_2_s!(l, &raw mut w[1], rb);
                    setobj_2_s!(l, &raw mut w[0], luaR_lookupmemberatoffset!(inst, slot));
                  } else {
                    // slow-er path: try to fetch the field manually.
                    let offset =
                      lua_h_getstr((*(*inst).lclass).memberstooffset, (*kv).as_string_ptr());
                    if matches!(ValueView::from_tvalue(&*offset), ValueView::Nil) {
                      lua_g_missingmembererror(l, rb, kv);
                    }
                    LUAU_ASSERT!((*offset).is_number());
                    let offsetnum = (*offset).as_number() as i32;
                    let w = frame.slots_mut(ra, 2);
                    setobj_2_s!(l, &raw mut w[1], rb);
                    setobj_2_s!(
                      l,
                      &raw mut w[0],
                      luaR_lookupmemberatoffset!(inst, offsetnum)
                    );
                    vm_patch_c(pc.sub(2), offsetnum);
                  }
                } else {
                  // slow-path: handles non-table __index
                  let w = frame.slots_mut(ra, 2);
                  setobj_2_s!(l, &raw mut w[1], rb);
                  vm_protect!(l, pc, base, {
                    lua_v_gettable(l, rb, kv, ra);
                  });
                  // recompute ra since stack might have been reallocated
                  ra = VM_REG!(luau_insn_a(insn), l, base);
                  let w = frame.slots_mut(ra, 2);
                  if matches!(ValueView::from_tvalue(&w[0]), ValueView::Nil) {
                    lua_g_methoderror(l, &w[1], (*kv).as_string_ptr());
                  }
                }
              }
            }

            if fflag::LuauCallFeedback.get() {
              continue 'dispatch;
            } else {
              // intentional fallthrough to CALL (C++ case fallthrough; pc
              // points at the CALL instruction, so forcing the dispatch op
              // is semantically identical)
              LUAU_ASSERT!(luau_insn_op(*pc) == LuauOpcode::LOP_CALL as u32);
              continue_op = LuauOpcode::LOP_CALL as u8;
              continue 'dispatch;
            }
          }
          // CALL/CALLFB 两臂逐行同构（单源见 call_arm! 宏文档）：
          LuauOpcode::LOP_CALL => {
            // lvmexecute.cpp:1038
            call_arm!(l, pc, base, cl, k, None::<Instruction>, 'dispatch);
          }
          LuauOpcode::LOP_CALLFB => {
            // lvmexecute.cpp:1145
            call_arm!(l, pc, base, cl, k, Some(*pc), 'dispatch);
          }
          LuauOpcode::LOP_RETURN => {
            // lvmexecute.cpp:1265
            VM_INTERRUPT!(l, pc, base);
            let insn = *pc;
            // note: this can point to l->top if b == LUA_MULTRET making VM_REG unsafe to use
            let ra: StkId = base.add(luau_insn_a(insn) as usize);
            let b = luau_insn_b(insn) as i32 - 1;

            // ci is our callinfo, cip is our parent
            let ci = (*l).ci;
            let cip = ci.sub(1);

            let nresults = (*ci).nresults;
            // copy as much as possible for MULTRET calls, and only as much as
            // needed otherwise
            let valend = if b == LUA_MULTRET {
              (*l).top
            } else {
              ra.add(b as usize)
            };

            // 将返回值拷回父栈（最多 nresults 个），不足补 nil，并弹出本帧
            // （lvmexecute.cpp:1265-1318，见 pop_frame_copy_results；
            // ci/cip 在弹帧后仍被用于 RETURN 标志与 reentry 恢复）
            pop_frame_copy_results(l, ra, valend, nresults);

            // we're done!
            if ((*ci).flags as i32 & LUA_CALLINFO_RETURN) != 0 {
              return; // goto exit
            }

            LUAU_ASSERT!(isLua!((*l).ci));

            let nextcl = (*(*cip).func).as_closure_ptr();
            let nextproto = {
              let l = &(*nextcl).inner.l;
              l.p
            };

            // VM_HAS_NATIVE
            if ((*cip).flags as i32 & LUA_CALLINFO_NATIVE) != 0
              && !SINGLE_STEP
              && let Some(enter) = (*(*l).global).ecb.enter
            {
              if enter(l, nextproto) == 1 {
                continue 'reentry; // goto reentry
              } else {
                return; // goto exit
              }
            }

            // reentry
            pc = (*cip).savedpc;
            cl = nextcl;
            base = (*l).base;
            k = (*nextproto).k;
            continue 'dispatch;
          }
          LuauOpcode::LOP_JUMP => {
            // lvmexecute.cpp:1332
            let insn = *pc;
            pc = pc.add(1);

            pc = pc.offset(luau_insn_d(insn) as isize);
            let p = cl_proto!(cl);
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }

          LuauOpcode::LOP_JUMPIF | LuauOpcode::LOP_JUMPIFNOT => {
            // lvmexecute.cpp:1341 / 1351
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);

            // §11 pass B：l_isfalse 的 nil/boolean 判链收敛为 ValueView match
            let is_not = op == LuauOpcode::LOP_JUMPIFNOT as u8;
            let truthy = (*ra).is_truthy();
            pc = pc.offset(if truthy ^ is_not {
              luau_insn_d(insn) as isize
            } else {
              0
            });
            let p = cl_proto!(cl);
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }

          LuauOpcode::LOP_JUMPIFEQ | LuauOpcode::LOP_JUMPIFNOTEQ => {
            // lvmexecute.cpp:1361 / 1494
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(aux, l, base);
            let is_not = op == LuauOpcode::LOP_JUMPIFNOTEQ as u8;

            // Note that all jumps below jump by 1 in the "false" case to skip over aux
            if ttype!(ra) == ttype!(rb) {
              // §11 pass B：ttype 魔法数字 if 链收敛为 ValueView 变体 match（lua_o_rawequal_key
              // 的 B2a 同型——外层 tag 判等成立后以 ra 视图分派即等价于按 tag 值 switch，
              // rb 侧 payload 仍经读取宏直取）；TABLE/USERDATA/OBJECT 臂留空落到
              // match 后的共享慢路径（cpp switch 的 break 语义）。
              match ValueView::from_tvalue(&*ra) {
                ValueView::Nil => jump_and_next!(pc, cl, insn, 'dispatch, !is_not),
                ValueView::Boolean(b) => {
                  jump_and_next!(pc, cl, insn, 'dispatch, (b == (*rb).as_boolean_raw()) ^ is_not)
                }
                ValueView::LightUserdata { pointer, tag } => {
                  jump_and_next!(
                    pc,
                    cl,
                    insn,
                    'dispatch,
                    (pointer == pvalue!(rb) && tag == lightuserdatatag!(rb)) ^ is_not
                  )
                }
                // IteratorDone 即 LightUserData tag 的 null 指针 + LU_TAG_ITERATOR 形态，
                // 原指针+usertag 双判等式按其定义原样展开，判定不变
                ValueView::IteratorDone => jump_and_next!(
                  pc,
                  cl,
                  insn,
                  'dispatch,
                  (pvalue!(rb).is_null() && lightuserdatatag!(rb) == LU_TAG_ITERATOR) ^ is_not
                ),
                ValueView::Number(n) => {
                  jump_and_next!(pc, cl, insn, 'dispatch, (n == (*rb).as_number()) ^ is_not)
                }
                ValueView::Integer(a) => {
                  jump_and_next!(pc, cl, insn, 'dispatch, (a == lvalue!(rb)) ^ is_not)
                }
                ValueView::Vector(v) => jump_and_next!(
                  pc,
                  cl,
                  insn,
                  'dispatch,
                  luai_veceq(v, (*rb).as_vector_ref()) ^ is_not
                ),
                ValueView::String(_)
                | ValueView::Function(_)
                | ValueView::Thread(_)
                | ValueView::Buffer(_) => {
                  jump_and_next!(pc, cl, insn, 'dispatch, (gcvalue!(ra) == gcvalue!(rb)) ^ is_not)
                }
                ValueView::Table(h) => {
                  // §11 pass B 次波：外层 tag 判等成立 ⇒ rb 侧必为 Table，故 rb 载荷
                  // 也经 ValueView 取出（取代 `(*(*rb).as_table_ptr()).metatable` 的无判裸
                  // 指针解引用）；视图不命中即落 match 后的共享慢路径，与 cpp switch
                  // 的 break 逐位一致。
                  if let ValueView::Table(hb) = ValueView::from_tvalue(&*rb) {
                    // fast-path: same metatable, no EQ metamethod
                    if h.metatable == hb.metatable
                      && frame.fast_tm(h.metatable.as_ref(), TMS::TmEq).is_none()
                    {
                      jump_and_next!(pc, cl, insn, 'dispatch, eq(h, hb) ^ is_not);
                    }
                  }
                  // slow path after switch()
                }
                ValueView::Userdata(u) => {
                  // §11 pass B 次波：同 Table 臂，rb 侧经 ValueView::Userdata 取载荷
                  if let ValueView::Userdata(u2) = ValueView::from_tvalue(&*rb) {
                    // fast-path: same metatable, no EQ metamethod or C metamethod
                    // cpp: `if (uvalue(ra)->metatable == uvalue(rb)->metatable)` 且
                    // `fasttm(L, uvalue(ra)->metatable, TM_EQ)`（lvmexecute.cpp:1432/1565）
                    if (*u).metatable == (*u2).metatable {
                      let fn_tm = frame.fast_tm((*u).metatable.as_ref(), TMS::TmEq);
                      if fn_tm.is_none() {
                        jump_and_next!(pc, cl, insn, 'dispatch, eq(u, u2) ^ is_not);
                      } else if let Some(fn_tm) =
                        // §11 pass B: tm 的 function tag+is_c 判定收敛为 ValueView match
                        fn_tm.filter(|&tm| {
                          matches!(ValueView::from_tvalue(&*tm), ValueView::Function(c) if c.is_c != 0)
                        })
                      {
                        // note: it's safe to push arguments past top for
                        // complicated reasons (see top of the file)
                        let res = (*l).top.offset_from(base) as i32;
                        call_c_tm(l, pc, &mut base, fn_tm, &[ra, rb], res);
                        // §11 pass B：l_isfalse 的 nil/boolean 判链收敛为 ValueView match
                        let truthy = !matches!(
                          ValueView::from_tvalue(&*base.add(res as usize)),
                          ValueView::Nil | ValueView::Boolean(0)
                        );
                        jump_and_next!(pc, cl, insn, 'dispatch, truthy ^ is_not);
                      }
                    }
                  }
                  // slow path after switch()
                }
                ValueView::Class(_) => {
                  // Class objects are only ever physically equal, so check
                  // for pointer equality.
                  jump_and_next!(
                    pc,
                    cl,
                    insn,
                    'dispatch,
                    eq(classvalue!(ra), classvalue!(rb)) ^ is_not
                  );
                }
                ValueView::Object(_) => {
                  // For now, hit the slow path after the switch (we may
                  // need to invoke metamethods).
                }
                ValueView::UpVal(_) | ValueView::Other(_) => {
                  LUAU_ASSERT!(false);
                  unreachable!()
                }
              }

              // slow-path: tables with metatables and userdata values
              // note that we don't have a fast path for userdata values
              // without metatables, since that's very rare
              let res: i32;
              vm_protect!(l, pc, base, {
                res = lua_v_equalval(l, ra, rb);
              });

              jump_and_next!(pc, cl, insn, 'dispatch, (res == 1) ^ is_not);
            } else {
              jump_and_next!(pc, cl, insn, 'dispatch, is_not);
            }
          }

          LuauOpcode::LOP_JUMPIFLE => {
            // lvmexecute.cpp:1627
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(aux, l, base);

            // fast-path: number
            if (*ra).is_number() && (*rb).is_number() {
              jump_and_next!(pc, cl, insn, 'dispatch, (*ra).as_number() <= (*rb).as_number());
            } else if (*ra).is_string() && (*rb).is_string() {
              let cmp = lua_v_strcmp((*ra).as_string(), (*rb).as_string());
              jump_and_next!(pc, cl, insn, 'dispatch, cmp <= 0);
            } else {
              let res: i32;
              vm_protect!(l, pc, base, {
                res = lua_v_lessequal(l, ra, rb);
              });
              jump_and_next!(pc, cl, insn, 'dispatch, res == 1);
            }
          }
          LuauOpcode::LOP_JUMPIFNOTLE => {
            // lvmexecute.cpp:1660
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(aux, l, base);

            // fast-path: number
            if (*ra).is_number() && (*rb).is_number() {
              jump_if_false_and_next!(pc, cl, insn, 'dispatch, (*ra).as_number() <= (*rb).as_number());
            } else if (*ra).is_string() && (*rb).is_string() {
              let cmp = lua_v_strcmp((*ra).as_string(), (*rb).as_string());
              jump_if_false_and_next!(pc, cl, insn, 'dispatch, cmp <= 0);
            } else {
              let res: i32;
              vm_protect!(l, pc, base, {
                res = lua_v_lessequal(l, ra, rb);
              });
              jump_and_next!(pc, cl, insn, 'dispatch, res == 0);
            }
          }
          LuauOpcode::LOP_JUMPIFLT => {
            // lvmexecute.cpp:1693
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(aux, l, base);

            // fast-path: number
            if (*ra).is_number() && (*rb).is_number() {
              jump_and_next!(pc, cl, insn, 'dispatch, (*ra).as_number() < (*rb).as_number());
            } else if (*ra).is_string() && (*rb).is_string() {
              let cmp = lua_v_strcmp((*ra).as_string(), (*rb).as_string());
              jump_and_next!(pc, cl, insn, 'dispatch, cmp < 0);
            } else {
              let res: i32;
              vm_protect!(l, pc, base, {
                res = lua_v_lessthan(l, ra, rb);
              });
              jump_and_next!(pc, cl, insn, 'dispatch, res == 1);
            }
          }
          LuauOpcode::LOP_JUMPIFNOTLT => {
            // lvmexecute.cpp:1726
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(aux, l, base);

            // fast-path: number
            if (*ra).is_number() && (*rb).is_number() {
              jump_if_false_and_next!(pc, cl, insn, 'dispatch, (*ra).as_number() < (*rb).as_number());
            } else if (*ra).is_string() && (*rb).is_string() {
              let cmp = lua_v_strcmp((*ra).as_string(), (*rb).as_string());
              jump_if_false_and_next!(pc, cl, insn, 'dispatch, cmp < 0);
            } else {
              let res: i32;
              vm_protect!(l, pc, base, {
                res = lua_v_lessthan(l, ra, rb);
              });
              jump_and_next!(pc, cl, insn, 'dispatch, res == 0);
            }
          }
          LuauOpcode::LOP_ADD => {
            // lvmexecute.cpp:1759
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            // fast-path: number
            if (*rb).is_number() && (*rc).is_number() {
              setnvalue!(ra, (*rb).as_number() + (*rc).as_number());
              continue 'dispatch;
            } else if (*rb).is_vector() && (*rc).is_vector() {
              let vb = frame.lanes(rb);
              let vc = frame.lanes(rc);
              setvvalue!(
                ra,
                vb[0] + vc[0],
                vb[1] + vc[1],
                vb[2] + vc[2],
                frame.lane_at(rb, 3) + frame.lane_at(rc, 3)
              );
              continue 'dispatch;
            } else if let Some(fn_tm) = frame.c_tm_by_obj(rb, TMS::TmAdd) {
              call_c_tm(l, pc, &mut base, fn_tm, &[rb, rc], luau_insn_a(insn) as i32);
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, rc, TMS::TmAdd, 'dispatch);
            }
          }
          LuauOpcode::LOP_SUB => {
            // lvmexecute.cpp:1805
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            // fast-path: number
            if (*rb).is_number() && (*rc).is_number() {
              setnvalue!(ra, (*rb).as_number() - (*rc).as_number());
              continue 'dispatch;
            } else if (*rb).is_vector() && (*rc).is_vector() {
              let vb = frame.lanes(rb);
              let vc = frame.lanes(rc);
              setvvalue!(
                ra,
                vb[0] - vc[0],
                vb[1] - vc[1],
                vb[2] - vc[2],
                frame.lane_at(rb, 3) - frame.lane_at(rc, 3)
              );
              continue 'dispatch;
            } else if let Some(fn_tm) = frame.c_tm_by_obj(rb, TMS::TmSub) {
              call_c_tm(l, pc, &mut base, fn_tm, &[rb, rc], luau_insn_a(insn) as i32);
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, rc, TMS::TmSub, 'dispatch);
            }
          }
          LuauOpcode::LOP_MUL => {
            // lvmexecute.cpp:1851
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            // fast-path: number
            if (*rb).is_number() && (*rc).is_number() {
              setnvalue!(ra, (*rb).as_number() * (*rc).as_number());
              continue 'dispatch;
            } else if (*rb).is_vector() && (*rc).is_number() {
              let vc = (*rc).as_number() as f32;
              vec_scalar_op!(frame, ra, rb, vc, |a: f32, b: f32| a * b, 'dispatch);
            } else if (*rb).is_vector() && (*rc).is_vector() {
              let vb = frame.lanes(rb);
              let vc = frame.lanes(rc);
              setvvalue!(
                ra,
                vb[0] * vc[0],
                vb[1] * vc[1],
                vb[2] * vc[2],
                frame.lane_at(rb, 3) * frame.lane_at(rc, 3)
              );
              continue 'dispatch;
            } else if (*rb).is_number() && (*rc).is_vector() {
              let vb = (*rb).as_number() as f32;
              let vc = frame.lanes(rc);
              setvvalue!(
                ra,
                vb * vc[0],
                vb * vc[1],
                vb * vc[2],
                vb * frame.lane_at(rc, 3)
              );
              continue 'dispatch;
            } else {
              let rbc = if (*rb).is_number() { rc } else { rb };
              if let Some(fn_tm) = frame.c_tm_by_obj(rbc, TMS::TmMul) {
                call_c_tm(l, pc, &mut base, fn_tm, &[rb, rc], luau_insn_a(insn) as i32);
                continue 'dispatch;
              } else {
                arith_slow!(l, pc, base, ra, rb, rc, TMS::TmMul, 'dispatch);
              }
            }
          }
          LuauOpcode::LOP_DIV => {
            // lvmexecute.cpp:1912
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            // fast-path: number
            if (*rb).is_number() && (*rc).is_number() {
              setnvalue!(ra, (*rb).as_number() / (*rc).as_number());
              continue 'dispatch;
            } else if (*rb).is_vector() && (*rc).is_number() {
              let vc = (*rc).as_number() as f32;
              vec_scalar_op!(frame, ra, rb, vc, |a: f32, b: f32| a / b, 'dispatch);
            } else if (*rb).is_vector() && (*rc).is_vector() {
              let vb = frame.lanes(rb);
              let vc = frame.lanes(rc);
              setvvalue!(
                ra,
                vb[0] / vc[0],
                vb[1] / vc[1],
                vb[2] / vc[2],
                frame.lane_at(rb, 3) / frame.lane_at(rc, 3)
              );
              continue 'dispatch;
            } else if (*rb).is_number() && (*rc).is_vector() {
              let vb = (*rb).as_number() as f32;
              let vc = frame.lanes(rc);
              setvvalue!(
                ra,
                vb / vc[0],
                vb / vc[1],
                vb / vc[2],
                vb / frame.lane_at(rc, 3)
              );
              continue 'dispatch;
            } else {
              let rbc = if (*rb).is_number() { rc } else { rb };
              if let Some(fn_tm) = frame.c_tm_by_obj(rbc, TMS::TmDiv) {
                call_c_tm(l, pc, &mut base, fn_tm, &[rb, rc], luau_insn_a(insn) as i32);
                continue 'dispatch;
              } else {
                arith_slow!(l, pc, base, ra, rb, rc, TMS::TmDiv, 'dispatch);
              }
            }
          }
          LuauOpcode::LOP_IDIV => {
            // lvmexecute.cpp:1973
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            // fast-path: number
            if (*rb).is_number() && (*rc).is_number() {
              setnvalue!(ra, luai_numidiv((*rb).as_number(), (*rc).as_number()));
              continue 'dispatch;
            } else if (*rb).is_vector() && (*rc).is_number() {
              let vc = (*rc).as_number() as f32;
              vec_scalar_op!(frame, ra, rb, vc, |a: f32, b: f32| luai_numidiv(a as f64, b as f64) as f32, 'dispatch);
            } else {
              let rbc = if (*rb).is_number() { rc } else { rb };
              if let Some(fn_tm) = frame.c_tm_by_obj(rbc, TMS::TmIDiv) {
                call_c_tm(l, pc, &mut base, fn_tm, &[rb, rc], luau_insn_a(insn) as i32);
                continue 'dispatch;
              } else {
                arith_slow!(l, pc, base, ra, rb, rc, TMS::TmIDiv, 'dispatch);
              }
            }
          }
          LuauOpcode::LOP_MOD => {
            // lvmexecute.cpp:2026
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            if (*rb).is_number() && (*rc).is_number() {
              setnvalue!(ra, luai_nummod((*rb).as_number(), (*rc).as_number()));
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, rc, TMS::TmMod, 'dispatch);
            }
          }
          LuauOpcode::LOP_POW => {
            // lvmexecute.cpp:2049
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            if (*rb).is_number() && (*rc).is_number() {
              setnvalue!(ra, (*rb).as_number().powf((*rc).as_number()));
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, rc, TMS::TmPow, 'dispatch);
            }
          }
          LuauOpcode::LOP_ADDK => {
            // lvmexecute.cpp:2070
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let kv = VM_KV!(luau_insn_c(insn), cl, k);

            if (*rb).is_number() {
              setnvalue!(ra, (*rb).as_number() + (*kv).as_number());
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, kv, TMS::TmAdd, 'dispatch);
            }
          }
          LuauOpcode::LOP_SUBK => {
            // lvmexecute.cpp:2091
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let kv = VM_KV!(luau_insn_c(insn), cl, k);

            if (*rb).is_number() {
              setnvalue!(ra, (*rb).as_number() - (*kv).as_number());
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, kv, TMS::TmSub, 'dispatch);
            }
          }
          LuauOpcode::LOP_MULK => {
            // lvmexecute.cpp:2112
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let kv = VM_KV!(luau_insn_c(insn), cl, k);

            if (*rb).is_number() {
              setnvalue!(ra, (*rb).as_number() * (*kv).as_number());
              continue 'dispatch;
            } else if (*rb).is_vector() {
              vec_scalar_op!(frame, ra, rb, (*kv).as_number() as f32, |a: f32, b: f32| a * b, 'dispatch);
            } else if let Some(fn_tm) = frame.c_tm_by_obj(rb, TMS::TmMul) {
              call_c_tm(l, pc, &mut base, fn_tm, &[rb, kv], luau_insn_a(insn) as i32);
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, kv, TMS::TmMul, 'dispatch);
            }
          }
          LuauOpcode::LOP_DIVK => {
            // lvmexecute.cpp:2158
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let kv = VM_KV!(luau_insn_c(insn), cl, k);

            if (*rb).is_number() {
              setnvalue!(ra, (*rb).as_number() / (*kv).as_number());
              continue 'dispatch;
            } else if (*rb).is_vector() {
              vec_scalar_op!(frame, ra, rb, (*kv).as_number() as f32, |a: f32, b: f32| a / b, 'dispatch);
            } else if let Some(fn_tm) = frame.c_tm_by_obj(rb, TMS::TmDiv) {
              call_c_tm(l, pc, &mut base, fn_tm, &[rb, kv], luau_insn_a(insn) as i32);
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, kv, TMS::TmDiv, 'dispatch);
            }
          }
          LuauOpcode::LOP_IDIVK => {
            // lvmexecute.cpp:2204
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let kv = VM_KV!(luau_insn_c(insn), cl, k);

            if (*rb).is_number() {
              setnvalue!(ra, luai_numidiv((*rb).as_number(), (*kv).as_number()));
              continue 'dispatch;
            } else if (*rb).is_vector() {
              vec_scalar_op!(frame, ra, rb, (*kv).as_number() as f32, |a: f32, b: f32| luai_numidiv(a as f64, b as f64) as f32, 'dispatch);
            } else if let Some(fn_tm) = frame.c_tm_by_obj(rb, TMS::TmIDiv) {
              call_c_tm(l, pc, &mut base, fn_tm, &[rb, kv], luau_insn_a(insn) as i32);
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, kv, TMS::TmIDiv, 'dispatch);
            }
          }
          LuauOpcode::LOP_MODK => {
            // lvmexecute.cpp:2256
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let kv = VM_KV!(luau_insn_c(insn), cl, k);

            if (*rb).is_number() {
              let nb = (*rb).as_number();
              let nk = (*kv).as_number();
              setnvalue!(ra, luai_nummod(nb, nk));
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, kv, TMS::TmMod, 'dispatch);
            }
          }
          LuauOpcode::LOP_POWK => {
            // lvmexecute.cpp:2279
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let kv = VM_KV!(luau_insn_c(insn), cl, k);

            if (*rb).is_number() {
              let nb = (*rb).as_number();
              let nk = (*kv).as_number();
              let r = if nk == 2.0 {
                nb * nb
              } else if nk == 0.5 {
                nb.sqrt()
              } else if nk == 3.0 {
                nb * nb * nb
              } else {
                nb.powf(nk)
              };

              setnvalue!(ra, r);
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, kv, TMS::TmPow, 'dispatch);
            }
          }
          LuauOpcode::LOP_AND => {
            // lvmexecute.cpp:2306
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            let res = if (*rb).is_falsy() { rb } else { rc };
            setobj_2_s!(l, ra, res);
            continue 'dispatch;
          }
          LuauOpcode::LOP_OR => {
            // lvmexecute.cpp:2317
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            let res = if (*rb).is_falsy() { rc } else { rb };
            setobj_2_s!(l, ra, res);
            continue 'dispatch;
          }
          LuauOpcode::LOP_ANDK => {
            // lvmexecute.cpp:2328
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let kv = VM_KV!(luau_insn_c(insn), cl, k);

            let res = if (*rb).is_falsy() { rb } else { kv };
            setobj_2_s!(l, ra, res);
            continue 'dispatch;
          }
          LuauOpcode::LOP_ORK => {
            // lvmexecute.cpp:2339
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let kv = VM_KV!(luau_insn_c(insn), cl, k);

            let res = if (*rb).is_falsy() { kv } else { rb };
            setobj_2_s!(l, ra, res);
            continue 'dispatch;
          }
          LuauOpcode::LOP_CONCAT => {
            // lvmexecute.cpp:2350
            let insn = *pc;
            pc = pc.add(1);
            let b = luau_insn_b(insn) as i32;
            let c = luau_insn_c(insn) as i32;

            // This call may realloc the stack! So we need to query args further down
            vm_protect!(l, pc, base, {
              lua_v_concat(l, c - b + 1, c);
            });

            let ra = VM_REG!(luau_insn_a(insn), l, base);

            setobj_2_s!(l, ra, base.add(b as usize));
            vm_protect!(l, pc, base, {
              lua_c_check_gc!(l);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_NOT => {
            // lvmexecute.cpp:2377
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);

            let res = (*rb).is_falsy() as i32;
            setbvalue!(ra, res);
            continue 'dispatch;
          }
          LuauOpcode::LOP_MINUS => {
            // lvmexecute.cpp:2399
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);

            if (*rb).is_number() {
              setnvalue!(ra, -(*rb).as_number());
              continue 'dispatch;
            } else if (*rb).is_vector() {
              let vb = frame.lanes(rb);
              setvvalue!(ra, -vb[0], -vb[1], -vb[2], -frame.lane_at(rb, 3));
              continue 'dispatch;
            } else if let Some(fn_tm) = frame.c_tm_by_obj(rb, TMS::TmUnm) {
              call_c_tm(l, pc, &mut base, fn_tm, &[rb], luau_insn_a(insn) as i32);
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, rb, rb, TMS::TmUnm, 'dispatch);
            }
          }
          LuauOpcode::LOP_LENGTH => {
            // lvmexecute.cpp:2420 (LOP_LENGTH)
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);

            // fast-path #1: tables
            if (*rb).is_table() {
              let h = (*rb).as_table_ptr();

              if fastnotm((*h).metatable, TMS::TmLen) {
                setnvalue!(ra, lua_h_getn(h) as f64);
                continue 'dispatch;
              } else {
                // slow-path, may invoke C/Lua via metamethods
                vm_protect!(l, pc, base, {
                  lua_v_dolen(l, ra, rb);
                });
                continue 'dispatch;
              }
            } else if (*rb).is_string() {
              let ts = (*rb).as_string_ptr();
              setnvalue!(ra, (*ts).len as f64);
              continue 'dispatch;
            } else {
              // slow-path, may invoke C/Lua via metamethods
              vm_protect!(l, pc, base, {
                lua_v_dolen(l, ra, rb);
              });
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_NEWTABLE => {
            // lvmexecute.cpp:2458
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let b = luau_insn_b(insn) as i32;
            let aux: u32 = *pc;
            pc = pc.add(1);

            (*(*l).ci).savedpc = pc; // vm_protect_pc(): lua_h_new may fail due to OOM

            sethvalue!(
              l,
              ra,
              lua_h_new(l, aux as i32, if b == 0 { 0 } else { 1 << (b - 1) })
            );
            vm_protect!(l, pc, base, {
              lua_c_check_gc!(l);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_DUPTABLE => {
            // lvmexecute.cpp:2472
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let kv = VM_KV!(luau_insn_d(insn), cl, k);

            (*(*l).ci).savedpc = pc; // vm_protect_pc(): lua_h_clone may fail due to OOM

            sethvalue!(l, ra, lua_h_clone(l, (*kv).as_table_ptr()));
            vm_protect!(l, pc, base, {
              lua_c_check_gc!(l);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_SETLIST => {
            // lvmexecute.cpp:2485
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            // note: this can point to l->top if c == LUA_MULTRET making VM_REG unsafe to use
            let rb: StkId = base.add(luau_insn_b(insn) as usize);
            let mut c = luau_insn_c(insn) as i32 - 1;
            let index: u32 = *pc;
            pc = pc.add(1);

            if c == LUA_MULTRET {
              c = (*l).top.offset_from(rb) as i32;
              (*l).top = (*(*l).ci).top;
            }

            let h = (*ra).as_table_ptr();

            // TODO: we really don't need this anymore
            // §11 pass B: table tag 判定收敛为 ValueView::Table 匹配
            if !matches!(ValueView::from_tvalue(&*ra), ValueView::Table(_)) {
              // temporary workaround to weaken a rather powerful exploitation
              // primitive in case of a MITM attack on bytecode
              return;
            }

            let last = index as i32 + c - 1;
            if last > (*h).sizearray {
              (*(*l).ci).savedpc = pc; // vm_protect_pc(): luaH_resizearray may fail due to OOM

              lua_h_resizearray(l, h, last);
            }

            let array = (*h).array;
            for i in 0..c {
              setobj2t!(
                l,
                array.add((index as i32 + i - 1) as usize),
                rb.add(i as usize)
              );
            }

            lua_c_barrierfast!(l, h);
            continue 'dispatch;
          }
          LuauOpcode::LOP_FORNPREP => {
            // lvmexecute.cpp:2546
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);

            if !(*ra).is_number() || !(*ra.add(1)).is_number() || !(*ra.add(2)).is_number() {
              // slow-path: can convert arguments to numbers and trigger Lua errors
              // Note: this doesn't reallocate stack so we don't need to recompute ra/base
              (*(*l).ci).savedpc = pc; // vm_protect_pc()

              // luaV_prepareFORN 按 StkId 形参收三槽可写指针
              lua_v_prepare_forn(l, ra, ra.add(1), ra.add(2));
            }

            let limit = (*ra).as_number();
            let step = (*ra.add(1)).as_number();
            let idx = (*ra.add(2)).as_number();

            // Note: make sure the loop condition is exactly the same between
            // this and LOP_FORNLOOP so that we handle NaN/etc. consistently
            pc = pc.offset(
              if if step > 0.0 {
                idx <= limit
              } else {
                limit <= idx
              } {
                0
              } else {
                luau_insn_d(insn) as isize
              },
            );
            let p = cl_proto!(cl);
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_FORNLOOP => {
            // lvmexecute.cpp:2573
            VM_INTERRUPT!(l, pc, base);
            // lvmexecute.cpp:2575-2576：LuauBackedgeHeapCheck 开启时回边额外 GC 检查
            if fflag::LuauBackedgeHeapCheck.get() {
              VM_CHECK_GC!(l, pc, base);
            }
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            LUAU_ASSERT!((*ra).is_number() && (*ra.add(1)).is_number() && (*ra.add(2)).is_number());

            let limit = (*ra).as_number();
            let step = (*ra.add(1)).as_number();
            let idx = (*ra.add(2)).as_number() + step;

            setnvalue!(ra.add(2), idx);

            // Note: make sure the loop condition is exactly the same between
            // this and LOP_FORNPREP so that we handle NaN/etc. consistently
            if if step > 0.0 {
              idx <= limit
            } else {
              limit <= idx
            } {
              pc = pc.offset(luau_insn_d(insn) as isize);
              let p = cl_proto!(cl);
              LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
              continue 'dispatch;
            } else {
              // fallthrough to exit
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_FORGPREP => {
            // lvmexecute.cpp:2695
            let insn = *pc;
            pc = pc.add(1);
            let mut ra = VM_REG!(luau_insn_a(insn), l, base);

            // If this is a function it will be called during FORGLOOP
            // §11 pass B: function tag 判定收敛为 ValueView::Function 匹配。
            // 旗标双分支（cpp 旗开/旗关两份拷贝）除「Object 实例经 lua_t_gettmbyobj
            // 兜底物化 __iter」这一段（旗关无此步）外逐行同构，合并为单分支；
            // fast_tm 为纯读，前置求值与原 `if let Some = fast_tm(..)` 同值同序。
            if !matches!(ValueView::from_tvalue(&*ra), ValueView::Function(_)) {
              let mt = frame.slot_metatable(ra);
              let mut fn_tm = frame.fast_tm(mt, TMS::TmIter);

              // 旗开时 Object 实例的兜底：§11 pass B 次波 object tag 判定收敛为
              // ValueView::Object 一级变体（本臂只需知类型，payload 由下方
              // lua_t_gettmbyobj 自槽内自取）；旗关短路，不读 tag、与旧路径一致。
              if fflag::DebugLuauUserDefinedClassesRuntime.get()
                && fn_tm.is_none()
                && matches!(ValueView::from_tvalue(&*ra), ValueView::Object(_))
              {
                // lua_t_gettmbyobj 永不返回 null（LUA_O_NILOBJECT 兜底），原
                // `!fn_tm.is_null()` 判定恒真
                let tm = lua_t_gettmbyobj(l, ra, TMS::TmIter);
                // if the metamethod is not present, error.
                if matches!(ValueView::from_tvalue(&*tm), ValueView::Nil) {
                  (*(*l).ci).savedpc = pc; // vm_protect_pc(): next call always errors
                  lua_g_typeerror_l(l, ra, ERR_ITERATE_OVER);
                }
                fn_tm = Some(tm);
              }

              if let Some(fn_tm) = fn_tm {
                // §2 (a)：[ra, ra+3) 三槽窗口（func/self/游标）经 `VmFrame::slots_mut`
                // 切片视图读写，取代逐槽 `ra.add(n)` 手写算术；luaD_call 的 StkId
                // 实参即窗口首槽，top 边界取窗口第 3 格地址（与原 ra.add(2) 同值）
                let w = frame.slots_mut(ra, 3);
                setobj_2_s!(l, &raw mut w[1], &raw const w[0]);
                setobj_2_s!(l, &raw mut w[0], fn_tm);

                frame.set_top(&raw mut w[2]); // func + self arg
                LUAU_ASSERT!((*l).top <= (*l).stack_last);

                vm_protect!(l, pc, base, {
                  lua_d_call(l, &raw mut w[0], 3);
                });
                (*l).top = (*(*l).ci).top;

                // recompute ra since stack might have been reallocated
                ra = VM_REG!(luau_insn_a(insn), l, base);

                // protect against __iter returning nil, since nil is used
                // as a marker for builtin iteration in FORGLOOP
                if matches!(ValueView::from_tvalue(&*ra), ValueView::Nil) {
                  (*(*l).ci).savedpc = pc; // vm_protect_pc(): next call always errors
                  lua_g_typeerror_l(l, ra, "call");
                }
              } else if frame.fast_tm(mt, TMS::TmCall).is_some() {
                // table or userdata with __call, will be called during FORGLOOP
                // TODO: we might be able to stop supporting this depending
                // on whether it's used in practice
              } else if matches!(ValueView::from_tvalue(&*ra), ValueView::Table(_)) {
                // set up registers for builtin iteration
                let w = frame.slots_mut(ra, 3);
                setobj_2_s!(l, &raw mut w[1], &raw const w[0]);
                set_iterator_done(&raw mut w[2]);
                setnilvalue!(&mut w[0]);
              } else {
                (*(*l).ci).savedpc = pc; // vm_protect_pc(): next call always errors
                lua_g_typeerror_l(l, ra, ERR_ITERATE_OVER);
              }
            }

            pc = pc.offset(luau_insn_d(insn) as isize);
            let p = cl_proto!(cl);
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_FORGLOOP => {
            // lvmexecute.cpp:2807
            VM_INTERRUPT!(l, pc, base);
            // lvmexecute.cpp:2710-2711：LuauBackedgeHeapCheck 开启时回边额外 GC 检查
            if fflag::LuauBackedgeHeapCheck.get() {
              VM_CHECK_GC!(l, pc, base);
            }
            let insn = *pc;
            pc = pc.add(1);
            let mut ra = VM_REG!(luau_insn_a(insn), l, base);
            let aux: u32 = *pc;

            // fast-path: builtin table iteration
            // note: ra=nil guarantees ra+1=table and ra+2=userdata because of
            // the setup by FORGPREP* opcodes
            // TODO: remove the table check per guarantee above
            if (*ra).is_nil() && (*ra.add(1)).is_table() {
              let h = (*ra.add(1)).as_table_ptr();
              let mut index = pvalue!(ra.add(2)) as usize as i32;

              let sizearray = (*h).sizearray;

              // clear extra variables since we might have more than two
              // note: while aux encodes ipairs bit, when set we always use 2
              // variables, so it's safe to check this via a signed comparison
              if (aux as i32) > 2 {
                for i in 5..(3 + aux as usize) {
                  setnilvalue!(ra.add(i));
                }
              }

              // terminate ipairs-style traversal early when encountering nil
              if (aux as i32) < 0
                && (index as u32 >= sizearray as u32 || (*(*h).array.add(index as usize)).is_nil())
              {
                pc = pc.add(1);
                continue 'dispatch;
              }

              // first we advance index through the array portion
              // SAFETY 区间：index < sizearray 保证数组元素访问在界内
              while (index as u32) < sizearray as u32 {
                let e = (*h).array.add(index as usize);

                if !(*e).is_nil() {
                  set_iterator_index(ra.add(2), index);
                  setnvalue!(ra.add(3), (index + 1) as f64);
                  setobj_2_s!(l, ra.add(4), e);

                  pc = pc.offset(luau_insn_d(insn) as isize);
                  let p = cl_proto!(cl);
                  LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                  continue 'dispatch;
                }

                index += 1;
              }

              let sizenode = 1i32 << (*h).lsizenode;

              // then we advance index through the hash portion
              // SAFETY 区间：index-sizearray < sizenode 保证哈希节点访问在界内
              while ((index - sizearray) as u32) < sizenode as u32 {
                let n = (*h).node.add((index - sizearray) as usize);

                if !(*gval!(n)).is_nil() {
                  set_iterator_index(ra.add(2), index);
                  getnodekey!(l, ra.add(3), n);
                  setobj_2_s!(l, ra.add(4), gval!(n));

                  pc = pc.offset(luau_insn_d(insn) as isize);
                  let p = cl_proto!(cl);
                  LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                  continue 'dispatch;
                }

                index += 1;
              }

              // fallthrough to exit
              pc = pc.add(1);
              continue 'dispatch;
            } else {
              // note: it's safe to push arguments past top for complicated
              // reasons (see top of the file)
              setobj_2_s!(l, ra.add(5), ra.add(2));
              setobj_2_s!(l, ra.add(4), ra.add(1));
              setobj_2_s!(l, ra.add(3), ra);

              frame.set_top(ra.add(3 + 3)); // func + 2 args (state and index)
              LUAU_ASSERT!((*l).top <= (*l).stack_last);

              // DELIBERATE DEVIATION：本地 cpp（lvmexecute.cpp:2791）无条件
              // luaD_performcally；本仓按更新上游登记 `LuauYieldIter2`
              //（fflag.rs，Luau* 前缀默认使能），旗关走旧 GETITERIMPORT 路径
              if fflag::LuauYieldIter2.get() {
                let yielded: bool;
                vm_protect!(l, pc, base, {
                  yielded = lua_d_performcally(l, ra.add(3), aux as u8 as i32);
                });

                if yielded {
                  return; // goto exit
                }
              } else {
                vm_protect!(l, pc, base, {
                  lua_d_call(l, ra.add(3), aux as u8 as i32);
                });
              }

              (*l).top = (*(*l).ci).top;

              // recompute ra since stack might have been reallocated
              ra = VM_REG!(luau_insn_a(insn), l, base);

              // copy first variable back into the iteration index
              setobj_2_s!(l, ra.add(2), ra.add(3));

              // note that we need to increment pc by 1 to exit the loop since
              // we need to skip over aux
              pc = pc.offset(if (*ra.add(3)).is_nil() {
                1
              } else {
                luau_insn_d(insn) as isize
              });
              let p = cl_proto!(cl);
              LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
              continue 'dispatch;
            }
          }
          // NEXT/INEXT 两臂（lvmexecute.cpp:2830/2853）除控制槽判据（pairs 要
          // Nil、ipairs 要数值 0）外逐行同构，合并为单一臂按 op 分流判据。
          LuauOpcode::LOP_FORGPREP_NEXT | LuauOpcode::LOP_FORGPREP_INEXT => {
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);

            let control_ok = if op == LuauOpcode::LOP_FORGPREP_NEXT as u8 {
              (*ra.add(2)).is_nil()
            } else {
              (*ra.add(2)).is_number() && (*ra.add(2)).as_number() == 0.0
            };
            if (*(*cl).env).safeenv != 0 && (*ra.add(1)).is_table() && control_ok {
              setnilvalue!(ra);
              // ra+1 is already the table
              set_iterator_done(ra.add(2));
            } else if !(*ra).is_function() {
              (*(*l).ci).savedpc = pc; // vm_protect_pc(): next call always errors
              lua_g_typeerror_l(l, ra, ERR_ITERATE_OVER);
            }

            pc = pc.offset(luau_insn_d(insn) as isize);
            let p = cl_proto!(cl);
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_NATIVECALL => {
            // lvmexecute.cpp:2860
            let p = cl_proto!(cl);
            LUAU_ASSERT!(!(*p).execdata.is_null());

            let ci = (*l).ci;
            // 上游：`if (FFlag::LuauFastpcall) ci->flags |= LUA_CALLINFO_NATIVE;
            //        else ci->flags = LUA_CALLINFO_NATIVE;`
            // 取位或：fastpcall 打开时必须保留同一帧上的 HANDLE/PCALL/CALLING 等位。
            // 本仓库尚未移植 luauPF_*/pushhandlerci，Lua 帧此刻只可能是 0 或
            // NATIVE，故与上游等价，且对将来的移植保持正确。
            (*ci).flags |= LUA_CALLINFO_NATIVE as u32;
            (*ci).savedpc = (*p).code;

            // VM_HAS_NATIVE：enter 返回 1 表示稍后要 reentry，0 表示执行已交给 native
            let Some(enter) = (*(*l).global).ecb.enter else {
              // 上游此处是 `LUAU_ASSERT(!"Opcode is only valid when VM_HAS_NATIVE
              // is defined")` + `LUAU_UNREACHABLE()`：LOP_NATIVECALL 只可能由 native
              // codegen 注入，且 `ecb.enter` 缺失说明内部不变量已被破坏（损坏的
              // 字节码）。静默 `return` 会被调用方当成一次「正常结束」的执行，
              // 丢掉后续指令与栈上结果，故与上游一样显式终止。
              LUAU_ASSERT!(false);
              // 不变量已破坏（损坏字节码/缺失 ecb.enter），与上游 LUAU_UNREACHABLE 一致显式终止
              unreachable!("LOP_NATIVECALL 仅在定义 VM_HAS_NATIVE（注入 native 入口回调）时可执行")
            };

            if enter(l, p) == 1 {
              continue 'reentry; // goto reentry
            }

            return; // goto exit
          }
          LuauOpcode::LOP_GETVARARGS => {
            // lvmexecute.cpp:2902
            let insn = *pc;
            pc = pc.add(1);
            let b = luau_insn_b(insn) as i32 - 1;
            let n = {
              let closure_l = &(*cl).inner.l;
              base.offset_from((*(*l).ci).func) as i32 - (*closure_l.p).numparams as i32 - 1
            };

            if b == LUA_MULTRET {
              vm_protect!(l, pc, base, {
                luaD_checkstack!(l, n);
              });
              // previous call may change the stack
              let ra = VM_REG!(luau_insn_a(insn), l, base);

              // 变参个数 n 非正时窗口为空；目标 ra（>= base）与源 base-n..base 分居 base
              // 两侧，两区间不重叠，故定界成两条切片 zip 遍历
              let cnt = n.max(0) as usize;
              let dst = frame.slots_mut(ra, cnt);
              let src = frame.slots(base.sub(cnt), cnt);
              for (d, s) in dst.iter_mut().zip(src) {
                setobj_2_s!(l, d, s);
              }

              frame.set_top(ra.add(cnt));
              continue 'dispatch;
            } else {
              let ra = VM_REG!(luau_insn_a(insn), l, base);

              // C++ `for (j = 0; j < b && j < n; j++)`：拷贝数即 min(b, n)（b/n 可为
              // -1/非正，min 后 max(0) 收敛为空拷贝，与条件循环严格等价）；源起点用
              // 同一个非负长度回退，cnt>0 时它等于 n
              let nn = n.max(0) as usize;
              let cnt = b.min(n).max(0) as usize;
              let dst = frame.slots_mut(ra, cnt);
              let src = frame.slots(base.sub(nn), cnt);
              for (d, s) in dst.iter_mut().zip(src) {
                setobj_2_s!(l, d, s);
              }
              // C++ `for (j = n; j < b; j++)`：起点取 n（非拷贝进度），n >= b 时
              // 窗口尾部自然为空，与原循环一致
              // SAFETY 区间：n < b 时 [ra+n, ra+b) 落在 ra..ra+b 预留区内
              if n < b {
                let w = frame.slots_mut(ra, b as usize);
                for slot in &mut w[n as usize..] {
                  setnilvalue!(slot);
                }
              }
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_DUPCLOSURE => {
            // lvmexecute.cpp:2959
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let kv = VM_KV!(luau_insn_d(insn), cl, k);

            let kcl = (*kv).as_closure_ptr();

            (*(*l).ci).savedpc = pc; // vm_protect_pc(): lua_f_new_lclosure may fail due to OOM

            // clone closure if the environment is not shared
            // note: we save closure to stack early in case the code below
            // wants to capture it by value
            let mut ncl = if (*kcl).env == (*cl).env {
              kcl
            } else {
              let kp = {
                let l = &(*kcl).inner.l;
                l.p
              };
              lua_f_new_lclosure(l, (*kcl).nupvalues as i32, (*cl).env, kp)
            };
            setclvalue!(l, ra, ncl);

            // this loop does three things:
            // - if the closure was created anew, it just fills it with upvalues
            // - if the closure from the constant table is used, it fills it with
            //   upvalues so that it can be shared in the future
            // - if the closure is reused, it checks if the reuse is safe via
            //   rawequal, and falls back to duplicating the closure
            // (C++ restarts the loop with ui = -1 on lazy clone)
            let mut ui: i32 = 0;
            // §2 (a)：nupvalues 条 LOP_CAPTURE 指令窗口定界成切片（与 LOP_NEWCLOSURE
            // 的 `frame.insns` 同一约定），取代逐槽 `*pc.add(ui)` 手写算术
            let captures = frame.insns(pc, (*kcl).nupvalues as usize);
            // SAFETY 区间：ui < nupvalues 由循环界保证，captures 已按 nupvalues 定界
            // 保留 while 游走：ui 可整体归零重启（lazy clone 后重填全部 upvalue），
            // 可重启游标无等差区间可迭代
            while ui < (*kcl).nupvalues as i32 {
              let uinsn = captures[ui as usize];
              LUAU_ASSERT!(luau_insn_op(uinsn) == LuauOpcode::LOP_CAPTURE as u32);
              LUAU_ASSERT!(
                luau_insn_a(uinsn) == LuauCaptureType::LCT_VAL as u32
                  || luau_insn_a(uinsn) == LuauCaptureType::LCT_UPVAL as u32
              );

              let uv: *mut TValue = if luau_insn_a(uinsn) == LuauCaptureType::LCT_VAL as u32 {
                VM_REG!(luau_insn_b(uinsn), l, base)
              } else {
                VM_UV!(luau_insn_b(uinsn), cl)
              };

              let uref = {
                let l = &mut (*ncl).inner.l;
                l.uprefs.as_mut_ptr().add(ui as usize)
              };

              // check if the existing closure is safe to reuse
              if ncl == kcl && lua_o_rawequal_obj(uref, uv) != 0 {
                ui += 1;
                continue;
              }

              // lazily clone the closure and update the upvalues
              if ncl == kcl && (*kcl).preload == 0 {
                let kp = {
                  let l = &(*kcl).inner.l;
                  l.p
                };
                ncl = lua_f_new_lclosure(l, (*kcl).nupvalues as i32, (*cl).env, kp);
                setclvalue!(l, ra, ncl);

                ui = 0; // C++ `ui = -1; continue` — restart the loop to fill all upvalues
                continue;
              }

              // this updates a newly created closure, or an existing closure
              // created during preload, in which case we need a barrier
              setobj!(l, uref, uv);
              lua_c_barrier!(l, ncl, uv);
              ui += 1;
            }

            // this is a noop if ncl is newly created or shared successfully, but
            // it has to run after the closure is preloaded for the first time
            (*ncl).preload = 0;

            if kcl != ncl {
              vm_protect!(l, pc, base, {
                lua_c_check_gc!(l);
              });
            }

            pc = pc.add((*kcl).nupvalues as usize);
            continue 'dispatch;
          }
          LuauOpcode::LOP_PREPVARARGS => {
            // lvmexecute.cpp:2988
            let insn = *pc;
            pc = pc.add(1);
            let numparams = luau_insn_a(insn) as i32;

            // all fixed parameters are copied after the top so we need more stack space
            vm_protect!(l, pc, base, {
              luaD_checkstack!(l, (*cl).stacksize as i32 + numparams);
            });

            // the caller must have filled extra fixed arguments with nil
            LUAU_ASSERT!((*l).top.offset_from(base) as i32 >= numparams);

            // move fixed parameters to final position
            let fixed = base; // first fixed argument
            base = (*l).top; // final position of first argument

            // 定界两条切片 zip 遍历：源 fixed..+numparams 是调用方留下的实参，目标
            // base..+numparams 是变参重排后的落位区；上方断言 fixed+numparams <= base
            // 保证两窗口不重叠，故可并行借用（源写回用 *mut 而非只读，因为拷完要清源格）
            // SAFETY: 上方 luaD_checkstack 预留 stacksize+numparams 格，两段均在存活栈内
            let nparams = numparams as usize;
            let src = frame.slots_mut(fixed, nparams);
            let dst = frame.slots_mut(base, nparams);
            for (s, d) in src.iter_mut().zip(dst.iter_mut()) {
              setobj_2_s!(l, d, s);
              setnilvalue!(s); // 原位清 nil，保持新 base 之前不留悬挂值
            }

            // rewire our stack frame to point to the new base
            (*(*l).ci).base = base;
            (*(*l).ci).top = base.add((*cl).stacksize as usize);

            (*l).base = base;
            (*l).top = (*(*l).ci).top;
            continue 'dispatch;
          }
          LuauOpcode::LOP_JUMPBACK => {
            // lvmexecute.cpp:2988
            VM_INTERRUPT!(l, pc, base);
            // lvmexecute.cpp:3005-3006：LuauBackedgeHeapCheck 开启时回边额外 GC 检查
            if fflag::LuauBackedgeHeapCheck.get() {
              VM_CHECK_GC!(l, pc, base);
            }
            let insn = *pc;
            pc = pc.add(1);

            pc = pc.offset(luau_insn_d(insn) as isize);
            let p = cl_proto!(cl);
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }

          LuauOpcode::LOP_LOADKX => {
            // lvmexecute.cpp:2998
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k);

            setobj_2_s!(l, ra, kv);
            continue 'dispatch;
          }

          LuauOpcode::LOP_JUMPX => {
            // lvmexecute.cpp:3009
            VM_INTERRUPT!(l, pc, base);
            // lvmexecute.cpp:3028-3029：LuauBackedgeHeapCheck 开启时回边额外 GC 检查
            if fflag::LuauBackedgeHeapCheck.get() {
              VM_CHECK_GC!(l, pc, base);
            }
            let insn = *pc;
            pc = pc.add(1);

            pc = pc.offset(luau_insn_e(insn) as isize);
            let p = cl_proto!(cl);
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_FASTCALL => {
            // lvmexecute.cpp:3068
            let insn = *pc;
            pc = pc.add(1);
            let bfid = luau_insn_a(insn) as i32;
            let skip = luau_insn_c(insn) as i32;
            {
              // 仅为限制 p 作用域的调试断言块（cpp: VM_ASSERT_UINSNRESULTS 类似物）
              let p = cl_proto!(cl);
              LUAU_ASSERT!(
                ((pc.offset_from((*p).code) as i32 + skip) as u32) < (*p).sizecode as u32
              );
            }

            let call: Instruction = *pc.add(skip as usize);
            LUAU_ASSERT!(luau_insn_op(call) == LuauOpcode::LOP_CALL as u32);

            let ra = VM_REG!(luau_insn_a(call), l, base);

            let mut nparams = luau_insn_b(call) as i32 - 1;
            let nresults = luau_insn_c(call) as i32 - 1;

            nparams = if nparams == LUA_MULTRET {
              (*l).top.offset_from(ra.add(1)) as i32
            } else {
              nparams
            };

            if let Some(f) = LUAU_F_TABLE[bfid as usize]
              && (*(*cl).env).safeenv != 0
            {
              (*(*l).ci).savedpc = pc; // vm_protect_pc(): f may fail due to OOM

              let n = f(l, ra, ra.add(1), nresults, Some(&mut *ra.add(2)), nparams);

              if n >= 0 {
                // when nresults != MULTRET, l->top might be pointing to the middle
                // of stack frame if nparams is equal to MULTRET; restore
                // unconditionally to skip an extra check
                (*l).top = if nresults == LUA_MULTRET {
                  ra.add(n as usize)
                } else {
                  (*(*l).ci).top
                };

                // skip instructions that compute function as well as CALL
                pc = pc.add((skip + 1) as usize);
                let p = cl_proto!(cl);
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
              }
              // n < 0：continue execution through the fallback code
            }
            continue 'dispatch;
          }
          LuauOpcode::LOP_COVERAGE => {
            // lvmexecute.cpp:3080
            let insn = *pc;
            pc = pc.add(1);
            let mut hits: i32 = luau_insn_e(insn);

            // update hits with saturated add and patch the instruction in place
            hits = if hits < (1 << 23) - 1 { hits + 1 } else { hits };
            vm_patch_e(pc.sub(1), hits);

            continue 'dispatch;
          }

          LuauOpcode::LOP_CAPTURE => {
            // lvmexecute.cpp:3086
            // C++ LUAU_ASSERT(!"CAPTURE is a pseudo-opcode and must be
            // executed as part of NEWCLOSURE")
            LUAU_ASSERT!(false);
            unreachable!() // LUAU_UNREACHABLE()
          }
          LuauOpcode::LOP_SUBRK => {
            // lvmexecute.cpp:3107
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let kv = VM_KV!(luau_insn_b(insn), cl, k);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            // fast-path（§11 pass B：rc tag 判收敛为 ValueView 变体 match；kv 为数值
            // 常量，无 tag 分支，payload 仍按宏直读）
            if let ValueView::Number(nc) = ValueView::from_tvalue(&*rc) {
              setnvalue!(ra, (*kv).as_number() - nc);
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, kv, rc, TMS::TmSub, 'dispatch);
            }
          }
          LuauOpcode::LOP_DIVRK => {
            // lvmexecute.cpp:3135
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let kv = VM_KV!(luau_insn_b(insn), cl, k);
            let rc = VM_REG!(luau_insn_c(insn), l, base);

            // fast-path（§11 pass B：rc tag 判链收敛为 ValueView 变体 match；kv 为数值
            // 常量，无 tag 分支，payload 仍按宏直读）
            let vrc = ValueView::from_tvalue(&*rc);
            if let ValueView::Number(nc) = vrc {
              setnvalue!(ra, (*kv).as_number() / nc);
              continue 'dispatch;
            } else if let ValueView::Vector(_) = vrc {
              let nb = (*kv).as_number() as f32;
              let vc = frame.lanes(rc);
              setvvalue!(
                ra,
                nb / vc[0],
                nb / vc[1],
                nb / vc[2],
                nb / frame.lane_at(rc, 3)
              );
              continue 'dispatch;
            } else {
              arith_slow!(l, pc, base, ra, kv, rc, TMS::TmDiv, 'dispatch);
            }
          }
          LuauOpcode::LOP_FASTCALL1 => {
            // lvmexecute.cpp:3183
            let insn = *pc;
            pc = pc.add(1);
            let bfid = luau_insn_a(insn) as i32;
            let arg = VM_REG!(luau_insn_b(insn), l, base);
            let skip = luau_insn_c(insn) as i32;
            dispatch_fastcall!(l, cl, pc, base, bfid, skip, arg, None, 1i32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_FASTCALL2 | LuauOpcode::LOP_FASTCALL2K => {
            // lvmexecute.cpp:3233,3283
            let insn = *pc;
            pc = pc.add(1);
            let bfid = luau_insn_a(insn) as i32;
            let skip = luau_insn_c(insn) as i32 - 1;
            let aux: u32 = *pc;
            pc = pc.add(1);
            let arg1 = VM_REG!(luau_insn_b(insn), l, base);
            let arg2 = if op == LuauOpcode::LOP_FASTCALL2 as u8 {
              VM_REG!(aux, l, base)
            } else {
              VM_KV!(aux, cl, k)
            };
            dispatch_fastcall!(l, cl, pc, base, bfid, skip, arg1, Some(&mut *arg2), 2i32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_FASTCALL3 => {
            // lvmexecute.cpp:3340
            let insn = *pc;
            pc = pc.add(1);
            let bfid = luau_insn_a(insn) as i32;
            let skip = luau_insn_c(insn) as i32 - 1;
            let aux: u32 = *pc;
            pc = pc.add(1);
            let arg1 = VM_REG!(luau_insn_b(insn), l, base);
            let arg2 = VM_REG!(luau_insn_aux_a(aux), l, base);
            let arg3 = VM_REG!(luau_insn_aux_b(aux), l, base);
            LUAU_ASSERT!((*l).top.add(2) < (*l).stack.add((*l).stacksize as usize));
            let top = (*l).top;
            setobj_2_s!(l, top, arg2);
            setobj_2_s!(l, top.add(1), arg3);
            dispatch_fastcall!(l, cl, pc, base, bfid, skip, arg1, Some(&mut *top), 3i32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_BREAK => {
            // lvmexecute.cpp:3359
            let proto = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!(!(*proto).debuginsn.is_null());

            let op = *(*proto)
              .debuginsn
              .add(pc.offset_from((*proto).code) as usize);
            LUAU_ASSERT!(op != LuauOpcode::LOP_BREAK as u8);

            if (*(*l).global).cb.debugbreak.is_some() {
              let debugbreak = (*(*l).global).cb.debugbreak;
              vm_protect!(l, pc, base, {
                luau_callhook(l, debugbreak, None);
              });

              // allow debugbreak hook to put thread into error/yield state
              if (*l).status != 0 {
                return; // goto exit
              }
            }

            // VM_CONTINUE(op): re-dispatch the original opcode without refetching
            continue_op = op;
            continue 'dispatch;
          }
          LuauOpcode::LOP_JUMPXEQKNIL
          | LuauOpcode::LOP_JUMPXEQKB
          | LuauOpcode::LOP_JUMPXEQKN
          | LuauOpcode::LOP_JUMPXEQKS => {
            // lvmexecute.cpp:3372,3383,3405,3418
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(luau_insn_a(insn), l, base);

            let hit = match LuauOpcode::from(op) {
              LuauOpcode::LOP_JUMPXEQKNIL => (*ra).is_nil(),
              LuauOpcode::LOP_JUMPXEQKB => {
                (*ra).is_boolean() && (*ra).as_boolean_raw() == luau_insn_aux_kb(aux) as i32
              }
              LuauOpcode::LOP_JUMPXEQKN => {
                let kv = VM_KV!(luau_insn_aux_kv(aux), cl, k);
                LUAU_ASSERT!((*kv).is_number());
                (*ra).is_number() && (*ra).as_number() == (*kv).as_number()
              }
              _ => {
                let kv = VM_KV!(luau_insn_aux_kv(aux), cl, k);
                LUAU_ASSERT!((*kv).is_string());
                (*ra).is_string() && eq((*ra).as_string_ptr(), (*kv).as_string_ptr())
              }
            };

            pc = pc.offset(if (hit as u32) != luau_insn_aux_not(aux) {
              luau_insn_d(insn) as isize
            } else {
              1
            });
            let p = cl_proto!(cl);
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_GETUDATAKS => {
            // lvmexecute.cpp:3498（快路径单源见 udata_direct_fast! 宏文档）
            let insn = *pc;
            pc = pc.add(1);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kidx = luau_insn_aux_kv16(aux);
            let kv = VM_KV!(kidx, cl, k);

            udata_direct_fast!(
              l, pc, base, insn, rb, aux, kidx, kv, index, indextm, None::<*const TValue>, 1,
              LOP_GETUDATAKS, true, 'dispatch
            );

            // Slow path - backpatch and dispatch to regular table access
            vm_patch_op(pc.sub(2), LuauOpcode::LOP_GETTABLEKS as u8);
            vm_patch_aux_slot(pc.sub(1), kidx, 0);

            pc = pc.sub(2);
            continue_op = LuauOpcode::LOP_GETTABLEKS as u8; // VM_CONTINUE
            continue 'dispatch;
          }
          LuauOpcode::LOP_SETUDATAKS => {
            // lvmexecute.cpp:3573（快路径单源见 udata_direct_fast! 宏文档）
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kidx = luau_insn_aux_kv16(aux);
            let kv = VM_KV!(kidx, cl, k);

            udata_direct_fast!(
              l, pc, base, insn, rb, aux, kidx, kv, newindex, newindextm, Some(ra), 0,
              LOP_SETUDATAKS, false, 'dispatch
            );

            // Slow path - backpatch and dispatch to regular table access
            vm_patch_op(pc.sub(2), LuauOpcode::LOP_SETTABLEKS as u8);
            vm_patch_aux_slot(pc.sub(1), kidx, 0);

            pc = pc.sub(2);
            continue_op = LuauOpcode::LOP_SETTABLEKS as u8; // VM_CONTINUE
            continue 'dispatch;
          }
          LuauOpcode::LOP_NAMECALLUDATA => {
            // lvmexecute.cpp:3670
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let rb = VM_REG!(luau_insn_b(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kidx = luau_insn_aux_kv16(aux);
            let kv = VM_KV!(kidx, cl, k);

            'udata_fast: {
              // §11 pass B: rb 的 userdata tag 判定收敛为 ValueView::Userdata 匹配
              // （uvalue! 同为 *const Udata，payload 语义不变）
              if let ValueView::Userdata(u) = ValueView::from_tvalue(&*rb) {
                // cpp: `int utag = uvalue(rb)->tag;`（lvmexecute.cpp:3445/3519/3588）
                let utag = (*u).tag as usize;
                let udatadirect = &mut (*(*l).global).udatadirect[utag];
                let onudatanamecall = udatadirect.namecall;
                let tm = &mut udatadirect.namecalltm as *mut TValue;

                if let Some(onudatanamecall) = onudatanamecall
                  && !matches!(ValueView::from_tvalue(&*tm), ValueView::Nil)
                {
                  let udata = {
                    // cpp: `void* udata = uvalue(rb)->data;`（lvmexecute.cpp:3452/3526/3595）
                    (*u).data.as_ptr() as *mut c_void
                  };

                  // §2 (a)：[ra, ra+2) 双槽窗口经 slots_mut 切片视图读写
                  let w = frame.slots_mut(ra, 2);
                  // note: order of copies allows rb to alias ra+1 or ra
                  setobj_2_s!(l, &raw mut w[1], rb);
                  setobj_2_s!(l, &raw mut w[0], tm);
                  let ncslot: *const Instruction = pc.sub(1);

                  LUAU_ASSERT!(
                    luau_insn_op(*pc) == LuauOpcode::LOP_CALL as u32
                      || luau_insn_op(*pc) == LuauOpcode::LOP_CALLFB as u32
                  );
                  let call_insn = *pc;
                  pc = pc.add(1);
                  if fflag::LuauCallFeedback.get()
                    && luau_insn_op(call_insn) == LuauOpcode::LOP_CALLFB as u32
                  {
                    pc = pc.add(1);
                  }

                  let call_ra = VM_REG!(luau_insn_a(call_insn), l, base);
                  LUAU_ASSERT!(call_ra == ra);

                  // first half of OP_CALL
                  let nparams = luau_insn_b(call_insn) as i32 - 1;
                  let nresults = luau_insn_c(call_insn) as i32 - 1;

                  (*(*l).ci).savedpc = pc;
                  (*l).namecall = (*kv).as_string_ptr();
                  (*l).top = if nparams == LUA_MULTRET {
                    (*l).top
                  } else {
                    ra.add(1 + nparams as usize)
                  };

                  // note: namecalls do not increase C call number and allow yielding

                  luau_setupcci(l, nresults, ra);

                  LUAU_ASSERT!((*(*kv).as_string_ptr()).atom >= 0);

                  let mut cachedslot: u16 = luau_insn_aux_slot(aux) as u16;
                  let results = onudatanamecall(
                    l,
                    udata,
                    (*(*kv).as_string_ptr()).atom as i32,
                    &mut cachedslot,
                    utag as i32,
                  );

                  // update cached slot if instruction didn't deoptimize
                  if cachedslot as u32 != luau_insn_aux_slot(aux)
                    && luau_insn_op(*ncslot.sub(1)) == LuauOpcode::LOP_NAMECALLUDATA as u32
                  {
                    vm_patch_aux_slot(ncslot, kidx, cachedslot as i32);
                  }

                  // yield
                  if results < 0 {
                    return;
                  }

                  // 将返回值拷回父栈（最多 nresults 个），不足补 nil，并弹出本帧
                  // （lvmexecute.cpp:3688-3706，见 pop_frame_copy_results）
                  pop_frame_copy_results(l, (*l).top.sub(results as usize), (*l).top, nresults);

                  // stack may have been reallocated, so we need to refresh base ptr
                  base = (*l).base;

                  continue 'dispatch;
                }
              }
              break 'udata_fast;
            }

            // Slow path - backpatch and dispatch to regular namecall
            vm_patch_op(pc.sub(2), LuauOpcode::LOP_NAMECALL as u8);
            vm_patch_aux_slot(pc.sub(1), kidx, 0);

            pc = pc.sub(2);
            continue_op = LuauOpcode::LOP_NAMECALL as u8; // VM_CONTINUE
            continue 'dispatch;
          }
          LuauOpcode::LOP_NEWCLASSMEMBER => {
            // lvmexecute.cpp:3670 (NEWCLASSMEMBER)
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let membername = VM_KV!(aux, cl, k);
            LUAU_ASSERT!((*membername).is_string());
            LUAU_ASSERT!(luau_insn_b(insn) == 0);
            let rc = VM_REG!(luau_insn_c(insn), l, base);
            (*(*l).ci).savedpc = pc; // vm_protect_pc()
            lua_r_addclassmember(l, classvalue!(ra), (*membername).as_string_ptr(), rc);
            continue 'dispatch;
          }
          LuauOpcode::LOP_CMPPROTO => {
            // lvmexecute.cpp:3684
            let insn = *pc;
            pc = pc.add(1);
            let funid: u32 = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(luau_insn_a(insn), l, base);

            // §11 pass B: function tag 判定收敛为 ValueView::Function 匹配
            if !matches!(ValueView::from_tvalue(&*ra), ValueView::Function(_)) {
              pc = pc.offset(luau_insn_d(insn) as isize - 1);
              let p = cl_proto!(cl);
              LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
              continue 'dispatch;
            }

            let ccl = (*ra).as_closure_ptr();
            if (*ccl).is_c != 0 || {
              let l = &(*ccl).inner.l;
              (*l.p).funid != funid
            } {
              pc = pc.offset(luau_insn_d(insn) as isize - 1);
            }

            let p = cl_proto!(cl);
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }

          LuauOpcode::LOP_FASTPCALL => {
            // lvmexecute.cpp:3703 —— 上游第一件事就是
            // `if (!FFlag::LuauFastpcall) VM_NEXT();`：runtime 关掉该旗标时，
            // FASTPCALL 只是编译器留在慢路径之前的一枚标记（CodeGenerator 在
            // `emitABC(LOP_FASTPCALL, ...)` 之后照常 emit `compileExprTemp(func)`
            // + CALL，并由 BytecodeBuilder 校验 CALL 位于 +skip 处），VM 跳过
            // 标记本身即回到等价的受保护调用慢路径。
            //
            // 本仓库未移植 `LuauFastpcall` / `LuauFastpcallInterrupt` 旗标与
            // `luauPF_table` + `luau_pushhandlerci` / `luau_pospcallsuccess` +
            // `LUA_CALLINFO_PCALL` 这套 handler-ci 机制，因此这里恒定等价于
            // 上游 `LuauFastpcall = false`（即上游默认）配置：只少掉 fastpcall
            // 这一层加速，语义不变。另注：编译器侧 `LuauCompileFastpcall` 默认
            // 关闭；开启后发射的 FASTPCALL 后紧跟 func 重算 + CALL 慢路径，与本
            // 分支行为一致，装载上游字节码同样落到该慢路径。
            //
            // 消费标记指令（上游 `insn = *pc++;` + `VM_NEXT()`），A/C 字段留给
            // 将来的 fastpcall 实现使用。
            pc = pc.add(1);
            continue 'dispatch;
          }

          LuauOpcode::LOP_NEWCLASS => {
            // lvmexecute.cpp:3778
            let insn = *pc;
            pc = pc.add(1);
            let super_reg = luau_insn_b(insn) as u8;
            let aux = *pc;
            pc = pc.add(1);

            let ra = VM_REG!(luau_insn_a(insn), l, base);
            let kv = VM_KV!(aux, cl, k);

            // cpp:3785 常量表中的类形状 readonly——每次执行克隆出新鲜类，
            // isopen/继承/增员都只落在克隆上，不得写穿共享常量形状
            let newcls = lua_r_cloneclass(l, classvalue!(kv));
            setclassvalue!(l, ra, newcls);
            (*newcls).isopen = (luau_insn_c(insn) & 0x1) != 0;

            if super_reg != 0xff {
              (*(*l).ci).savedpc = pc; // vm_protect_pc()

              let rb = VM_REG!(super_reg, l, base);

              // §11 pass B 次波：rb 的 class tag 判定收敛为 ValueView::Class 一级变体
              // （inheritclass 要 `*mut LuauClass` 写出口，payload 仍按 `classvalue!`
              // 直取，与 Table/hvalue! 同款「已判 tag 后取载荷」形状）
              if !matches!(ValueView::from_tvalue(&*rb), ValueView::Class(_)) {
                luaG_typeerror!(l, rb, "extend");
              }

              // cpp:3798 inheritclass 为 void，原地改写 newcls——ra 已持克隆
              // 引用，继承期的分配不会使克隆失锚
              lua_r_inheritclass(l, newcls, classvalue!(rb));
            }

            continue 'dispatch;
          }

          // 操作码分发兜底：pc 取指后均已归入可执行操作码，落空即损坏字节码（cpp: LUAU_UNREACHABLE）
          _ => unreachable!("该字节不是可执行操作码"),
        }
      }
    }
  }
}
