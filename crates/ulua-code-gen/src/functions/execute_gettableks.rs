use ulua_common::{
  enums::luau_opcode::LuauOpcode,
  fflag::LuauDirectFieldGet,
  macros::{
    luau_assert::LUAU_ASSERT, luau_insn_a::luau_insn_a, luau_insn_aux_kv_16::luau_insn_aux_kv16,
    luau_insn_c::luau_insn_c, luau_insn_op::luau_insn_op,
  },
};
use ulua_vm::{
  enums::{lua_type::LuaType, tms::TMS},
  macros::{lua_o_nilobject::LUA_O_NILOBJECT, pvalue::pvalue},
  records::{lua_table::LuaTable, udata::Udata},
  type_aliases::{lua_userdata_direct_field_get::from_ptr, stk_id::StkId, t_value::TValue},
};

use crate::{
  records::vm_frame::VmFrame,
  type_aliases::{instruction_ir_builder::Instruction, lua_state::LuaState},
};

/// udata direct-fields 的 C 函数调用：dispatch gval 槽的唯一写入方是
/// `lua_registeruserdatadirectfieldget`（以类型化 `Option<LuaUserdataDirectFieldGet>`
/// 经 `setpvalue!` 存入 fn 指针位模式），此处经其声明的逆运算 `from_ptr` 做
/// niche 安全回读，再以 `(udata 数据指针, ra)` 调用。全部 direct-field dispatch 共用此收口。
///
/// 这是本 crate 与宿主注册回调的真实函数指针边界：位模式来自 VM 侧单写方注册，
/// 只在真正跨 C ABI 的那一调用上还原为裸地址形态（payload 首地址 / 结果栈槽）。
///
/// # Safety
///
/// `fn_tv` 必须指向由 `lua_registeruserdatadirectfieldget` 写入的 dispatch gval 槽
/// （位模式即该类型化函数指针，满足 `from_ptr` 契约）；`udata` 必须是 `uvalue!` 取自存活
/// userdata TValue 的 `Udata` 视图；`ra` 必须是有效可写结果栈槽。
#[inline]
unsafe fn call_udata_direct_field(fn_tv: *const TValue, udata: *const Udata, ra: StkId) {
  // Safety: 槽位类型化单写方保证位模式合法（见函数注释）；空位读回 None，
  // 调用点已先经 `!ttisnil!` 守卫，此处 `if let` 仅为纵深防御。返回地址只在真正跨
  // C ABI 的那一调上还原为裸地址形态（payload 首地址 / 结果栈槽）。
  unsafe {
    if let Some(get_field) = from_ptr(pvalue!(fn_tv)) {
      get_field((*udata).data.as_ptr().cast_mut().cast(), ra.cast());
    }
  }
}

/// 折出 `__index` C 函数元方法槽：userdata fasttm / vector 元表两分支共用，
/// cpp `(fn = fasttm(L, mt, TM_INDEX)) && ttisfunction(fn) && clvalue(fn)->isC` 短路链的等价收口。
///
/// 返回裸指针而非 `&TValue`：槽位属 GC 堆上的元表节点，而本函数所在慢路径随后要跨
/// `lua_v_call_tm`/宿主 C 回调（可重入 VM 与触发 GC），伪造共享借用会掩盖这段别名史。
#[inline]
fn c_index_tm(frame: &VmFrame, mt: Option<*mut LuaTable>) -> Option<*const TValue> {
  frame
    .meta_method(mt, TMS::TmIndex)
    .filter(|tm| frame.is_function(*tm) && frame.closure_is_c(frame.clvalue(*tm)))
}

