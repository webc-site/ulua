//! Source: `VM/src/lclass.h:51-54` (hand-fixed: the original translation used
//! C ternary syntax `? :` and could never have compiled). Yields `*mut TValue`.

#[macro_export]
macro_rules! luaR_lookupmemberatoffset {
  ($inst:expr, $offset:expr) => {{
    let inst = $inst;
    let offset = $offset;
    ulua_common::LUAU_ASSERT!(
      $crate::macros::lua_r_checkoffsetinbounds::luaR_checkoffsetinbounds!(inst, offset)
    );
    // 镜像 r11-vmud/fef1e75 切片化形：两段裸 `.add` 改窗内定位，界长各取分配真值
    // （实例段 = object.numberofmembers、静态段 = all-inst），越界由 UB 降 panic；
    // 展开式与改前同样要求调用点处于 unsafe 上下文（裸指针解引用），unsafe 面零涨
    let numberofinstancemembers = (*(*inst).lclass).numberofinstancemembers;
    if offset < numberofinstancemembers {
      ulua_common::functions::c_slice::c_slice_mut((*inst).members, (*inst).numberofmembers as usize)
        [offset as usize..]
        .as_mut_ptr()
    } else {
      let lclass = &*(*inst).lclass;
      ulua_common::functions::c_slice::c_slice_mut(
        lclass.staticmembers,
        (lclass.numberofallmembers - numberofinstancemembers) as usize,
      )[(offset - numberofinstancemembers) as usize..]
        .as_mut_ptr()
    }
  }};
}

pub use luaR_lookupmemberatoffset;
