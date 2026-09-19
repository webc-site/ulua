use alloc::fmt;
use core::{
  ffi::{CStr, c_char, c_int},
  mem::size_of,
  ptr::{copy_nonoverlapping, null_mut},
  slice::from_raw_parts,
};

use ulua_common::{
  enums::{
    luau_bytecode_tag::{
      LBC_CONSTANT_BOOLEAN, LBC_CONSTANT_CLASS_SHAPE, LBC_CONSTANT_CLOSURE, LBC_CONSTANT_IMPORT,
      LBC_CONSTANT_INTEGER, LBC_CONSTANT_NIL, LBC_CONSTANT_NUMBER, LBC_CONSTANT_STRING,
      LBC_CONSTANT_TABLE, LBC_CONSTANT_TABLE_WITH_CONSTANTS, LBC_CONSTANT_VECTOR,
      LBC_TYPE_VERSION_MAX, LBC_TYPE_VERSION_MIN, LBC_VERSION_CLASSES, LBC_VERSION_MAX,
      LBC_VERSION_MIN,
    },
    luau_bytecode_type::{
      LBC_TYPE_FUNCTION, LBC_TYPE_TAGGED_USERDATA_BASE, LBC_TYPE_TAGGED_USERDATA_END,
      LBC_TYPE_USERDATA,
    },
    luau_feedback_type::LuauFeedbackType,
    luau_opcode::LuauOpcode,
    luau_proto_flag::LuauProtoFlag,
  },
  fflag,
  functions::get_op_length::get_op_length,
  macros::{luau_assert::LUAU_ASSERT, luau_insn_op::luau_insn_op},
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

/// 首个带 typesversion 字段的字节码版本（cpp lvmload.cpp:311）
const VERSION_WITH_TYPES: u8 = 4;
/// 首个自带 feedbackvec 的字节码版本（cpp lvmload.cpp:747）
const VERSION_WITH_FEEDBACK: u8 = 11;
/// 首个每个 proto 带前缀 protoSize 的字节码版本（cpp lvmload.cpp:375）
const VERSION_WITH_PROTO_SIZE: u8 = 12;

/// 不可信数据（id / 长度 / count）越界时的统一收口。
///
/// cpp 侧只有 `LUAU_ASSERT`，release 编译掉后就是越界读；Rust 侧必须转成
/// 「损坏字节码」错误：push 错误字符串并返回 1，与版本不匹配三处同形态。
macro_rules! malformed {
  ($l:expr, $chunkname:expr, $($detail:tt)+) => {
    return push_chunk_error($l, $chunkname, format_args!($($detail)+))
  };
}

/// `[offset, offset + len)` 是否完整落在 blob 内（不可信长度/count 的硬校验，
/// `checked_add` 兼顾 `offset + len` 自身溢出）。
fn fits(offset: usize, len: usize, size: usize) -> bool {
  offset.checked_add(len).is_some_and(|end| end <= size)
}

/// 不可信 count 的硬校验：`u32 as c_int` 可以是负数，且 blob 尾部必须容得下
/// 每项至少 `item_bytes` 字节，否则随后的 `c_slice_mut`/分配就是越界构造。
fn count_fits(count: i64, item_bytes: usize, size: usize, offset: usize) -> bool {
  let remaining = size.saturating_sub(offset);

  count >= 0
    && (count as usize)
      .checked_mul(item_bytes)
      .is_some_and(|need| need <= remaining)
}

/// 取已填充前缀内的 proto。
///
/// `loaded` 为已写入的槽数：合法字节码里被引用的 proto 总先于引用点写入
/// （cpp lvmload.cpp 的 proto 表是自内而后的后序），因此越界或槽仍为 `null`
/// 都属损坏字节码，绝不能把裸 `protos[fid]` 的越界读/未初始化读带进来。
fn proto_at(protos: &TempBuffer<*mut Proto>, id: u32, loaded: usize) -> Option<*mut Proto> {
  let index = id as usize;

  if index >= loaded.min(protos.count) {
    return None;
  }

  // 上面已硬校验过边界，Index 内的 LUAU_ASSERT 恒真
  let proto = protos[index];

  (!proto.is_null()).then_some(proto)
}

/// 常量表索引：cpp 直接取 `&p->k[key]`，而 `key` 来自不可信字节码，
/// 负值或越界都会跳出数组，故统一在此判定。
unsafe fn constant_at(p: *mut Proto, id: i64) -> Option<*mut TValue> {
  let sizek = unsafe { (*p).sizek };
  let k = unsafe { (*p).k };

  ((0..sizek as i64).contains(&id)).then(|| unsafe { k.add(id as usize) })
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
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
      let chunkid = chunkid_buf(&mut chunkbuf, chunkname);
      push_chunk_prefixed_slice(l, chunkid, data.add(offset), size - offset);
      return 1;
    }

    // cpp: `cpp/VM/src/lvmload.cpp:301` —
    // `(version < MIN || version > MAX) && version != LBC_VERSION_CLASSES`
    if (version < LBC_VERSION_MIN.0 as u8 || version > LBC_VERSION_MAX.0 as u8)
      && version != LBC_VERSION_CLASSES.0 as u8
    {
      return push_chunk_error(
        l,
        chunkname,
        format_args!(
          "bytecode version mismatch (expected [{}..{}], got {})",
          LBC_VERSION_MIN.0, LBC_VERSION_MAX.0, version
        ),
      );
    }

    let mut typesversion: u8 = 0;

    if version >= VERSION_WITH_TYPES {
      typesversion = read(data, size, &mut offset);

      if typesversion < LBC_TYPE_VERSION_MIN.0 as u8 || typesversion > LBC_TYPE_VERSION_MAX.0 as u8
      {
        return push_chunk_error(
          l,
          chunkname,
          format_args!(
            "bytecode type version mismatch (expected [{}..{}], got {})",
            LBC_TYPE_VERSION_MIN.0, LBC_TYPE_VERSION_MAX.0, typesversion
          ),
        );
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

      // 字符串体是 blob 内原字节，长度来自不可信数据：必须硬校验后再取
      if !fits(offset, length as usize, size) {
        malformed!(l, chunkname, "bytecode string table is truncated");
      }

      *slot = luaS_newlstr(l, data.add(offset), length as usize);
      offset += length as usize;
    }

    // userdata type remapping table
    // for unknown userdata types, the entry will remap to common 'userdata' type
    let mut userdata_remapping = [LBC_TYPE_USERDATA.0 as u8; USERDATA_TYPE_LIMIT];

    if typesversion == 3 {
      let mut index: u8 = read(data, size, &mut offset);

      while index != 0 {
        let Some(name) = read_string(strings, data, size, &mut offset) else {
          malformed!(l, chunkname, "bytecode userdata type name id is out of range");
        };

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

    // cpp 的 TempBuffer 槽是裸内存，`protos[fid]` 读到未写入槽只是垃圾指针；
    // Rust 侧读到未初始化内存本身就是 UB，故先填 null，由 proto_at 据此判定损坏
    for slot in c_slice_mut(protos.data, proto_count as usize) {
      *slot = null_mut();
    }

    for (i, slot) in c_slice_mut(protos.data, proto_count as usize)
      .iter_mut()
      .enumerate()
    {
      // cpp: `cpp/VM/src/lvmload.cpp:374-377` —
      // version >= VERSION_WITH_PROTO_SIZE 时每个 proto 前带 protoSize，末尾按起点跳过未知数据
      let proto_size = if version >= VERSION_WITH_PROTO_SIZE {
        read_var_int(data, size, &mut offset)
      } else {
        0
      };
      let proto_start_offset = offset;

      // protoSize 决定下次读取起点，越界会让后续所有偏移都跑出 blob
      if !fits(proto_start_offset, proto_size as usize, size) {
        malformed!(l, chunkname, "bytecode proto size is out of range");
      }

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

      if version >= VERSION_WITH_TYPES {
        (*p).flags = read(data, size, &mut offset);

        if typesversion == 1 {
          let typesize = read_var_int(data, size, &mut offset);

          if typesize != 0 {
            // v1 头部断言会读 types[0..2]，故按 max(2) 收紧校验
            if !fits(offset, (typesize as usize).max(2), size) {
              malformed!(l, chunkname, "bytecode type info is truncated");
            }

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
            if !fits(offset, typesize as usize, size) {
              malformed!(l, chunkname, "bytecode type info is truncated");
            }

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

      // 指令定长 4 字节且逐项从 blob 读出，count 与剩余长度必须自洽
      if !count_fits(i64::from(sizecode), size_of::<Instruction>(), size, offset) {
        malformed!(l, chunkname, "bytecode code size is out of range");
      }

      (*p).code = luaM_newarray!(l, sizecode as usize, Instruction, (*p).hdr.memcat);
      (*p).sizecode = sizecode;

      for code in c_slice_mut((*p).code, (*p).sizecode as usize) {
        *code = read::<u32>(data, size, &mut offset);
      }

      (*p).codeentry = (*p).code;

      let sizek = read_var_int(data, size, &mut offset) as c_int;

      // 每个常量至少占 1 字节（tag），据此卡住虚高的 count
      if !count_fits(i64::from(sizek), 1, size, offset) {
        malformed!(l, chunkname, "bytecode constant count is out of range");
      }

      (*p).k = luaM_newarray!(l, sizek as usize, TValue, (*p).hdr.memcat);
      (*p).sizek = sizek;

      // Initialize the constants to nil to ensure they have a valid state
      // in the event that some operation in the following loop fails with
      // an exception.
      // SAFETY：k 数组刚按 sizek 分配完成，全部元素可写。
      for k in c_slice_mut((*p).k, (*p).sizek as usize) {
        setnilvalue!(k);
      }

      // SAFETY：k 数组刚按 sizek 分配完成，全部元素可写。
      for k in c_slice_mut((*p).k, (*p).sizek as usize) {
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
            let Some(v) = read_string(strings, data, size, &mut offset) else {
              malformed!(l, chunkname, "bytecode string constant id is out of range");
            };
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
              let Some(kslot) = constant_at(p, key as i64) else {
                malformed!(l, chunkname, "bytecode table key index is out of range");
              };
              let val = luaH_set(l, h, kslot as *const TValue);
              setnvalue!(val, 0.0);
            }
            sethvalue!(l, k, h);
          }

          LBC_CONSTANT_TABLE_WITH_CONSTANTS_U8 => {
            let keys = read_var_int(data, size, &mut offset);
            let h = lua_h_new(l, 0, keys as c_int);

            // 每个键值对至少占 5 字节（key varint + i32 constantIdx）
            if !count_fits(i64::from(keys), 5, size, offset) {
              malformed!(l, chunkname, "bytecode table key count is out of range");
            }

            let mut nil_keys: TempBuffer<i32> = TempBuffer::new();
            nil_keys.allocate(l, keys as usize);
            let mut nil_keys_size: usize = 0;

            for _ in 0..keys {
              let key = read_var_int(data, size, &mut offset) as i32;
              let Some(kslot) = constant_at(p, key as i64) else {
                malformed!(l, chunkname, "bytecode table key index is out of range");
              };
              let val = luaH_set(l, h, kslot as *const TValue);
              let constant_idx: i32 = read(data, size, &mut offset);
              if let Some(constant) = constant_at(p, constant_idx as i64) {
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

            // key 已经过 constant_at 校验，回写 nil 时同一判定必然成立
            for key in c_slice_mut(nil_keys.data, nil_keys_size) {
              let Some(kslot) = constant_at(p, *key as i64) else {
                malformed!(l, chunkname, "bytecode table key index is out of range");
              };
              let val = luaH_set(l, h, kslot as *const TValue);
              setnilvalue!(val);
            }

            sethvalue!(l, k, h);
          }

          LBC_CONSTANT_CLOSURE_U8 => {
            let fid = read_var_int(data, size, &mut offset);
            // 闭包只允许引用本轮之前已装载完的 proto（槽 0..i）
            let Some(proto) = proto_at(protos, fid, i) else {
              malformed!(l, chunkname, "bytecode closure proto id is out of range");
            };
            let cl = lua_f_new_lclosure(l, (*proto).nups as c_int, envt, proto);
            (*cl).preload = if (*cl).nupvalues > 0 { 1 } else { 0 };
            setclvalue!(l, k, cl);
          }

          LBC_CONSTANT_CLASS_SHAPE_U8 => {
            let cnid = read_var_int(data, size, &mut offset);
            let Some(classname) = constant_at(p, i64::from(cnid)) else {
              malformed!(l, chunkname, "bytecode class name index is out of range");
            };
            LUAU_ASSERT!(ttisstring!(classname));
            let num_properties = read_var_int(data, size, &mut offset);
            let num_methods = read_var_int(data, size, &mut offset);
            let num_members = num_methods.wrapping_add(num_properties);
            // 每个成员至少占 1 字节（mid varint）
            if !count_fits(i64::from(num_members), 1, size, offset) {
              malformed!(l, chunkname, "bytecode class member count is out of range");
            }
            let offset_to_member =
              luaM_newarray!(l, num_members as usize, *mut tstring, (*l).activememcat);
            let members_to_offset = lua_h_new(l, 0, num_members as c_int);

            for (idx, slot) in c_slice_mut(offset_to_member, num_members as usize)
              .iter_mut()
              .enumerate()
            {
              let mid = read_var_int(data, size, &mut offset);
              let Some(member_name) = constant_at(p, i64::from(mid)) else {
                malformed!(l, chunkname, "bytecode class member index is out of range");
              };
              LUAU_ASSERT!(ttisstring!(member_name));
              let member = tsvalue!(member_name) as *mut tstring;
              *slot = member;
              let val = lua_h_setstr(l, members_to_offset, member);
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

      if fflag::LuauUdataDirectAccess6.get() {
        let mut instruction = (*p).code;
        let end = (*p).code.add((*p).sizecode as usize);

        while instruction < end {
          let mut target_op = -1i32;

          match LuauOpcode::from(luau_insn_op(*instruction) as u8) {
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
            // AUX 取下一条指令字：损坏流里它可能已经越过 code 末尾
            if instruction.add(1) >= end {
              malformed!(l, chunkname, "bytecode instruction aux is out of range");
            }

            let aux = *instruction.add(1);
            LUAU_ASSERT!(aux < sizek as u32);

            // We take over the upper 16 bits of AUX - so no constants with big indices.
            if aux < 0x10000 {
              let Some(k) = constant_at(p, i64::from(aux)) else {
                malformed!(l, chunkname, "bytecode direct access constant index is out of range");
              };
              let s = tsvalue!(k) as *mut tstring;

              luaS_updateatom!(l, s);

              if (*s).atom >= 0 {
                *instruction = (*instruction & 0xffffff00) | target_op as u32;
              }
            }
          }

          instruction = instruction
            .add(get_op_length(LuauOpcode::from(luau_insn_op(*instruction) as u8)) as usize);
        }
      }

      let sizep = read_var_int(data, size, &mut offset) as c_int;
      // 每个 fid 至少占 1 字节
      if !count_fits(i64::from(sizep), 1, size, offset) {
        malformed!(l, chunkname, "bytecode proto count is out of range");
      }

      (*p).p = luaM_newarray!(l, sizep as usize, *mut Proto, (*p).hdr.memcat);
      (*p).sizep = sizep;

      for slot in c_slice_mut((*p).p, (*p).sizep as usize) {
        let fid = read_var_int(data, size, &mut offset);
        // 内层 proto 同样只能引用已装载完的槽（0..i）
        let Some(proto) = proto_at(protos, fid, i) else {
          malformed!(l, chunkname, "bytecode inner proto id is out of range");
        };
        *slot = proto;
      }

      (*p).linedefined = read_var_int(data, size, &mut offset) as c_int;
      let Some(debugname) = read_string(strings, data, size, &mut offset) else {
        malformed!(l, chunkname, "bytecode debug name string id is out of range");
      };
      (*p).debugname = debugname;

      let lineinfo: u8 = read(data, size, &mut offset);

      if lineinfo != 0 {
        (*p).linegaplog2 = read::<u8>(data, size, &mut offset) as c_int;

        // 移位量来自不可信字节码：超过 c_int 位宽即 UB/panic
        if (*p).linegaplog2 < 0 || (*p).linegaplog2 >= size_of::<c_int>() as c_int * 8 {
          malformed!(l, chunkname, "bytecode line gap log2 is out of range");
        }

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
        // 每个 LocVar 至少占 4 字节（varname/startpc/endpc 各 1 + reg 1）
        if !count_fits(i64::from(sizelocvars), 4, size, offset) {
          malformed!(l, chunkname, "bytecode local variable count is out of range");
        }

        (*p).locvars = luaM_newarray!(l, sizelocvars as usize, LocVar, (*p).hdr.memcat);
        (*p).sizelocvars = sizelocvars;

        for locvar in c_slice_mut((*p).locvars, (*p).sizelocvars as usize) {
          let Some(varname) = read_string(strings, data, size, &mut offset) else {
            malformed!(l, chunkname, "bytecode local variable name id is out of range");
          };
          locvar.varname = varname;
          locvar.startpc = read_var_int(data, size, &mut offset) as c_int;
          locvar.endpc = read_var_int(data, size, &mut offset) as c_int;
          locvar.reg = read(data, size, &mut offset);
        }

        let sizeupvalues = read_var_int(data, size, &mut offset) as c_int;
        LUAU_ASSERT!(sizeupvalues == (*p).nups as c_int);

        // 每个 upvalue 名至少占 1 字节（varint id）
        if !count_fits(i64::from(sizeupvalues), 1, size, offset) {
          malformed!(l, chunkname, "bytecode upvalue count is out of range");
        }

        (*p).upvalues = luaM_newarray!(l, sizeupvalues as usize, *mut tstring, (*p).hdr.memcat);
        (*p).sizeupvalues = sizeupvalues;

        for uv in c_slice_mut((*p).upvalues, (*p).sizeupvalues as usize) {
          let Some(name) = read_string(strings, data, size, &mut offset) else {
            malformed!(l, chunkname, "bytecode upvalue name id is out of range");
          };
          *uv = name;
        }
      }

      if version >= VERSION_WITH_FEEDBACK {
        // cpp 对 version >= 11 无条件读 feedbackvec（字节码格式 v11 自带）。
        (*p).feedbackvecsize = read_var_int(data, size, &mut offset);

        // 每个 feedback 槽至少占 2 字节（slottype + pc varint）
        if !count_fits(i64::from((*p).feedbackvecsize), 2, size, offset) {
          malformed!(l, chunkname, "bytecode feedback slot count is out of range");
        }

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

      // cpp: `cpp/VM/src/lvmload.cpp:767-777` —
      // version >= 12 时读 inlinable cost 并跳过 proto 末尾的未知数据
      if version >= VERSION_WITH_PROTO_SIZE
        && ((*p).flags & LuauProtoFlag::LPF_INLINABLE as u8) != 0
      {
        (*p).cost = read_var_int_64(data, size, &mut offset);
      }

      if version >= VERSION_WITH_PROTO_SIZE {
        offset = proto_start_offset + proto_size as usize;
      }

      *slot = p;
    }

    // "main" proto is pushed to Lua stack
    let mainid = read_var_int(data, size, &mut offset);
    let Some(main) = proto_at(protos, mainid, protos.count) else {
      malformed!(l, chunkname, "bytecode main proto id is out of range");
    };

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

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
/// chunkname 经 lua_o_chunkid 截入 `buf`（cpp lvmload 同款三处）。
/// 缓冲必须由调用方持有：若在函数内创建数组再连指针一起返回，
/// move 会把数组拷到新栈址，返回的指针随即悬垂。
/// 拼 chunkid 前缀 + 错误消息并 push 到栈上（cpp 三处版本/类型版本错误的
/// 共用形态），返回 1 表示已产出错误字符串。
unsafe fn push_chunk_error(
  l: *mut lua_State,
  chunkname: *const c_char,
  detail: fmt::Arguments,
) -> c_int {
  unsafe {
    let mut chunkbuf = [0 as c_char; LUA_IDSIZE as usize];
    let chunkid = chunkid_buf(&mut chunkbuf, chunkname);
    let message = alloc::format!("{}: {}", c_string_lossy(chunkid), detail);
    push_rust_string(l, &message);
    1
  }
}

unsafe fn chunkid_buf(
  buf: &mut [c_char; LUA_IDSIZE as usize],
  chunkname: *const c_char,
) -> *const c_char {
  let n = unsafe { CStr::from_ptr(chunkname) }.to_bytes().len();
  unsafe { lua_o_chunkid(buf.as_mut_ptr(), buf.len(), chunkname, n) }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
unsafe fn c_string_lossy(s: *const c_char) -> String {
  // SAFETY：s 为 NUL 结尾的 C 字符串，CStr 求长与 cpp strlen 语义一致。
  unsafe { CStr::from_ptr(s) }.to_string_lossy().into_owned()
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
unsafe fn push_chunk_prefixed_slice(
  l: *mut lua_State,
  chunkid: *const c_char,
  bytes: *const c_char,
  len: usize,
) {
  unsafe {
    let prefix = CStr::from_ptr(chunkid).to_bytes();
    let payload = from_raw_parts(bytes as *const u8, len);
    let mut message = Vec::with_capacity(prefix.len() + payload.len());
    message.extend_from_slice(prefix);
    message.extend_from_slice(payload);
    lua_pushlstring(l, message.as_ptr() as *const c_char, message.len());
  }
}

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
unsafe fn push_rust_string(l: *mut lua_State, message: &str) {
  unsafe {
    lua_pushlstring(l, message.as_ptr() as *const c_char, message.len());
  }
}