/// `__index` C 函数慢路径（userdata fasttm / vector 元表两分支共用）：
/// 压入 (fn, obj, key) 后以 calltm 调用，并回填 cachedslot。
///
/// `next_pc` 是本指令之后那条指令的地址（cpp 里两次 `pc++` 之后的 pc，用作 savedpc 与返回值），
/// `insn_pc` 是本指令主字的地址（`VM_PATCH_C` 的落地槽）。
#[inline]
fn call_index_c_function(
  frame: &VmFrame,
  next_pc: *const Instruction,
  insn_pc: *const Instruction,
  fn_tm: *const TValue,
  obj: *const TValue,
  kv: *const TValue,
  insn: Instruction,
) -> *const Instruction {
  // 入口 LUAU_ASSERT 复核 top+3 未越界后，三次栈槽写与 top 推进均界内。
  frame.assert_top_fits(3);
  let top = frame.top();
  frame.set_stack_value(frame.slot_at(top, 0), fn_tm);
  frame.set_stack_value(frame.slot_at(top, 1), obj);
  frame.set_stack_value(frame.slot_at(top, 2), kv);
  frame.set_top(frame.slot_at(top, 3));

  frame.set_cachedslot(luau_insn_c(insn) as i32);
  frame.protect(next_pc, |frame| {
    frame.call_tm(2, luau_insn_a(insn) as i32);
  });
  frame.patch_c(insn_pc, frame.cachedslot());
  next_pc
}

