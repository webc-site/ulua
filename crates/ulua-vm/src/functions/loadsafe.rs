use core::{
  ffi::{CStr, c_char, c_int},
  mem::size_of,
  ptr::copy_nonoverlapping,
  slice::from_raw_parts,
};

use ulua_common::{
  FFlag,
  enums::{
    luau_bytecode_tag::{
      LBC_CONSTANT_BOOLEAN, LBC_CONSTANT_CLASS_SHAPE, LBC_CONSTANT_CLOSURE, LBC_CONSTANT_IMPORT,
      LBC_CONSTANT_INTEGER, LBC_CONSTANT_NIL, LBC_CONSTANT_NUMBER, LBC_CONSTANT_STRING,
      LBC_CONSTANT_TABLE, LBC_CONSTANT_TABLE_WITH_CONSTANTS, LBC_CONSTANT_VECTOR,
      LBC_TYPE_VERSION_MAX, LBC_TYPE_VERSION_MIN, LBC_VERSION_MAX, LBC_VERSION_MIN,
    },
    luau_bytecode_type::{
      LBC_TYPE_FUNCTION, LBC_TYPE_TAGGED_USERDATA_BASE, LBC_TYPE_TAGGED_USERDATA_END,
      LBC_TYPE_USERDATA,
    },
    luau_feedback_type::LuauFeedbackType,
    luau_opcode::LuauOpcode,
  },
  functions::get_op_length::getOpLength,
  macros::{luau_assert::LUAU_ASSERT, luau_insn_op::LUAU_INSN_OP},
};

use crate::{
  enums::feedback_vector_slot_kind::FeedbackVectorSlotKind,
  functions::{
    c_slice_mut, lua_a_toobject::luaA_toobject, lua_c_barrierback::lua_c_barrierback,
    lua_f_new_lclosure::lua_f_new_lclosure, lua_f_newproto::lua_f_newproto, lua_h_new::lua_h_new,
    lua_h_set::luaH_set, lua_h_setstr::lua_h_setstr, lua_o_chunkid::lua_o_chunkid,
    lua_pushlstring::lua_pushlstring, lua_r_newclass::lua_r_newclass, lua_s_newlstr::luaS_newlstr,
    read::read, read_string::read_string, read_var_int::read_var_int,
    read_var_int_64::read_var_int_64, remap_userdata_types::remap_userdata_types,
    resolve_import_safe::resolve_import_safe,
  },
  macros::{
    getstr::getstr, hvalue::hvalue, incr_top::incr_top, isblack::isblack,
    lua_c_barriert::luaC_barriert, lua_idsize::LUA_IDSIZE, lua_m_newarray::luaM_newarray,
    lua_s_new::luaS_new, lua_s_updateatom::luaS_updateatom, setbvalue::setbvalue,
    setclassvalue::setclassvalue, setclvalue::setclvalue, sethvalue::sethvalue,
    setlvalue::setlvalue, setnilvalue::setnilvalue, setnvalue::setnvalue, setobj::setobj,
    setobj_2_t::setobj2t, setsvalue::setsvalue, setvvalue::setvvalue, tsvalue::tsvalue,
    ttisnil::ttisnil, ttisstring::ttisstring,
  },
  records::{
    feedback_vector_slot::FeedbackVectorSlot, gc_object::GCObject, loc_var::LocVar, proto::Proto,
    t_string::tstring, temp_buffer::TempBuffer,
  },
  type_aliases::{
    instruction::Instruction, lua_state::lua_State, lua_table::LuaTable, t_value::TValue,
  },
};

