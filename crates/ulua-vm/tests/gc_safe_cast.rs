//! `GcObject` 安全视图门面的契约测试。
//! 边界契约测试：null 系 GC 视图/裸指针合法哨兵实参（既有约定 review.md §2），见 §9.3 保留理由。
//!
//! 生产侧 10 个 GC 类型 × 5 套同构访问器（`as_X`/`as_X_mut`/`try_as_X`/
//! `try_as_X_ref`/`try_as_X_ptr`）由 `macros/gc_object_accessors.rs` 单源生成，
//! 契约逐字同形，故此处以一条模板宏取代原先 11 份复制粘贴块：每类都跑
//! 「自身命中 + 缺省借用/空指针短路 + 通用视图解构」，并把原先只有 Table
//! 才有的 `None`/`null_mut()` 负例补齐到所有类型，覆盖只增不减。

use core::ptr::null_mut;

use ulua_vm::{
  enums::lua_type::LuaType,
  records::{
    g_cheader::GCheader,
    gc_object::{
      GCObject, GcView, GcViewMut, try_as_buffer, try_as_buffer_ptr, try_as_buffer_ref,
      try_as_class, try_as_class_ptr, try_as_class_ref, try_as_closure, try_as_closure_ptr,
      try_as_closure_ref, try_as_object, try_as_object_ptr, try_as_object_ref, try_as_proto,
      try_as_proto_ptr, try_as_proto_ref, try_as_string, try_as_string_ptr, try_as_string_ref,
      try_as_table, try_as_table_ptr, try_as_table_ref, try_as_thread, try_as_thread_ptr,
      try_as_thread_ref, try_as_udata, try_as_udata_ptr, try_as_udata_ref, try_as_upval,
      try_as_upval_ptr, try_as_upval_ref, try_as_view, try_as_view_mut,
    },
  },
};

/// 正向模板：类型标签命中时，五套视图全部解构成功，缺省入参安全短路。
macro_rules! assert_views_match {
  (
    $tt:path,
    $as:ident,
    $as_mut:ident,
    $try:ident,
    $try_ref:ident,
    $try_ptr:ident,
    $var:ident $(,)?
  ) => {{
    let mut obj = GCObject {
      gch: GCheader {
        tt: $tt as u8,
        marked: 0,
        memcat: 0,
      },
    };

    assert_eq!(obj.lua_type(), Some($tt));

    // 成员方法形态（只读与可变）
    assert!(obj.$as().is_some());
    assert!(obj.$as_mut().is_some());

    // 枚举视图解构
    assert!(matches!(obj.as_view(), Some(GcView::$var(_))));
    assert!(matches!(obj.as_view_mut(), Some(GcViewMut::$var(_))));

    // 借用入参的安全门面
    assert!($try(Some(&mut obj)).is_some());
    assert!($try_ref(Some(&obj)).is_some());

    // 裸指针形态的 *_ptr 变体（freeobj/lua_replace 的 C 形状消费方契约）
    let obj_ptr = &mut obj as *mut GCObject;
    unsafe {
      assert!($try_ptr(obj_ptr).is_some());
    }

    // 缺省借用与空指针哨兵（原空指针路径）
    assert!($try(None).is_none());
    assert!($try_ref(None).is_none());
    unsafe {
      assert!($try_ptr(null_mut()).is_none());
    }
  }};
}

/// 反向模板：标签为 `$tt` 的对象，其余类型的视图一律拒绝。
macro_rules! assert_views_reject {
  ($obj:expr, [$($as:ident),* $(,)?], [$($try:ident),* $(,)?]) => {{
    $(
      assert!(
        $obj.$as().is_none(),
        concat!("异类型标签不应解构出 ", stringify!($as))
      );
    )*
    $(
      assert!($try(Some(&mut $obj)).is_none());
    )*
  }};
}