/// 生成码回写的 GETTABLEKS/GETUDATAKS 慢路径解释器（cpp `executeGETTABLEKS`）。
///
/// # Safety
/// VM 回调 ABI 约定：`l` 为存活 `LuaState`（其 `ci.func` 为闭包、`top`/`stack`/`stacksize`/
/// `global` 一致有效）；`pc` 指向当前 `LOP_GETTABLEKS`/`LOP_GETUDATAKS` 指令（主字 + AUX
/// 常量字）；`base` 为合法栈基址；`k` 指向当前 proto 的常量表。边界契约集中于
/// [`VmFrame::new`] 与 `call_udata_direct_field`，其余为安全逻辑。
pub unsafe fn execute_gettableks(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  let frame = unsafe { VmFrame::new(l, base) };

  let cl = frame.current_closure();
  // 本指令占两字：主字在 `pc`、AUX 常量字在 `pc + 1`。`next_pc` 即 cpp 里两次 `pc++` 之后的
  // pc（下一条指令，同时是 savedpc 与本函数的返回值）；patch 目标回到主字 `pc`（cpp `pc - 2`）。
  let words = frame.insns(pc, 2);
  let (insn, aux) = (words[0], words[1]);
  let next_pc = frame.insn_offset(pc, 2);

  let ra = frame.reg_a(insn);
  let rb = frame.reg_b(insn);
  // GETUDATAKS 的常量号在 AUX 字低 16 位，其余走整 AUX 字（cpp 两 opcode 共用本路径）。
  let kw = if luau_insn_op(insn) == LuauOpcode::LOP_GETUDATAKS as u32 {
    luau_insn_aux_kv16(aux)
  } else {
    aux
  };
  let kv = frame.kv(kw, cl, k);
  LUAU_ASSERT!(frame.is_string(kv));

  let (rb_is_table, rb_is_userdata, rb_is_vector) = (
    frame.is_table(rb),
    frame.is_userdata(rb),
    frame.is_vector(rb),
  );

  // 快路径：内置表
  if rb_is_table {
    let h = frame.hvalue(rb);

    if frame.table_metatable(h).is_none() {
      // 快路径：值不在预期槽，但查表不涉及元表
      let res = frame.get_str(h, frame.tsvalue(kv).cast_mut());

      if res != LUA_O_NILOBJECT {
        // 回填 cachedslot 以加速后续查找
        frame.patch_c(pc, frame.value_to_slot(h, res));
      }
      frame.set_stack_value(ra, res);
      return next_pc;
    }

    // 慢路径，可能经 __index 元方法调用 Lua
    frame.set_cachedslot(luau_insn_c(insn) as i32 & frame.table_nodemask8(h));
    frame.protect(next_pc, |frame| {
      frame.gettable(rb, kv, ra);
    });
    frame.patch_c(pc, frame.cachedslot());
    return next_pc;
  }

  if rb_is_userdata {
    let udata = frame.uvalue(rb);

    // 快路径：注册过的 direct field 处理器
    if LuauDirectFieldGet.get()
      && let Some(dispatch) = frame.udata_direct_field_dispatch(udata)
    {
      // 已缓存槽命中检查：node 数组按槽号寻址（VmFrame::table_node 的 TValue 视图约定）。
      let (node_key, val_slot) = frame.table_node(
        dispatch,
        (luau_insn_c(insn) as i32 & frame.table_nodemask8(dispatch)) as usize,
      );

      let cached_hit = frame.is_string(node_key)
        && frame.tsvalue(node_key) == frame.tsvalue(kv)
        && !frame.is_nil(val_slot);
      if cached_hit {
        // Safety: val_slot 为 dispatch 表缓存的 direct-field 函数槽（单写方约定）。
        unsafe { call_udata_direct_field(val_slot, udata, ra) };
        return next_pc;
      }

      let fptr = frame.get_str(dispatch, frame.tsvalue(kv).cast_mut());
      if !frame.is_nil(fptr) {
        // 缓存槽位以加速后续查找
        frame.patch_c(pc, frame.value_to_slot(dispatch, fptr));
        // Safety: fptr 为查表返回的 direct-field 函数槽（单写方约定）。
        unsafe { call_udata_direct_field(fptr, udata, ra) };
        return next_pc;
      }
    }

    // 快路径：带 C __index 元方法的 userdata；否则落穿到慢路径
    if let Some(fn_tm) = c_index_tm(&frame, frame.udata_metatable(udata)) {
      return call_index_c_function(&frame, next_pc, pc, fn_tm, rb, kv, insn);
    }
  } else if rb_is_vector {
    // 快路径：与 "X"/"Y"/"Z" 做大小写无关的单字符比较。`string_bytes` 契约保证可读
    // len + 1 字节（空串也有 NUL 终止符），故切片视图覆盖到终止符；第二字节为 NUL 即
    // cpp 的 `name[1] == '\0'`（名字实际到此结束）。
    let vec = frame.vector_lanes(rb);
    let name = frame.string_bytes(frame.tsvalue(kv));
    let component = match name {
      [b'x' | b'X', b'\0', ..] => Some(0),
      [b'y' | b'Y', b'\0', ..] => Some(1),
      [b'z' | b'Z', b'\0', ..] => Some(2),
      _ => None,
    };

    if let Some(component) = component {
      frame.set_number(ra, f64::from(vec[component]));
      return next_pc;
    }

    // 组件名不匹配时回退到 vector 元表的 C `__index`；否则落穿到慢路径
    if let Some(fn_tm) = c_index_tm(&frame, frame.global_metatable(LuaType::Vector as u32)) {
      return call_index_c_function(&frame, next_pc, pc, fn_tm, rb, kv, insn);
    }
  }

  // 慢路径，可能经 __index 元方法调用 Lua
  frame.protect(next_pc, |frame| {
    frame.gettable(rb, kv, ra);
  });
  next_pc
}

/// # Safety
/// C-ABI 导出边界:由宿主/VM 依 Lua codegen 回调约定调用,`l`/`pc`/`base`/`k` 的合法性与存活前提
/// 与 [`execute_gettableks`] 的契约完全一致(本函数仅原样透传)。
pub unsafe extern "C-unwind" fn execute_gettableks_export(
  l: *mut LuaState,
  pc: *const Instruction,
  base: StkId,
  k: *mut TValue,
) -> *const Instruction {
  // Safety: 本 export 的 `unsafe fn` 契约与 `execute_gettableks` 的入参要求逐字相同(见上),
  // 故把同一组 `l`/`pc`/`base`/`k` 原样透传即满足被调 unsafe fn 的全部前置条件。
  unsafe { execute_gettableks(l, pc, base, k) }
}