const LBC_CONSTANT_NIL_U8: u8 = LBC_CONSTANT_NIL.0 as u8;
const LBC_CONSTANT_BOOLEAN_U8: u8 = LBC_CONSTANT_BOOLEAN.0 as u8;
const LBC_CONSTANT_NUMBER_U8: u8 = LBC_CONSTANT_NUMBER.0 as u8;
const LBC_CONSTANT_STRING_U8: u8 = LBC_CONSTANT_STRING.0 as u8;
const LBC_CONSTANT_IMPORT_U8: u8 = LBC_CONSTANT_IMPORT.0 as u8;
const LBC_CONSTANT_TABLE_U8: u8 = LBC_CONSTANT_TABLE.0 as u8;
const LBC_CONSTANT_CLOSURE_U8: u8 = LBC_CONSTANT_CLOSURE.0 as u8;
const LBC_CONSTANT_VECTOR_U8: u8 = LBC_CONSTANT_VECTOR.0 as u8;
const LBC_CONSTANT_TABLE_WITH_CONSTANTS_U8: u8 = LBC_CONSTANT_TABLE_WITH_CONSTANTS.0 as u8;
const LBC_CONSTANT_INTEGER_U8: u8 = LBC_CONSTANT_INTEGER.0 as u8;
const LBC_CONSTANT_CLASS_SHAPE_U8: u8 = LBC_CONSTANT_CLASS_SHAPE.0 as u8;
const USERDATA_TYPE_LIMIT: usize =
  (LBC_TYPE_TAGGED_USERDATA_END.0 - LBC_TYPE_TAGGED_USERDATA_BASE.0) as usize;

