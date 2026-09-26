use alloc::fmt;
use core::{ffi::c_char, mem::size_of, ptr::from_ref};

use memchr::memchr;
use ulua_common::{
  enums::{
    luau_bytecode_tag::LuauBytecodeTag, luau_bytecode_type::LuauBytecodeType,
    luau_feedback_type::LuauFeedbackType, luau_opcode::LuauOpcode, luau_proto_flag::LuauProtoFlag,
  },
  fflag,
  functions::{get_op_length::get_op_length, read_var_int_64::try_read_var_int_64},
  macros::luau_insn_op::luau_insn_op,
};

use crate::{
  enums::{feedback_vector_slot_kind::FeedbackVectorSlotKind, value_view::ValueView},
  functions::{
    c_slice_mut, cstr_bytes, fits,
    lua_a_toobject::lua_a_toobject,
    lua_c_barrierback::lua_c_barrierback,
    lua_f_new_lclosure::lua_f_new_lclosure,
    lua_f_newproto::lua_f_newproto,
    lua_h_new::lua_h_new,
    lua_h_set::lua_h_set,
    lua_h_setstr::lua_h_setstr,
    lua_o_chunkid::lua_o_chunkid,
    lua_pushlstring::lua_pushlstring,
    lua_r_newclass::lua_r_newclass,
    lua_s_newlstr::lua_s_newlstr,
    read::read,
    read_string::{ReadStringError, read_string},
    read_var_int::read_var_int,
    remap_userdata_types::remap_userdata_types,
    resolve_import_safe::resolve_import_safe,
  },
  macros::{
    getstr::getstr, incr_top::incr_top, isblack::isblack, lua_c_barriert::luaC_barriert,
    lua_idsize::LUA_IDSIZE, lua_m_newarray::luaM_newarray, lua_s_updateatom::lua_s_updateatom,
    setbvalue::setbvalue, setclassvalue::setclassvalue, setclvalue::setclvalue,
    sethvalue::sethvalue, setlvalue::setlvalue, setnilvalue::setnilvalue, setnvalue::setnvalue,
    setobj::setobj, setobj_2_t::setobj2t, setsvalue::setsvalue, setvvalue::setvvalue,
  },
  records::{
    feedback_vector_slot::FeedbackVectorSlot, gc_object::GCObject, loc_var::LocVar,
    lua_state::LuaState, proto::Proto, t_string::tstring, temp_buffer::TempBuffer,
  },
  type_aliases::{instruction::Instruction, t_value::TValue},
};

const LBC_CONSTANT_NIL_U8: u8 = LuauBytecodeTag::LBC_CONSTANT_NIL.0 as u8;
const LBC_CONSTANT_BOOLEAN_U8: u8 = LuauBytecodeTag::LBC_CONSTANT_BOOLEAN.0 as u8;
const LBC_CONSTANT_NUMBER_U8: u8 = LuauBytecodeTag::LBC_CONSTANT_NUMBER.0 as u8;
const LBC_CONSTANT_STRING_U8: u8 = LuauBytecodeTag::LBC_CONSTANT_STRING.0 as u8;
const LBC_CONSTANT_IMPORT_U8: u8 = LuauBytecodeTag::LBC_CONSTANT_IMPORT.0 as u8;
const LBC_CONSTANT_TABLE_U8: u8 = LuauBytecodeTag::LBC_CONSTANT_TABLE.0 as u8;
const LBC_CONSTANT_CLOSURE_U8: u8 = LuauBytecodeTag::LBC_CONSTANT_CLOSURE.0 as u8;
const LBC_CONSTANT_VECTOR_U8: u8 = LuauBytecodeTag::LBC_CONSTANT_VECTOR.0 as u8;
const LBC_CONSTANT_VECTORD_U8: u8 = LuauBytecodeTag::LBC_CONSTANT_VECTORD.0 as u8;
const LBC_CONSTANT_TABLE_WITH_CONSTANTS_U8: u8 =
  LuauBytecodeTag::LBC_CONSTANT_TABLE_WITH_CONSTANTS.0 as u8;
const LBC_CONSTANT_INTEGER_U8: u8 = LuauBytecodeTag::LBC_CONSTANT_INTEGER.0 as u8;
const LBC_CONSTANT_CLASS_SHAPE_U8: u8 = LuauBytecodeTag::LBC_CONSTANT_CLASS_SHAPE.0 as u8;
const USERDATA_TYPE_LIMIT: usize = (LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_END.0
  - LuauBytecodeType::LBC_TYPE_TAGGED_USERDATA_BASE.0) as usize;

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

/// 不可信 count 的硬校验：`u32 as i32` 可以是负数，且 blob 尾部必须容得下
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
/// （cpp lvmload.cpp 的 proto 表是自内而后的后序），因此未写入槽从不被读——
/// 分配后的原生槽只会在其 proto 完整装载后由 `*slot = p` 发布，越出
/// `0..loaded` 一律判为损坏字节码，绝不触碰未初始化内存。
fn proto_at(protos: &TempBuffer<*mut Proto>, id: u32, loaded: usize) -> Option<*mut Proto> {
  let index = id as usize;

  (index < loaded && index < protos.count).then(|| protos[index])
}

