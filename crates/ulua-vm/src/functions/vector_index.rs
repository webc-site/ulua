use crate::{
  functions::{lua_l_checklstring::lua_l_checklstring_ref, vector_shared::check_vector},
  macros::{lua_l_error::luaL_error, lua_lib_fn::lua_lib_fn, lua_vector_size::LUA_VECTOR_SIZE},
  records::lua_state::LuaState,
};

/// 分量定长窗长：`LUA_VECTOR_SIZE` 的 usize 形（r12-R-D 切片化，编译期折叠）。
const LANES: usize = LUA_VECTOR_SIZE as usize;

/// `vector` 元表 `__index`：按分量名（x/y/z/w，大小写均可）取实参向量的一个分量并以
/// number 压回。
///
/// 调用序契约（正确性，非内存安全——`l` 的存活前提已由 `&mut` 接收者类型承载）：以 Lua 库函数
/// 约定处于受保护帧——索引 1 须为 vector（否则 `check_vector` 经 `tag_error` 抛错发散），其分量
/// 经值窗 `[f32; 4]` 按 `ic < LANES` 界内取用（越界名同步落错误臂，无指针算术）；索引 2 须为
/// 可转字符串的值（数字按 `tolstring` 转写，其余经 `checklstring` 抛 "string expected"
/// 发散），非法分量名经 `luaL_error` 抛错不返回。`push_number`
/// 自身扩栈，可触发 GC。
/// cpp VM/src/lveclib.cpp:256
pub(crate) fn vector_index(l: &mut LuaState) -> i32 {
  let v = check_vector(l, 1);
  let name_bytes = lua_l_checklstring_ref(l, 2);
  // 锚定形：快路径先行（借用窗口只读单字节、快照为 Copy 值，push 前借用已结束；
  // `l` 早返路径与窗口借用互不交叠）；错误臂再取 owned 快照后重建 `l` 裸参
  let first_byte = (name_bytes.len() == 1).then(|| name_bytes[0]);
  if let Some(b) = first_byte {
    // 单字节读逐位等值：旧形 `*name as i32` 按 c_char 符号扩展、此处按 u8 零扩展，
    // 分岔仅在高位字节——i8 形 ic<0 转 usize 成巨大值、u8 形 ic>=40，两形皆过不了
    // `ic<LANES` 守卫而同步落错误臂；界内命中的 x/y/z/w（大小写）恒为 ASCII，同值。
    let ic = (b as i32 | 0x20) - 'x' as i32;

    const W_OFFSET: i32 = -1; // 'w' - 'x'
    let ic = if ic == W_OFFSET { 3 } else { ic as usize };

    if ic < LANES {
      l.push_number(v[ic] as f64);
      return 1;
    }
  }

  let name = String::from_utf8_lossy(name_bytes).into_owned();
  // SAFETY: `l.as_mut_ptr()` 为由 `&mut` 借用重建的存活帧裸参（有效性与独占由引用承载），
  // `luaL_error` 经其格式化并抛出错误、不返回；受保护帧前提见函数文档的调用序契约。
  unsafe { luaL_error!(l.as_mut_ptr(), "attempt to index vector with '{}'", name) }
}

lua_lib_fn!(pub(crate) fn vector_index @ref, vector_index_arm);

// r7-tprod2 尾矿台账（本文件票面 1 枚：让 1）——下方 `String::from_utf8_lossy`
// 仅错误臂可达（len==1 快路径已早返；失配名进 luaL_error 即 unwind），合法
// UTF-8 走 Borrowed 零堆配，剥壳运行期省 0；文案须逐字节对齐 cpp lveclib.cpp:280
// `'%s'`，Cow 即下限形态，不动。