pub(crate) unsafe fn loadsafe(
  l: *mut lua_State,
  strings: &mut TempBuffer<*mut tstring>,
  protos: &mut TempBuffer<*mut Proto>,
  chunkname: *const c_char,
  data: *const c_char,
  size: usize,
  env: c_int,
) -> c_int {
  unsafe {
    let mut offset: usize = 0;

    let version: u8 = read(data, size, &mut offset);

    // 0 means the rest of the bytecode is the error message
    if version == 0 {
      let mut chunkbuf = [0 as c_char; LUA_IDSIZE as usize];
      let chunkid = lua_o_chunkid(
        chunkbuf.as_mut_ptr(),
        chunkbuf.len(),
        chunkname,
        c_strlen(chunkname),
      );
      push_chunk_prefixed_slice(l, chunkid, data.add(offset), size - offset);
      return 1;
    }

    if version < LBC_VERSION_MIN.0 as u8 || version > LBC_VERSION_MAX.0 as u8 {
      let mut chunkbuf = [0 as c_char; LUA_IDSIZE as usize];
      let chunkid = lua_o_chunkid(
        chunkbuf.as_mut_ptr(),
        chunkbuf.len(),
        chunkname,
        c_strlen(chunkname),
      );
      let message = format!(
        "{}: bytecode version mismatch (expected [{}..{}], got {})",
        c_string_lossy(chunkid),
        LBC_VERSION_MIN.0,
        LBC_VERSION_MAX.0,
        version
      );
      push_rust_string(l, &message);
      return 1;
    }

    let mut typesversion: u8 = 0;

    if version >= 4 {
      typesversion = read(data, size, &mut offset);

      if typesversion < LBC_TYPE_VERSION_MIN.0 as u8 || typesversion > LBC_TYPE_VERSION_MAX.0 as u8
      {
        let mut chunkbuf = [0 as c_char; LUA_IDSIZE as usize];
        let chunkid = lua_o_chunkid(
          chunkbuf.as_mut_ptr(),
          chunkbuf.len(),
          chunkname,
          c_strlen(chunkname),
        );
        let message = format!(
          "{}: bytecode type version mismatch (expected [{}..{}], got {})",
          c_string_lossy(chunkid),
          LBC_TYPE_VERSION_MIN.0,
          LBC_TYPE_VERSION_MAX.0,
          typesversion
        );
        push_rust_string(l, &message);
        return 1;
      }
    }

    // env is 0 for current environment and a stack index otherwise
    let envt: *mut LuaTable = if env == 0 {
      (*l).gt
    } else {
      hvalue!(luaA_toobject(l, env))
    };

    let source: *mut tstring = luaS_new(l, chunkname);

    // string table
    let string_count = read_var_int(data, size, &mut offset);
    strings.allocate(l, string_count as usize);

    for slot in c_slice_mut(strings.data, string_count as usize) {
      let length = read_var_int(data, size, &mut offset);

      *slot = luaS_newlstr(l, data.add(offset), length as usize);
      offset += length as usize;
    }

    // userdata type remapping table
    // for unknown userdata types, the entry will remap to common 'userdata' type
    let mut userdata_remapping = [LBC_TYPE_USERDATA.0 as u8; USERDATA_TYPE_LIMIT];

    if typesversion == 3 {
      let mut index: u8 = read(data, size, &mut offset);

      while index != 0 {
        let name = read_string(strings, data, size, &mut offset);

        if ((index - 1) as usize) < USERDATA_TYPE_LIMIT
          && let Some(cb) = (*(*l).global).ecb.gettypemapping
        {
          userdata_remapping[(index - 1) as usize] = cb(l, getstr(name), (*name).len as usize);
        }

        index = read(data, size, &mut offset);
      }
    }

    // proto table
    let proto_count = read_var_int(data, size, &mut offset);
    protos.allocate(l, proto_count as usize);

    for i in 0..proto_count {
      let p = lua_f_newproto(l);
      (*p).source = source;
      (*p).bytecodeid = i as c_int;
      (*p).funid = if (*(*l).global).lastprotoid == 0 {
        0
      } else {
        let id = (*(*l).global).lastprotoid;
        (*(*l).global).lastprotoid = (*(*l).global).lastprotoid.wrapping_add(1);
        id
      };

      (*p).maxstacksize = read(data, size, &mut offset);
      (*p).numparams = read(data, size, &mut offset);
      (*p).nups = read(data, size, &mut offset);
      (*p).is_vararg = read(data, size, &mut offset);

      if version >= 4 {
        (*p).flags = read(data, size, &mut offset);

        if typesversion == 1 {
          let typesize = read_var_int(data, size, &mut offset);

          if typesize != 0 {
            let types = data.add(offset) as *mut u8;

            LUAU_ASSERT!(typesize == 2 + (*p).numparams as u32);
            LUAU_ASSERT!(*types.add(0) == LBC_TYPE_FUNCTION.0 as u8);
            LUAU_ASSERT!(*types.add(1) == (*p).numparams);

            // transform v1 into v2 format
            let headersize = if typesize > 127 { 4usize } else { 3usize };

            (*p).typeinfo = luaM_newarray!(l, headersize + typesize as usize, u8, (*p).hdr.memcat);
            (*p).sizetypeinfo = (headersize + typesize as usize) as c_int;

            if headersize == 4 {
              *(*p).typeinfo.add(0) = ((typesize & 127) | (1 << 7)) as u8;
              *(*p).typeinfo.add(1) = (typesize >> 7) as u8;
              *(*p).typeinfo.add(2) = 0;
              *(*p).typeinfo.add(3) = 0;
            } else {
              *(*p).typeinfo.add(0) = typesize as u8;
              *(*p).typeinfo.add(1) = 0;
              *(*p).typeinfo.add(2) = 0;
            }

            copy_nonoverlapping(types, (*p).typeinfo.add(headersize), typesize as usize);
          }

          offset += typesize as usize;
        } else if typesversion == 2 || typesversion == 3 {
          let typesize = read_var_int(data, size, &mut offset);

          if typesize != 0 {
            let types = data.add(offset) as *mut u8;

            (*p).typeinfo = luaM_newarray!(l, typesize as usize, u8, (*p).hdr.memcat);
            (*p).sizetypeinfo = typesize as c_int;
            copy_nonoverlapping(types, (*p).typeinfo, typesize as usize);
            offset += typesize as usize;

            if typesversion == 3 {
              remap_userdata_types(
                (*p).typeinfo as *mut c_char,
                (*p).sizetypeinfo as usize,
                userdata_remapping.as_mut_ptr(),
                USERDATA_TYPE_LIMIT as u32,
              );
            }
          }
        }
      }

      let sizecode = read_var_int(data, size, &mut offset) as c_int;
      (*p).code = luaM_newarray!(l, sizecode as usize, Instruction, (*p).hdr.memcat);
      (*p).sizecode = sizecode;

      for code in c_slice_mut((*p).code, (*p).sizecode as usize) {
        *code = read::<u32>(data, size, &mut offset);
      }

      (*p).codeentry = (*p).code;

      let sizek = read_var_int(data, size, &mut offset) as c_int;
      (*p).k = luaM_newarray!(l, sizek as usize, TValue, (*p).hdr.memcat);
      (*p).sizek = sizek;

      // Initialize the constants to nil to ensure they have a valid state
      // in the event that some operation in the following loop fails with
      // an exception.
      // SAFETY：k 数组刚按 sizek 分配完成，全部元素可写。
      for k in c_slice_mut((*p).k, (*p).sizek as usize) {
        setnilvalue!(k);
      }

      for j in 0..(*p).sizek {
        let k = (*p).k.add(j as usize);

        match read::<u8>(data, size, &mut offset) {
          LBC_CONSTANT_NIL_U8 => {
            // All constants have already been pre-initialized to nil
          }

          LBC_CONSTANT_BOOLEAN_U8 => {
            let v: u8 = read(data, size, &mut offset);
            setbvalue!(k, v);
          }

          LBC_CONSTANT_NUMBER_U8 => {
            let v: f64 = read(data, size, &mut offset);
            setnvalue!(k, v);
          }

          LBC_CONSTANT_VECTOR_U8 => {
            let x: f32 = read(data, size, &mut offset);
            let y: f32 = read(data, size, &mut offset);
            let z: f32 = read(data, size, &mut offset);
            let w: f32 = read(data, size, &mut offset);
            setvvalue!(k, x, y, z, w);
          }

          LBC_CONSTANT_STRING_U8 => {
            let v = read_string(strings, data, size, &mut offset);
            setsvalue!(l, k, v);
          }

          LBC_CONSTANT_IMPORT_U8 => {
            let iid: u32 = read(data, size, &mut offset);
            resolve_import_safe(l, envt, (*p).k, iid);
            setobj!(l, k, (*l).top.sub(1));
            (*l).top = (*l).top.sub(1);
          }

          LBC_CONSTANT_TABLE_U8 => {
            let keys = read_var_int(data, size, &mut offset) as c_int;
            let h = lua_h_new(l, 0, keys);
            for _ in 0..keys {
              let key = read_var_int(data, size, &mut offset) as c_int;
              let val = luaH_set(l, h, (*p).k.add(key as usize) as *const TValue);
              setnvalue!(val, 0.0);
            }
            sethvalue!(l, k, h);
          }

          LBC_CONSTANT_TABLE_WITH_CONSTANTS_U8 => {
            let keys = read_var_int(data, size, &mut offset);
            let h = lua_h_new(l, 0, keys as c_int);

            let mut nil_keys: TempBuffer<i32> = TempBuffer::new();
            nil_keys.allocate(l, keys as usize);
            let mut nil_keys_size: usize = 0;

            for _ in 0..keys {
              let key = read_var_int(data, size, &mut offset) as i32;
              let val = luaH_set(l, h, (*p).k.add(key as usize) as *const TValue);
              let constant_idx: i32 = read(data, size, &mut offset);
              if constant_idx >= 0 {
                let constant = (*p).k.add(constant_idx as usize);
                if ttisnil!(constant) {
                  *nil_keys.data.add(nil_keys_size) = key;
                  nil_keys_size += 1;
                } else {
                  setobj2t!(l, val, constant);
                  luaC_barriert!(l, h, constant);
                  continue;
                }
              }
              setnvalue!(val, 0.0);
            }

            for key in c_slice_mut(nil_keys.data, nil_keys_size) {
              let val = luaH_set(l, h, (*p).k.add(*key as usize) as *const TValue);
              setnilvalue!(val);
            }

            sethvalue!(l, k, h);
          }

          LBC_CONSTANT_CLOSURE_U8 => {
            let fid = read_var_int(data, size, &mut offset);
            let proto = *protos.data.add(fid as usize);
            let cl = lua_f_new_lclosure(l, (*proto).nups as c_int, envt, proto);
            (*cl).preload = if (*cl).nupvalues > 0 { 1 } else { 0 };
            setclvalue!(l, k, cl);
          }

          LBC_CONSTANT_CLASS_SHAPE_U8 => {
            let cnid = read_var_int(data, size, &mut offset);
            let classname = (*p).k.add(cnid as usize);
            LUAU_ASSERT!(ttisstring!(classname));
            let num_properties = read_var_int(data, size, &mut offset);
            let num_methods = read_var_int(data, size, &mut offset);
            let num_members = num_methods + num_properties;
            let offset_to_member =
              luaM_newarray!(l, num_members as usize, *mut tstring, (*l).activememcat);
            let members_to_offset = lua_h_new(l, 0, num_members as c_int);

            for (idx, slot) in c_slice_mut(offset_to_member, num_members as usize)
              .iter_mut()
              .enumerate()
            {
              let mid = read_var_int(data, size, &mut offset);
              let member_name = (*p).k.add(mid as usize);
              LUAU_ASSERT!(ttisstring!(member_name));
              *slot = tsvalue!(member_name) as *mut tstring;
              let val = lua_h_setstr(l, members_to_offset, tsvalue!(member_name) as *mut tstring);
              setnvalue!(val, idx as f64);
            }

            (*members_to_offset).readonly = 1;

            let lco = lua_r_newclass(
              l,
              tsvalue!(classname) as *mut tstring,
              members_to_offset,
              offset_to_member,
              num_properties as c_int,
              num_methods as c_int,
            );
            setclassvalue!(l, k, lco);
          }

          LBC_CONSTANT_INTEGER_U8 => {
            let is_negative: u8 = read(data, size, &mut offset);
            let magnitude = read_var_int_64(data, size, &mut offset);
            let value = if is_negative != 0 {
              (!magnitude).wrapping_add(1) as i64
            } else {
              magnitude as i64
            };
            setlvalue!(k, value);
          }

          _ => {
            LUAU_ASSERT!(false);
          }
        }
      }

      if FFlag::LuauUdataDirectAccess6.get() {
        let mut instruction = (*p).code;
        let end = (*p).code.add((*p).sizecode as usize);

        while instruction < end {
          let mut target_op = -1i32;

          match LuauOpcode::from(LUAU_INSN_OP(*instruction) as u8) {
            LuauOpcode::LOP_GETTABLEKS => {
              target_op = LuauOpcode::LOP_GETUDATAKS as u8 as i32;
            }

            LuauOpcode::LOP_SETTABLEKS => {
              target_op = LuauOpcode::LOP_SETUDATAKS as u8 as i32;
            }

            LuauOpcode::LOP_NAMECALL => {
              target_op = LuauOpcode::LOP_NAMECALLUDATA as u8 as i32;
            }

            _ => {}
          }

          if target_op != -1 {
            LUAU_ASSERT!(*instruction.add(1) < sizek as u32);

            // We take over the upper 16 bits of AUX - so no constants with big indices.
            if *instruction.add(1) < 0x10000 {
              let k = (*p).k.add(*instruction.add(1) as usize);
              let s = tsvalue!(k) as *mut tstring;

              luaS_updateatom!(l, s);

              if (*s).atom >= 0 {
                *instruction = (*instruction & 0xffffff00) | target_op as u32;
              }
            }
          }

          instruction = instruction
            .add(getOpLength(LuauOpcode::from(LUAU_INSN_OP(*instruction) as u8)) as usize);
        }
      }

      let sizep = read_var_int(data, size, &mut offset) as c_int;
      (*p).p = luaM_newarray!(l, sizep as usize, *mut Proto, (*p).hdr.memcat);
      (*p).sizep = sizep;

      for slot in c_slice_mut((*p).p, (*p).sizep as usize) {
        let fid = read_var_int(data, size, &mut offset);
        *slot = *protos.data.add(fid as usize);
      }

      (*p).linedefined = read_var_int(data, size, &mut offset) as c_int;
      (*p).debugname = read_string(strings, data, size, &mut offset);

      let lineinfo: u8 = read(data, size, &mut offset);

      if lineinfo != 0 {
        (*p).linegaplog2 = read::<u8>(data, size, &mut offset) as c_int;

        let intervals = (((*p).sizecode - 1) >> (*p).linegaplog2) + 1;
        let absoffset = ((*p).sizecode + 3) & !3;

        let sizelineinfo = absoffset + intervals * size_of::<c_int>() as c_int;
        (*p).lineinfo = luaM_newarray!(l, sizelineinfo as usize, u8, (*p).hdr.memcat);
        (*p).sizelineinfo = sizelineinfo;

        (*p).abslineinfo = (*p).lineinfo.add(absoffset as usize) as *mut c_int;

        let mut lastoffset: u8 = 0;
        for line in c_slice_mut((*p).lineinfo, (*p).sizecode as usize) {
          lastoffset = lastoffset.wrapping_add(read::<u8>(data, size, &mut offset));
          *line = lastoffset;
        }

        let mut lastline: i32 = 0;
        for line in c_slice_mut((*p).abslineinfo, intervals as usize) {
          lastline = lastline.wrapping_add(read::<i32>(data, size, &mut offset));
          *line = lastline;
        }
      }

      let debuginfo: u8 = read(data, size, &mut offset);

      if debuginfo != 0 {
        let sizelocvars = read_var_int(data, size, &mut offset) as c_int;
        (*p).locvars = luaM_newarray!(l, sizelocvars as usize, LocVar, (*p).hdr.memcat);
        (*p).sizelocvars = sizelocvars;

        for locvar in c_slice_mut((*p).locvars, (*p).sizelocvars as usize) {
          locvar.varname = read_string(strings, data, size, &mut offset);
          locvar.startpc = read_var_int(data, size, &mut offset) as c_int;
          locvar.endpc = read_var_int(data, size, &mut offset) as c_int;
          locvar.reg = read(data, size, &mut offset);
        }

        let sizeupvalues = read_var_int(data, size, &mut offset) as c_int;
        LUAU_ASSERT!(sizeupvalues == (*p).nups as c_int);

        (*p).upvalues = luaM_newarray!(l, sizeupvalues as usize, *mut tstring, (*p).hdr.memcat);
        (*p).sizeupvalues = sizeupvalues;

        for uv in c_slice_mut((*p).upvalues, (*p).sizeupvalues as usize) {
          *uv = read_string(strings, data, size, &mut offset);
        }
      }

      if version >= 11 {
        // cpp 对 version >= 11 无条件读 feedbackvec（字节码格式 v11 自带）。
        (*p).feedbackvecsize = read_var_int(data, size, &mut offset);

        if (*p).feedbackvecsize > 0 {
          (*p).feedbackvec = luaM_newarray!(
            l,
            (*p).feedbackvecsize as usize,
            FeedbackVectorSlot,
            (*p).hdr.memcat
          );
        }
        for slot in c_slice_mut((*p).feedbackvec, (*p).feedbackvecsize as usize) {
          let slottype: u8 = read(data, size, &mut offset);
          LUAU_ASSERT!(slottype == LuauFeedbackType::LFT_CALLTARGET as u8);
          slot.kind = FeedbackVectorSlotKind::CallTarget;
          slot.data.call_target.pc = read_var_int(data, size, &mut offset);
          slot.data.call_target.proto = 0;
          slot.data.call_target.hits = 0;
        }
      }

      *protos.data.add(i as usize) = p;
    }

    // "main" proto is pushed to Lua stack
    let mainid = read_var_int(data, size, &mut offset);
    let main = *protos.data.add(mainid as usize);

    let thread_obj = l as *mut GCObject;
    if isblack!(thread_obj) {
      lua_c_barrierback(l, thread_obj, &mut (*l).gclist);
    }

    let cl = lua_f_new_lclosure(l, 0, envt, main);
    setclvalue!(l, (*l).top, cl);
    incr_top!(l);

    0
  }
}

