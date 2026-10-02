use crate::{
  enums::lua_type::LuaType,
  functions::{
    lua_g_readonlyerror::check_writable, lua_h_resizearray::lua_h_resizearray,
    moveelements::moveelements,
  },
  macros::{lua_lib_fn::lua_lib_fn, sizenode::sizenode},
  records::lua_state::LuaState,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub(crate) unsafe fn tmove(l: *mut LuaState) -> i32 {
  // SAFETY: 契约保证 l 为存活 C 闭包帧，check_type/check_integer 系既有 api 门面读面，
  // 两帧槽取表指针经 slot() 界内正索引换算（见下方定性注记），表字段裸读属各自记录红线面
  //
  // r14-p3 逐点定性（w6d 口径保留面）：源/目两帧槽 hvalue 式取表指针收编为
  // records/slot.rs 既有 api 索引域构造方法 slot()（非新增门面，形制照抄 moveelements
  // r13-w1a 收编判例的 get+as_table_ptr 读面同形）：check_type(1, Table) 与三段
  // check_integer 先行，C 帧契约令顶基窗深 ≥4，idx=1 走 index_2_addr 正索引界内分支
  // base.add(0)，与旧帧基址直接解引用逐位同址；tt∈{1,5} 已在取值点与 check_type(tt)
  // 两处判过槽位非 nil 表，正索引 off=tt-1 恒落于已用窗内 → base.add(tt-1) 与旧基址
  // 偏移读数逐位同址，nil 哨兵塌缩分支仅在违约域可达（取值判定即经同一换算读得该表，
  // 与 :43 既有 slot 域调用点同源，cpp 越界 UB 域无 oracle 输出）。表裸字段 sizearray
  // 读、lua_h_resizearray/check_writable/moveelements/push_value 与 arg_check 调用点
  // 系既有门面/自由函数取参面，保留。
  unsafe {
    (*l).check_type(1, LuaType::Table);
    let f = (*l).check_integer(2);
    let e = (*l).check_integer(3);
    let t = (*l).check_integer(4);
    let tt = if !(*l).is_none_or_nil(5) { 5 } else { 1 };

    (*l).check_type(tt, LuaType::Table);

    if e >= f {
      (*l).arg_check(f > 0 || e < i32::MAX + f, 3, "too many elements to move");
      let n = e - f + 1;
      (*l).arg_check(t <= i32::MAX - n + 1, 4, "destination wrap around");

      let src = (*l).slot(1).get().as_table_ptr();
      let dst = (*l).slot(tt).get().as_table_ptr();

      check_writable(l, dst);

      let srcelems = (*src).sizearray + sizenode!(src);
      let dstelems = (*dst).sizearray + sizenode!(dst);
      let maxelems = srcelems.max(dstelems);
      let minsparsemoveelems = 32;
      let sparsemove = n > minsparsemoveelems && n / 2 > maxelems;

      if t > 0 && (t - 1) <= (*dst).sizearray && (t - 1 + n) > (*dst).sizearray {
        lua_h_resizearray(l, dst, t - 1 + n);
      }

      moveelements(l, 1, tt, f, e, t, sparsemove);
    }

    (*l).push_value(tt);
    1
  }
}

lua_lib_fn!(pub(crate) fn tmove, tmove_arm);
