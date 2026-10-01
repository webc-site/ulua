use crate::{
  functions::{
    c_slice, lua_l_checklstring::lua_l_checklstring, lua_l_checkvector::lua_l_checkvector,
  },
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::LuaState,
};

/// 分量定长窗长：`LUA_VECTOR_SIZE` 的 usize 形（r12-R-D 切片化，编译期折叠）。
const LANES: usize = LUA_VECTOR_SIZE as usize;

/// # Safety
/// `l` 须为存活 `LuaState` 且处于受保护帧：`luaL_checkvector(l,1)` 取 vector（返回指针
/// 指向 `LUA_VECTOR_SIZE` 个连续 `f32`，在本函数指针边界处收成 `&[f32; LANES]` 窗，仅按
/// `ic<LANES` 切片索引读）；`luaL_checklstring(l,2,&len)` 取单字符分量名（字节窗经
/// `c_slice` 收口，null/0 长守卫与旧 `from_raw_parts` 可达域同形）；非法名经 `luaL_error`
/// 抛错回退。`lua_pushnumber` 需 `(*l).top` 后 ≥1 空槽；可触发 GC。
/// cpp VM/src/lveclib.cpp:256
pub(crate) unsafe fn vector_index(l: *mut LuaState) -> i32 {
  // SAFETY: 本函数唯一指针边界：按上述契约把 `v`/`name` 两枚数据指针各收成窗口引用
  // （窗构造只转形状不读内存，读点与收口前 `*v.add(ic)`/`from_raw_parts` 逐位同域同序），
  // 其后全部分量/字节访问走切片索引，越界读界内被守卫拦下、不再发生裸指针算术。
  unsafe {
    let v = lua_l_checkvector(&mut *l, 1);
    let mut len = 0usize;
    let name = lua_l_checklstring(&mut *l, 2, &mut len);
    let lanes = &*(v.cast::<[f32; LANES]>());
    let name_bytes = c_slice(name as *const u8, len);

    if len == 1 {
      // 单字节读逐位等值：旧形 `*name as i32` 按 c_char 符号扩展、此处按 u8 零扩展，
      // 分岔仅在高位字节——i8 形 ic<0 转 usize 成巨大值、u8 形 ic>=40，两形皆过不了
      // `ic<LANES` 守卫而同步落错误臂；界内命中的 x/y/z/w（大小写）恒为 ASCII，同值。
      let ic = (name_bytes[0] as i32 | 0x20) - 'x' as i32;

      const W_OFFSET: i32 = -1; // 'w' - 'x'
      let ic = if ic == W_OFFSET { 3 } else { ic as usize };

      if ic < LANES {
        (*l).push_number(lanes[ic] as f64);
        return 1;
      }
    }

    let name = String::from_utf8_lossy(name_bytes);
    luaL_error!(l, "attempt to index vector with '{}'", name)
  }
}

lua_lib_fn!(pub(crate) fn vector_index, vector_index_arm);

// r7-tprod2 尾矿台账（本文件票面 1 枚：让 1）——下方 `String::from_utf8_lossy`
// 仅错误臂可达（len==1 快路径已早返；失配名进 luaL_error 即 unwind），合法
// UTF-8 走 Borrowed 零堆配，剥壳运行期省 0；文案须逐字节对齐 cpp lveclib.cpp:280
// `'%s'`，Cow 即下限形态，不动。