#[test]
fn test_gcheader_predicates() {
  let mut hdr = GCheader {
    tt: LuaType::Table as u8,
    marked: 0b0000_0001,
    memcat: 2,
  };

  assert_eq!(hdr.lua_type(), Some(LuaType::Table));
  assert!(hdr.is_table());
  assert!(!hdr.is_closure());
  assert!(!hdr.is_proto());
  assert!(!hdr.is_string());
  assert!(!hdr.is_userdata());
  assert!(!hdr.is_thread());
  assert!(!hdr.is_buffer());
  assert!(!hdr.is_class());
  assert!(!hdr.is_object());
  assert!(!hdr.is_upval());
  assert!(!hdr.is_nil());

  hdr.tt = LuaType::Function as u8;
  assert!(hdr.is_closure());
  assert!(!hdr.is_table());

  hdr.tt = LuaType::Nil as u8;
  assert!(hdr.is_nil());
}

/// `GCObject` 头部三个透传字段、通用视图门面与跨类型拒绝。
#[test]
fn test_gcobject_header_and_rejection() {
  let mut obj = GCObject {
    gch: GCheader {
      tt: LuaType::Table as u8,
      marked: 0b0000_0010,
      memcat: 3,
    },
  };

  assert_eq!(obj.tt(), LuaType::Table as u8);
  assert_eq!(obj.marked(), 0b0000_0010);
  assert_eq!(obj.memcat(), 3);
  assert_eq!(obj.lua_type(), Some(LuaType::Table));

  // 通用视图门面的缺省入参同样以 `None` 表达
  assert!(try_as_view(None).is_none());
  assert!(try_as_view_mut(None).is_none());

  // Table 标签下其余 9 类视图全部拒绝（union 读前先匹配 tag）
  assert_views_reject!(
    obj,
    [
      as_closure, as_proto, as_string, as_udata, as_thread, as_buffer, as_class, as_object,
      as_upval
    ],
    [try_as_closure, try_as_proto, try_as_string]
  );
  let obj_ptr = &mut obj as *mut GCObject;
  unsafe {
    assert!(try_as_closure_ptr(obj_ptr).is_none());
  }
}

/// 10 个 GC 类型标签与 `as_view`/`try_as_*` 三形态的对称契约。
#[test]
fn test_gcobject_all_type_views() {
  assert_views_match!(
    LuaType::Table,
    as_table,
    as_table_mut,
    try_as_table,
    try_as_table_ref,
    try_as_table_ptr,
    Table
  );
  assert_views_match!(
    LuaType::Function,
    as_closure,
    as_closure_mut,
    try_as_closure,
    try_as_closure_ref,
    try_as_closure_ptr,
    Closure
  );
  assert_views_match!(
    LuaType::Proto,
    as_proto,
    as_proto_mut,
    try_as_proto,
    try_as_proto_ref,
    try_as_proto_ptr,
    Proto
  );
  assert_views_match!(
    LuaType::String,
    as_string,
    as_string_mut,
    try_as_string,
    try_as_string_ref,
    try_as_string_ptr,
    String
  );
  assert_views_match!(
    LuaType::UserData,
    as_udata,
    as_udata_mut,
    try_as_udata,
    try_as_udata_ref,
    try_as_udata_ptr,
    UserData
  );
  assert_views_match!(
    LuaType::Thread,
    as_thread,
    as_thread_mut,
    try_as_thread,
    try_as_thread_ref,
    try_as_thread_ptr,
    Thread
  );
  assert_views_match!(
    LuaType::Buffer,
    as_buffer,
    as_buffer_mut,
    try_as_buffer,
    try_as_buffer_ref,
    try_as_buffer_ptr,
    Buffer
  );
  assert_views_match!(
    LuaType::Class,
    as_class,
    as_class_mut,
    try_as_class,
    try_as_class_ref,
    try_as_class_ptr,
    Class
  );
  assert_views_match!(
    LuaType::Object,
    as_object,
    as_object_mut,
    try_as_object,
    try_as_object_ref,
    try_as_object_ptr,
    Object
  );
  assert_views_match!(
    LuaType::Upval,
    as_upval,
    as_upval_mut,
    try_as_upval,
    try_as_upval_ref,
    try_as_upval_ptr,
    UpVal
  );
}
