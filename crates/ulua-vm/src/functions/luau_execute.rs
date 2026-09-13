//! Node: `cxx:Function:Luau.VM:VM/src/lvmexecute.cpp:3716:luau_execute`
//! Source: `VM/src/lvmexecute.cpp:228-3722` (hand-port in progress; design
//! card at `translation/design-cards/lvmexecute.md`)
//!
//! ALL 89 `VM_CASE` opcode arms live HERE as match arms (the per-arm
//! `vm_case_lvmexecute*.rs` node files are macro-extraction artifacts and
//! become doc-pointers as their arms land). Control-flow mapping:
//! `VM_NEXT()` -> `continue 'dispatch`; `VM_CONTINUE(op)` -> set
//! `continue_op` + `continue 'dispatch`; `goto exit` -> `return`.

use core::{
  cmp::Ordering::{Equal, Less},
  ffi::{c_char, c_void},
  mem::{transmute, zeroed},
  ptr::{null, null_mut},
};

use ulua_common::{
  FFlag,
  enums::{luau_capture_type::LuauCaptureType, luau_opcode::LuauOpcode},
  macros::{
    luau_assert::LUAU_ASSERT, luau_insn_a::LUAU_INSN_A, luau_insn_aux_a::LUAU_INSN_AUX_A,
    luau_insn_aux_b::LUAU_INSN_AUX_B, luau_insn_aux_kb::LUAU_INSN_AUX_KB,
    luau_insn_aux_kv::LUAU_INSN_AUX_KV, luau_insn_aux_kv_16::LUAU_INSN_AUX_KV16,
    luau_insn_aux_not::LUAU_INSN_AUX_NOT, luau_insn_aux_slot::LUAU_INSN_AUX_SLOT,
    luau_insn_b::LUAU_INSN_B, luau_insn_c::LUAU_INSN_C, luau_insn_d::LUAU_INSN_D,
    luau_insn_e::LUAU_INSN_E, luau_insn_fbslot_sealed::LUAU_INSN_FBSLOT_SEALED,
    luau_insn_op::LUAU_INSN_OP,
  },
};

use crate::{
  enums::{lua_type::LuaType, tms::TMS},
  functions::{
    lua_d_call::lua_d_call, lua_d_check_cstack::luaD_checkCstack,
    lua_d_performcally::lua_d_performcally, lua_f_close::lua_f_close,
    lua_f_findupval::lua_f_findupval, lua_f_new_lclosure::lua_f_new_lclosure,
    lua_f_recordhit::luaF_recordhit, lua_g_methoderror::luaG_methoderror,
    lua_g_missingmembererror::luaG_missingmembererror, lua_g_typeerror_l::luaG_typeerrorL,
    lua_h_clone::lua_h_clone, lua_h_getn::lua_h_getn, lua_h_getstr::luaH_getstr,
    lua_h_new::lua_h_new, lua_h_resizearray::lua_h_resizearray,
    lua_h_setstr::lua_h_setstr as luaH_setstr, lua_o_rawequal_obj::luaO_rawequalObj,
    lua_r_addclassmember::lua_r_addclassmember, lua_r_inheritclass::lua_r_inheritclass,
    lua_t_gettmbyobj::lua_t_gettmbyobj, lua_v_call_tm::lua_v_call_tm, lua_v_concat::lua_v_concat,
    lua_v_doarithimpl::lua_v_doarithimpl, lua_v_dolen::lua_v_dolen, lua_v_equalval::lua_v_equalval,
    lua_v_getimport::lua_v_getimport, lua_v_gettable::lua_v_gettable,
    lua_v_lessequal::lua_v_lessequal, lua_v_lessthan::lua_v_lessthan,
    lua_v_prepare_forn::lua_v_prepare_forn, lua_v_settable::lua_v_settable,
    lua_v_strcmp::lua_v_strcmp, lua_v_tryfunc_tm::lua_v_tryfunc_tm, luai_numidiv::luai_numidiv,
    luai_nummod::luai_nummod, luai_veceq::luai_veceq, luau_callhook::luau_callhook,
    luau_setupcci::luau_setupcci, luau_skipstep::luau_skipstep,
  },
  macros::{
    bvalue::bvalue,
    classvalue::classvalue,
    clvalue::clvalue,
    fastnotm::fastnotm,
    fasttm::fasttm,
    gcvalue::gcvalue,
    getnodekey::getnodekey,
    getstr::getstr,
    gkey::{gkey, gval},
    gnext::gnext,
    gval_2_slot::gval2slot,
    hvalue::hvalue,
    incr_ci::incr_ci,
    is_lua::isLua,
    l_isfalse::l_isfalse,
    lightuserdatatag::lightuserdatatag,
    lu_tag_iterator::LU_TAG_ITERATOR,
    lua_c_barrier::luaC_barrier,
    lua_c_barrierfast::luaC_barrierfast,
    lua_c_barriert::luaC_barriert,
    lua_c_check_gc::luaC_checkGC,
    lua_callinfo_native::LUA_CALLINFO_NATIVE,
    lua_callinfo_return::LUA_CALLINFO_RETURN,
    lua_d_checkstack::luaD_checkstack,
    lua_d_checkstackfornewci::luaD_checkstackfornewci,
    lua_g_typeerror::luaG_typeerror,
    lua_multret::LUA_MULTRET,
    lua_o_nilobject::luaO_nilobject,
    lua_r_lookupmemberatoffset::luaR_lookupmemberatoffset,
    luai_maxccalls::LUAI_MAXCCALLS,
    luau_f_table::luauF_table,
    lvalue::lvalue,
    nvalue::nvalue,
    objectvalue::objectvalue,
    pvalue::pvalue,
    setbvalue::setbvalue,
    setclassvalue::setclassvalue,
    setclvalue::setclvalue,
    sethvalue::sethvalue,
    setnilvalue::setnilvalue,
    setnvalue::setnvalue,
    setobj::setobj,
    setobj_2_s::setobj_2_s,
    setobj_2_t::setobj2t,
    setpvalue::setpvalue,
    setupvalue::setupvalue,
    setvvalue::setvvalue,
    sizenode::sizenode,
    tsvalue::tsvalue,
    ttisboolean::ttisboolean,
    ttisclass::ttisclass,
    ttisfunction::ttisfunction,
    ttisnil::ttisnil,
    ttisnumber::ttisnumber,
    ttisobject::ttisobject,
    ttisstring::ttisstring,
    ttistable::ttistable,
    ttisupval::ttisupval,
    ttisuserdata::ttisuserdata,
    ttisvector::ttisvector,
    ttype::ttype,
    upvalue::upvalue,
    uvalue::uvalue,
    vm_interrupt::VM_INTERRUPT,
    vm_kv::VM_KV,
    vm_patch_aux::vm_patch_aux,
    vm_patch_aux_slot::vm_patch_aux_slot,
    vm_patch_c::vm_patch_c,
    vm_patch_e::vm_patch_e,
    vm_patch_op::vm_patch_op,
    vm_protect::vm_protect,
    vm_reg::VM_REG,
    vm_uv::VM_UV,
    vvalue::vvalue,
  },
  records::{
    closure::{Closure, LClosure},
    lua_state::lua_State,
    luau_class::LuauClass,
    luau_object::LuauObject,
    t_string::tstring,
    up_val::UpVal,
  },
  type_aliases::{instruction::Instruction, stk_id::StkId, t_value::TValue},
};

/// C++ `void luau_execute(lua_State* l)` (lvmexecute.cpp:3716) — dispatches
/// to the `template<bool SingleStep>` monomorphs.
/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn luau_execute(l: *mut lua_State) {
  unsafe {
    if (*l).singlestep {
      luau_execute_impl::<true>(l)
    } else {
      luau_execute_impl::<false>(l)
    }
  }
}

