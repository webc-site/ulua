//! Source: `VM/src/lclass.cpp:122`
//!
//! cpp `luaR_cloneclass`：按 `classobject` 克隆出新鲜类对象——proto 常量表中
//! 的类形状被标 readonly，NEWCLASS 每次执行必须克隆后再改写（isopen/继承/
//! 增员都只落在克隆上），不得写穿共享常量形状。

use ulua_common::LUAU_ASSERT;

use crate::{
  functions::{
    c_slice, c_slice_mut,
    getcurrenv::getcurrenv,
    lua_h_clone::lua_h_clone,
    lua_r_newclass::{lua_r_newblankclass, lua_r_setupconstructor},
  },
  macros::{iswhite::iswhite, lua_m_newarray::luaM_newarray, obj_2_gco::obj2gco},
  records::{lua_state::LuaState, luau_class::LuauClass, t_string::tstring},
  type_aliases::t_value::TValue,
};

/// 分配并返回一个与 `classobject` 成员相同的新类对象
/// （cpp lclass.cpp:122 `luaR_cloneclass`）。
///
/// # Safety
/// `l` 须存活且处于受保护帧（分配失败经 `l` 抛 ErrMem）；`classobject` 为
/// 存活且成员数组成员按 numberof* 分配完整的类。新类为白色对象，克隆写入
/// 无需写屏障（cpp lclass.cpp:129 同断言）。
///
/// r16-v22 保留外层 `*mut LuaState` 形：本函数外层消费方含出范围文件
/// （`luau_execute.rs`——并发会话正在改的性能热区），改签名即撞车；本票仅调整文件内对
/// 已改形核心（`lua_r_newblankclass`/`lua_r_setupconstructor`）的转调形：一次 `&mut *l`
/// 就地重建引用（借用窗仅在当句内），并按「拆语句」判例把 `getcurrenv` 现读与 `setupconstructor`
/// 转调分列两句，避免同句内 `&mut *l` 独占借用与 `getcurrenv(l)` 的裸读点交叠。外层收形列入
/// 未尽事项，交主控另票排入。
pub(crate) unsafe fn lua_r_cloneclass(
  l: *mut LuaState,
  classobject: *mut LuauClass,
) -> *mut LuauClass {
  // SAFETY: 前置契约保证 l/类对象存活，克隆构造与 lua_h_clone 均在受保护帧内
  unsafe {
    let newclass = lua_r_newblankclass(&mut *l, (*classobject).name, (*classobject).isopen);

    // newclass was just allocated, so it is white and none of the writes below
    // need a write barrier.（cpp lclass.cpp:129 同款断言）
    LUAU_ASSERT!(iswhite!(obj2gco!(newclass)));

    (*newclass).super_ = (*classobject).super_;
    (*newclass).hasuserinitinchain = (*classobject).hasuserinitinchain;

    let numallmembers = (*classobject).numberofallmembers;
    let numstaticmembers = numallmembers - (*classobject).numberofinstancemembers;

    // cpp lclass.cpp:131-132 的行内注释称 "the clone shares it"，但紧接着的代码
    // （lclass.cpp:133）实为 `luaH_clone(L, classobject->memberstooffset)` 独立
    // 拷贝——cpp 注释与代码分歧，以 cpp 代码行为为准：NEWCLASS 每次执行都克隆类
    // 形状，后续 luaR_inheritclass 会对 memberstooffset 整体上移父类实例成员数
    // （cpp lclass.cpp:227-241），共享模板表即写穿 proto 常量表里的共享形状，
    // 同一主闭包二次执行即偏移越界。
    (*newclass).memberstooffset = lua_h_clone(l, (*classobject).memberstooffset);

    (*newclass).offsettomember = luaM_newarray!(l, numallmembers, *mut tstring, (*newclass).memcat);
    // 成员名数组克隆改切片对称形（镜像 r11-vmud/fef1e75）：源共享窗+目标独占窗
    // 等长 copy_from_slice，界长由各自分配真值给出，越界由 UB 降 panic
    c_slice_mut((*newclass).offsettomember, numallmembers as usize).copy_from_slice(c_slice(
      (*classobject).offsettomember,
      numallmembers as usize,
    ));

    (*newclass).numberofallmembers = numallmembers;

    (*newclass).staticmembers = luaM_newarray!(l, numstaticmembers, TValue, (*newclass).memcat);
    // 静态成员值数组克隆同款切片对称形；TValue 为 #[derive(Clone, Copy)] POD，
    // copy_from_slice 逐位即原 memcpy 语义
    c_slice_mut((*newclass).staticmembers, numstaticmembers as usize).copy_from_slice(c_slice(
      (*classobject).staticmembers,
      numstaticmembers as usize,
    ));

    (*newclass).numberofinstancemembers = (*classobject).numberofinstancemembers;

    if !(*classobject).instancemetatable.is_null() {
      (*newclass).instancemetatable = lua_h_clone(l, (*classobject).instancemetatable);
    }

    // r16-v22 拆语句：`setupconstructor` 收形后其首个实参位需 `&mut *l`（借用窗仅在当句内），
    // 若与 `getcurrenv(l)`（体内对 `(*l).ci`/`(*l).gt`/闭包 env 的裸读）同句求值，`&mut` 独占
    // 与 `getcurrenv` 的解引用会交叠（Stacked Borrows UB 风险）。按仓内「拆语句、原位现读、
    // 句间无字段写」判例分列两句：先现读环境，再转调构造器注册；`(*newclass)` 侧字段写与
    // `lua_c_barrier` 义务均已在上一段完成，两句间无 `l` 场写字段，时序不变。
    let env = getcurrenv(l);
    lua_r_setupconstructor(&mut *l, newclass, env);

    newclass
  }
}
