use crate::enums::lua_type::LuaType;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
#[derive(Default)]
pub struct GCheader {
  pub tt: u8,
  pub marked: u8,
  pub memcat: u8,
}

impl GCheader {
  /// 获取当前 GC 头的类型枚举判别式
  #[inline]
  pub const fn lua_type(&self) -> Option<LuaType> {
    LuaType::from_repr(self.tt as i32)
  }

  /// 是否为表对象
  #[inline]
  pub const fn is_table(&self) -> bool {
    self.tt == LuaType::Table as u8
  }

  /// 是否为闭包函数
  #[inline]
  pub const fn is_closure(&self) -> bool {
    self.tt == LuaType::Function as u8
  }

  /// 是否为函数原型
  #[inline]
  pub const fn is_proto(&self) -> bool {
    self.tt == LuaType::Proto as u8
  }

  /// 是否为字符串
  #[inline]
  pub const fn is_string(&self) -> bool {
    self.tt == LuaType::String as u8
  }

  /// 是否为用户数据
  #[inline]
  pub const fn is_userdata(&self) -> bool {
    self.tt == LuaType::UserData as u8
  }

  /// 是否为线程/协程
  #[inline]
  pub const fn is_thread(&self) -> bool {
    self.tt == LuaType::Thread as u8
  }

  /// 是否为缓冲区
  #[inline]
  pub const fn is_buffer(&self) -> bool {
    self.tt == LuaType::Buffer as u8
  }

  /// 是否为类
  #[inline]
  pub const fn is_class(&self) -> bool {
    self.tt == LuaType::Class as u8
  }

  /// 是否为类实例
  #[inline]
  pub const fn is_object(&self) -> bool {
    self.tt == LuaType::Object as u8
  }

  /// 是否为 Upvalue
  #[inline]
  pub const fn is_upval(&self) -> bool {
    self.tt == LuaType::Upval as u8
  }

  /// 是否为空（已释放）块
  #[inline]
  pub const fn is_nil(&self) -> bool {
    self.tt == LuaType::Nil as u8
  }
}