/// 常量表索引：cpp 直接取 `&p->k[key]`，而 `key` 来自不可信字节码，
/// 负值或越界都会跳出数组，故统一在此判定为只读 handle；需要写方以
/// `cast_mut()` 在明确的写入点升级。
fn constant_at(p: &Proto, id: i64) -> Option<*const TValue> {
  // 界内性由上方 contains 判定保证，`wrapping_add` 在此与 `add` 同址（用安全方法免 unsafe）
  ((0..p.sizek as i64).contains(&id)).then(|| p.k.wrapping_add(id as usize).cast_const())
}

/// §11 pass B（字节码加载读簇）：常量装载/class shape/direct access 路径上
/// `ttisstring! + tsvalue!`、`ttisnil!` 的 tag/payload 读链收敛为 [`ValueView`] 变体
/// match——tag 判定与串指针提取同臂完成，报错点与原检查逐位同址；视图借用止于各
/// match 臂，带出的裸指针不含对 GC 可改对象的长程借用。`lua_a_toobject` 环境的裸
/// `hvalue!` 无 tag 判定可收敛（且下游以 LuaTable 裸 handle 跨分配长期持有），保留原形。
///
/// # Safety
///
/// `l` 必须指向存活的 `LuaState`（版本/malformed 错误在其栈上压消息并可 longjmp/抛出）；
/// `strings`/`protos` 临时缓冲与 `env` 栈索引在本调用全程有效，`data` 为待加载字节码原文。
pub(crate) unsafe fn loadsafe(
  l: *mut LuaState,
  strings: &mut TempBuffer<*mut tstring>,
  protos: &mut TempBuffer<*mut Proto>,
  chunkname: &str,
  data: &[u8],
  env: i32,
) -> i32 {
  // Safety: 契约保证 l 存活可报错压栈；块内对所有 (*p)/(*f) 读取前均有长度/边界硬校验兜底
  unsafe {
    // 空 blob 连版本字节都没有：cpp 的 `read<uint8_t>` 是越界读，Rust 侧
    // 必须先收口，否则后续所有偏移判定都建立在零长缓冲上
    if data.is_empty() {
      malformed!(l, chunkname, "bytecode is empty");
    }

    // cpp 的 `data` + `size` 两个形参在这里就是一个切片：长度由切片自身携带，
    // 下面的 `size` 只是 `data.len()` 的别名，供 `fits`/`count_fits` 复用。
    let size = data.len();

    let mut offset: usize = 0;

    // 四个读取宏：把「截断 / 越界」就地收口成损坏字节码错误。
    //
    // cpp 侧对应 `read<T>` / `readVarInt` / `readVarInt64` / `readString`
    // （lvmload.cpp:125/135/152/169），它们只有 `LUAU_ASSERT`，release 编译掉
    // 就是越界读；Rust 侧这几个原语已改成可失败（返回 `Option`/`Result`），
    // 这里统一转成 `malformed!`。宏必须定义在函数体内：宏体里的自由标识符按
    // 展开处作用域解析，`l`/`chunkname`/`data`/`size`/`offset`/`strings` 都要
    // 可见。`$what` 是错误消息里的字段名，逐站命名以便定位损坏点。
    macro_rules! read_value {
      ($ty:ty, $what:literal) => {
        match read::<$ty>(data, &mut offset) {
          Some(value) => value,
          None => malformed!(l, chunkname, "bytecode {} is truncated", $what),
        }
      };
    }

    macro_rules! read_var {
      ($what:literal) => {
        match read_var_int(data, &mut offset) {
          Some(value) => value,
          None => malformed!(l, chunkname, "bytecode {} is truncated", $what),
        }
      };
    }

    macro_rules! read_var64 {
      ($what:literal) => {
        match try_read_var_int_64(data, &mut offset) {
          Some(value) => value,
          None => malformed!(l, chunkname, "bytecode {} is truncated", $what),
        }
      };
    }

    macro_rules! read_str {
      ($what:literal) => {
        match read_string(strings, data, &mut offset) {
          Ok(value) => value,
          Err(ReadStringError::OutOfRange) => {
            malformed!(l, chunkname, "bytecode {} is out of range", $what)
          }
          // id 的 varint 跑出 blob 也是「取不到合法 id」，与越界同措辞并附截断位置
          Err(ReadStringError::Truncated) => malformed!(
            l,
            chunkname,
            "bytecode {} is out of range (varint truncated at offset {})",
            $what,
            offset
          ),
        }
      };
    }

    let version = read_value!(u8, "version header");

    // 0 means the rest of the bytecode is the error message
    if version == 0 {
      let name = ChunkName::new(chunkname);
      let mut chunkbuf = [0; LUA_IDSIZE as usize];
      let chunkid = chunkid_buf(&mut chunkbuf, name.as_nul_bytes());
      push_bytes(l, &[chunkid, &data[offset..]].concat());
      return 1;
    }

    // cpp: `cpp/VM/src/lvmload.cpp:301` —
    // `(version < MIN || version > MAX) && version != LBC_VERSION_CLASSES`
    if (version < LuauBytecodeTag::LBC_VERSION_MIN.0 as u8
      || version > LuauBytecodeTag::LBC_VERSION_MAX.0 as u8)
      && version != LuauBytecodeTag::LBC_VERSION_CLASSES.0 as u8
    {
      return push_chunk_error(
        l,
        chunkname,
        format_args!(
          "bytecode version mismatch (expected [{}..{}], got {})",
          LuauBytecodeTag::LBC_VERSION_MIN.0,
          LuauBytecodeTag::LBC_VERSION_MAX.0,
          version
        ),
      );
    }

    let mut typesversion: u8 = 0;

    if version >= VERSION_WITH_TYPES {
      typesversion = read_value!(u8, "types version");

      if typesversion < LuauBytecodeTag::LBC_TYPE_VERSION_MIN.0 as u8
        || typesversion > LuauBytecodeTag::LBC_TYPE_VERSION_MAX.0 as u8
      {
        return push_chunk_error(
          l,
          chunkname,
          format_args!(
            "bytecode type version mismatch (expected [{}..{}], got {})",
            LuauBytecodeTag::LBC_TYPE_VERSION_MIN.0,
            LuauBytecodeTag::LBC_TYPE_VERSION_MAX.0,
            typesversion
          ),
        );
      }
    }

    // env is 0 for current environment and a stack index otherwise
    let envt = if env == 0 {
      (*l).gt
    } else {
      (*lua_a_toobject(l, env)).as_table_ptr()
    };

    // cpp: `TString* source = luaS_new(L, chunkname)` —— 宏体里的长度是
    // `strlen(chunkname)`，故这里按同款规则取首个 NUL 前的字节，零拷贝交
    // `lua_s_newlstr` 驻留。
    let sname = chunkname_bytes(chunkname);
    let source = lua_s_newlstr(l, sname);

    // string table
    let string_count = read_var!("string table size");
    strings.allocate(l, string_count as usize);

    for slot in c_slice_mut(strings.data, string_count as usize) {
      let length = read_var!("string length");

      // 字符串体是 blob 内原字节，长度来自不可信数据：必须硬校验后再取
      if !fits(offset, length as usize, size) {
        malformed!(l, chunkname, "bytecode string table is truncated");
      }

      // 切片区间已校验，直接交 `lua_s_newlstr` 驻留（长度即 body.len()，等于校验过的 length）
      let body = &data[offset..offset + length as usize];
      *slot = lua_s_newlstr(l, body);
      offset += length as usize;
    }

    // userdata type remapping table
    // for unknown userdata types, the entry will remap to common 'userdata' type
    let mut userdata_remapping = [LuauBytecodeType::LBC_TYPE_USERDATA.0 as u8; USERDATA_TYPE_LIMIT];

    if typesversion == 3 {
      let mut index = read_value!(u8, "userdata type index");

      while index != 0 {
        let name = read_str!("userdata type name id");

        if ((index - 1) as usize) < USERDATA_TYPE_LIMIT
          && let Some(cb) = (*(*l).global).ecb.gettypemapping
        {
          userdata_remapping[(index - 1) as usize] = cb(l, getstr(name), (*name).len as usize);
        }

        index = read_value!(u8, "userdata type index");
      }
    }

    // proto table
    let proto_count = read_var!("proto table size");
    protos.allocate(l, proto_count as usize);

    // 槽位只在对应 proto 完整装载后于本轮末尾发布（`*slot = p`），
    // `proto_at` 按 `loaded` 计数只读已发布前缀，原生未初始化内存从不被读

    for (i, slot) in c_slice_mut(protos.data, proto_count as usize)
      .iter_mut()
      .enumerate()
    {
      // cpp: `cpp/VM/src/lvmload.cpp:374-377` —
      // version >= VERSION_WITH_PROTO_SIZE 时每个 proto 前带 protoSize，末尾按起点跳过未知数据
      let proto_size = if version >= VERSION_WITH_PROTO_SIZE {
        read_var!("proto size")
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
      (*p).bytecodeid = i as i32;
      (*p).funid = if (*(*l).global).lastprotoid == 0 {
        0
      } else {
        let id = (*(*l).global).lastprotoid;
        (*(*l).global).lastprotoid = (*(*l).global).lastprotoid.wrapping_add(1);
        id
      };

      (*p).maxstacksize = read_value!(u8, "max stack size");
      (*p).numparams = read_value!(u8, "num params");
      (*p).nups = read_value!(u8, "num upvalues");
      (*p).is_vararg = read_value!(u8, "is vararg");

      if version >= VERSION_WITH_TYPES {
        (*p).flags = read_value!(u8, "proto flags");

        if typesversion == 1 {
          let typesize = read_var!("type info size");

          if typesize != 0 {
            // v1 头部断言会读 types[0..2]，故按 max(2) 收紧校验
            if !fits(offset, (typesize as usize).max(2), size) {
              malformed!(l, chunkname, "bytecode type info is truncated");
            }

            // 以已校验宽度的切片读取代替裸指针解引用（fits 已保证 typesize.max(2) 在界内）
            let types = &data[offset..offset + (typesize as usize).max(2)];

            // cpp 在此只有 LUAU_ASSERT（release 编译掉即按垃圾头部继续），
            // 不可信流必须收口成损坏字节码错误
            if typesize != 2 + (*p).numparams as u32
              || types[0] != LuauBytecodeType::LBC_TYPE_FUNCTION.0 as u8
              || types[1] != (*p).numparams
            {
              malformed!(l, chunkname, "bytecode v1 type info header is invalid");
            }

            // transform v1 into v2 format：v2 头即 typesize 的 varint 编码，
            // 其后原样拷贝 v1 类型表（与 cpp 逐字节写入序列一致）
            let headersize = if typesize > 127 { 4usize } else { 3usize };
            let total = headersize + typesize as usize;

            (*p).typeinfo = luaM_newarray!(l, total, u8, (*p).hdr.memcat);
            (*p).sizetypeinfo = total as i32;

            let header_bytes: &[u8] = if headersize == 4 {
              &[
                ((typesize & 127) | (1 << 7)) as u8,
                (typesize >> 7) as u8,
                0,
                0,
              ]
            } else {
              &[typesize as u8, 0, 0]
            };

            let (header, body) = c_slice_mut((*p).typeinfo, total).split_at_mut(headersize);
            header.copy_from_slice(header_bytes);
            body.copy_from_slice(&data[offset..offset + typesize as usize]);
          }

          offset += typesize as usize;
        } else if matches!(typesversion, 2 | 3) {
          let typesize = read_var!("type info size");

          if typesize != 0 {
            if !fits(offset, typesize as usize, size) {
              malformed!(l, chunkname, "bytecode type info is truncated");
            }

            (*p).typeinfo = luaM_newarray!(l, typesize as usize, u8, (*p).hdr.memcat);
            (*p).sizetypeinfo = typesize as i32;
            // fits 已校验字节区间，切片一次拷贝代替裸指针 copy_nonoverlapping
            c_slice_mut((*p).typeinfo, typesize as usize)
              .copy_from_slice(&data[offset..offset + typesize as usize]);
            offset += typesize as usize;

            if typesversion == 3
              && !remap_userdata_types(
                c_slice_mut((*p).typeinfo, (*p).sizetypeinfo as usize),
                &userdata_remapping,
              )
            {
              malformed!(l, chunkname, "bytecode type info layout is invalid");
            }
          }
        }
      }

      let sizecode = read_var!("code size") as i32;

      // 指令定长 4 字节且逐项从 blob 读出，count 与剩余长度必须自洽
      if !count_fits(i64::from(sizecode), size_of::<Instruction>(), size, offset) {
        malformed!(l, chunkname, "bytecode code size is out of range");
      }

      (*p).code = luaM_newarray!(l, sizecode as usize, Instruction, (*p).hdr.memcat);
      (*p).sizecode = sizecode;

      for code in c_slice_mut((*p).code, (*p).sizecode as usize) {
        *code = read_value!(u32, "instruction");
      }

      (*p).codeentry = (*p).code;

      let sizek = read_var!("constant count") as i32;

      // 每个常量至少占 1 字节（tag），据此卡住虚高的 count；
      // 常量体的真实宽度（number 8 字节、vector 16 字节…）由每次
      // `read_value!`/`read_var!` 现场判定，虚报 count 只会浪费这个粗筛，
      // 越界读不可能漏到 `read::<T>`
      if !count_fits(i64::from(sizek), 1, size, offset) {
        malformed!(l, chunkname, "bytecode constant count is out of range");
      }

      (*p).k = luaM_newarray!(l, sizek as usize, TValue, (*p).hdr.memcat);
      (*p).sizek = sizek;

      // Initialize the constants to nil to ensure they have a valid state
      // in the event that some operation in the following loop fails with
      // an exception.
      // Safety:k 数组刚按 sizek 分配完成，全部元素可写。
      for k in c_slice_mut((*p).k, (*p).sizek as usize) {
        setnilvalue!(k);
      }

      // Safety:k 数组刚按 sizek 分配完成，全部元素可写。
      for k in c_slice_mut((*p).k, (*p).sizek as usize) {
        match read_value!(u8, "constant tag") {
          LBC_CONSTANT_NIL_U8 => {
            // All constants have already been pre-initialized to nil
          }

          LBC_CONSTANT_BOOLEAN_U8 => {
            let v = read_value!(u8, "boolean constant");
            setbvalue!(k, v);
          }

          LBC_CONSTANT_NUMBER_U8 => {
            let v = read_value!(f64, "number constant");
            setnvalue!(k, v);
          }

          LBC_CONSTANT_VECTOR_U8 => {
            let x = read_value!(f32, "vector constant x");
            let y = read_value!(f32, "vector constant y");
            let z = read_value!(f32, "vector constant z");
            let w = read_value!(f32, "vector constant w");
            setvvalue!(k, x, y, z, w);
          }

          // 双精度向量：cpp 同样按 float(x) 截断进 32 位存储（lobject.h setvvalue）
          LBC_CONSTANT_VECTORD_U8 => {
            let x = read_value!(f64, "vectorD constant x");
            let y = read_value!(f64, "vectorD constant y");
            let z = read_value!(f64, "vectorD constant z");
            let w = read_value!(f64, "vectorD constant w");
            setvvalue!(k, x as f32, y as f32, z as f32, w as f32);
          }

          LBC_CONSTANT_STRING_U8 => {
            let v = read_str!("string constant id");
            setsvalue!(l, k, v);
          }

          LBC_CONSTANT_IMPORT_U8 => {
            let iid = read_value!(u32, "import constant");
            resolve_import_safe(l, envt, (*p).k, iid);
            setobj!(l, k, (*l).top.sub(1));
            (*l).top = (*l).top.sub(1);
          }

          LBC_CONSTANT_TABLE_U8 => {
            let keys = read_var!("table key count") as i32;
            let h = lua_h_new(l, 0, keys);
            // 保留计数重复：keys 是协议声明的键条数，键 id 逐轮由 read_var 从字节流解码并推进
            // offset，数据此刻尚未成数组，没有可切片迭代的存在
            for _ in 0..keys {
              let key = read_var!("table key id") as i32;
              let Some(kslot) = constant_at(&*p, key as i64) else {
                malformed!(l, chunkname, "bytecode table key index is out of range");
              };
              let val = lua_h_set(l, h, kslot);
              setnvalue!(val, 0.0);
            }
            sethvalue!(l, k, h);
          }

          LBC_CONSTANT_TABLE_WITH_CONSTANTS_U8 => {
            let keys = read_var!("table key count");
            let h = lua_h_new(l, 0, keys as i32);

            // 每个键值对至少占 5 字节（key varint + i32 constantIdx）
            if !count_fits(i64::from(keys), 5, size, offset) {
              malformed!(l, chunkname, "bytecode table key count is out of range");
            }

            let mut nil_keys: TempBuffer<i32> = TempBuffer::new();
            nil_keys.allocate(l, keys as usize);
            let mut nil_keys_size: usize = 0;

            // 保留计数重复：同上，每轮解码键 id + 常量索引（变长 varint），游标随解码推进，
            // 循环变量不参与任何取数
            for _ in 0..keys {
              let key = read_var!("table key id") as i32;
              let Some(kslot) = constant_at(&*p, key as i64) else {
                malformed!(l, chunkname, "bytecode table key index is out of range");
              };
              let val = lua_h_set(l, h, kslot);
              let constant_idx = read_value!(i32, "table constant index");
              if let Some(constant) = constant_at(&*p, constant_idx as i64) {
                if matches!(ValueView::from_tvalue(&*constant), ValueView::Nil) {
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
              let Some(kslot) = constant_at(&*p, *key as i64) else {
                malformed!(l, chunkname, "bytecode table key index is out of range");
              };
              let val = lua_h_set(l, h, kslot);
              setnilvalue!(val);
            }

            sethvalue!(l, k, h);
          }

          LBC_CONSTANT_CLOSURE_U8 => {
            let fid = read_var!("closure proto id");
            // 闭包只允许引用本轮之前已装载完的 proto（槽 0..i）
            let Some(proto) = proto_at(protos, fid, i) else {
              malformed!(l, chunkname, "bytecode closure proto id is out of range");
            };
            let cl = lua_f_new_lclosure(l, (*proto).nups as i32, envt, proto);
            (*cl).preload = u8::from((*cl).nupvalues > 0);
            setclvalue!(l, k, cl);
          }

          LBC_CONSTANT_CLASS_SHAPE_U8 => {
            let cnid = read_var!("class name id");
            let Some(classname) = constant_at(&*p, i64::from(cnid)) else {
              malformed!(l, chunkname, "bytecode class name index is out of range");
            };
            // 非字符串常量的 payload 当指针用就是拿垃圾位，tag 判定与串指针提取
            // 一并收敛为 ValueView::String 臂 match（与原 ttisstring! 检查同点报错）
            let classname_ts = match ValueView::from_tvalue(&*classname) {
              ValueView::String(ts) => ts_handle(ts),
              _ => malformed!(l, chunkname, "bytecode class name is not a string"),
            };
            let num_properties = read_var!("class property count");
            let num_methods = read_var!("class method count");
            let num_members = num_methods.wrapping_add(num_properties);
            // 每个成员至少占 1 字节（mid varint）
            if !count_fits(i64::from(num_members), 1, size, offset) {
              malformed!(l, chunkname, "bytecode class member count is out of range");
            }
            let offset_to_member =
              luaM_newarray!(l, num_members as usize, *mut tstring, (*l).activememcat);
            let members_to_offset = lua_h_new(l, 0, num_members as i32);

            for (idx, slot) in c_slice_mut(offset_to_member, num_members as usize)
              .iter_mut()
              .enumerate()
            {
              let mid = read_var!("class member id");
              let Some(member_name) = constant_at(&*p, i64::from(mid)) else {
                malformed!(l, chunkname, "bytecode class member index is out of range");
              };
              let member = match ValueView::from_tvalue(&*member_name) {
                ValueView::String(ts) => ts_handle(ts),
                _ => malformed!(l, chunkname, "bytecode class member name is not a string"),
              };
              *slot = member;
              let val = lua_h_setstr(l, members_to_offset, member);
              setnvalue!(val, idx as f64);
            }

            (*members_to_offset).readonly = 1;

            // cpp lvmload.cpp:619：class 的 `new`/`__init` C 闭包须绑到当前
            // 加载环境 `envt`（非 `L->gt`），故随 `luaR_newclass` 一并传入。
            let lco = lua_r_newclass(
              l,
              classname_ts,
              members_to_offset,
              offset_to_member,
              num_properties as i32,
              num_methods as i32,
              envt,
            );
            setclassvalue!(l, k, lco);
          }

          LBC_CONSTANT_INTEGER_U8 => {
            let is_negative = read_value!(u8, "integer constant sign");
            let magnitude = read_var64!("integer constant magnitude");
            let value = if is_negative != 0 {
              (!magnitude).wrapping_add(1) as i64
            } else {
              magnitude as i64
            };
            setlvalue!(k, value);
          }

          _ => {
            // 常量 tag 直接来自不可信字节码：未知 tag 就是损坏输入，不是内部矛盾
            malformed!(l, chunkname, "bytecode constant tag is unknown");
          }
        }
      }

      if fflag::LuauUdataDirectAccess6.get() {
        // 指令窗口原地重写 pass：裸指针游标 → 一次定界的切片 + 下标游走。
        // lua_s_updateatom 不触碰本 Proto 的 code 数组（只改串原子态），且加载期
        // GC 冻结（gc_threshold == usize::MAX 前置断言），故切片基址/长度全程稳定，
        // 与逐轮重读 (*p).code 的旧形态地址序列逐位相同；越界判定与 cpp 指针比较
        // `instruction.add(1) >= end` 等价（get(pc+1) 仅当 pc+1 >= sizecode 落空）。
        let code = c_slice_mut((*p).code, (*p).sizecode as usize);
        let mut pc: usize = 0;

        while pc < code.len() {
          let mut target_op = -1i32;

          match LuauOpcode::from(luau_insn_op(code[pc]) as u8) {
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
            let Some(&aux) = code.get(pc + 1) else {
              malformed!(l, chunkname, "bytecode instruction aux is out of range");
            };
            // AUX 的常量下标来自指令字，越界即损坏输入（cpp 只有 release
            // 编译掉的 LUAU_ASSERT）
            if aux >= sizek as u32 {
              malformed!(
                l,
                chunkname,
                "bytecode instruction aux constant index is out of range"
              );
            }

            // We take over the upper 16 bits of AUX - so no constants with big indices.
            if aux < 0x10000 {
              let Some(k) = constant_at(&*p, i64::from(aux)) else {
                malformed!(
                  l,
                  chunkname,
                  "bytecode direct access constant index is out of range"
                );
              };
              // 合法字节码里 direct access 的 AUX 必指向字符串常量；非字符串
              // 就是拿垃圾位当指针，lua_s_updateatom 会任意写（同 CLASS_SHAPE）——
              // tag 判定与串指针提取收敛为 ValueView::String 臂 match
              let s = match ValueView::from_tvalue(&*k) {
                ValueView::String(ts) => ts_handle(ts),
                _ => malformed!(
                  l,
                  chunkname,
                  "bytecode direct access constant is not a string"
                ),
              };

              lua_s_updateatom!(l, s);

              if (*s).atom >= 0 {
                code[pc] = (code[pc] & 0xffffff00) | target_op as u32;
              }
            }
          }

          // 步长取当前（可能已被重写为 BREAK 族以外操作码的）指令，与原
          // `*instruction` 读点同址同序
          pc += get_op_length(LuauOpcode::from(luau_insn_op(code[pc]) as u8)) as usize;
        }
      }

      let sizep = read_var!("inner proto count") as i32;
      // 每个 fid 至少占 1 字节
      if !count_fits(i64::from(sizep), 1, size, offset) {
        malformed!(l, chunkname, "bytecode proto count is out of range");
      }

      (*p).p = luaM_newarray!(l, sizep as usize, *mut Proto, (*p).hdr.memcat);
      (*p).sizep = sizep;

      for slot in c_slice_mut((*p).p, (*p).sizep as usize) {
        let fid = read_var!("inner proto id");
        // 内层 proto 同样只能引用已装载完的槽（0..i）
        let Some(proto) = proto_at(protos, fid, i) else {
          malformed!(l, chunkname, "bytecode inner proto id is out of range");
        };
        *slot = proto;
      }

      (*p).linedefined = read_var!("line defined") as i32;
      let debugname = read_str!("debug name string id");
      (*p).debugname = debugname;

      let lineinfo = read_value!(u8, "line info flag");

      if lineinfo != 0 {
        let linegaplog2 = read_value!(u8, "line gap log2");
        (*p).linegaplog2 = linegaplog2 as i32;

        // 移位量来自不可信字节码：超过 i32 位宽即 UB/panic
        if (*p).linegaplog2 < 0 || (*p).linegaplog2 >= size_of::<i32>() as i32 * 8 {
          malformed!(l, chunkname, "bytecode line gap log2 is out of range");
        }

        let intervals = (((*p).sizecode - 1) >> (*p).linegaplog2) + 1;
        let absoffset = ((*p).sizecode + 3) & !3;

        let sizelineinfo = absoffset + intervals * size_of::<i32>() as i32;
        (*p).lineinfo = luaM_newarray!(l, sizelineinfo as usize, u8, (*p).hdr.memcat);
        (*p).sizelineinfo = sizelineinfo;

        (*p).abslineinfo = (*p).lineinfo.add(absoffset as usize).cast();

        let mut lastoffset: u8 = 0;
        for line in c_slice_mut((*p).lineinfo, (*p).sizecode as usize) {
          lastoffset = lastoffset.wrapping_add(read_value!(u8, "line info byte"));
          *line = lastoffset;
        }

        let mut lastline: i32 = 0;
        for line in c_slice_mut((*p).abslineinfo, intervals as usize) {
          lastline = lastline.wrapping_add(read_value!(i32, "absolute line info"));
          *line = lastline;
        }
      }

      let debuginfo = read_value!(u8, "debug info flag");

      if debuginfo != 0 {
        let sizelocvars = read_var!("local variable count") as i32;
        // 每个 LocVar 至少占 4 字节（varname/startpc/endpc 各 1 + reg 1）
        if !count_fits(i64::from(sizelocvars), 4, size, offset) {
          malformed!(
            l,
            chunkname,
            "bytecode local variable count is out of range"
          );
        }

        (*p).locvars = luaM_newarray!(l, sizelocvars as usize, LocVar, (*p).hdr.memcat);
        (*p).sizelocvars = sizelocvars;

        for locvar in c_slice_mut((*p).locvars, (*p).sizelocvars as usize) {
          let varname = read_str!("local variable name id");
          locvar.varname = varname;
          locvar.startpc = read_var!("local variable start pc") as i32;
          locvar.endpc = read_var!("local variable end pc") as i32;
          locvar.reg = read_value!(u8, "local variable reg");
        }

        let sizeupvalues = read_var!("upvalue count") as i32;
        // upvalue 名表长度必须与 proto 头的 nups 自洽（cpp 仅有 LUAU_ASSERT）
        if sizeupvalues != (*p).nups as i32 {
          malformed!(
            l,
            chunkname,
            "bytecode upvalue count mismatches proto header"
          );
        }

        // 每个 upvalue 名至少占 1 字节（varint id）
        if !count_fits(i64::from(sizeupvalues), 1, size, offset) {
          malformed!(l, chunkname, "bytecode upvalue count is out of range");
        }

        (*p).upvalues = luaM_newarray!(l, sizeupvalues as usize, *mut tstring, (*p).hdr.memcat);
        (*p).sizeupvalues = sizeupvalues;

        for uv in c_slice_mut((*p).upvalues, (*p).sizeupvalues as usize) {
          let name = read_str!("upvalue name id");
          *uv = name;
        }
      }

      if version >= VERSION_WITH_FEEDBACK {
        // cpp 对 version >= 11 无条件读 feedbackvec（字节码格式 v11 自带）。
        (*p).feedbackvecsize = read_var!("feedback slot count");

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
          let slottype = read_value!(u8, "feedback slot type");
          // 目前格式里只有 CALLTARGET 一种槽类型，其余即损坏输入（cpp 的
          // LUAU_ASSERT 在 release 下会带着未知 kind 继续跑）
          if slottype != LuauFeedbackType::LFT_CALLTARGET as u8 {
            malformed!(l, chunkname, "bytecode feedback slot type is unknown");
          }
          slot.kind = FeedbackVectorSlotKind::CallTarget;
          slot.data.call_target.pc = read_var!("feedback slot pc");
          slot.data.call_target.proto = 0;
          slot.data.call_target.hits = 0;
        }
      }

      // cpp: `cpp/VM/src/lvmload.cpp:767-777` —
      // version >= 12 时读 inlinable cost 并跳过 proto 末尾的未知数据
      if version >= VERSION_WITH_PROTO_SIZE
        && ((*p).flags & LuauProtoFlag::LPF_INLINABLE as u8) != 0
      {
        (*p).cost = read_var64!("inlinable cost");
      }

      if version >= VERSION_WITH_PROTO_SIZE {
        offset = proto_start_offset + proto_size as usize;
      }

      *slot = p;
    }

    // "main" proto is pushed to Lua stack
    let mainid = read_var!("main proto id");
    let Some(main) = proto_at(protos, mainid, protos.count) else {
      malformed!(l, chunkname, "bytecode main proto id is out of range");
    };

    let thread_obj = l.cast::<GCObject>();
    if isblack!(thread_obj) {
      lua_c_barrierback(l, thread_obj, &mut (*l).gclist);
    }

    let cl = lua_f_new_lclosure(l, 0, envt, main);
    setclvalue!(l, (*l).top, cl);
    incr_top!(l);

    0
  }
}

/// cpp 侧 `chunkname` 是 NUL 结尾的 `const char*`：`luaS_new` 与
/// `luaO_chunkid(..., strlen(chunkname))` 都按 `strlen` 取值，即有效字节为
/// 首个 NUL 之前的部分。`&str` 可以含内部 NUL，故在此还原同款截断，
/// 不让源名与错误文案相对 oracle 发生偏移。
fn chunkname_bytes(chunkname: &str) -> &[u8] {
  let bytes = chunkname.as_bytes();
  let end = memchr(0, bytes).unwrap_or(bytes.len());

  &bytes[..end]
}

/// 把 [`ValueView::String`] 的共享视图升级回 `TString` 裸 handle：下游
/// `luaR_newclass`/`lua_h_setstr`/`lua_s_updateatom` 均按 GC handle 收参并
/// 自行维持存活契约，此处只做视图转换，不改变任何字节。
fn ts_handle(ts: &tstring) -> *mut tstring {
  from_ref(ts).cast_mut()
}

/// `chunkname` 的 NUL 结尾副本，供 `lua_o_chunkid` 使用。
///
/// cpp 的 `luaO_chunkid` 在 `=`/`@` 且未超长时直接返回 `source + 1`（指向源名
/// 本体），只有截断/`[string "..."]` 分支才写 `chunkbuf`；因此错误消息构造需要
/// 一份带终止符、且比 chunkid 活得更久的副本 —— 用本类型把两者绑在同一作用域。
/// 仅错误路径构造，一次堆分配。
struct ChunkName(Vec<u8>);

impl ChunkName {
  fn new(chunkname: &str) -> Self {
    let name = chunkname_bytes(chunkname);
    let mut bytes = Vec::with_capacity(name.len() + 1);
    bytes.extend_from_slice(name);
    bytes.push(0);

    Self(bytes)
  }

  /// NUL 结尾字节串：前置条件由 `new` 保证——内部 NUL 已在 `chunkname_bytes`
  /// 处截掉，末尾恰好补了一个终止符，故 `len() - 1` 即 cpp 的 `strlen`。
  fn as_nul_bytes(&self) -> &[u8] {
    &self.0
  }
}

/// # Safety
///
/// `l` 必须指向存活的 `LuaState`（错误串经 `push_rust_string` 压其栈顶）；
/// `detail` 为一次性 `fmt::Arguments`，仅在本函数内渲染一次。
/// 拼 chunkid 前缀 + 错误消息并 push 到栈上（cpp 三处版本/类型版本错误的
/// 共用形态），返回 1 表示已产出错误字符串。
unsafe fn push_chunk_error(l: *mut LuaState, chunkname: &str, detail: fmt::Arguments) -> i32 {
  // Safety: chunkbuf 为栈上数组、move 不发生（chunkid 仅在本块内使用），契约保证 l 可压栈
  unsafe {
    let name = ChunkName::new(chunkname);
    let mut chunkbuf = [0; LUA_IDSIZE as usize];
    let chunkid = chunkid_buf(&mut chunkbuf, name.as_nul_bytes());
    // from_utf8_lossy 合法 UTF-8 时借用为 Cow::Borrowed，省去一次中间 String 分配
    let message = alloc::format!("{}: {}", String::from_utf8_lossy(chunkid), detail);
    push_bytes(l, message.as_bytes());
    1
  }
}

/// 按 cpp `luaO_chunkid(chunkbuf, sizeof(chunkbuf), chunkname, strlen(chunkname))`
/// 的入参形态求 chunkid。
///
/// # Safety
///
/// 返回的 NUL 前字节切片借用 `buf` 或 `name`（`=`/`@` 未超长时 cpp 直接返回
/// `source + 1`），故两者都必须在返回值存活期间不被释放/复用；生命周期参数
/// 已把 `buf` 与 `name` 的借用绑成同一 `'a`。`name` 须为恰以一个 NUL 结尾的
/// 字节串（`ChunkName::as_nul_bytes` 的构造保证）。
unsafe fn chunkid_buf<'a>(buf: &'a mut [c_char; LUA_IDSIZE as usize], name: &'a [u8]) -> &'a [u8] {
  // Safety: buf 独占可变、name 为恰以一个 NUL 结尾的字节串，lua_o_chunkid 只写 buf 与读 name
  unsafe {
    // 末位是终止符，`len() - 1` 即 cpp 的 strlen
    let chunkid = lua_o_chunkid(
      buf.as_mut_ptr(),
      buf.len(),
      name.as_ptr().cast(),
      name.len() - 1,
    );
    // Safety: lua_o_chunkid 的两个出口（buf / source + 1）都是 NUL 结尾串，
    // 且其存储由 `buf` 或 `name` 持有（'a），满足 cstr_bytes 门面前置条件。
    cstr_bytes(chunkid)
  }
}

/// 把错误消息字节压入 `l` 栈顶：`lua_pushlstring` 在返回前完成拷贝，
/// `bytes` 只需在调用期间可读。
///
/// # Safety
///
/// `l` 必须指向存活的 `LuaState`（且栈有可写余量，错误路径均在受保护帧内）。
unsafe fn push_bytes(l: *mut LuaState, bytes: &[u8]) {
  // Safety: 契约保证 l 存活可承接压栈；&[u8] 的 ptr/len 构成合法可读区间
  unsafe { lua_pushlstring(l, bytes.as_ptr().cast(), bytes.len()) };
}
