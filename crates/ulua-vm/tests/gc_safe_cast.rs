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

#[test]
fn test_gcobject_table_safe_views() {
  let mut obj = GCObject {
    gch: GCheader {
      tt: LuaType::Table as u8,
      marked: 0,
      memcat: 1,
    },
  };

  assert_eq!(obj.tt(), LuaType::Table as u8);
  assert_eq!(obj.marked(), 0);
  assert_eq!(obj.memcat(), 1);
  assert_eq!(obj.lua_type(), Some(LuaType::Table));

  // 只读与可变视图
  assert!(obj.as_table().is_some());
  assert!(obj.as_table_mut().is_some());
  assert!(obj.as_closure().is_none());
  assert!(obj.as_closure_mut().is_none());
  assert!(obj.as_proto().is_none());
  assert!(obj.as_string().is_none());
  assert!(obj.as_udata().is_none());
  assert!(obj.as_thread().is_none());
  assert!(obj.as_buffer().is_none());
  assert!(obj.as_class().is_none());
  assert!(obj.as_object().is_none());
  assert!(obj.as_upval().is_none());

  // 枚举视图解构
  match obj.as_view() {
    Some(GcView::Table(_)) => {}
    _ => panic!("预期为 GcView::Table"),
  }

  match obj.as_view_mut() {
    Some(GcViewMut::Table(_)) => {}
    _ => panic!("预期为 GcViewMut::Table"),
  }

  // 裸指针安全辅助函数
  let obj_ptr = &mut obj as *mut GCObject;
  unsafe {
    assert!(try_as_table(obj_ptr).is_some());
    assert!(try_as_table_ref(obj_ptr).is_some());
    assert!(try_as_table_ptr(obj_ptr).is_some());

    assert!(try_as_closure(obj_ptr).is_none());
    assert!(try_as_closure_ref(obj_ptr).is_none());
    assert!(try_as_closure_ptr(obj_ptr).is_none());

    // 空指针安全
    assert!(try_as_table(null_mut()).is_none());
    assert!(try_as_table_ref(null_mut()).is_none());
    assert!(try_as_table_ptr(null_mut()).is_none());
    assert!(try_as_view(null_mut()).is_none());
    assert!(try_as_view_mut(null_mut()).is_none());
  }
}

#[test]
fn test_gcobject_closure_and_proto_views() {
  let mut obj = GCObject {
    gch: GCheader {
      tt: LuaType::Function as u8,
      marked: 0,
      memcat: 0,
    },
  };

  assert!(obj.as_closure().is_some());
  assert!(obj.as_closure_mut().is_some());
  assert!(obj.as_table().is_none());

  let obj_ptr = &mut obj as *mut GCObject;
  unsafe {
    assert!(try_as_closure(obj_ptr).is_some());
    assert!(try_as_closure_ref(obj_ptr).is_some());
    assert!(try_as_closure_ptr(obj_ptr).is_some());
  }

  // Proto 测试
  let mut proto_obj = GCObject {
    gch: GCheader {
      tt: LuaType::Proto as u8,
      marked: 0,
      memcat: 0,
    },
  };
  assert!(proto_obj.as_proto().is_some());
  assert!(proto_obj.as_proto_mut().is_some());
  assert!(proto_obj.as_closure().is_none());

  let proto_ptr = &mut proto_obj as *mut GCObject;
  unsafe {
    assert!(try_as_proto(proto_ptr).is_some());
    assert!(try_as_proto_ref(proto_ptr).is_some());
    assert!(try_as_proto_ptr(proto_ptr).is_some());
  }
}

#[test]
fn test_gcobject_other_types_views() {
  // String
  let mut str_obj = GCObject {
    gch: GCheader {
      tt: LuaType::String as u8,
      marked: 0,
      memcat: 0,
    },
  };
  assert!(str_obj.as_string().is_some());
  assert!(str_obj.as_string_mut().is_some());
  unsafe {
    assert!(try_as_string(&mut str_obj).is_some());
    assert!(try_as_string_ref(&str_obj).is_some());
    assert!(try_as_string_ptr(&mut str_obj).is_some());
  }

  // UserData
  let mut udata_obj = GCObject {
    gch: GCheader {
      tt: LuaType::UserData as u8,
      marked: 0,
      memcat: 0,
    },
  };
  assert!(udata_obj.as_udata().is_some());
  assert!(udata_obj.as_udata_mut().is_some());
  unsafe {
    assert!(try_as_udata(&mut udata_obj).is_some());
    assert!(try_as_udata_ref(&udata_obj).is_some());
    assert!(try_as_udata_ptr(&mut udata_obj).is_some());
  }

  // Thread
  let mut th_obj = GCObject {
    gch: GCheader {
      tt: LuaType::Thread as u8,
      marked: 0,
      memcat: 0,
    },
  };
  assert!(th_obj.as_thread().is_some());
  assert!(th_obj.as_thread_mut().is_some());
  unsafe {
    assert!(try_as_thread(&mut th_obj).is_some());
    assert!(try_as_thread_ref(&th_obj).is_some());
    assert!(try_as_thread_ptr(&mut th_obj).is_some());
  }

  // Buffer
  let mut buf_obj = GCObject {
    gch: GCheader {
      tt: LuaType::Buffer as u8,
      marked: 0,
      memcat: 0,
    },
  };
  assert!(buf_obj.as_buffer().is_some());
  assert!(buf_obj.as_buffer_mut().is_some());
  unsafe {
    assert!(try_as_buffer(&mut buf_obj).is_some());
    assert!(try_as_buffer_ref(&buf_obj).is_some());
    assert!(try_as_buffer_ptr(&mut buf_obj).is_some());
  }

  // Class
  let mut class_obj = GCObject {
    gch: GCheader {
      tt: LuaType::Class as u8,
      marked: 0,
      memcat: 0,
    },
  };
  assert!(class_obj.as_class().is_some());
  assert!(class_obj.as_class_mut().is_some());
  unsafe {
    assert!(try_as_class(&mut class_obj).is_some());
    assert!(try_as_class_ref(&class_obj).is_some());
    assert!(try_as_class_ptr(&mut class_obj).is_some());
  }

  // Object
  let mut inst_obj = GCObject {
    gch: GCheader {
      tt: LuaType::Object as u8,
      marked: 0,
      memcat: 0,
    },
  };
  assert!(inst_obj.as_object().is_some());
  assert!(inst_obj.as_object_mut().is_some());
  unsafe {
    assert!(try_as_object(&mut inst_obj).is_some());
    assert!(try_as_object_ref(&inst_obj).is_some());
    assert!(try_as_object_ptr(&mut inst_obj).is_some());
  }

  // UpVal
  let mut uv_obj = GCObject {
    gch: GCheader {
      tt: LuaType::Upval as u8,
      marked: 0,
      memcat: 0,
    },
  };
  assert!(uv_obj.as_upval().is_some());
  assert!(uv_obj.as_upval_mut().is_some());
  unsafe {
    assert!(try_as_upval(&mut uv_obj).is_some());
    assert!(try_as_upval_ref(&uv_obj).is_some());
    assert!(try_as_upval_ptr(&mut uv_obj).is_some());
  }
}
