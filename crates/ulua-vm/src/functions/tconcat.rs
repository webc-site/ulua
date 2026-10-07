use core::slice::from_raw_parts;

use crate::{
  enums::lua_type::LuaType,
  functions::{
    addfield::addfield, lua_l_addlstring::lua_l_addlstring, lua_l_buffinit::lua_l_buffinit,
    lua_l_optinteger::lua_l_optinteger, lua_l_pushresult::lua_l_pushresult,
  },
  macros::lua_lib_fn::lua_lib_fn,
  records::{lua_l_strbuf::LuaLStrbuf, lua_state::LuaState},
};

/// 调用序契约（正确性，非内存安全——`l` 的存活/独占前提已由 `&mut` 接收者类型承载；r16-v41 收形后
/// 判型/取参与缓冲协议全经安全门面与自由函数，体内裸操作仅余两类有守卫窗：1 号槽表句柄（紧邻
/// `check_type` 钉住存活，NULL 即 JIT 空表位点降为 `None` 走慢路）与 2 号槽分隔符串体脱借窗
/// （Lua 串不可变不搬移、槽引用钉住存活，建缓冲全程无用户代码可入——`lua_rawgeti` 系 raw 取、
/// 错误路径发散——故无「合法输入令内部不变量失守即 UB」通路，本体降为安全 `fn`）：`l` 须处于
/// 可抛错受保护帧，栈 1 号位为 table（否则 `check_type` 发散）、2 号位可选串、3/4 号位可选整数
/// 边界（越界经 `addfield` 报错文本发散）。
/// cpp/VM/src/ltablib.cpp:321 tconcat。
pub fn tconcat(l: &mut LuaState) -> i32 {
  // 脱借窗：cpp `sep/lsep` 跨全函数存续，切片形会独占 `l` 借用令后继门面调用失序，故取
  // (ptr, len) 后即释放借用；窗内容读取仅在下方 addlstring 点发生。
  let (sepptr, sepnb) = {
    let sep = l.opt_bytes(2, b"");
    (sep.as_ptr(), sep.len())
  };
  l.check_type(1, LuaType::Table);
  let i = lua_l_optinteger(l, 3, 1);
  let last = l.obj_len(1) as i32;
  let last = lua_l_optinteger(l, 4, last);

  // SAFETY: 上方 `check_type` 保证 1 号槽为存活表；`slot` 为正帧索引界内换算，`as_table_ptr`
  // 即该槽的类型化读数（cpp `hvalue(L->base)`）；可空位点降级：NULL 表指针（JIT 空表）→ None，
  // 非空 → 共享只读借用（表头不随数组段重分配搬移，`addfield` 快路每轮现取 array_window）。
  let t = unsafe {
    let tp = l.slot(1).get().as_table_ptr();
    if tp.is_null() { None } else { Some(&*tp) }
  };

  let mut b = LuaLStrbuf::new();
  lua_l_buffinit(l, &mut b);
  // 尾元素前的每字段后随分隔符（cpp `while current_i < last` 游走收为区间
  // 迭代）；收尾判定 i <= last 与原循环退出时 `current_i == last` 等价
  //（i > last 时区间为空且两判定同假，字段一个不输出）
  for current_i in i..last {
    // SAFETY: `l` 存活独占（本调用就地派生裸转手），`b` 已 init 未提交，`t` 窗存续如上，
    // `current_i` 界内与否由 addfield 快慢路各自收敛（慢路 rawgeti 任意下标合法）。
    unsafe { addfield(l.as_mut_ptr(), &mut b, current_i, t) };
    if sepnb != 0 {
      // SAFETY: 脱借窗为 2 号槽串体全长，读取界由 `sepnb` 自载；空分隔符已由此闸短路
      // （cpp `lsep != 0` 同形），`b` 契约同上。
      unsafe { lua_l_addlstring(&mut b, from_raw_parts(sepptr, sepnb)) };
    }
  }
  if i <= last {
    // SAFETY: 同环内调用点，末元素补写一次（区间非空时 cpp `i == last` 判定同真）。
    unsafe { addfield(l.as_mut_ptr(), &mut b, last, t) };
  }
  // w6e 降级消费点：`lua_l_pushresult` 已降为安全 fn，外层包裹消亡；`b` 为本函数
  // 独占缓冲且已 init，提交即写结果串并复位其栈位。
  lua_l_pushresult(&mut b);

  1
}

lua_lib_fn!(pub fn tconcat @ref, tconcat_arm);