unsafe fn c_strlen(s: *const c_char) -> usize {
  // CStr::from_ptr 语义与手写循环一致：读到首个 NUL
  unsafe { CStr::from_ptr(s).to_bytes().len() }
}

unsafe fn c_string_lossy(s: *const c_char) -> String {
  // SAFETY：s 为 NUL 结尾的 C 字符串，c_strlen 给出精确可读长度。
  unsafe { String::from_utf8_lossy(from_raw_parts(s as *const u8, c_strlen(s))).into_owned() }
}

unsafe fn push_chunk_prefixed_slice(
  l: *mut lua_State,
  chunkid: *const c_char,
  bytes: *const c_char,
  len: usize,
) {
  unsafe {
    let prefix = from_raw_parts(chunkid as *const u8, c_strlen(chunkid));
    let payload = from_raw_parts(bytes as *const u8, len);
    let mut message = Vec::with_capacity(prefix.len() + payload.len());
    message.extend_from_slice(prefix);
    message.extend_from_slice(payload);
    lua_pushlstring(l, message.as_ptr() as *const c_char, message.len());
  }
}

unsafe fn push_rust_string(l: *mut lua_State, message: &str) {
  unsafe {
    lua_pushlstring(l, message.as_ptr() as *const c_char, message.len());
  }
}
