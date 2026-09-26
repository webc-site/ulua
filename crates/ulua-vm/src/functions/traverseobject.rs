use crate::{
  functions::c_slice,
  macros::{markobject::markobject, markvalue::markvalue},
  records::{global_state::global_State, luau_object::LuauObject},
};

/// # Safety
/// 调用方须保证：`g` 处于 GC mark 阶段且存活；`classinst` 为待传播的存活 Object 实例——
/// `lclass` 非空、`members` 数组按 `numberofmembers` 分配且各槽为合法 TValue；违反即 markvalue!
/// 越界读或对悬垂引用写灰标签。cpp lgc.cpp:482 `traverseobject`
pub(crate) unsafe fn traverseobject(g: *mut global_State, classinst: *mut LuauObject) {
  // Safety: 契约保证 `o` 为存活 collectable 对象且 setpointer 回调可安全替换其引用字段，块内按 LuaType 分支仅触达对象界内字段
  unsafe {
    // markobject(g, classinst->lclass);
    markobject!(g, (*classinst).lclass);

    // for (int i = 0; i < classinst->numberofmembers; i++)
    //     markvalue(g, &classinst->members[i]);
    let numberofmembers = (*classinst).numberofmembers;
    let members = (*classinst).members;
    for member in c_slice(members, numberofmembers as usize) {
      markvalue!(g, member);
    }
  }
}