/// C++ `template<bool SingleStep> static void luau_execute(lua_State* l)`
/// (lvmexecute.cpp:228). The computed-goto dispatch table becomes the match
/// below (both blindly index by the opcode byte).
unsafe fn luau_execute_impl<const SINGLE_STEP: bool>(l: *mut lua_State) {
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
      let native_cl = clvalue!((*(*l).ci).func);
      let native_lcl = core::ptr::addr_of!((*native_cl).inner.l).cast::<LClosure>();
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
      cl = clvalue!((*(*l).ci).func);
      base = (*l).base;
      k = {
        let l = &(*cl).inner.l;
        (*l.p).k
      };

      // C++ `VM_CONTINUE(op)` re-dispatches WITHOUT refetching `*pc`.
      let mut continue_op: Option<u8> = None;

      // C++ `dispatch:` label; `VM_NEXT()` == `continue 'dispatch`.
      'dispatch: loop {
        // Note: in C++ this assert block is bypassed by computed goto
        // except in single-step mode; asserts only.
        if SINGLE_STEP
          && continue_op.is_none()
          && (*(*l).global).cb.debugstep.is_some()
          && !luau_skipstep(LUAU_INSN_OP!(*pc) as u8)
        {
          let debugstep = (*(*l).global).cb.debugstep;
          vm_protect!(l, pc, base, {
            luau_callhook(l, debugstep, null_mut());
          });
          // allow debugstep hook to put thread into error/yield state
          if (*l).status != 0 {
            return; // goto exit
          }
        }

        let op: u8 = match continue_op.take() {
          Some(op) => op,
          None => LUAU_INSN_OP!(*pc) as u8,
        };

        // The C++ jump table indexes the opcode byte blindly; so do we.
        match transmute::<u8, LuauOpcode>(op) {
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
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);

            setnilvalue!(ra as *mut TValue);
            continue 'dispatch;
          }

          LuauOpcode::LOP_LOADB => {
            // lvmexecute.cpp:335
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);

            setbvalue!(ra as *mut TValue, LUAU_INSN_B!(insn) as i32);

            pc = pc.add(LUAU_INSN_C!(insn) as usize);
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }

          LuauOpcode::LOP_LOADN => {
            // lvmexecute.cpp:347
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);

            setnvalue!(ra as *mut TValue, LUAU_INSN_D!(insn) as f64);
            continue 'dispatch;
          }

          LuauOpcode::LOP_LOADK => {
            // lvmexecute.cpp:356
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);
            let kv = VM_KV!(LUAU_INSN_D!(insn), cl, k);

            setobj_2_s!(l, ra as *mut TValue, kv as *const TValue);
            continue 'dispatch;
          }

          LuauOpcode::LOP_MOVE => {
            // lvmexecute.cpp:366
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base);

            setobj_2_s!(l, ra as *mut TValue, rb as *const TValue);
            continue 'dispatch;
          }

          LuauOpcode::LOP_GETGLOBAL => {
            // lvmexecute.cpp:376
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k);
            LUAU_ASSERT!(ttisstring!(kv as *const TValue));

            // fast-path: value is in expected slot
            let h = (*cl).env;
            let slot = (LUAU_INSN_C!(insn) as i32) & (*h).nodemask8 as i32;
            let n = (*h).node.add(slot as usize);

            if ttisstring!(gkey!(n) as *const TValue)
              && tsvalue!(gkey!(n) as *const TValue) == tsvalue!(kv as *const TValue)
              && !ttisnil!(gval!(n))
            {
              setobj_2_s!(l, ra as *mut TValue, gval!(n));
              continue 'dispatch;
            } else {
              // slow-path, may invoke Lua calls via __index metamethod
              let mut g: TValue = zeroed();
              sethvalue!(l, &mut g as *mut TValue, h);
              (*l).cachedslot = slot;
              vm_protect!(l, pc, base, {
                lua_v_gettable(l, &g as *const TValue, kv as *mut TValue, ra);
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
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k);
            LUAU_ASSERT!(ttisstring!(kv as *const TValue));

            // fast-path: value is in expected slot
            let h = (*cl).env;
            let slot = (LUAU_INSN_C!(insn) as i32) & (*h).nodemask8 as i32;
            let n = (*h).node.add(slot as usize);

            if ttisstring!(gkey!(n) as *const TValue)
              && tsvalue!(gkey!(n) as *const TValue) == tsvalue!(kv as *const TValue)
              && !ttisnil!(gval!(n))
              && (*h).readonly == 0
            {
              setobj2t!(l, gval!(n), ra as *const TValue);
              luaC_barriert!(l, h, ra as *const TValue);
              continue 'dispatch;
            } else {
              // slow-path, may invoke Lua calls via __newindex metamethod
              let mut g: TValue = zeroed();
              sethvalue!(l, &mut g as *mut TValue, h);
              (*l).cachedslot = slot;
              vm_protect!(l, pc, base, {
                lua_v_settable(l, &g as *const TValue, kv as *mut TValue, ra);
              });
              vm_patch_c(pc.sub(2), (*l).cachedslot);
              continue 'dispatch;
            }
          }

          LuauOpcode::LOP_GETUPVAL => {
            // lvmexecute.cpp:439
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);
            let ur = VM_UV!(LUAU_INSN_B!(insn), cl);
            let v: *mut TValue = if ttisupval!(ur as *const TValue) {
              upvalue!(ur as *mut TValue).v
            } else {
              ur as *mut TValue
            };

            setobj_2_s!(l, ra as *mut TValue, v as *const TValue);
            continue 'dispatch;
          }

          LuauOpcode::LOP_SETUPVAL => {
            // lvmexecute.cpp:450
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);
            let ur = VM_UV!(LUAU_INSN_B!(insn), cl);
            let uv = &mut **upvalue!(ur as *mut TValue) as *mut UpVal;

            setobj!(l, (*uv).v, ra as *const TValue);
            luaC_barrier!(l, uv, ra as *const TValue);
            continue 'dispatch;
          }

          LuauOpcode::LOP_CLOSEUPVALS => {
            // lvmexecute.cpp:462
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);

            if !(*l).openupval.is_null() && (*(*l).openupval).v >= ra as *mut TValue {
              lua_f_close(l, ra as *mut TValue);
            }
            continue 'dispatch;
          }

          LuauOpcode::LOP_GETIMPORT => {
            // lvmexecute.cpp:472
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);
            let kv = VM_KV!(LUAU_INSN_D!(insn), cl, k);

            // fast-path: import resolution was successful and closure
            // environment is "safe" for import
            if !ttisnil!(kv as *const TValue) && (*(*cl).env).safeenv != 0 {
              setobj_2_s!(l, ra as *mut TValue, kv as *const TValue);
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
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k) as *mut TValue;
            LUAU_ASSERT!(ttisstring!(kv as *const TValue));

            // fast-path: built-in table
            if ttistable!(rb as *const TValue) {
              let h = hvalue!(rb as *const TValue);

              let slot = (LUAU_INSN_C!(insn) as i32) & (*h).nodemask8 as i32;
              let n = (*h).node.add(slot as usize);

              // fast-path: value is in expected slot
              if ttisstring!(gkey!(n) as *const TValue)
                && tsvalue!(gkey!(n) as *const TValue) == tsvalue!(kv as *const TValue)
                && !ttisnil!(gval!(n))
              {
                setobj_2_s!(l, ra, gval!(n));
                continue 'dispatch;
              } else if (*h).metatable.is_null() {
                // fast-path: value is not in expected slot, but the table
                // lookup doesn't involve metatable
                let res = luaH_getstr(h, tsvalue!(kv as *const TValue) as *mut tstring);

                if res != luaO_nilobject {
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
                  lua_v_gettable(l, rb as *const TValue, kv, ra);
                });
                vm_patch_c(pc.sub(2), (*l).cachedslot);
                continue 'dispatch;
              }
            } else {
              // fast-path: registered direct field handler
              if FFlag::LuauDirectFieldGet.get() && ttisuserdata!(rb as *const TValue) {
                let dispatch_t = {
                  let u = uvalue!(rb as *const TValue);
                  (*(*l).global).udatadirectfields[u.tag as usize]
                };
                if !dispatch_t.is_null() {
                  let slot = (LUAU_INSN_C!(insn) as i32) & (*dispatch_t).nodemask8 as i32;
                  let n = (*dispatch_t).node.add(slot as usize);

                  if ttisstring!(gkey!(n) as *const TValue)
                    && tsvalue!(gkey!(n) as *const TValue) == tsvalue!(kv as *const TValue)
                    && !ttisnil!(gval!(n))
                  {
                    let f: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void) =
                      transmute(pvalue!(gval!(n) as *const TValue));
                    let u = uvalue!(rb as *const TValue);
                    f(u.data.as_ptr() as *mut c_void, ra as *mut c_void);
                    continue 'dispatch;
                  }

                  let fptr = luaH_getstr(dispatch_t, tsvalue!(kv as *const TValue) as *mut tstring);
                  if !ttisnil!(fptr) {
                    // cache slot for future lookups
                    vm_patch_c(pc.sub(2), gval2slot!(dispatch_t, fptr));
                    let f: unsafe extern "C-unwind" fn(*mut c_void, *mut c_void) =
                      transmute(pvalue!(fptr));
                    let u = uvalue!(rb as *const TValue);
                    f(u.data.as_ptr() as *mut c_void, ra as *mut c_void);
                    continue 'dispatch;
                  }
                }

                // fall through to slow path
              }

              // fast-path: user data with C __index TM
              let fn_tm = if ttisuserdata!(rb as *const TValue) {
                fasttm(
                  l,
                  uvalue!(rb as *const TValue).metatable,
                  TMS::TmIndex as i32,
                )
              } else {
                null()
              };
              if !fn_tm.is_null() && ttisfunction!(fn_tm) && (*clvalue!(fn_tm)).is_c != 0 {
                // note: it's safe to push arguments past top for
                // complicated reasons (see top of the file)
                LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                let top = (*l).top;
                setobj_2_s!(l, top.add(0), fn_tm);
                setobj_2_s!(l, top.add(1), rb as *const TValue);
                setobj_2_s!(l, top.add(2), kv as *const TValue);
                (*l).top = top.add(3);

                (*l).cachedslot = LUAU_INSN_C!(insn) as i32;
                vm_protect!(l, pc, base, {
                  lua_v_call_tm(l, 2, LUAU_INSN_A!(insn) as i32);
                });
                vm_patch_c(pc.sub(2), (*l).cachedslot);
                continue 'dispatch;
              } else if ttisvector!(rb as *const TValue) {
                // fast-path: quick case-insensitive comparison with "X"/"Y"/"Z"
                let name = getstr(tsvalue!(kv as *const TValue));
                let ic = ((*name.add(0)) as u8 | b' ') as i32 - b'x' as i32;
                // (LUA_VECTOR_SIZE == 3 in this port; the C++ `== 4` branch
                // maps 'w' -> 3 and is omitted)

                if (ic as u32) < 3 && *name.add(1) == 0 {
                  let v = vvalue!(rb as *const TValue).as_ptr(); // silences ubsan when indexing v[]
                  setnvalue!(ra, *v.add(ic as usize) as f64);
                  continue 'dispatch;
                }

                let fn_tm = fasttm(
                  l,
                  (*(*l).global).mt[LuaType::Vector as usize],
                  TMS::TmIndex as i32,
                );

                if !fn_tm.is_null() && ttisfunction!(fn_tm) && (*clvalue!(fn_tm)).is_c != 0 {
                  // note: it's safe to push arguments past top for
                  // complicated reasons (see top of the file)
                  LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                  let top = (*l).top;
                  setobj_2_s!(l, top.add(0), fn_tm);
                  setobj_2_s!(l, top.add(1), rb as *const TValue);
                  setobj_2_s!(l, top.add(2), kv as *const TValue);
                  (*l).top = top.add(3);

                  (*l).cachedslot = LUAU_INSN_C!(insn) as i32;
                  vm_protect!(l, pc, base, {
                    lua_v_call_tm(l, 2, LUAU_INSN_A!(insn) as i32);
                  });
                  vm_patch_c(pc.sub(2), (*l).cachedslot);
                  continue 'dispatch;
                }

                // fall through to slow path
              } else if FFlag::DebugLuauUserDefinedClassesRuntime.get()
                && ttisobject!(rb as *const TValue)
              {
                // fast-path: the "hash line" is an offset that points
                // to the class member with the same name.
                let slot = LUAU_INSN_C!(insn) as u8;
                let inst = &mut **objectvalue!(rb as *const TValue) as *mut LuauObject;
                if (slot as i32) < (*(*inst).lclass).numberofallmembers
                  && tsvalue!(kv as *const TValue)
                    == *(*(*inst).lclass).offsettomember.add(slot as usize)
                {
                  setobj_2_s!(l, ra, luaR_lookupmemberatoffset!(inst, slot as i32));
                  continue 'dispatch;
                } else {
                  // slow-er path: the slot mismatched so we fall back to
                  // looking up the offset from the string.
                  let offset = luaH_getstr(
                    (*(*inst).lclass).memberstooffset,
                    tsvalue!(kv as *const TValue) as *mut tstring,
                  );
                  if ttisnil!(offset) {
                    luaG_missingmembererror(l, rb as *const TValue, kv as *const TValue);
                  }
                  LUAU_ASSERT!(ttisnumber!(offset));
                  let offsetnum = nvalue!(offset) as i32;
                  setobj_2_s!(l, ra, luaR_lookupmemberatoffset!(inst, offsetnum));
                  vm_patch_c(pc.sub(2), offsetnum);
                  continue 'dispatch;
                }
              }

              // fall through to slow path
            }

            // slow-path, may invoke Lua calls via __index metamethod
            vm_protect!(l, pc, base, {
              lua_v_gettable(l, rb as *const TValue, kv, ra);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_SETTABLEKS => {
            // lvmexecute.cpp:665
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k) as *mut TValue;
            LUAU_ASSERT!(ttisstring!(kv as *const TValue));

            // fast-path: built-in table
            if ttistable!(rb as *const TValue) {
              let h = hvalue!(rb as *const TValue);

              let slot = (LUAU_INSN_C!(insn) as i32) & (*h).nodemask8 as i32;
              let n = (*h).node.add(slot as usize);

              // fast-path: value is in expected slot
              if ttisstring!(gkey!(n) as *const TValue)
                && tsvalue!(gkey!(n) as *const TValue) == tsvalue!(kv as *const TValue)
                && !ttisnil!(gval!(n))
                && (*h).readonly == 0
              {
                setobj2t!(l, gval!(n), ra as *const TValue);
                luaC_barriert!(l, h, ra as *const TValue);
                continue 'dispatch;
              } else if fastnotm((*h).metatable, TMS::TmNewIndex as i32) && (*h).readonly == 0 {
                (*(*l).ci).savedpc = pc; // vm_protect_pc(): set may fail

                let res = luaH_setstr(l, h, tsvalue!(kv as *const TValue) as *mut tstring);
                let cachedslot = gval2slot!(h, res as *const TValue);
                // save cachedslot to accelerate future lookups; patches
                // currently executing instruction since pc-2 rolls back two pc++
                vm_patch_c(pc.sub(2), cachedslot);
                setobj2t!(l, res, ra as *const TValue);
                luaC_barriert!(l, h, ra as *const TValue);
                continue 'dispatch;
              } else {
                // slow-path, may invoke Lua calls via __newindex metamethod
                (*l).cachedslot = slot;
                vm_protect!(l, pc, base, {
                  lua_v_settable(l, rb as *const TValue, kv, ra);
                });
                vm_patch_c(pc.sub(2), (*l).cachedslot);
                continue 'dispatch;
              }
            } else {
              // fast-path: user data with C __newindex TM
              let fn_tm = if ttisuserdata!(rb as *const TValue) {
                fasttm(
                  l,
                  uvalue!(rb as *const TValue).metatable,
                  TMS::TmNewIndex as i32,
                )
              } else {
                null()
              };
              if !fn_tm.is_null() && ttisfunction!(fn_tm) && (*clvalue!(fn_tm)).is_c != 0 {
                // note: it's safe to push arguments past top for
                // complicated reasons (see top of the file)
                LUAU_ASSERT!((*l).top.add(4) < (*l).stack.add((*l).stacksize as usize));
                let top = (*l).top;
                setobj_2_s!(l, top.add(0), fn_tm);
                setobj_2_s!(l, top.add(1), rb as *const TValue);
                setobj_2_s!(l, top.add(2), kv as *const TValue);
                setobj_2_s!(l, top.add(3), ra as *const TValue);
                (*l).top = top.add(4);

                (*l).cachedslot = LUAU_INSN_C!(insn) as i32;
                vm_protect!(l, pc, base, {
                  lua_v_call_tm(l, 3, -1);
                });
                vm_patch_c(pc.sub(2), (*l).cachedslot);
                continue 'dispatch;
              } else {
                // slow-path, may invoke Lua calls via __newindex metamethod
                vm_protect!(l, pc, base, {
                  lua_v_settable(l, rb as *const TValue, kv, ra);
                });
                continue 'dispatch;
              }
            }
          }
          LuauOpcode::LOP_GETTABLE => {
            // lvmexecute.cpp:741
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            // fast-path: array lookup
            if ttistable!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              let h = hvalue!(rb as *const TValue);

              let indexd = nvalue!(rc as *const TValue);
              let index = indexd as i32;

              // index has to be an exact integer and in-bounds for the array portion
              if ((index as u32).wrapping_sub(1)) < (*h).sizearray as u32
                && (*h).metatable.is_null()
                && index as f64 == indexd
              {
                setobj_2_s!(
                  l,
                  ra,
                  (*h).array.add((index - 1) as u32 as usize) as *const TValue
                );
                continue 'dispatch;
              }

              // fall through to slow path
            }

            // slow-path: handles out of bounds array lookups, non-integer
            // numeric keys, non-array table lookup, __index MT calls
            vm_protect!(l, pc, base, {
              lua_v_gettable(l, rb as *const TValue, rc, ra);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_SETTABLE => {
            // lvmexecute.cpp:771
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            // fast-path: array assign
            if ttistable!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              let h = hvalue!(rb as *const TValue);

              let indexd = nvalue!(rc as *const TValue);
              let index = indexd as i32;

              // index has to be an exact integer and in-bounds for the array portion
              if ((index as u32).wrapping_sub(1)) < (*h).sizearray as u32
                && (*h).metatable.is_null()
                && (*h).readonly == 0
                && index as f64 == indexd
              {
                setobj2t!(
                  l,
                  (*h).array.add((index - 1) as u32 as usize),
                  ra as *const TValue
                );
                luaC_barriert!(l, h, ra as *const TValue);
                continue 'dispatch;
              }

              // fall through to slow path
            }

            // slow-path: handles out of bounds array assignments, non-integer
            // numeric keys, non-array table access, __newindex MT calls
            vm_protect!(l, pc, base, {
              lua_v_settable(l, rb as *const TValue, rc, ra);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_GETTABLEN => {
            // lvmexecute.cpp:802
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let c = LUAU_INSN_C!(insn) as i32;

            // fast-path: array lookup
            if ttistable!(rb as *const TValue) {
              let h = hvalue!(rb as *const TValue);

              if (c as u32) < (*h).sizearray as u32 && (*h).metatable.is_null() {
                setobj_2_s!(l, ra, (*h).array.add(c as usize) as *const TValue);
                continue 'dispatch;
              }

              // fall through to slow path
            }

            // slow-path: handles out of bounds array lookups
            let mut n: TValue = zeroed();
            setnvalue!(&mut n as *mut TValue, (c + 1) as f64);
            vm_protect!(l, pc, base, {
              lua_v_gettable(l, rb as *const TValue, &mut n as *mut TValue, ra);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_SETTABLEN => {
            // lvmexecute.cpp:830
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let c = LUAU_INSN_C!(insn) as i32;

            // fast-path: array assign
            if ttistable!(rb as *const TValue) {
              let h = hvalue!(rb as *const TValue);

              if (c as u32) < (*h).sizearray as u32
                && (*h).metatable.is_null()
                && (*h).readonly == 0
              {
                setobj2t!(l, (*h).array.add(c as usize), ra as *const TValue);
                luaC_barriert!(l, h, ra as *const TValue);
                continue 'dispatch;
              }

              // fall through to slow path
            }

            // slow-path: handles out of bounds array lookups
            let mut n: TValue = zeroed();
            setnvalue!(&mut n as *mut TValue, (c + 1) as f64);
            vm_protect!(l, pc, base, {
              lua_v_settable(l, rb as *const TValue, &mut n as *mut TValue, ra);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_NEWCLOSURE => {
            // lvmexecute.cpp:859
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

            let (pv, sizep) = {
              let l = &(*cl).inner.l;
              (*(*l.p).p.add(LUAU_INSN_D!(insn) as usize), (*l.p).sizep)
            };
            LUAU_ASSERT!((LUAU_INSN_D!(insn) as u32) < sizep as u32);

            (*(*l).ci).savedpc = pc; // vm_protect_pc(): luaF_newLclosure may fail due to OOM

            // note: we save closure to stack early in case the code below
            // wants to capture it by value
            let ncl = lua_f_new_lclosure(l, (*pv).nups as i32, (*cl).env, pv);
            setclvalue!(l, ra, ncl);

            for ui in 0..(*pv).nups as usize {
              let uinsn = *pc;
              pc = pc.add(1);
              LUAU_ASSERT!(LUAU_INSN_OP!(uinsn) == LuauOpcode::LOP_CAPTURE as u32);

              let uref = {
                let l = &mut (*ncl).inner.l;
                l.uprefs.as_mut_ptr().add(ui)
              };
              match LUAU_INSN_A!(uinsn) {
                x if x == LuauCaptureType::LCT_VAL as u32 => {
                  setobj!(
                    l,
                    uref,
                    VM_REG!(LUAU_INSN_B!(uinsn), l, base) as *const TValue
                  );
                }
                x if x == LuauCaptureType::LCT_REF as u32 => {
                  setupvalue!(
                    l,
                    uref,
                    lua_f_findupval(l, VM_REG!(LUAU_INSN_B!(uinsn), l, base) as *mut TValue)
                  );
                }
                x if x == LuauCaptureType::LCT_UPVAL as u32 => {
                  setobj!(l, uref, VM_UV!(LUAU_INSN_B!(uinsn), cl) as *const TValue);
                }
                _ => {
                  // LUAU_ASSERT(!"Unknown upvalue capture type")
                  LUAU_ASSERT!(false);
                  unreachable!() // LUAU_UNREACHABLE()
                }
              }
            }

            vm_protect!(l, pc, base, {
              luaC_checkGC!(l);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_NAMECALL => {
            // lvmexecute.cpp:902
            let insn = *pc;
            pc = pc.add(1);
            let mut ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k) as *mut TValue;
            LUAU_ASSERT!(ttisstring!(kv as *const TValue));

            if ttistable!(rb as *const TValue) {
              let h = hvalue!(rb as *const TValue);
              // note: we can't use nodemask8 here because we need to query the
              // main position of the table, and 8-bit nodemask8 only works for
              // predictive lookups
              let n = (*h)
                .node
                .add(((*tsvalue!(kv as *const TValue)).hash & (sizenode!(h) - 1) as u32) as usize);

              // fast-path: key is in the table in expected slot
              if ttisstring!(gkey!(n) as *const TValue)
                && tsvalue!(gkey!(n) as *const TValue) == tsvalue!(kv as *const TValue)
                && !ttisnil!(gval!(n))
              {
                // note: order of copies allows rb to alias ra+1 or ra
                setobj_2_s!(l, ra.add(1), rb as *const TValue);
                setobj_2_s!(l, ra, gval!(n));
              } else {
                // fast-path: key is absent from the base, table has an
                // __index table, and it has the result in the expected slot
                let mut hit_mt_fast = false;
                if gnext!(n) == 0 {
                  let mt = fasttm(
                    l,
                    (*hvalue!(rb as *const TValue)).metatable,
                    TMS::TmIndex as i32,
                  );
                  if !mt.is_null() && ttistable!(mt) {
                    let mtn = (*hvalue!(mt)).node.add(
                      ((LUAU_INSN_C!(insn) as i32) & (*hvalue!(mt)).nodemask8 as i32) as usize,
                    );
                    if ttisstring!(gkey!(mtn) as *const TValue)
                      && tsvalue!(gkey!(mtn) as *const TValue) == tsvalue!(kv as *const TValue)
                      && !ttisnil!(gval!(mtn))
                    {
                      // note: order of copies allows rb to alias ra+1 or ra
                      setobj_2_s!(l, ra.add(1), rb as *const TValue);
                      setobj_2_s!(l, ra, gval!(mtn));
                      hit_mt_fast = true;
                    }
                  }
                }
                if !hit_mt_fast {
                  // slow-path: handles full table lookup
                  setobj_2_s!(l, ra.add(1), rb as *const TValue);
                  (*l).cachedslot = LUAU_INSN_C!(insn) as i32;
                  vm_protect!(l, pc, base, {
                    lua_v_gettable(l, rb as *const TValue, kv, ra);
                  });
                  vm_patch_c(pc.sub(2), (*l).cachedslot);
                  // recompute ra since stack might have been reallocated
                  ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
                  if ttisnil!(ra as *const TValue) {
                    luaG_methoderror(l, ra.add(1) as *const TValue, tsvalue!(kv as *const TValue));
                  }
                }
              }
            } else {
              let mt = if ttisuserdata!(rb as *const TValue) {
                uvalue!(rb as *const TValue).metatable
              } else {
                (*(*l).global).mt[ttype!(rb as *const TValue) as usize]
              };

              // fast-path: metatable with __namecall
              let fn_nc = fasttm(l, mt, TMS::TmNameCall as i32);
              if !fn_nc.is_null() {
                // note: order of copies allows rb to alias ra+1 or ra
                setobj_2_s!(l, ra.add(1), rb as *const TValue);
                setobj_2_s!(l, ra, fn_nc);

                (*l).namecall = tsvalue!(kv as *const TValue) as *mut tstring;
              } else {
                let tmi = fasttm(l, mt, TMS::TmIndex as i32);
                if !tmi.is_null() && ttistable!(tmi) {
                  let h = hvalue!(tmi);
                  let slot = (LUAU_INSN_C!(insn) as i32) & (*h).nodemask8 as i32;
                  let n = (*h).node.add(slot as usize);

                  // fast-path: metatable with __index that has method in expected slot
                  if ttisstring!(gkey!(n) as *const TValue)
                    && tsvalue!(gkey!(n) as *const TValue) == tsvalue!(kv as *const TValue)
                    && !ttisnil!(gval!(n))
                  {
                    // note: order of copies allows rb to alias ra+1 or ra
                    setobj_2_s!(l, ra.add(1), rb as *const TValue);
                    setobj_2_s!(l, ra, gval!(n));
                  } else {
                    // slow-path: handles slot mismatch
                    setobj_2_s!(l, ra.add(1), rb as *const TValue);
                    (*l).cachedslot = slot;
                    vm_protect!(l, pc, base, {
                      lua_v_gettable(l, rb as *const TValue, kv, ra);
                    });
                    vm_patch_c(pc.sub(2), (*l).cachedslot);
                    // recompute ra since stack might have been reallocated
                    ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
                    if ttisnil!(ra as *const TValue) {
                      luaG_methoderror(
                        l,
                        ra.add(1) as *const TValue,
                        tsvalue!(kv as *const TValue),
                      );
                    }
                  }
                } else if FFlag::DebugLuauUserDefinedClassesRuntime.get()
                  && ttisobject!(rb as *const TValue)
                {
                  let slot = LUAU_INSN_C!(insn) as i32;
                  let inst = &mut **objectvalue!(rb as *const TValue) as *mut LuauObject;
                  if slot < (*(*inst).lclass).numberofallmembers
                    && tsvalue!(kv as *const TValue)
                      == *(*(*inst).lclass).offsettomember.add(slot as usize)
                  {
                    // note: order of copies allows rb to alias ra+1 or ra
                    setobj_2_s!(l, ra.add(1), rb as *const TValue);
                    setobj_2_s!(l, ra, luaR_lookupmemberatoffset!(inst, slot));
                  } else {
                    // slow-er path: try to fetch the field manually.
                    let offset = luaH_getstr(
                      (*(*inst).lclass).memberstooffset,
                      tsvalue!(kv as *const TValue) as *mut tstring,
                    );
                    if ttisnil!(offset) {
                      luaG_missingmembererror(l, rb as *const TValue, kv as *const TValue);
                    }
                    LUAU_ASSERT!(ttisnumber!(offset));
                    let offsetnum = nvalue!(offset) as i32;
                    setobj_2_s!(l, ra.add(1), rb as *const TValue);
                    setobj_2_s!(l, ra, luaR_lookupmemberatoffset!(inst, offsetnum));
                    vm_patch_c(pc.sub(2), offsetnum);
                  }
                } else {
                  // slow-path: handles non-table __index
                  setobj_2_s!(l, ra.add(1), rb as *const TValue);
                  vm_protect!(l, pc, base, {
                    lua_v_gettable(l, rb as *const TValue, kv, ra);
                  });
                  // recompute ra since stack might have been reallocated
                  ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
                  if ttisnil!(ra as *const TValue) {
                    luaG_methoderror(l, ra.add(1) as *const TValue, tsvalue!(kv as *const TValue));
                  }
                }
              }
            }

            if FFlag::LuauCallFeedback.get() {
              continue 'dispatch;
            } else {
              // intentional fallthrough to CALL (C++ case fallthrough; pc
              // points at the CALL instruction, so forcing the dispatch op
              // is semantically identical)
              LUAU_ASSERT!(LUAU_INSN_OP!(*pc) == LuauOpcode::LOP_CALL as u32);
              continue_op = Some(LuauOpcode::LOP_CALL as u8);
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_CALL => {
            // lvmexecute.cpp:1038
            VM_INTERRUPT!(l, pc, base);
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

            let nparams = LUAU_INSN_B!(insn) as i32 - 1;
            let nresults = LUAU_INSN_C!(insn) as i32 - 1;

            let mut argtop = (*l).top;
            argtop = if nparams == LUA_MULTRET {
              argtop
            } else {
              ra.add(1 + nparams as usize)
            };

            if !ttisfunction!(ra as *const TValue) {
              // slow-path: not a function call
              (*(*l).ci).savedpc = pc; // vm_protect_pc(): luaV_tryfuncTM may fail

              lua_v_tryfunc_tm(l, ra);
              argtop = argtop.add(1); // __call adds an extra self
            }

            let ccl = clvalue!(ra as *const TValue);
            (*(*l).ci).savedpc = pc;

            incr_ci!(l);
            let ci = (*l).ci;
            (*ci).func = ra;
            (*ci).base = ra.add(1);
            // note: technically UB since we haven't reallocated the stack yet
            (*ci).top = argtop.add((*ccl).stacksize as usize);
            (*ci).savedpc = null();
            (*ci).flags = 0;
            (*ci).nresults = nresults;

            (*l).base = (*ci).base;
            (*l).top = argtop;

            if FFlag::LuauClosureUsageCounter.get() {
              (*ccl).usage += 1;
            }

            // note: this reallocs stack, but we don't need to VM_PROTECT this
            // this is because we're going to modify base/savedpc manually anyhow
            // crucially, we can't use ra/argtop after this line
            luaD_checkstackfornewci(l, (*ccl).stacksize as i32);

            LUAU_ASSERT!((*ci).top <= (*l).stack_last);

            if (*ccl).is_c == 0 {
              let p = {
                let l = &(*ccl).inner.l;
                l.p
              };

              // fill unused parameters with nil
              let mut argi = (*l).top;
              let argend = (*l).base.add((*p).numparams as usize);
              while argi < argend {
                setnilvalue!(argi); // complete missing arguments
                argi = argi.add(1);
              }
              (*l).top = if (*p).is_vararg != 0 { argi } else { (*ci).top };

              // reentry
              // codeentry may point to NATIVECALL instruction when proto is
              // compiled to native code; execution continues in native code.
              // note that p->codeentry may point *outside* of
              // p->code..p->code+p->sizecode, but that pointer never gets
              // saved to savedpc.
              pc = if SINGLE_STEP {
                (*p).code
              } else {
                (*p).codeentry
              };
              cl = ccl;
              base = (*l).base;
              k = (*p).k;
              continue 'dispatch;
            } else {
              let func = {
                let c = &(*ccl).inner.c;
                c.f
              };
              let n = match func {
                Some(f) => f(l),
                None => 0,
              };

              // yield
              if n < 0 {
                return; // goto exit
              }

              // ci is our callinfo, cip is our parent
              let ci = (*l).ci;
              let cip = ci.sub(1);

              if FFlag::LuauClosureUsageCounter.get() {
                LUAU_ASSERT!((*ccl).usage > 0);
                (*ccl).usage -= 1;
              }

              // copy return values into parent stack (but only up to
              // nresults!), fill the rest with nil
              // note: in MULTRET context nresults starts as -1 so i != 0
              // condition never activates intentionally
              let mut res = (*ci).func;
              let mut vali = (*l).top.sub(n as usize);
              let valend = (*l).top;

              let mut i = nresults;
              while i != 0 && vali < valend {
                setobj_2_s!(l, res, vali as *const TValue);
                res = res.add(1);
                vali = vali.add(1);
                i -= 1;
              }
              while i > 0 {
                setnilvalue!(res);
                res = res.add(1);
                i -= 1;
              }

              // pop the stack frame
              (*l).ci = cip;
              (*l).base = (*cip).base;
              (*l).top = if nresults == LUA_MULTRET {
                res
              } else {
                (*cip).top
              };

              // stack may have been reallocated, so we need to refresh base ptr
              base = (*l).base;
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_CALLFB => {
            // lvmexecute.cpp:1145
            VM_INTERRUPT!(l, pc, base);
            let insn = *pc;
            pc = pc.add(1);
            let feedback_slot: Instruction = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

            let nparams = LUAU_INSN_B!(insn) as i32 - 1;
            let nresults = LUAU_INSN_C!(insn) as i32 - 1;

            let mut argtop = (*l).top;
            argtop = if nparams == LUA_MULTRET {
              argtop
            } else {
              ra.add(1 + nparams as usize)
            };

            // slow-path: not a function call
            if !ttisfunction!(ra as *const TValue) {
              if feedback_slot != LUAU_INSN_FBSLOT_SEALED {
                vm_patch_aux(pc.sub(1), LUAU_INSN_FBSLOT_SEALED as i32);
              }

              (*(*l).ci).savedpc = pc; // vm_protect_pc(): luaV_tryfuncTM may fail

              lua_v_tryfunc_tm(l, ra);
              argtop = argtop.add(1); // __call adds an extra self
            }

            let ccl = clvalue!(ra as *const TValue);
            (*(*l).ci).savedpc = pc;

            incr_ci!(l);
            let ci = (*l).ci;
            (*ci).func = ra;
            (*ci).base = ra.add(1);
            // note: technically UB since we haven't reallocated the stack yet
            (*ci).top = argtop.add((*ccl).stacksize as usize);
            (*ci).savedpc = null();
            (*ci).flags = 0;
            (*ci).nresults = nresults;

            (*l).base = (*ci).base;
            (*l).top = argtop;

            if FFlag::LuauClosureUsageCounter.get() {
              (*ccl).usage += 1;
            }

            // note: this reallocs stack, but we don't need to VM_PROTECT this
            // this is because we're going to modify base/savedpc manually anyhow
            // crucially, we can't use ra/argtop after this line
            luaD_checkstackfornewci(l, (*ccl).stacksize as i32);

            LUAU_ASSERT!((*ci).top <= (*l).stack_last);

            if (*ccl).is_c == 0 {
              let p = {
                let l = &(*ccl).inner.l;
                l.p
              };

              if feedback_slot != LUAU_INSN_FBSLOT_SEALED
                && !luaF_recordhit(l, cl, ccl, feedback_slot)
              {
                vm_patch_aux(pc.sub(1), LUAU_INSN_FBSLOT_SEALED as i32);
              }

              // fill unused parameters with nil
              let mut argi = (*l).top;
              let argend = (*l).base.add((*p).numparams as usize);
              while argi < argend {
                setnilvalue!(argi); // complete missing arguments
                argi = argi.add(1);
              }
              (*l).top = if (*p).is_vararg != 0 { argi } else { (*ci).top };

              // reentry (see LOP_CALL for the codeentry note)
              pc = if SINGLE_STEP {
                (*p).code
              } else {
                (*p).codeentry
              };
              cl = ccl;
              base = (*l).base;
              k = (*p).k;
              continue 'dispatch;
            } else {
              if feedback_slot != LUAU_INSN_FBSLOT_SEALED {
                vm_patch_aux(pc.sub(1), LUAU_INSN_FBSLOT_SEALED as i32);
              }

              let func = {
                let c = &(*ccl).inner.c;
                c.f
              };
              let n = match func {
                Some(f) => f(l),
                None => 0,
              };

              // yield
              if n < 0 {
                return; // goto exit
              }

              // ci is our callinfo, cip is our parent
              let ci = (*l).ci;
              let cip = ci.sub(1);

              if FFlag::LuauClosureUsageCounter.get() {
                LUAU_ASSERT!((*ccl).usage > 0);
                (*ccl).usage -= 1;
              }

              // copy return values into parent stack (but only up to
              // nresults!), fill the rest with nil
              let mut res = (*ci).func;
              let mut vali = (*l).top.sub(n as usize);
              let valend = (*l).top;

              let mut i = nresults;
              while i != 0 && vali < valend {
                setobj_2_s!(l, res, vali as *const TValue);
                res = res.add(1);
                vali = vali.add(1);
                i -= 1;
              }
              while i > 0 {
                setnilvalue!(res);
                res = res.add(1);
                i -= 1;
              }

              // pop the stack frame
              (*l).ci = cip;
              (*l).base = (*cip).base;
              (*l).top = if nresults == LUA_MULTRET {
                res
              } else {
                (*cip).top
              };

              // stack may have been reallocated, so we need to refresh base ptr
              base = (*l).base;
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_RETURN => {
            // lvmexecute.cpp:1265
            VM_INTERRUPT!(l, pc, base);
            let insn = *pc;
            // note: this can point to l->top if b == LUA_MULTRET making VM_REG unsafe to use
            let ra: StkId = base.add(LUAU_INSN_A!(insn) as usize);
            let b = LUAU_INSN_B!(insn) as i32 - 1;

            // ci is our callinfo, cip is our parent
            let ci = (*l).ci;
            let cip = ci.sub(1);

            if FFlag::LuauClosureUsageCounter.get() {
              let cicl = clvalue!((*ci).func);
              LUAU_ASSERT!((*cicl).usage > 0);
              (*cicl).usage -= 1;
            }

            // note: we assume CALL always puts func+args and expects results
            // to start at func
            let mut res = (*ci).func;

            let mut vali = ra;
            // copy as much as possible for MULTRET calls, and only as much as
            // needed otherwise
            let valend = if b == LUA_MULTRET {
              (*l).top
            } else {
              ra.add(b as usize)
            };

            let nresults = (*ci).nresults;

            // copy return values into parent stack (but only up to nresults!),
            // fill the rest with nil
            // note: in MULTRET context nresults starts as -1 so i != 0
            // condition never activates intentionally
            let mut i = nresults;
            while i != 0 && vali < valend {
              setobj_2_s!(l, res, vali as *const TValue);
              res = res.add(1);
              vali = vali.add(1);
              i -= 1;
            }
            while i > 0 {
              setnilvalue!(res);
              res = res.add(1);
              i -= 1;
            }

            // pop the stack frame
            (*l).ci = cip;
            (*l).base = (*cip).base;
            (*l).top = if nresults == LUA_MULTRET {
              res
            } else {
              (*cip).top
            };

            // we're done!
            if ((*ci).flags as i32 & LUA_CALLINFO_RETURN) != 0 {
              return; // goto exit
            }

            LUAU_ASSERT!(isLua!((*l).ci));

            let nextcl = clvalue!((*cip).func);
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

            pc = pc.offset(LUAU_INSN_D!(insn) as isize);
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }

          LuauOpcode::LOP_JUMPIF => {
            // lvmexecute.cpp:1341
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);

            pc = pc.offset(if l_isfalse!(ra as *const TValue) {
              0
            } else {
              LUAU_INSN_D!(insn) as isize
            });
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }

          LuauOpcode::LOP_JUMPIFNOT => {
            // lvmexecute.cpp:1351
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);

            pc = pc.offset(if l_isfalse!(ra as *const TValue) {
              LUAU_INSN_D!(insn) as isize
            } else {
              0
            });
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_JUMPIFEQ => {
            // lvmexecute.cpp:1361
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(aux, l, base) as *mut TValue;

            macro_rules! jump_and_next {
              ($cond:expr) => {{
                pc = pc.offset(if $cond {
                  LUAU_INSN_D!(insn) as isize
                } else {
                  1
                });
                let p = {
                  let l = &(*cl).inner.l;
                  l.p
                };
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                continue 'dispatch;
              }};
            }

            // Note that all jumps below jump by 1 in the "false" case to skip over aux
            if ttype!(ra as *const TValue) == ttype!(rb as *const TValue) {
              let t = ttype!(ra as *const TValue);
              // C++ switch over ttype; TABLE/USERDATA/OBJECT `break` out of
              // the switch into the shared slow path below the match.
              if t == LuaType::Nil as i32 {
                jump_and_next!(true);
              } else if t == LuaType::Boolean as i32 {
                jump_and_next!(bvalue!(ra as *const TValue) == bvalue!(rb as *const TValue));
              } else if t == LuaType::LightUserData as i32 {
                jump_and_next!(
                  pvalue!(ra as *const TValue) == pvalue!(rb as *const TValue)
                    && lightuserdatatag!(ra as *const TValue)
                      == lightuserdatatag!(rb as *const TValue)
                );
              } else if t == LuaType::Number as i32 {
                jump_and_next!(nvalue!(ra as *const TValue) == nvalue!(rb as *const TValue));
              } else if t == LuaType::Vector as i32 {
                jump_and_next!(luai_veceq(
                  vvalue!(ra as *const TValue).as_ptr(),
                  vvalue!(rb as *const TValue).as_ptr()
                ));
              } else if t == LuaType::String as i32
                || t == LuaType::Function as i32
                || t == LuaType::Thread as i32
                || t == LuaType::Buffer as i32
              {
                jump_and_next!(gcvalue!(ra as *const TValue) == gcvalue!(rb as *const TValue));
              } else if t == LuaType::Table as i32 {
                // fast-path: same metatable, no EQ metamethod
                if (*hvalue!(ra as *const TValue)).metatable
                  == (*hvalue!(rb as *const TValue)).metatable
                {
                  let fn_tm = fasttm(
                    l,
                    (*hvalue!(ra as *const TValue)).metatable,
                    TMS::TmEq as i32,
                  );
                  if fn_tm.is_null() {
                    jump_and_next!(hvalue!(ra as *const TValue) == hvalue!(rb as *const TValue));
                  }
                }
                // slow path after switch()
              } else if t == LuaType::UserData as i32 {
                // fast-path: same metatable, no EQ metamethod or C metamethod
                if uvalue!(ra as *const TValue).metatable == uvalue!(rb as *const TValue).metatable
                {
                  let fn_tm = fasttm(l, uvalue!(ra as *const TValue).metatable, TMS::TmEq as i32);
                  if fn_tm.is_null() {
                    jump_and_next!(
                      uvalue!(ra as *const TValue) as *const _ as *const c_void
                        == uvalue!(rb as *const TValue) as *const _ as *const c_void
                    );
                  } else if ttisfunction!(fn_tm) && (*clvalue!(fn_tm)).is_c != 0 {
                    // note: it's safe to push arguments past top for
                    // complicated reasons (see top of the file)
                    LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                    let top = (*l).top;
                    setobj_2_s!(l, top.add(0), fn_tm);
                    setobj_2_s!(l, top.add(1), ra as *const TValue);
                    setobj_2_s!(l, top.add(2), rb as *const TValue);
                    let res = top.offset_from(base) as i32;
                    (*l).top = top.add(3);

                    vm_protect!(l, pc, base, {
                      lua_v_call_tm(l, 2, res);
                    });
                    jump_and_next!(!l_isfalse!(base.add(res as usize) as *const TValue));
                  }
                }
                // slow path after switch()
              } else if t == LuaType::Class as i32 {
                // Class objects are only ever physically equal, so check
                // for pointer equality.
                jump_and_next!(
                  classvalue!(ra as *const TValue) as *const _ as *const c_void
                    == classvalue!(rb as *const TValue) as *const _ as *const c_void
                );
              } else if t == LuaType::Object as i32 {
                // For now, hit the slow path after the switch (we may
                // need to invoke metamethods).
              } else if t == LuaType::Integer as i32 {
                jump_and_next!(lvalue!(ra as *const TValue) == lvalue!(rb as *const TValue));
              } else {
                // LUAU_ASSERT(!"Unknown value type")
                LUAU_ASSERT!(false);
                unreachable!() // LUAU_UNREACHABLE()
              }

              // slow-path: tables with metatables and userdata values
              // note that we don't have a fast path for userdata values
              // without metatables, since that's very rare
              let res: i32;
              vm_protect!(l, pc, base, {
                res = lua_v_equalval(l, ra as *const TValue, rb as *const TValue);
              });

              jump_and_next!(res == 1);
            } else {
              pc = pc.offset(1);
              let p = {
                let l = &(*cl).inner.l;
                l.p
              };
              LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_JUMPIFNOTEQ => {
            // lvmexecute.cpp:1494
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(aux, l, base) as *mut TValue;

            macro_rules! jump_and_next {
              ($cond:expr) => {{
                pc = pc.offset(if $cond {
                  LUAU_INSN_D!(insn) as isize
                } else {
                  1
                });
                let p = {
                  let l = &(*cl).inner.l;
                  l.p
                };
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                continue 'dispatch;
              }};
            }

            // Note that all jumps below jump by 1 in the "true" case to skip over aux
            if ttype!(ra as *const TValue) == ttype!(rb as *const TValue) {
              let t = ttype!(ra as *const TValue);
              // C++ switch over ttype; TABLE/USERDATA/OBJECT `break` out of
              // the switch into the shared slow path below.
              if t == LuaType::Nil as i32 {
                jump_and_next!(false); // pc += 1
              } else if t == LuaType::Boolean as i32 {
                jump_and_next!(bvalue!(ra as *const TValue) != bvalue!(rb as *const TValue));
              } else if t == LuaType::LightUserData as i32 {
                jump_and_next!(
                  pvalue!(ra as *const TValue) != pvalue!(rb as *const TValue)
                    || lightuserdatatag!(ra as *const TValue)
                      != lightuserdatatag!(rb as *const TValue)
                );
              } else if t == LuaType::Number as i32 {
                jump_and_next!(nvalue!(ra as *const TValue) != nvalue!(rb as *const TValue));
              } else if t == LuaType::Vector as i32 {
                jump_and_next!(!luai_veceq(
                  vvalue!(ra as *const TValue).as_ptr(),
                  vvalue!(rb as *const TValue).as_ptr()
                ));
              } else if t == LuaType::String as i32
                || t == LuaType::Function as i32
                || t == LuaType::Thread as i32
                || t == LuaType::Buffer as i32
              {
                jump_and_next!(gcvalue!(ra as *const TValue) != gcvalue!(rb as *const TValue));
              } else if t == LuaType::Table as i32 {
                // fast-path: same metatable, no EQ metamethod
                if (*hvalue!(ra as *const TValue)).metatable
                  == (*hvalue!(rb as *const TValue)).metatable
                {
                  let fn_tm = fasttm(
                    l,
                    (*hvalue!(ra as *const TValue)).metatable,
                    TMS::TmEq as i32,
                  );
                  if fn_tm.is_null() {
                    jump_and_next!(hvalue!(ra as *const TValue) != hvalue!(rb as *const TValue));
                  }
                }
                // slow path after switch()
              } else if t == LuaType::UserData as i32 {
                // fast-path: same metatable, no EQ metamethod or C metamethod
                if uvalue!(ra as *const TValue).metatable == uvalue!(rb as *const TValue).metatable
                {
                  let fn_tm = fasttm(l, uvalue!(ra as *const TValue).metatable, TMS::TmEq as i32);
                  if fn_tm.is_null() {
                    jump_and_next!(
                      uvalue!(ra as *const TValue) as *const _ as *const c_void
                        != uvalue!(rb as *const TValue) as *const _ as *const c_void
                    );
                  } else if ttisfunction!(fn_tm) && (*clvalue!(fn_tm)).is_c != 0 {
                    // note: it's safe to push arguments past top for
                    // complicated reasons (see top of the file)
                    LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                    let top = (*l).top;
                    setobj_2_s!(l, top.add(0), fn_tm);
                    setobj_2_s!(l, top.add(1), ra as *const TValue);
                    setobj_2_s!(l, top.add(2), rb as *const TValue);
                    let res = top.offset_from(base) as i32;
                    (*l).top = top.add(3);

                    vm_protect!(l, pc, base, {
                      lua_v_call_tm(l, 2, res);
                    });
                    jump_and_next!(l_isfalse!(base.add(res as usize) as *const TValue));
                  }
                }
                // slow path after switch()
              } else if t == LuaType::Class as i32 {
                // Class objects are only ever physically equal, so check
                // for pointer inequality.
                jump_and_next!(
                  classvalue!(ra as *const TValue) as *const _ as *const c_void
                    != classvalue!(rb as *const TValue) as *const _ as *const c_void
                );
              } else if t == LuaType::Object as i32 {
                // For now, hit the slow path after the switch (we may
                // need to invoke metamethods).
              } else if t == LuaType::Integer as i32 {
                jump_and_next!(lvalue!(ra as *const TValue) != lvalue!(rb as *const TValue));
              } else {
                // LUAU_ASSERT(!"Unknown value type")
                LUAU_ASSERT!(false);
                unreachable!() // LUAU_UNREACHABLE()
              }

              // slow-path: tables with metatables and userdata values
              // note that we don't have a fast path for userdata values
              // without metatables, since that's very rare
              let res: i32;
              vm_protect!(l, pc, base, {
                res = lua_v_equalval(l, ra as *const TValue, rb as *const TValue);
              });

              jump_and_next!(res == 0);
            } else {
              pc = pc.offset(LUAU_INSN_D!(insn) as isize);
              let p = {
                let l = &(*cl).inner.l;
                l.p
              };
              LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_JUMPIFLE => {
            // lvmexecute.cpp:1627
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(aux, l, base) as *mut TValue;

            macro_rules! jump_and_next {
              ($cond:expr) => {{
                pc = pc.offset(if $cond {
                  LUAU_INSN_D!(insn) as isize
                } else {
                  1
                });
                let p = {
                  let l = &(*cl).inner.l;
                  l.p
                };
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                continue 'dispatch;
              }};
            }

            // fast-path: number
            // Note that all jumps below jump by 1 in the "false" case to skip over aux
            if ttisnumber!(ra as *const TValue) && ttisnumber!(rb as *const TValue) {
              jump_and_next!(nvalue!(ra as *const TValue) <= nvalue!(rb as *const TValue));
            }
            // fast-path: string
            else if ttisstring!(ra as *const TValue) && ttisstring!(rb as *const TValue) {
              jump_and_next!(
                lua_v_strcmp(tsvalue!(ra as *const TValue), tsvalue!(rb as *const TValue)) <= 0
              );
            } else {
              let res: i32;
              vm_protect!(l, pc, base, {
                res = lua_v_lessequal(l, ra as *const TValue, rb as *const TValue);
              });

              jump_and_next!(res == 1);
            }
          }
          LuauOpcode::LOP_JUMPIFNOTLE => {
            // lvmexecute.cpp:1660
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(aux, l, base) as *mut TValue;

            macro_rules! jump_and_next {
              ($cond:expr) => {{
                pc = pc.offset(if $cond {
                  LUAU_INSN_D!(insn) as isize
                } else {
                  1
                });
                let p = {
                  let l = &(*cl).inner.l;
                  l.p
                };
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                continue 'dispatch;
              }};
            }

            // fast-path: number
            // Note that all jumps below jump by 1 in the "true" case to skip over aux
            if ttisnumber!(ra as *const TValue) && ttisnumber!(rb as *const TValue) {
              jump_and_next!(!matches!(
                nvalue!(ra as *const TValue).partial_cmp(&nvalue!(rb as *const TValue)),
                Some(Less | Equal)
              ));
            }
            // fast-path: string
            else if ttisstring!(ra as *const TValue) && ttisstring!(rb as *const TValue) {
              jump_and_next!(
                !(lua_v_strcmp(tsvalue!(ra as *const TValue), tsvalue!(rb as *const TValue)) <= 0)
              );
            } else {
              let res: i32;
              vm_protect!(l, pc, base, {
                res = lua_v_lessequal(l, ra as *const TValue, rb as *const TValue);
              });

              jump_and_next!(res == 0);
            }
          }
          LuauOpcode::LOP_JUMPIFLT => {
            // lvmexecute.cpp:1693
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(aux, l, base) as *mut TValue;

            macro_rules! jump_and_next {
              ($cond:expr) => {{
                pc = pc.offset(if $cond {
                  LUAU_INSN_D!(insn) as isize
                } else {
                  1
                });
                let p = {
                  let l = &(*cl).inner.l;
                  l.p
                };
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                continue 'dispatch;
              }};
            }

            // fast-path: number
            // Note that all jumps below jump by 1 in the "false" case to skip over aux
            if ttisnumber!(ra as *const TValue) && ttisnumber!(rb as *const TValue) {
              jump_and_next!(nvalue!(ra as *const TValue) < nvalue!(rb as *const TValue));
            }
            // fast-path: string
            else if ttisstring!(ra as *const TValue) && ttisstring!(rb as *const TValue) {
              jump_and_next!(
                lua_v_strcmp(tsvalue!(ra as *const TValue), tsvalue!(rb as *const TValue)) < 0
              );
            } else {
              let res: i32;
              vm_protect!(l, pc, base, {
                res = lua_v_lessthan(l, ra as *const TValue, rb as *const TValue);
              });

              jump_and_next!(res == 1);
            }
          }
          LuauOpcode::LOP_JUMPIFNOTLT => {
            // lvmexecute.cpp:1726
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(aux, l, base) as *mut TValue;

            macro_rules! jump_and_next {
              ($cond:expr) => {{
                pc = pc.offset(if $cond {
                  LUAU_INSN_D!(insn) as isize
                } else {
                  1
                });
                let p = {
                  let l = &(*cl).inner.l;
                  l.p
                };
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                continue 'dispatch;
              }};
            }

            // fast-path: number
            // Note that all jumps below jump by 1 in the "true" case to skip over aux
            if ttisnumber!(ra as *const TValue) && ttisnumber!(rb as *const TValue) {
              jump_and_next!(!matches!(
                nvalue!(ra as *const TValue).partial_cmp(&nvalue!(rb as *const TValue)),
                Some(Less)
              ));
            }
            // fast-path: string
            else if ttisstring!(ra as *const TValue) && ttisstring!(rb as *const TValue) {
              jump_and_next!(
                !(lua_v_strcmp(tsvalue!(ra as *const TValue), tsvalue!(rb as *const TValue)) < 0)
              );
            } else {
              let res: i32;
              vm_protect!(l, pc, base, {
                res = lua_v_lessthan(l, ra as *const TValue, rb as *const TValue);
              });

              jump_and_next!(res == 0);
            }
          }
          LuauOpcode::LOP_ADD => {
            // lvmexecute.cpp:1759
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              setnvalue!(
                ra,
                nvalue!(rb as *const TValue) + nvalue!(rc as *const TValue)
              );
              continue 'dispatch;
            } else if ttisvector!(rb as *const TValue) && ttisvector!(rc as *const TValue) {
              let vb = vvalue!(rb as *const TValue).as_ptr();
              let vc = vvalue!(rc as *const TValue).as_ptr();
              setvvalue!(
                ra,
                *vb.add(0) + *vc.add(0),
                *vb.add(1) + *vc.add(1),
                *vb.add(2) + *vc.add(2),
                *vb.add(3) + *vc.add(3)
              );
              continue 'dispatch;
            } else {
              // fast-path for userdata with C functions
              let fn_tm: *const TValue;
              if ttisuserdata!(rb as *const TValue)
                && {
                  fn_tm = lua_t_gettmbyobj(l, rb as *const TValue, TMS::TmAdd);
                  !fn_tm.is_null()
                }
                && ttisfunction!(fn_tm)
                && (*clvalue!(fn_tm)).is_c != 0
              {
                // note: it's safe to push arguments past top for
                // complicated reasons (see top of the file)
                LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                let top = (*l).top;
                setobj_2_s!(l, top.add(0), fn_tm);
                setobj_2_s!(l, top.add(1), rb as *const TValue);
                setobj_2_s!(l, top.add(2), rc as *const TValue);
                (*l).top = top.add(3);

                vm_protect!(l, pc, base, {
                  lua_v_call_tm(l, 2, LUAU_INSN_A!(insn) as i32);
                });
                continue 'dispatch;
              } else {
                // slow-path, may invoke C/Lua via metamethods
                vm_protect!(l, pc, base, {
                  lua_v_doarithimpl(l, ra, rb as *const TValue, rc as *const TValue, TMS::TmAdd);
                });
                continue 'dispatch;
              }
            }
          }

          LuauOpcode::LOP_SUB => {
            // lvmexecute.cpp:1805
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              setnvalue!(
                ra,
                nvalue!(rb as *const TValue) - nvalue!(rc as *const TValue)
              );
              continue 'dispatch;
            } else if ttisvector!(rb as *const TValue) && ttisvector!(rc as *const TValue) {
              let vb = vvalue!(rb as *const TValue).as_ptr();
              let vc = vvalue!(rc as *const TValue).as_ptr();
              setvvalue!(
                ra,
                *vb.add(0) - *vc.add(0),
                *vb.add(1) - *vc.add(1),
                *vb.add(2) - *vc.add(2),
                *vb.add(3) - *vc.add(3)
              );
              continue 'dispatch;
            } else {
              // fast-path for userdata with C functions
              let fn_tm: *const TValue;
              if ttisuserdata!(rb as *const TValue)
                && {
                  fn_tm = lua_t_gettmbyobj(l, rb as *const TValue, TMS::TmSub);
                  !fn_tm.is_null()
                }
                && ttisfunction!(fn_tm)
                && (*clvalue!(fn_tm)).is_c != 0
              {
                // note: it's safe to push arguments past top for
                // complicated reasons (see top of the file)
                LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                let top = (*l).top;
                setobj_2_s!(l, top.add(0), fn_tm);
                setobj_2_s!(l, top.add(1), rb as *const TValue);
                setobj_2_s!(l, top.add(2), rc as *const TValue);
                (*l).top = top.add(3);

                vm_protect!(l, pc, base, {
                  lua_v_call_tm(l, 2, LUAU_INSN_A!(insn) as i32);
                });
                continue 'dispatch;
              } else {
                // slow-path, may invoke C/Lua via metamethods
                vm_protect!(l, pc, base, {
                  lua_v_doarithimpl(l, ra, rb as *const TValue, rc as *const TValue, TMS::TmSub);
                });
                continue 'dispatch;
              }
            }
          }
          LuauOpcode::LOP_MUL => {
            // lvmexecute.cpp:1851
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              setnvalue!(
                ra,
                nvalue!(rb as *const TValue) * nvalue!(rc as *const TValue)
              );
              continue 'dispatch;
            } else if ttisvector!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              let vb = vvalue!(rb as *const TValue).as_ptr();
              let vc = nvalue!(rc as *const TValue) as f32;
              setvvalue!(
                ra,
                *vb.add(0) * vc,
                *vb.add(1) * vc,
                *vb.add(2) * vc,
                *vb.add(3) * vc
              );
              continue 'dispatch;
            } else if ttisvector!(rb as *const TValue) && ttisvector!(rc as *const TValue) {
              let vb = vvalue!(rb as *const TValue).as_ptr();
              let vc = vvalue!(rc as *const TValue).as_ptr();
              setvvalue!(
                ra,
                *vb.add(0) * *vc.add(0),
                *vb.add(1) * *vc.add(1),
                *vb.add(2) * *vc.add(2),
                *vb.add(3) * *vc.add(3)
              );
              continue 'dispatch;
            } else if ttisnumber!(rb as *const TValue) && ttisvector!(rc as *const TValue) {
              let vb = nvalue!(rb as *const TValue) as f32;
              let vc = vvalue!(rc as *const TValue).as_ptr();
              setvvalue!(
                ra,
                vb * *vc.add(0),
                vb * *vc.add(1),
                vb * *vc.add(2),
                vb * *vc.add(3)
              );
              continue 'dispatch;
            } else {
              // fast-path for userdata with C functions
              let rbc = if ttisnumber!(rb as *const TValue) {
                rc
              } else {
                rb
              };
              let fn_tm: *const TValue;
              if ttisuserdata!(rbc as *const TValue)
                && {
                  fn_tm = lua_t_gettmbyobj(l, rbc as *const TValue, TMS::TmMul);
                  !fn_tm.is_null()
                }
                && ttisfunction!(fn_tm)
                && (*clvalue!(fn_tm)).is_c != 0
              {
                // note: it's safe to push arguments past top for
                // complicated reasons (see top of the file)
                LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                let top = (*l).top;
                setobj_2_s!(l, top.add(0), fn_tm);
                setobj_2_s!(l, top.add(1), rb as *const TValue);
                setobj_2_s!(l, top.add(2), rc as *const TValue);
                (*l).top = top.add(3);

                vm_protect!(l, pc, base, {
                  lua_v_call_tm(l, 2, LUAU_INSN_A!(insn) as i32);
                });
                continue 'dispatch;
              } else {
                // slow-path, may invoke C/Lua via metamethods
                vm_protect!(l, pc, base, {
                  lua_v_doarithimpl(l, ra, rb as *const TValue, rc as *const TValue, TMS::TmMul);
                });
                continue 'dispatch;
              }
            }
          }
          LuauOpcode::LOP_DIV => {
            // lvmexecute.cpp:1912
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              setnvalue!(
                ra,
                nvalue!(rb as *const TValue) / nvalue!(rc as *const TValue)
              );
              continue 'dispatch;
            } else if ttisvector!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              let vb = vvalue!(rb as *const TValue).as_ptr();
              let vc = nvalue!(rc as *const TValue) as f32;
              setvvalue!(
                ra,
                *vb.add(0) / vc,
                *vb.add(1) / vc,
                *vb.add(2) / vc,
                *vb.add(3) / vc
              );
              continue 'dispatch;
            } else if ttisvector!(rb as *const TValue) && ttisvector!(rc as *const TValue) {
              let vb = vvalue!(rb as *const TValue).as_ptr();
              let vc = vvalue!(rc as *const TValue).as_ptr();
              setvvalue!(
                ra,
                *vb.add(0) / *vc.add(0),
                *vb.add(1) / *vc.add(1),
                *vb.add(2) / *vc.add(2),
                *vb.add(3) / *vc.add(3)
              );
              continue 'dispatch;
            } else if ttisnumber!(rb as *const TValue) && ttisvector!(rc as *const TValue) {
              let vb = nvalue!(rb as *const TValue) as f32;
              let vc = vvalue!(rc as *const TValue).as_ptr();
              setvvalue!(
                ra,
                vb / *vc.add(0),
                vb / *vc.add(1),
                vb / *vc.add(2),
                vb / *vc.add(3)
              );
              continue 'dispatch;
            } else {
              // fast-path for userdata with C functions
              let rbc = if ttisnumber!(rb as *const TValue) {
                rc
              } else {
                rb
              };
              let fn_tm: *const TValue;
              if ttisuserdata!(rbc as *const TValue)
                && {
                  fn_tm = lua_t_gettmbyobj(l, rbc as *const TValue, TMS::TmDiv);
                  !fn_tm.is_null()
                }
                && ttisfunction!(fn_tm)
                && (*clvalue!(fn_tm)).is_c != 0
              {
                // note: it's safe to push arguments past top for
                // complicated reasons (see top of the file)
                LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                let top = (*l).top;
                setobj_2_s!(l, top.add(0), fn_tm);
                setobj_2_s!(l, top.add(1), rb as *const TValue);
                setobj_2_s!(l, top.add(2), rc as *const TValue);
                (*l).top = top.add(3);

                vm_protect!(l, pc, base, {
                  lua_v_call_tm(l, 2, LUAU_INSN_A!(insn) as i32);
                });
                continue 'dispatch;
              } else {
                // slow-path, may invoke C/Lua via metamethods
                vm_protect!(l, pc, base, {
                  lua_v_doarithimpl(l, ra, rb as *const TValue, rc as *const TValue, TMS::TmDiv);
                });
                continue 'dispatch;
              }
            }
          }
          LuauOpcode::LOP_IDIV => {
            // lvmexecute.cpp:1973
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              setnvalue!(
                ra,
                luai_numidiv(nvalue!(rb as *const TValue), nvalue!(rc as *const TValue))
              );
              continue 'dispatch;
            } else if ttisvector!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              let vb = vvalue!(rb as *const TValue).as_ptr();
              let vc = nvalue!(rc as *const TValue) as f32;
              setvvalue!(
                ra,
                luai_numidiv(*vb.add(0) as f64, vc as f64) as f32,
                luai_numidiv(*vb.add(1) as f64, vc as f64) as f32,
                luai_numidiv(*vb.add(2) as f64, vc as f64) as f32,
                luai_numidiv(*vb.add(3) as f64, vc as f64) as f32
              );
              continue 'dispatch;
            } else {
              // fast-path for userdata with C functions
              let rbc = if ttisnumber!(rb as *const TValue) {
                rc
              } else {
                rb
              };
              let fn_tm: *const TValue;
              if ttisuserdata!(rbc as *const TValue)
                && {
                  fn_tm = lua_t_gettmbyobj(l, rbc as *const TValue, TMS::TmIDiv);
                  !fn_tm.is_null()
                }
                && ttisfunction!(fn_tm)
                && (*clvalue!(fn_tm)).is_c != 0
              {
                // note: it's safe to push arguments past top for
                // complicated reasons (see top of the file)
                LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                let top = (*l).top;
                setobj_2_s!(l, top.add(0), fn_tm);
                setobj_2_s!(l, top.add(1), rb as *const TValue);
                setobj_2_s!(l, top.add(2), rc as *const TValue);
                (*l).top = top.add(3);

                vm_protect!(l, pc, base, {
                  lua_v_call_tm(l, 2, LUAU_INSN_A!(insn) as i32);
                });
                continue 'dispatch;
              } else {
                // slow-path, may invoke C/Lua via metamethods
                vm_protect!(l, pc, base, {
                  lua_v_doarithimpl(l, ra, rb as *const TValue, rc as *const TValue, TMS::TmIDiv);
                });
                continue 'dispatch;
              }
            }
          }
          LuauOpcode::LOP_MOD => {
            // lvmexecute.cpp:2026
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              let nb = nvalue!(rb as *const TValue);
              let nc = nvalue!(rc as *const TValue);
              setnvalue!(ra, luai_nummod(nb, nc));
              continue 'dispatch;
            } else {
              // slow-path, may invoke C/Lua via metamethods
              vm_protect!(l, pc, base, {
                lua_v_doarithimpl(l, ra, rb as *const TValue, rc as *const TValue, TMS::TmMod);
              });
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_POW => {
            // lvmexecute.cpp:2049
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) && ttisnumber!(rc as *const TValue) {
              setnvalue!(
                ra,
                nvalue!(rb as *const TValue).powf(nvalue!(rc as *const TValue))
              );
              continue 'dispatch;
            } else {
              // slow-path, may invoke C/Lua via metamethods
              vm_protect!(l, pc, base, {
                lua_v_doarithimpl(l, ra, rb as *const TValue, rc as *const TValue, TMS::TmPow);
              });
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_ADDK => {
            // lvmexecute.cpp:2070
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_C!(insn), cl, k) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) {
              setnvalue!(
                ra,
                nvalue!(rb as *const TValue) + nvalue!(kv as *const TValue)
              );
              continue 'dispatch;
            } else {
              // slow-path, may invoke C/Lua via metamethods
              vm_protect!(l, pc, base, {
                lua_v_doarithimpl(l, ra, rb as *const TValue, kv as *const TValue, TMS::TmAdd);
              });
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_SUBK => {
            // lvmexecute.cpp:2091
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_C!(insn), cl, k) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) {
              setnvalue!(
                ra,
                nvalue!(rb as *const TValue) - nvalue!(kv as *const TValue)
              );
              continue 'dispatch;
            } else {
              // slow-path, may invoke C/Lua via metamethods
              vm_protect!(l, pc, base, {
                lua_v_doarithimpl(l, ra, rb as *const TValue, kv as *const TValue, TMS::TmSub);
              });
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_MULK => {
            // lvmexecute.cpp:2112
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_C!(insn), cl, k) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) {
              setnvalue!(
                ra,
                nvalue!(rb as *const TValue) * nvalue!(kv as *const TValue)
              );
              continue 'dispatch;
            } else if ttisvector!(rb as *const TValue) {
              let vb = vvalue!(rb as *const TValue).as_ptr();
              let vc = nvalue!(kv as *const TValue) as f32;
              setvvalue!(
                ra,
                *vb.add(0) * vc,
                *vb.add(1) * vc,
                *vb.add(2) * vc,
                *vb.add(3) * vc
              );
              continue 'dispatch;
            } else {
              // fast-path for userdata with C functions
              let fn_tm: *const TValue;
              if ttisuserdata!(rb as *const TValue)
                && {
                  fn_tm = lua_t_gettmbyobj(l, rb as *const TValue, TMS::TmMul);
                  !fn_tm.is_null()
                }
                && ttisfunction!(fn_tm)
                && (*clvalue!(fn_tm)).is_c != 0
              {
                // note: it's safe to push arguments past top for
                // complicated reasons (see top of the file)
                LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                let top = (*l).top;
                setobj_2_s!(l, top.add(0), fn_tm);
                setobj_2_s!(l, top.add(1), rb as *const TValue);
                setobj_2_s!(l, top.add(2), kv as *const TValue);
                (*l).top = top.add(3);

                vm_protect!(l, pc, base, {
                  lua_v_call_tm(l, 2, LUAU_INSN_A!(insn) as i32);
                });
                continue 'dispatch;
              } else {
                // slow-path, may invoke C/Lua via metamethods
                vm_protect!(l, pc, base, {
                  lua_v_doarithimpl(l, ra, rb as *const TValue, kv as *const TValue, TMS::TmMul);
                });
                continue 'dispatch;
              }
            }
          }
          LuauOpcode::LOP_DIVK => {
            // lvmexecute.cpp:2158
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_C!(insn), cl, k) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) {
              setnvalue!(
                ra,
                nvalue!(rb as *const TValue) / nvalue!(kv as *const TValue)
              );
              continue 'dispatch;
            } else if ttisvector!(rb as *const TValue) {
              let vb = vvalue!(rb as *const TValue).as_ptr();
              let vc = nvalue!(kv as *const TValue) as f32;
              setvvalue!(
                ra,
                *vb.add(0) / vc,
                *vb.add(1) / vc,
                *vb.add(2) / vc,
                *vb.add(3) / vc
              );
              continue 'dispatch;
            } else {
              // fast-path for userdata with C functions
              let fn_tm: *const TValue;
              if ttisuserdata!(rb as *const TValue)
                && {
                  fn_tm = lua_t_gettmbyobj(l, rb as *const TValue, TMS::TmDiv);
                  !fn_tm.is_null()
                }
                && ttisfunction!(fn_tm)
                && (*clvalue!(fn_tm)).is_c != 0
              {
                // note: it's safe to push arguments past top for
                // complicated reasons (see top of the file)
                LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                let top = (*l).top;
                setobj_2_s!(l, top.add(0), fn_tm);
                setobj_2_s!(l, top.add(1), rb as *const TValue);
                setobj_2_s!(l, top.add(2), kv as *const TValue);
                (*l).top = top.add(3);

                vm_protect!(l, pc, base, {
                  lua_v_call_tm(l, 2, LUAU_INSN_A!(insn) as i32);
                });
                continue 'dispatch;
              } else {
                // slow-path, may invoke C/Lua via metamethods
                vm_protect!(l, pc, base, {
                  lua_v_doarithimpl(l, ra, rb as *const TValue, kv as *const TValue, TMS::TmDiv);
                });
                continue 'dispatch;
              }
            }
          }
          LuauOpcode::LOP_IDIVK => {
            // lvmexecute.cpp:2204
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_C!(insn), cl, k) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) {
              setnvalue!(
                ra,
                luai_numidiv(nvalue!(rb as *const TValue), nvalue!(kv as *const TValue))
              );
              continue 'dispatch;
            } else if ttisvector!(rb as *const TValue) {
              let vb = vvalue!(rb as *const TValue).as_ptr();
              let vc = nvalue!(kv as *const TValue) as f32;
              setvvalue!(
                ra,
                luai_numidiv(*vb.add(0) as f64, vc as f64) as f32,
                luai_numidiv(*vb.add(1) as f64, vc as f64) as f32,
                luai_numidiv(*vb.add(2) as f64, vc as f64) as f32,
                luai_numidiv(*vb.add(3) as f64, vc as f64) as f32
              );
              continue 'dispatch;
            } else {
              // fast-path for userdata with C functions
              let fn_tm: *const TValue;
              if ttisuserdata!(rb as *const TValue)
                && {
                  fn_tm = lua_t_gettmbyobj(l, rb as *const TValue, TMS::TmIDiv);
                  !fn_tm.is_null()
                }
                && ttisfunction!(fn_tm)
                && (*clvalue!(fn_tm)).is_c != 0
              {
                // note: it's safe to push arguments past top for
                // complicated reasons (see top of the file)
                LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                let top = (*l).top;
                setobj_2_s!(l, top.add(0), fn_tm);
                setobj_2_s!(l, top.add(1), rb as *const TValue);
                setobj_2_s!(l, top.add(2), kv as *const TValue);
                (*l).top = top.add(3);

                vm_protect!(l, pc, base, {
                  lua_v_call_tm(l, 2, LUAU_INSN_A!(insn) as i32);
                });
                continue 'dispatch;
              } else {
                // slow-path, may invoke C/Lua via metamethods
                vm_protect!(l, pc, base, {
                  lua_v_doarithimpl(l, ra, rb as *const TValue, kv as *const TValue, TMS::TmIDiv);
                });
                continue 'dispatch;
              }
            }
          }
          LuauOpcode::LOP_MODK => {
            // lvmexecute.cpp:2256
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_C!(insn), cl, k) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) {
              setnvalue!(
                ra,
                luai_nummod(nvalue!(rb as *const TValue), nvalue!(kv as *const TValue))
              );
              continue 'dispatch;
            } else {
              // slow-path, may invoke C/Lua via metamethods
              vm_protect!(l, pc, base, {
                lua_v_doarithimpl(l, ra, rb as *const TValue, kv as *const TValue, TMS::TmMod);
              });
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_POWK => {
            // lvmexecute.cpp:2279
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_C!(insn), cl, k) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) {
              let nb = nvalue!(rb as *const TValue);
              let nk = nvalue!(kv as *const TValue);

              // pow is very slow so we specialize this for ^2, ^0.5 and ^3
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
              // slow-path, may invoke C/Lua via metamethods
              vm_protect!(l, pc, base, {
                lua_v_doarithimpl(l, ra, rb as *const TValue, kv as *const TValue, TMS::TmPow);
              });
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_AND => {
            // lvmexecute.cpp:2306
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            setobj_2_s!(
              l,
              ra,
              if l_isfalse!(rb as *const TValue) {
                rb
              } else {
                rc
              } as *const TValue
            );
            continue 'dispatch;
          }
          LuauOpcode::LOP_OR => {
            // lvmexecute.cpp:2317
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            setobj_2_s!(
              l,
              ra,
              if l_isfalse!(rb as *const TValue) {
                rc
              } else {
                rb
              } as *const TValue
            );
            continue 'dispatch;
          }
          LuauOpcode::LOP_ANDK => {
            // lvmexecute.cpp:2328
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_C!(insn), cl, k) as *mut TValue;

            setobj_2_s!(
              l,
              ra,
              if l_isfalse!(rb as *const TValue) {
                rb
              } else {
                kv
              } as *const TValue
            );
            continue 'dispatch;
          }
          LuauOpcode::LOP_ORK => {
            // lvmexecute.cpp:2339
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_C!(insn), cl, k) as *mut TValue;

            setobj_2_s!(
              l,
              ra,
              if l_isfalse!(rb as *const TValue) {
                kv
              } else {
                rb
              } as *const TValue
            );
            continue 'dispatch;
          }
          LuauOpcode::LOP_CONCAT => {
            // lvmexecute.cpp:2350
            let insn = *pc;
            pc = pc.add(1);
            let b = LUAU_INSN_B!(insn) as i32;
            let c = LUAU_INSN_C!(insn) as i32;

            // This call may realloc the stack! So we need to query args further down
            vm_protect!(l, pc, base, {
              lua_v_concat(l, c - b + 1, c);
            });

            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

            setobj_2_s!(l, ra, base.add(b as usize) as *const TValue);
            vm_protect!(l, pc, base, {
              luaC_checkGC!(l);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_NOT => {
            // lvmexecute.cpp:2377
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;

            let res = l_isfalse!(rb as *const TValue) as i32;
            setbvalue!(ra, res);
            continue 'dispatch;
          }
          LuauOpcode::LOP_MINUS => {
            // lvmexecute.cpp:2420
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;

            // fast-path
            if ttisnumber!(rb as *const TValue) {
              setnvalue!(ra, -nvalue!(rb as *const TValue));
              continue 'dispatch;
            } else if ttisvector!(rb as *const TValue) {
              let vb = vvalue!(rb as *const TValue).as_ptr();
              setvvalue!(ra, -*vb.add(0), -*vb.add(1), -*vb.add(2), -*vb.add(3));
              continue 'dispatch;
            } else {
              // fast-path for userdata with C functions
              let fn_tm: *const TValue;
              if ttisuserdata!(rb as *const TValue)
                && {
                  fn_tm = lua_t_gettmbyobj(l, rb as *const TValue, TMS::TmUnm);
                  !fn_tm.is_null()
                }
                && ttisfunction!(fn_tm)
                && (*clvalue!(fn_tm)).is_c != 0
              {
                // note: it's safe to push arguments past top for
                // complicated reasons (see top of the file)
                LUAU_ASSERT!((*l).top.add(2) < (*l).stack.add((*l).stacksize as usize));
                let top = (*l).top;
                setobj_2_s!(l, top.add(0), fn_tm);
                setobj_2_s!(l, top.add(1), rb as *const TValue);
                (*l).top = top.add(2);

                vm_protect!(l, pc, base, {
                  lua_v_call_tm(l, 1, LUAU_INSN_A!(insn) as i32);
                });
                continue 'dispatch;
              } else {
                // slow-path, may invoke C/Lua via metamethods
                vm_protect!(l, pc, base, {
                  lua_v_doarithimpl(l, ra, rb as *const TValue, rb as *const TValue, TMS::TmUnm);
                });
                continue 'dispatch;
              }
            }
          }
          LuauOpcode::LOP_LENGTH => {
            // lvmexecute.cpp:2420 (LOP_LENGTH)
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;

            // fast-path #1: tables
            if ttistable!(rb as *const TValue) {
              let h = hvalue!(rb as *const TValue);

              if fastnotm((*h).metatable, TMS::TmLen as i32) {
                setnvalue!(ra, lua_h_getn(h) as f64);
                continue 'dispatch;
              } else {
                // slow-path, may invoke C/Lua via metamethods
                vm_protect!(l, pc, base, {
                  lua_v_dolen(l, ra, rb as *const TValue);
                });
                continue 'dispatch;
              }
            }
            // fast-path #2: strings (not very important but easy to do)
            else if ttisstring!(rb as *const TValue) {
              let ts = tsvalue!(rb as *const TValue);
              setnvalue!(ra, (*ts).len as f64);
              continue 'dispatch;
            } else {
              // slow-path, may invoke C/Lua via metamethods
              vm_protect!(l, pc, base, {
                lua_v_dolen(l, ra, rb as *const TValue);
              });
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_NEWTABLE => {
            // lvmexecute.cpp:2458
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let b = LUAU_INSN_B!(insn) as i32;
            let aux: u32 = *pc;
            pc = pc.add(1);

            (*(*l).ci).savedpc = pc; // vm_protect_pc(): luaH_new may fail due to OOM

            sethvalue!(
              l,
              ra,
              lua_h_new(l, aux as i32, if b == 0 { 0 } else { 1 << (b - 1) })
            );
            vm_protect!(l, pc, base, {
              luaC_checkGC!(l);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_DUPTABLE => {
            // lvmexecute.cpp:2472
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_D!(insn), cl, k) as *mut TValue;

            (*(*l).ci).savedpc = pc; // vm_protect_pc(): luaH_clone may fail due to OOM

            sethvalue!(l, ra, lua_h_clone(l, hvalue!(kv as *const TValue)));
            vm_protect!(l, pc, base, {
              luaC_checkGC!(l);
            });
            continue 'dispatch;
          }
          LuauOpcode::LOP_SETLIST => {
            // lvmexecute.cpp:2485
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            // note: this can point to l->top if c == LUA_MULTRET making VM_REG unsafe to use
            let rb: StkId = base.add(LUAU_INSN_B!(insn) as usize);
            let mut c = LUAU_INSN_C!(insn) as i32 - 1;
            let index: u32 = *pc;
            pc = pc.add(1);

            if c == LUA_MULTRET {
              c = (*l).top.offset_from(rb) as i32;
              (*l).top = (*(*l).ci).top;
            }

            let h = hvalue!(ra as *const TValue);

            // TODO: we really don't need this anymore
            if !ttistable!(ra as *const TValue) {
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
                rb.add(i as usize) as *const TValue
              );
            }

            luaC_barrierfast!(l, h);
            continue 'dispatch;
          }
          LuauOpcode::LOP_FORNPREP => {
            // lvmexecute.cpp:2546
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

            if !ttisnumber!(ra.add(0) as *const TValue)
              || !ttisnumber!(ra.add(1) as *const TValue)
              || !ttisnumber!(ra.add(2) as *const TValue)
            {
              // slow-path: can convert arguments to numbers and trigger Lua errors
              // Note: this doesn't reallocate stack so we don't need to recompute ra/base
              (*(*l).ci).savedpc = pc; // vm_protect_pc()

              lua_v_prepare_forn(l, ra.add(0), ra.add(1), ra.add(2));
            }

            let limit = nvalue!(ra.add(0) as *const TValue);
            let step = nvalue!(ra.add(1) as *const TValue);
            let idx = nvalue!(ra.add(2) as *const TValue);

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
                LUAU_INSN_D!(insn) as isize
              },
            );
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_FORNLOOP => {
            // lvmexecute.cpp:2573
            VM_INTERRUPT!(l, pc, base);
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            LUAU_ASSERT!(
              ttisnumber!(ra.add(0) as *const TValue)
                && ttisnumber!(ra.add(1) as *const TValue)
                && ttisnumber!(ra.add(2) as *const TValue)
            );

            let limit = nvalue!(ra.add(0) as *const TValue);
            let step = nvalue!(ra.add(1) as *const TValue);
            let idx = nvalue!(ra.add(2) as *const TValue) + step;

            setnvalue!(ra.add(2), idx);

            // Note: make sure the loop condition is exactly the same between
            // this and LOP_FORNPREP so that we handle NaN/etc. consistently
            if if step > 0.0 {
              idx <= limit
            } else {
              limit <= idx
            } {
              pc = pc.offset(LUAU_INSN_D!(insn) as isize);
              let p = {
                let l = &(*cl).inner.l;
                l.p
              };
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
            let mut ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

            if FFlag::DebugLuauUserDefinedClassesRuntime.get() {
              // If this is a function it will be called during FORGLOOP
              if !ttisfunction!(ra as *const TValue) {
                let mt = if ttistable!(ra as *const TValue) {
                  (*hvalue!(ra as *const TValue)).metatable
                } else if ttisuserdata!(ra as *const TValue) {
                  uvalue!(ra as *const TValue).metatable
                } else {
                  null_mut()
                };
                let mut fn_tm = fasttm(l, mt, TMS::TmIter as i32);

                if fn_tm.is_null() && ttisobject!(ra as *const TValue) {
                  fn_tm = lua_t_gettmbyobj(l, ra as *const TValue, TMS::TmIter);
                  // if the metamethod is not present, error.
                  if ttisnil!(fn_tm) {
                    (*(*l).ci).savedpc = pc; // vm_protect_pc(): next call always errors
                    luaG_typeerrorL(
                      l,
                      ra as *const TValue,
                      c"iterate over".as_ptr() as *const c_char,
                    );
                  }
                }

                if !fn_tm.is_null() {
                  setobj_2_s!(l, ra.add(1), ra as *const TValue);
                  setobj_2_s!(l, ra, fn_tm);

                  (*l).top = ra.add(2); // func + self arg
                  LUAU_ASSERT!((*l).top <= (*l).stack_last);

                  vm_protect!(l, pc, base, {
                    lua_d_call(l, ra, 3);
                  });
                  (*l).top = (*(*l).ci).top;

                  // recompute ra since stack might have been reallocated
                  ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

                  // protect against __iter returning nil, since nil is used
                  // as a marker for builtin iteration in FORGLOOP
                  if ttisnil!(ra as *const TValue) {
                    (*(*l).ci).savedpc = pc; // vm_protect_pc(): next call always errors
                    luaG_typeerrorL(l, ra as *const TValue, c"call".as_ptr() as *const c_char);
                  }
                } else if !fasttm(l, mt, TMS::TmCall as i32).is_null() {
                  // table or userdata with __call, will be called during FORGLOOP
                  // TODO: we might be able to stop supporting this depending
                  // on whether it's used in practice
                } else if ttistable!(ra as *const TValue) {
                  // set up registers for builtin iteration
                  setobj_2_s!(l, ra.add(1), ra as *const TValue);
                  setpvalue!(ra.add(2), null_mut::<c_void>(), LU_TAG_ITERATOR);
                  setnilvalue!(ra);
                } else {
                  (*(*l).ci).savedpc = pc; // vm_protect_pc(): next call always errors
                  luaG_typeerrorL(
                    l,
                    ra as *const TValue,
                    c"iterate over".as_ptr() as *const c_char,
                  );
                }
              }
            } else {
              if ttisfunction!(ra as *const TValue) {
                // will be called during FORGLOOP
              } else {
                let mt = if ttistable!(ra as *const TValue) {
                  (*hvalue!(ra as *const TValue)).metatable
                } else if ttisuserdata!(ra as *const TValue) {
                  uvalue!(ra as *const TValue).metatable
                } else {
                  null_mut()
                };

                let fn_tm = fasttm(l, mt, TMS::TmIter as i32);
                if !fn_tm.is_null() {
                  setobj_2_s!(l, ra.add(1), ra as *const TValue);
                  setobj_2_s!(l, ra, fn_tm);

                  (*l).top = ra.add(2); // func + self arg
                  LUAU_ASSERT!((*l).top <= (*l).stack_last);

                  vm_protect!(l, pc, base, {
                    lua_d_call(l, ra, 3);
                  });
                  (*l).top = (*(*l).ci).top;

                  // recompute ra since stack might have been reallocated
                  ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

                  // protect against __iter returning nil, since nil is used
                  // as a marker for builtin iteration in FORGLOOP
                  if ttisnil!(ra as *const TValue) {
                    (*(*l).ci).savedpc = pc; // vm_protect_pc(): next call always errors
                    luaG_typeerrorL(l, ra as *const TValue, c"call".as_ptr() as *const c_char);
                  }
                } else if !fasttm(l, mt, TMS::TmCall as i32).is_null() {
                  // table or userdata with __call, will be called during FORGLOOP
                  // TODO: we might be able to stop supporting this depending
                  // on whether it's used in practice
                } else if ttistable!(ra as *const TValue) {
                  // set up registers for builtin iteration
                  setobj_2_s!(l, ra.add(1), ra as *const TValue);
                  setpvalue!(ra.add(2), null_mut::<c_void>(), LU_TAG_ITERATOR);
                  setnilvalue!(ra);
                } else {
                  (*(*l).ci).savedpc = pc; // vm_protect_pc(): next call always errors
                  luaG_typeerrorL(
                    l,
                    ra as *const TValue,
                    c"iterate over".as_ptr() as *const c_char,
                  );
                }
              }
            }

            pc = pc.offset(LUAU_INSN_D!(insn) as isize);
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_FORGLOOP => {
            // lvmexecute.cpp:2807
            VM_INTERRUPT!(l, pc, base);
            let insn = *pc;
            pc = pc.add(1);
            let mut ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let aux: u32 = *pc;

            // fast-path: builtin table iteration
            // note: ra=nil guarantees ra+1=table and ra+2=userdata because of
            // the setup by FORGPREP* opcodes
            // TODO: remove the table check per guarantee above
            if ttisnil!(ra as *const TValue) && ttistable!(ra.add(1) as *const TValue) {
              let h = hvalue!(ra.add(1) as *const TValue);
              let mut index = pvalue!(ra.add(2) as *const TValue) as usize as i32;

              let sizearray = (*h).sizearray;

              // clear extra variables since we might have more than two
              // note: while aux encodes ipairs bit, when set we always use 2
              // variables, so it's safe to check this via a signed comparison
              if (aux as i32) > 2 {
                for i in 2..aux as i32 {
                  setnilvalue!(ra.add(3 + i as usize));
                }
              }

              // terminate ipairs-style traversal early when encountering nil
              if (aux as i32) < 0
                && (index as u32 >= sizearray as u32
                  || ttisnil!((*h).array.add(index as usize) as *const TValue))
              {
                pc = pc.add(1);
                continue 'dispatch;
              }

              // first we advance index through the array portion
              while (index as u32) < sizearray as u32 {
                let e = (*h).array.add(index as usize);

                if !ttisnil!(e as *const TValue) {
                  setpvalue!(
                    ra.add(2),
                    (index + 1) as usize as *mut c_void,
                    LU_TAG_ITERATOR
                  );
                  setnvalue!(ra.add(3), (index + 1) as f64);
                  setobj_2_s!(l, ra.add(4), e as *const TValue);

                  pc = pc.offset(LUAU_INSN_D!(insn) as isize);
                  let p = {
                    let l = &(*cl).inner.l;
                    l.p
                  };
                  LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                  continue 'dispatch;
                }

                index += 1;
              }

              let sizenode = 1i32 << (*h).lsizenode;

              // then we advance index through the hash portion
              while ((index - sizearray) as u32) < sizenode as u32 {
                let n = (*h).node.add((index - sizearray) as usize);

                if !ttisnil!(gval!(n) as *const TValue) {
                  setpvalue!(
                    ra.add(2),
                    (index + 1) as usize as *mut c_void,
                    LU_TAG_ITERATOR
                  );
                  getnodekey!(l, ra.add(3), n);
                  setobj_2_s!(l, ra.add(4), gval!(n) as *const TValue);

                  pc = pc.offset(LUAU_INSN_D!(insn) as isize);
                  let p = {
                    let l = &(*cl).inner.l;
                    l.p
                  };
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
              setobj_2_s!(l, ra.add(3 + 2), ra.add(2) as *const TValue);
              setobj_2_s!(l, ra.add(3 + 1), ra.add(1) as *const TValue);
              setobj_2_s!(l, ra.add(3), ra as *const TValue);

              (*l).top = ra.add(3 + 3); // func + 2 args (state and index)
              LUAU_ASSERT!((*l).top <= (*l).stack_last);

              if FFlag::LuauYieldIter2.get() {
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
              ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

              // copy first variable back into the iteration index
              setobj_2_s!(l, ra.add(2), ra.add(3) as *const TValue);

              // note that we need to increment pc by 1 to exit the loop since
              // we need to skip over aux
              pc = pc.offset(if ttisnil!(ra.add(3) as *const TValue) {
                1
              } else {
                LUAU_INSN_D!(insn) as isize
              });
              let p = {
                let l = &(*cl).inner.l;
                l.p
              };
              LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_FORGPREP_INEXT => {
            // lvmexecute.cpp:2830
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

            // fast-path: ipairs/inext
            if (*(*cl).env).safeenv != 0
              && ttistable!(ra.add(1) as *const TValue)
              && ttisnumber!(ra.add(2) as *const TValue)
              && nvalue!(ra.add(2) as *const TValue) == 0.0
            {
              setnilvalue!(ra);
              // ra+1 is already the table
              setpvalue!(ra.add(2), null_mut::<c_void>(), LU_TAG_ITERATOR);
            } else if !ttisfunction!(ra as *const TValue) {
              (*(*l).ci).savedpc = pc; // vm_protect_pc(): next call always errors
              luaG_typeerrorL(
                l,
                ra as *const TValue,
                c"iterate over".as_ptr() as *const c_char,
              );
            }

            pc = pc.offset(LUAU_INSN_D!(insn) as isize);
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_FORGPREP_NEXT => {
            // lvmexecute.cpp:2853
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

            // fast-path: pairs/next
            if (*(*cl).env).safeenv != 0
              && ttistable!(ra.add(1) as *const TValue)
              && ttisnil!(ra.add(2) as *const TValue)
            {
              setnilvalue!(ra);
              // ra+1 is already the table
              setpvalue!(ra.add(2), null_mut::<c_void>(), LU_TAG_ITERATOR);
            } else if !ttisfunction!(ra as *const TValue) {
              (*(*l).ci).savedpc = pc; // vm_protect_pc(): next call always errors
              luaG_typeerrorL(
                l,
                ra as *const TValue,
                c"iterate over".as_ptr() as *const c_char,
              );
            }

            pc = pc.offset(LUAU_INSN_D!(insn) as isize);
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_NATIVECALL => {
            // lvmexecute.cpp:2873
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!(!(*p).execdata.is_null());

            let ci = (*l).ci;
            (*ci).flags = LUA_CALLINFO_NATIVE as u32;
            (*ci).savedpc = (*p).code;

            // VM_HAS_NATIVE
            if let Some(enter) = (*(*l).global).ecb.enter {
              if enter(l, p) == 1 {
                continue 'reentry; // goto reentry
              } else {
                return; // goto exit
              }
            }
            // (no native entry callback installed)
            return;
          }
          LuauOpcode::LOP_GETVARARGS => {
            // lvmexecute.cpp:2902
            let insn = *pc;
            pc = pc.add(1);
            let b = LUAU_INSN_B!(insn) as i32 - 1;
            let n = {
              let closure_l = &(*cl).inner.l;
              base.offset_from((*(*l).ci).func) as i32 - (*closure_l.p).numparams as i32 - 1
            };

            if b == LUA_MULTRET {
              vm_protect!(l, pc, base, {
                luaD_checkstack!(l, n);
              });
              // previous call may change the stack
              let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

              for j in 0..n {
                setobj_2_s!(
                  l,
                  ra.add(j as usize),
                  base.sub(n as usize).add(j as usize) as *const TValue
                );
              }

              (*l).top = ra.add(n as usize);
              continue 'dispatch;
            } else {
              let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

              let mut j = 0;
              while j < b && j < n {
                setobj_2_s!(
                  l,
                  ra.add(j as usize),
                  base.sub(n as usize).add(j as usize) as *const TValue
                );
                j += 1;
              }
              let mut j = n;
              while j < b {
                setnilvalue!(ra.add(j as usize));
                j += 1;
              }
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_DUPCLOSURE => {
            // lvmexecute.cpp:2959
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_D!(insn), cl, k) as *mut TValue;

            let kcl = clvalue!(kv as *const TValue);

            (*(*l).ci).savedpc = pc; // vm_protect_pc(): luaF_newLclosure may fail due to OOM

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
            while ui < (*kcl).nupvalues as i32 {
              let uinsn = *pc.add(ui as usize);
              LUAU_ASSERT!(LUAU_INSN_OP!(uinsn) == LuauOpcode::LOP_CAPTURE as u32);
              LUAU_ASSERT!(
                LUAU_INSN_A!(uinsn) == LuauCaptureType::LCT_VAL as u32
                  || LUAU_INSN_A!(uinsn) == LuauCaptureType::LCT_UPVAL as u32
              );

              let uv: *mut TValue = if LUAU_INSN_A!(uinsn) == LuauCaptureType::LCT_VAL as u32 {
                VM_REG!(LUAU_INSN_B!(uinsn), l, base) as *mut TValue
              } else {
                VM_UV!(LUAU_INSN_B!(uinsn), cl) as *mut TValue
              };

              let uref = {
                let l = &mut (*ncl).inner.l;
                l.uprefs.as_mut_ptr().add(ui as usize)
              };

              // check if the existing closure is safe to reuse
              if ncl == kcl && luaO_rawequalObj(uref as *const TValue, uv as *const TValue) != 0 {
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
              setobj!(l, uref, uv as *const TValue);
              luaC_barrier!(l, ncl, uv as *const TValue);
              ui += 1;
            }

            // this is a noop if ncl is newly created or shared successfully, but
            // it has to run after the closure is preloaded for the first time
            (*ncl).preload = 0;

            if kcl != ncl {
              vm_protect!(l, pc, base, {
                luaC_checkGC!(l);
              });
            }

            pc = pc.add((*kcl).nupvalues as usize);
            continue 'dispatch;
          }
          LuauOpcode::LOP_PREPVARARGS => {
            // lvmexecute.cpp:2988
            let insn = *pc;
            pc = pc.add(1);
            let numparams = LUAU_INSN_A!(insn) as i32;

            // all fixed parameters are copied after the top so we need more stack space
            vm_protect!(l, pc, base, {
              luaD_checkstack!(l, (*cl).stacksize as i32 + numparams);
            });

            // the caller must have filled extra fixed arguments with nil
            LUAU_ASSERT!((*l).top.offset_from(base) as i32 >= numparams);

            // move fixed parameters to final position
            let fixed = base; // first fixed argument
            base = (*l).top; // final position of first argument

            for i in 0..numparams as usize {
              setobj_2_s!(l, base.add(i), fixed.add(i) as *const TValue);
              setnilvalue!(fixed.add(i));
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
            let insn = *pc;
            pc = pc.add(1);

            pc = pc.offset(LUAU_INSN_D!(insn) as isize);
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }

          LuauOpcode::LOP_LOADKX => {
            // lvmexecute.cpp:2998
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kv = VM_KV!(aux, cl, k);

            setobj_2_s!(l, ra as *mut TValue, kv as *const TValue);
            continue 'dispatch;
          }

          LuauOpcode::LOP_JUMPX => {
            // lvmexecute.cpp:3009
            VM_INTERRUPT!(l, pc, base);
            let insn = *pc;
            pc = pc.add(1);

            pc = pc.offset(LUAU_INSN_E!(insn) as isize);
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_FASTCALL => {
            // lvmexecute.cpp:3068
            let insn = *pc;
            pc = pc.add(1);
            let bfid = LUAU_INSN_A!(insn) as i32;
            let skip = LUAU_INSN_C!(insn) as i32;
            {
              {
                let p = {
                  {
                    let l = &(*cl).inner.l;
                    l.p
                  }
                };
                LUAU_ASSERT!(
                  ((pc.offset_from((*p).code) as i32 + skip) as u32) < (*p).sizecode as u32
                );
              }
            }

            let call: Instruction = *pc.add(skip as usize);
            LUAU_ASSERT!(LUAU_INSN_OP!(call) == LuauOpcode::LOP_CALL as u32);

            let ra = VM_REG!(LUAU_INSN_A!(call), l, base) as *mut TValue;

            let mut nparams = LUAU_INSN_B!(call) as i32 - 1;
            let nresults = LUAU_INSN_C!(call) as i32 - 1;

            nparams = if nparams == LUA_MULTRET {
              (*l).top.offset_from(ra.add(1)) as i32
            } else {
              nparams
            };

            let f = luauF_table[bfid as usize];
            LUAU_ASSERT!(f.is_some());

            if (*(*cl).env).safeenv != 0 {
              (*(*l).ci).savedpc = pc; // vm_protect_pc(): f may fail due to OOM

              let n = f.unwrap()(l, ra, ra.add(1), nresults, ra.add(2), nparams);

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
                let p = {
                  let l = &(*cl).inner.l;
                  l.p
                };
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                continue 'dispatch;
              } else {
                // continue execution through the fallback code
                continue 'dispatch;
              }
            } else {
              // continue execution through the fallback code
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_COVERAGE => {
            // lvmexecute.cpp:3080
            let insn = *pc;
            pc = pc.add(1);
            let mut hits: i32 = LUAU_INSN_E!(insn);

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
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_B!(insn), cl, k) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            // fast-path
            if ttisnumber!(rc as *const TValue) {
              setnvalue!(
                ra,
                nvalue!(kv as *const TValue) - nvalue!(rc as *const TValue)
              );
              continue 'dispatch;
            } else {
              // slow-path, may invoke C/Lua via metamethods
              vm_protect!(l, pc, base, {
                lua_v_doarithimpl(l, ra, kv as *const TValue, rc as *const TValue, TMS::TmSub);
              });
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_DIVRK => {
            // lvmexecute.cpp:3135
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_B!(insn), cl, k) as *mut TValue;
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;

            // fast-path
            if ttisnumber!(rc as *const TValue) {
              setnvalue!(
                ra,
                nvalue!(kv as *const TValue) / nvalue!(rc as *const TValue)
              );
              continue 'dispatch;
            } else if ttisvector!(rc as *const TValue) {
              let nb = nvalue!(kv as *const TValue) as f32;
              let vc = vvalue!(rc as *const TValue).as_ptr();
              setvvalue!(
                ra,
                nb / *vc.add(0),
                nb / *vc.add(1),
                nb / *vc.add(2),
                nb / *vc.add(3)
              );
              continue 'dispatch;
            } else {
              // slow-path, may invoke C/Lua via metamethods
              vm_protect!(l, pc, base, {
                lua_v_doarithimpl(l, ra, kv as *const TValue, rc as *const TValue, TMS::TmDiv);
              });
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_FASTCALL1 => {
            // lvmexecute.cpp:3183
            let insn = *pc;
            pc = pc.add(1);
            let bfid = LUAU_INSN_A!(insn) as i32;
            let arg = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let skip = LUAU_INSN_C!(insn) as i32;
            {
              {
                let p = {
                  {
                    let l = &(*cl).inner.l;
                    l.p
                  }
                };
                LUAU_ASSERT!(
                  ((pc.offset_from((*p).code) as i32 + skip) as u32) < (*p).sizecode as u32
                );
              }
            }

            let call: Instruction = *pc.add(skip as usize);
            LUAU_ASSERT!(LUAU_INSN_OP!(call) == LuauOpcode::LOP_CALL as u32);

            let ra = VM_REG!(LUAU_INSN_A!(call), l, base) as *mut TValue;

            let nparams = 1i32;
            let nresults = LUAU_INSN_C!(call) as i32 - 1;

            let f = luauF_table[bfid as usize];
            LUAU_ASSERT!(f.is_some());

            if (*(*cl).env).safeenv != 0 {
              (*(*l).ci).savedpc = pc; // vm_protect_pc(): f may fail due to OOM

              let n = f.unwrap()(l, ra, arg, nresults, null_mut(), nparams);

              if n >= 0 {
                if nresults == LUA_MULTRET {
                  (*l).top = ra.add(n as usize);
                }

                // skip instructions that compute function as well as CALL
                pc = pc.add((skip + 1) as usize);
                let p = {
                  let l = &(*cl).inner.l;
                  l.p
                };
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                continue 'dispatch;
              } else {
                // continue execution through the fallback code
                continue 'dispatch;
              }
            } else {
              // continue execution through the fallback code
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_FASTCALL2 => {
            // lvmexecute.cpp:3233
            let insn = *pc;
            pc = pc.add(1);
            let bfid = LUAU_INSN_A!(insn) as i32;
            let skip = LUAU_INSN_C!(insn) as i32 - 1;
            let aux: u32 = *pc;
            pc = pc.add(1);
            let arg1 = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let arg2 = VM_REG!(aux, l, base) as *mut TValue;
            {
              let p = {
                let l = &(*cl).inner.l;
                l.p
              };
              LUAU_ASSERT!(
                ((pc.offset_from((*p).code) as i32 + skip) as u32) < (*p).sizecode as u32
              );
            }

            let call: Instruction = *pc.add(skip as usize);
            LUAU_ASSERT!(LUAU_INSN_OP!(call) == LuauOpcode::LOP_CALL as u32);

            let ra = VM_REG!(LUAU_INSN_A!(call), l, base) as *mut TValue;

            let nparams = 2i32;
            let nresults = LUAU_INSN_C!(call) as i32 - 1;

            let f = luauF_table[bfid as usize];
            LUAU_ASSERT!(f.is_some());

            if (*(*cl).env).safeenv != 0 {
              (*(*l).ci).savedpc = pc; // vm_protect_pc(): f may fail due to OOM

              let n = f.unwrap()(l, ra, arg1, nresults, arg2, nparams);

              if n >= 0 {
                if nresults == LUA_MULTRET {
                  (*l).top = ra.add(n as usize);
                }

                // skip instructions that compute function as well as CALL
                pc = pc.add((skip + 1) as usize);
                let p = {
                  let l = &(*cl).inner.l;
                  l.p
                };
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                continue 'dispatch;
              } else {
                // continue execution through the fallback code
                continue 'dispatch;
              }
            } else {
              // continue execution through the fallback code
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_FASTCALL2K => {
            // lvmexecute.cpp:3283
            let insn = *pc;
            pc = pc.add(1);
            let bfid = LUAU_INSN_A!(insn) as i32;
            let skip = LUAU_INSN_C!(insn) as i32 - 1;
            let aux: u32 = *pc;
            pc = pc.add(1);
            let arg1 = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let arg2 = VM_KV!(aux, cl, k) as *mut TValue;
            {
              let p = {
                let l = &(*cl).inner.l;
                l.p
              };
              LUAU_ASSERT!(
                ((pc.offset_from((*p).code) as i32 + skip) as u32) < (*p).sizecode as u32
              );
            }

            let call: Instruction = *pc.add(skip as usize);
            LUAU_ASSERT!(LUAU_INSN_OP!(call) == LuauOpcode::LOP_CALL as u32);

            let ra = VM_REG!(LUAU_INSN_A!(call), l, base) as *mut TValue;

            let nparams = 2i32;
            let nresults = LUAU_INSN_C!(call) as i32 - 1;

            let f = luauF_table[bfid as usize];
            LUAU_ASSERT!(f.is_some());

            if (*(*cl).env).safeenv != 0 {
              (*(*l).ci).savedpc = pc; // vm_protect_pc(): f may fail due to OOM

              let n = f.unwrap()(l, ra, arg1, nresults, arg2, nparams);

              if n >= 0 {
                if nresults == LUA_MULTRET {
                  (*l).top = ra.add(n as usize);
                }

                // skip instructions that compute function as well as CALL
                pc = pc.add((skip + 1) as usize);
                let p = {
                  let l = &(*cl).inner.l;
                  l.p
                };
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                continue 'dispatch;
              } else {
                // continue execution through the fallback code
                continue 'dispatch;
              }
            } else {
              // continue execution through the fallback code
              continue 'dispatch;
            }
          }
          LuauOpcode::LOP_FASTCALL3 => {
            // lvmexecute.cpp:3340
            let insn = *pc;
            pc = pc.add(1);
            let bfid = LUAU_INSN_A!(insn) as i32;
            let skip = LUAU_INSN_C!(insn) as i32 - 1;
            let aux: u32 = *pc;
            pc = pc.add(1);
            let arg1 = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let arg2 = VM_REG!(LUAU_INSN_AUX_A(aux), l, base) as *mut TValue;
            let arg3 = VM_REG!(LUAU_INSN_AUX_B(aux), l, base) as *mut TValue;
            {
              let p = {
                let l = &(*cl).inner.l;
                l.p
              };
              LUAU_ASSERT!(
                ((pc.offset_from((*p).code) as i32 + skip) as u32) < (*p).sizecode as u32
              );
            }

            let call: Instruction = *pc.add(skip as usize);
            LUAU_ASSERT!(LUAU_INSN_OP!(call) == LuauOpcode::LOP_CALL as u32);

            let ra = VM_REG!(LUAU_INSN_A!(call), l, base) as *mut TValue;

            let nparams = 3i32;
            let nresults = LUAU_INSN_C!(call) as i32 - 1;

            let f = luauF_table[bfid as usize];
            LUAU_ASSERT!(f.is_some());

            if (*(*cl).env).safeenv != 0 {
              (*(*l).ci).savedpc = pc; // vm_protect_pc(): f may fail due to OOM

              // note: it's safe to push arguments past top for complicated reasons (see top of the file)
              LUAU_ASSERT!((*l).top.add(2) < (*l).stack.add((*l).stacksize as usize));
              let top = (*l).top;
              setobj_2_s!(l, top, arg2 as *const TValue);
              setobj_2_s!(l, top.add(1), arg3 as *const TValue);

              let n = f.unwrap()(l, ra, arg1, nresults, top, nparams);

              if n >= 0 {
                if nresults == LUA_MULTRET {
                  (*l).top = ra.add(n as usize);
                }

                // skip instructions that compute function as well as CALL
                pc = pc.add((skip + 1) as usize);
                let p = {
                  let l = &(*cl).inner.l;
                  l.p
                };
                LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
                continue 'dispatch;
              } else {
                // continue execution through the fallback code
                continue 'dispatch;
              }
            } else {
              // continue execution through the fallback code
              continue 'dispatch;
            }
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
                luau_callhook(l, debugbreak, null_mut());
              });

              // allow debugbreak hook to put thread into error/yield state
              if (*l).status != 0 {
                return; // goto exit
              }
            }

            // VM_CONTINUE(op): re-dispatch the original opcode without refetching
            continue_op = Some(op);
            continue 'dispatch;
          }
          LuauOpcode::LOP_JUMPXEQKNIL => {
            // lvmexecute.cpp:3372
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

            // static_assert(LUA_TNIL == 0): type-1 is negative iff type is nil.
            // condition is equivalent to: int(ttisnil(ra)) != LUAU_INSN_AUX_NOT(aux)
            pc = pc.offset(
              if (((ttype!(ra as *const TValue) - 1) as u32 ^ aux) as i32) < 0 {
                LUAU_INSN_D!(insn) as isize
              } else {
                1
              },
            );
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_JUMPXEQKB => {
            // lvmexecute.cpp:3383
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

            pc = pc.offset(
              if (ttisboolean!(ra as *const TValue)
                && bvalue!(ra as *const TValue) == LUAU_INSN_AUX_KB(aux) as i32)
                as i32
                != LUAU_INSN_AUX_NOT(aux) as i32
              {
                LUAU_INSN_D!(insn) as isize
              } else {
                1
              },
            );
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_JUMPXEQKN => {
            // lvmexecute.cpp:3405 (non-aarch64 flavor; the __aarch64__
            // branch is a codegen-only variant with identical semantics)
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_AUX_KV(aux), cl, k) as *mut TValue;
            LUAU_ASSERT!(ttisnumber!(kv as *const TValue));

            pc = pc.offset(
              if (ttisnumber!(ra as *const TValue)
                && nvalue!(ra as *const TValue) == nvalue!(kv as *const TValue))
                as i32
                != LUAU_INSN_AUX_NOT(aux) as i32
              {
                LUAU_INSN_D!(insn) as isize
              } else {
                1
              },
            );
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_JUMPXEQKS => {
            // lvmexecute.cpp:3418
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(LUAU_INSN_AUX_KV(aux), cl, k) as *mut TValue;
            LUAU_ASSERT!(ttisstring!(kv as *const TValue));

            pc = pc.offset(
              if (ttisstring!(ra as *const TValue)
                && gcvalue!(ra as *const TValue) == gcvalue!(kv as *const TValue))
                as i32
                != LUAU_INSN_AUX_NOT(aux) as i32
              {
                LUAU_INSN_D!(insn) as isize
              } else {
                1
              },
            );
            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }
          LuauOpcode::LOP_GETUDATAKS => {
            // lvmexecute.cpp:3498
            let insn = *pc;
            pc = pc.add(1);
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kidx = LUAU_INSN_AUX_KV16(aux);
            let kv = VM_KV!(kidx, cl, k) as *mut TValue;

            'udata_fast: {
              if ttisuserdata!(rb as *const TValue) {
                let utag = uvalue!(rb as *const TValue).tag as usize;
                let udatadirect = &mut (*(*l).global).udatadirect[utag];
                let onudataindex = udatadirect.index;
                let tm = &mut udatadirect.indextm as *mut TValue;

                if let Some(onudataindex) = onudataindex
                  && !ttisnil!(tm as *const TValue)
                {
                  let udata = {
                    let u = uvalue!(rb as *const TValue);
                    u.data.as_ptr() as *mut c_void
                  };

                  // note: it's safe to push arguments past top for
                  // complicated reasons (see top of the file)
                  LUAU_ASSERT!((*l).top.add(3) < (*l).stack.add((*l).stacksize as usize));
                  let top = (*l).top;
                  setobj_2_s!(l, top.add(0), tm as *const TValue);
                  setobj_2_s!(l, top.add(1), rb as *const TValue);
                  setobj_2_s!(l, top.add(2), kv as *const TValue);
                  (*l).top = (*l).top.add(3);

                  (*(*l).ci).savedpc = pc;

                  (*l).n_ccalls += 1;

                  if ((*l).n_ccalls as i32) >= LUAI_MAXCCALLS {
                    luaD_checkCstack(l);
                  }

                  luau_setupcci(l, 1, top);

                  let mut cachedslot: u16 = LUAU_INSN_AUX_SLOT!(aux) as u16;
                  onudataindex(
                    l,
                    udata,
                    (*tsvalue!(kv as *const TValue)).atom as i32,
                    &mut cachedslot,
                    utag as i32,
                  );

                  // update cached slot if instruction didn't deoptimize
                  if cachedslot as u32 != LUAU_INSN_AUX_SLOT!(aux)
                    && LUAU_INSN_OP!(*pc.sub(2)) == LuauOpcode::LOP_GETUDATAKS as u32
                  {
                    vm_patch_aux_slot(pc.sub(1), kidx, cachedslot as i32);
                  }

                  // ci is our callinfo, cip is our parent
                  let ci = (*l).ci;
                  let cip = ci.sub(1);

                  if FFlag::LuauClosureUsageCounter.get() {
                    let cicl = clvalue!((*ci).func);
                    LUAU_ASSERT!((*cicl).usage > 0);
                    (*cicl).usage -= 1;
                  }

                  (*l).ci = cip;
                  (*l).base = (*cip).base;
                  (*l).n_ccalls -= 1;

                  // stack may have been reallocated, so we need to refresh base ptr
                  base = (*l).base;
                  let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

                  // grab result while l->top is still pointed to the
                  // previous function frame
                  setobj_2_s!(l, ra, (*l).top.sub(1) as *const TValue);

                  // then update top
                  (*l).top = (*cip).top;

                  continue 'dispatch;
                }
              }
              break 'udata_fast;
            }

            // Slow path - backpatch and dispatch to regular table access
            vm_patch_op(pc.sub(2), LuauOpcode::LOP_GETTABLEKS as u8);
            vm_patch_aux_slot(pc.sub(1), kidx, 0);

            pc = pc.sub(2);
            continue_op = Some(LuauOpcode::LOP_GETTABLEKS as u8); // VM_CONTINUE
            continue 'dispatch;
          }
          LuauOpcode::LOP_SETUDATAKS => {
            // lvmexecute.cpp:3573
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kidx = LUAU_INSN_AUX_KV16(aux);
            let kv = VM_KV!(kidx, cl, k) as *mut TValue;

            'udata_fast: {
              if ttisuserdata!(rb as *const TValue) {
                let utag = uvalue!(rb as *const TValue).tag as usize;
                let udatadirect = &mut (*(*l).global).udatadirect[utag];
                let onudatanewindex = udatadirect.newindex;
                let tm = &mut udatadirect.newindextm as *mut TValue;

                if let Some(onudatanewindex) = onudatanewindex
                  && !ttisnil!(tm as *const TValue)
                {
                  let udata = {
                    let u = uvalue!(rb as *const TValue);
                    u.data.as_ptr() as *mut c_void
                  };

                  // note: it's safe to push arguments past top for
                  // complicated reasons (see top of the file)
                  LUAU_ASSERT!((*l).top.add(4) < (*l).stack.add((*l).stacksize as usize));
                  let top = (*l).top;
                  setobj_2_s!(l, top.add(0), tm as *const TValue);
                  setobj_2_s!(l, top.add(1), rb as *const TValue);
                  setobj_2_s!(l, top.add(2), kv as *const TValue);
                  setobj_2_s!(l, top.add(3), ra as *const TValue);
                  (*l).top = (*l).top.add(4);

                  (*(*l).ci).savedpc = pc;

                  (*l).n_ccalls += 1;

                  if ((*l).n_ccalls as i32) >= LUAI_MAXCCALLS {
                    luaD_checkCstack(l);
                  }

                  luau_setupcci(l, 0, top);

                  let mut cachedslot: u16 = LUAU_INSN_AUX_SLOT!(aux) as u16;
                  onudatanewindex(
                    l,
                    udata,
                    (*tsvalue!(kv as *const TValue)).atom as i32,
                    &mut cachedslot,
                    utag as i32,
                  );

                  // update cached slot if instruction didn't deoptimize
                  if cachedslot as u32 != LUAU_INSN_AUX_SLOT!(aux)
                    && LUAU_INSN_OP!(*pc.sub(2)) == LuauOpcode::LOP_SETUDATAKS as u32
                  {
                    vm_patch_aux_slot(pc.sub(1), kidx, cachedslot as i32);
                  }

                  // ci is our callinfo, cip is our parent
                  let ci = (*l).ci;
                  let cip = ci.sub(1);

                  if FFlag::LuauClosureUsageCounter.get() {
                    let cicl = clvalue!((*ci).func);
                    LUAU_ASSERT!((*cicl).usage > 0);
                    (*cicl).usage -= 1;
                  }

                  (*l).ci = cip;
                  (*l).base = (*cip).base;
                  (*l).top = (*cip).top;
                  (*l).n_ccalls -= 1;

                  // stack may have been reallocated, so we need to refresh base ptr
                  base = (*l).base;

                  continue 'dispatch;
                }
              }
              break 'udata_fast;
            }

            // Slow path - backpatch and dispatch to regular table access
            vm_patch_op(pc.sub(2), LuauOpcode::LOP_SETTABLEKS as u8);
            vm_patch_aux_slot(pc.sub(1), kidx, 0);

            pc = pc.sub(2);
            continue_op = Some(LuauOpcode::LOP_SETTABLEKS as u8); // VM_CONTINUE
            continue 'dispatch;
          }
          LuauOpcode::LOP_NAMECALLUDATA => {
            // lvmexecute.cpp:3670
            let insn = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let rb = VM_REG!(LUAU_INSN_B!(insn), l, base) as *mut TValue;
            let aux: u32 = *pc;
            pc = pc.add(1);
            let kidx = LUAU_INSN_AUX_KV16(aux);
            let kv = VM_KV!(kidx, cl, k) as *mut TValue;

            'udata_fast: {
              if ttisuserdata!(rb as *const TValue) {
                let utag = uvalue!(rb as *const TValue).tag as usize;
                let udatadirect = &mut (*(*l).global).udatadirect[utag];
                let onudatanamecall = udatadirect.namecall;
                let tm = &mut udatadirect.namecalltm as *mut TValue;

                if let Some(onudatanamecall) = onudatanamecall
                  && !ttisnil!(tm as *const TValue)
                {
                  let udata = {
                    let u = uvalue!(rb as *const TValue);
                    u.data.as_ptr() as *mut c_void
                  };

                  // note: order of copies allows rb to alias ra+1 or ra
                  setobj_2_s!(l, ra.add(1), rb as *const TValue);
                  setobj_2_s!(l, ra, tm as *const TValue);
                  let ncslot: *const Instruction = pc.sub(1);

                  LUAU_ASSERT!(
                    LUAU_INSN_OP!(*pc) == LuauOpcode::LOP_CALL as u32
                      || LUAU_INSN_OP!(*pc) == LuauOpcode::LOP_CALLFB as u32
                  );
                  let call_insn = *pc;
                  pc = pc.add(1);
                  if FFlag::LuauCallFeedback.get()
                    && LUAU_INSN_OP!(call_insn) == LuauOpcode::LOP_CALLFB as u32
                  {
                    pc = pc.add(1);
                  }

                  let call_ra = VM_REG!(LUAU_INSN_A!(call_insn), l, base) as *mut TValue;
                  LUAU_ASSERT!(call_ra == ra);

                  // first half of OP_CALL
                  let nparams = LUAU_INSN_B!(call_insn) as i32 - 1;
                  let nresults = LUAU_INSN_C!(call_insn) as i32 - 1;

                  (*(*l).ci).savedpc = pc;
                  (*l).namecall = tsvalue!(kv as *const TValue) as *mut tstring;
                  (*l).top = if nparams == LUA_MULTRET {
                    (*l).top
                  } else {
                    ra.add(1 + nparams as usize)
                  };

                  // note: namecalls do not increase C call number and allow yielding

                  luau_setupcci(l, nresults, ra);

                  LUAU_ASSERT!((*tsvalue!(kv as *const TValue)).atom >= 0);

                  let mut cachedslot: u16 = LUAU_INSN_AUX_SLOT!(aux) as u16;
                  let results = onudatanamecall(
                    l,
                    udata,
                    (*tsvalue!(kv as *const TValue)).atom as i32,
                    &mut cachedslot,
                    utag as i32,
                  );

                  // update cached slot if instruction didn't deoptimize
                  if cachedslot as u32 != LUAU_INSN_AUX_SLOT!(aux)
                    && LUAU_INSN_OP!(*ncslot.sub(1)) == LuauOpcode::LOP_NAMECALLUDATA as u32
                  {
                    vm_patch_aux_slot(ncslot, kidx, cachedslot as i32);
                  }

                  // yield
                  if results < 0 {
                    return;
                  }

                  // ci is our callinfo, cip is our parent
                  let ci = (*l).ci;
                  let cip = ci.sub(1);

                  if FFlag::LuauClosureUsageCounter.get() {
                    let cicl = clvalue!((*ci).func);
                    LUAU_ASSERT!((*cicl).usage > 0);
                    (*cicl).usage -= 1;
                  }

                  let mut res = (*ci).func;
                  let mut vali = (*l).top.sub(results as usize);
                  let valend = (*l).top;

                  let mut i = nresults;
                  while i != 0 && vali < valend {
                    setobj_2_s!(l, res, vali as *const TValue);
                    res = res.add(1);
                    vali = vali.add(1);
                    i -= 1;
                  }
                  while i > 0 {
                    setnilvalue!(res);
                    res = res.add(1);
                    i -= 1;
                  }

                  (*l).ci = cip;
                  (*l).base = (*cip).base;
                  (*l).top = if nresults == LUA_MULTRET {
                    res
                  } else {
                    (*cip).top
                  };

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
            continue_op = Some(LuauOpcode::LOP_NAMECALL as u8); // VM_CONTINUE
            continue 'dispatch;
          }
          LuauOpcode::LOP_NEWCLASSMEMBER => {
            // lvmexecute.cpp:3670 (NEWCLASSMEMBER)
            let insn = *pc;
            pc = pc.add(1);
            let aux: u32 = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let membername = VM_KV!(aux, cl, k) as *mut TValue;
            LUAU_ASSERT!(ttisstring!(membername as *const TValue));
            LUAU_ASSERT!(LUAU_INSN_B!(insn) == 0);
            let rc = VM_REG!(LUAU_INSN_C!(insn), l, base) as *mut TValue;
            (*(*l).ci).savedpc = pc; // vm_protect_pc()
            lua_r_addclassmember(
              l,
              &mut **classvalue!(ra as *const TValue) as *mut LuauClass,
              tsvalue!(membername as *const TValue) as *mut tstring,
              rc,
            );
            continue 'dispatch;
          }
          LuauOpcode::LOP_CMPPROTO => {
            // lvmexecute.cpp:3684
            let insn = *pc;
            pc = pc.add(1);
            let funid: u32 = *pc;
            pc = pc.add(1);
            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;

            if !ttisfunction!(ra as *const TValue) {
              pc = pc.offset(LUAU_INSN_D!(insn) as isize - 1);
              let p = {
                let l = &(*cl).inner.l;
                l.p
              };
              LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
              continue 'dispatch;
            }

            let ccl = clvalue!(ra as *const TValue);
            if (*ccl).is_c != 0 || {
              let l = &(*ccl).inner.l;
              (*l.p).funid != funid
            } {
              pc = pc.offset(LUAU_INSN_D!(insn) as isize - 1);
            }

            let p = {
              let l = &(*cl).inner.l;
              l.p
            };
            LUAU_ASSERT!((pc.offset_from((*p).code) as u32) < (*p).sizecode as u32);
            continue 'dispatch;
          }

          LuauOpcode::LOP_FASTPCALL => {
            // lvmexecute.cpp:3703
            pc = pc.add(1);
            continue 'dispatch;
          }

          LuauOpcode::LOP_NEWCLASS => {
            // lvmexecute.cpp:3778
            let insn = *pc;
            pc = pc.add(1);
            let super_reg = LUAU_INSN_B!(insn) as u8;
            let aux = *pc;
            pc = pc.add(1);

            let ra = VM_REG!(LUAU_INSN_A!(insn), l, base) as *mut TValue;
            let kv = VM_KV!(aux, cl, k) as *mut TValue;

            setobj_2_s!(l, ra, kv as *const TValue);

            let newcls = &mut **classvalue!(ra as *const TValue) as *mut LuauClass;
            (*newcls).isopen = (LUAU_INSN_C!(insn) & 0x1) != 0;

            if super_reg != 0xff {
              (*(*l).ci).savedpc = pc; // vm_protect_pc()

              let rb = VM_REG!(super_reg, l, base) as *mut TValue;

              if !ttisclass!(rb as *const TValue) {
                luaG_typeerror!(l, rb, c"extend".as_ptr());
              }

              let inherited = lua_r_inheritclass(
                l,
                newcls,
                &mut **classvalue!(rb as *const TValue) as *mut LuauClass,
              );
              setclassvalue!(l, ra, inherited);
            }

            continue 'dispatch;
          }

          _ => unreachable!("byte is not an executable opcode"),
        }
      }
    }
  }
}
