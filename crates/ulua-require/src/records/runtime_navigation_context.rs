use alloc::{vec, vec::Vec};
use core::{
  cell::Cell,
  ffi::{c_char, c_void},
};

use coarsetime::Instant;

use crate::{
  enums::{
    config_behavior::ConfigBehavior, config_status::ConfigStatus,
    luarequire_write_result::LuarequireWriteResult, navigate_result::NavigateResult,
  },
  functions::{
    c_str_prefix::with_c_str, convert_config_status::convert_config_status,
    convert_navigate_result::convert_navigate_result,
  },
  records::{
    luarequire_configuration::{
      AliasWriterFn, NavWithInputFn, QueryFn, WriterFn, luarequire_Configuration,
    },
    runtime_luau_config_timer::RuntimeLuauConfigTimer,
  },
};

/// 标识符类字符串（chunkname/loadname/cache_key/alias）的初始缓冲区大小
/// （对应 C++ `initalIdentifierBufferSize`）。
pub(crate) const INITIAL_IDENTIFIER_BUFFER_SIZE: usize = 64;

/// 配置文件内容的初始缓冲区大小（对应 C++ `initalFileBufferSize`）。
pub(crate) const INITIAL_FILE_BUFFER_SIZE: usize = 1024;

/// 运行时导航上下文：借用 Lua 栈上 userdata 内的配置（裸指针已在
/// `lua_requireinternal` 的真 FFI 边界收口为共享引用），其余成员沿用
/// cpp `RuntimeNavigationContext` 的形态。
pub(crate) struct RuntimeNavigationContext<'ctx> {
  pub(crate) config: &'ctx luarequire_Configuration,
  pub(crate) l: *mut c_void,
  pub(crate) ctx: *mut c_void,
  /// 借用调用方提供的 requirer chunkname 字节串（Lua 字符串非 UTF-8），
  /// 避免每次 require 分配；只在跨 C-ABI 时经 `call_with_c_str` /
  /// `is_require_allowed` 收口点补 NUL
  pub(crate) requirer_chunkname: &'ctx [u8],
  pub(crate) timer: RuntimeLuauConfigTimer,
}

/// C writer 调用形态：无输入，或带一个字节串输入（仅在真 FFI 边界补 NUL）。
enum Writer<'a> {
  Plain(WriterFn),
  WithInput(AliasWriterFn, &'a [u8]),
}

impl Writer<'_> {
  /// 调用 C writer 写入缓冲区，返回写入结果。
  ///
  /// # Safety
  /// `l`/`ctx`/`buffer` 须为调用 C writer 所需的合法指针；
  /// writer 回调须按 `buffer_size` 约束写入并在 `size_out` 返回长度。
  unsafe fn call(
    &self,
    l: *mut c_void,
    ctx: *mut c_void,
    buffer: *mut c_char,
    buffer_size: usize,
    size_out: &mut usize,
  ) -> LuarequireWriteResult {
    match *self {
      // Safety: l/ctx 来自导航上下文捕获且调用期存活的 state 与 lightuserdata；buffer/buffer_size 指向同一本地 Vec 的可写容量，size_out 为本地 &mut；writer 按配置契约不得越界写入。
      Writer::Plain(write) => unsafe { write(l, ctx, buffer, buffer_size, size_out) },
      // Safety: 同上，另 input 由 with_c_str 补 NUL、仅本次回调存活；其余指针均为上下文/本地缓冲派生。
      // with_c_str 在此不可省：`AliasWriterFn` 的 alias 形参按 cpp `Require.h` 是
      // `*const c_char`（宿主 extern "C-unwind" 回调，跨语言契约定格）。
      Writer::WithInput(write, input) => with_c_str(input, |input| unsafe {
        write(l, ctx, input, buffer, buffer_size, size_out)
      }),
    }
  }
}

impl<'ctx> RuntimeNavigationContext<'ctx> {
  /// 组装运行时导航上下文；`config`/`requirer_chunkname` 由借用保证存活期，
  /// `l`/`ctx` 只是转手给 C 回调的不透明指针，本类型不对其解引用。
  pub(crate) fn new(
    config: &'ctx luarequire_Configuration,
    l: *mut c_void,
    ctx: *mut c_void,
    requirer_chunkname: &'ctx [u8],
  ) -> Self {
    Self {
      config,
      l,
      ctx,
      requirer_chunkname,
      timer: RuntimeLuauConfigTimer {
        start_time: Cell::new(Instant::now()),
        timeout_duration: Cell::new(None),
      },
    }
  }

  /// 无额外实参的只读 C 查询回调的唯一调用收口（`to_parent` / `is_module_present` /
  /// `get_config_status` 三处共用的 `(l, ctx)` 形态）：回调缺失时返回 `None`，
  /// 缺省值与结果转换的语义留在调用方，门面不代为决定。
  fn query<T>(&self, cb: Option<QueryFn<T>>) -> Option<T> {
    let cb = cb?;
    // Safety: self.l/self.ctx 是 RuntimeNavigationContext::new 自 require 闭包 upvalue
    // 捕获、导航期间存活的 state 与 lightuserdata；cb 为 validate_config 确认存在的
    // 只读查询回调，除这两个指针外无其它实参。
    Some(unsafe { cb(self.l, self.ctx) })
  }

  /// 带一个字节串入参的导航类 C 回调（`reset`/`jump_to_alias`/`to_child`/
  /// `to_alias_override`/`to_alias_fallback`）的唯一调用收口：回调缺失时返回 NotFound。
  ///
  /// 这里的 `with_c_str` 是真 C-ABI 边界而非过渡垫片：`NavWithInputFn` 是
  /// `unsafe extern "C-unwind" fn(.., input: *const c_char)`，与 cpp `Require.h`
  /// 的宿主回调契约共用（`luarequire_configuration` 已定格、跨 crate 公开），
  /// 无法改成 `&[u8]`/`&str` 形参；输入按字节串传进来，只在此处补 NUL
  /// （短名走栈内联缓冲，零分配），指针不外泄。
  fn call_with_c_str(&self, nav: Option<NavWithInputFn>, input: &[u8]) -> NavigateResult {
    nav.map_or(NavigateResult::NotFound, |nav| {
      // Safety: self.l/self.ctx 是 RuntimeNavigationContext::new 自 require 闭包 upvalue 捕获的 state 与 lightuserdata，导航期间存活；nav 为 validate_config 确认存在的配置回调；input 指针由 with_c_str 按首个 NUL 截断并补零，仅闭包调用期内有效。
      convert_navigate_result(with_c_str(input, |input| unsafe {
        nav(self.l, self.ctx, input)
      }))
    })
  }

  /// 通过 C writer 获取字节串：先按初始缓冲区调用，`BUFFER_TOO_SMALL` 时按
  /// `size_out` 扩容重试一次；成功时按 `size_out` 截取原样返回字节。
  ///
  /// 与 cpp `getStringFromCWriter` 一致：结果按 `std::string` 字节返回，
  /// 不做 UTF-8 校验（chunkname/loadname/cache_key 可能是任意字节）。
  fn write_bytes(&self, writer: Writer<'_>, initial_buffer_size: usize) -> Option<Vec<u8>> {
    let mut buffer = vec![0; initial_buffer_size];
    let mut size: usize = 0;
    // C 写入器调用唯一收口：Vec<u8> 缓冲只在这里以可写指针形态跨过 C-ABI 边界
    // Safety: 闭包每次调用时 buffer 借出独占可写指针（Vec 存活期即借用期），writer.call 的前置条件由上述 l/ctx/buffer 不变量满足；C 指针不外泄，返回后借用复位。
    let call = |buffer: &mut Vec<u8>, size: &mut usize| unsafe {
      writer.call(
        self.l,
        self.ctx,
        buffer.as_mut_ptr().cast::<c_char>(),
        buffer.len(),
        size,
      )
    };

    let mut result = call(&mut buffer, &mut size);
    if result == LuarequireWriteResult::WriteBufferTooSmall {
      // 按 size_out 扩容后重试一次（cpp 同语义，不做无界循环）
      buffer.resize(size, 0);
      result = call(&mut buffer, &mut size);
    }

    if result != LuarequireWriteResult::WriteSuccess {
      return None;
    }

    // cpp/Navigation.cpp:155-158 仅 `buffer.resize(size)`：size_out 即内容字节数，
    // 不剥任何结尾零。
    let end = size.min(buffer.len());
    buffer.truncate(end);
    Some(buffer)
  }

  pub(crate) fn get_string_from_c_writer(
    &self,
    writer: WriterFn,
    initial_buffer_size: usize,
  ) -> Option<Vec<u8>> {
    self.write_bytes(Writer::Plain(writer), initial_buffer_size)
  }

  /// 复位到 requirer 所在目录（cpp `reset(requirerChunkname.c_str())`）：入参是
  /// 上下文借用的 chunkname 字节串，跨 C-ABI 的补 NUL 由 `call_with_c_str` 收口。
  pub(crate) fn reset_to_requirer(&mut self) -> NavigateResult {
    self.call_with_c_str(self.config.reset, self.requirer_chunkname)
  }

  pub(crate) fn jump_to_alias(&mut self, path: &[u8]) -> NavigateResult {
    self.call_with_c_str(self.config.jump_to_alias, path)
  }

  pub(crate) fn to_alias_override(&self, alias_unprefixed: &[u8]) -> NavigateResult {
    self.call_with_c_str(self.config.to_alias_override, alias_unprefixed)
  }

  pub(crate) fn to_alias_fallback(&self, alias_unprefixed: &[u8]) -> NavigateResult {
    self.call_with_c_str(self.config.to_alias_fallback, alias_unprefixed)
  }

  /// 回调缺失（`validate_config` 之后不可达）时按 cpp 的缺省给 `NotFound`。
  pub(crate) fn to_parent(&self) -> NavigateResult {
    self
      .query(self.config.to_parent)
      .map_or(NavigateResult::NotFound, convert_navigate_result)
  }

  pub(crate) fn to_child(&self, component: &[u8]) -> NavigateResult {
    self.call_with_c_str(self.config.to_child, component)
  }

  /// 回调缺失（`validate_config` 之后不可达）按 cpp 的「查询不通过」处理，返回 false。
  pub(crate) fn is_module_present(&self) -> bool {
    self.query(self.config.is_module_present).unwrap_or(false)
  }

  /// 当前上下文的 chunkname 字节串（cpp `getChunkname()`）。
  pub(crate) fn get_chunkname(&self) -> Option<Vec<u8>> {
    let writer = self.config.get_chunkname?;
    self.get_string_from_c_writer(writer, INITIAL_IDENTIFIER_BUFFER_SIZE)
  }

  /// 当前上下文的 loadname 字节串（cpp `getLoadname()`）：可能含非 UTF-8 字节。
  pub(crate) fn get_loadname(&self) -> Option<Vec<u8>> {
    let writer = self.config.get_loadname?;
    self.get_string_from_c_writer(writer, INITIAL_IDENTIFIER_BUFFER_SIZE)
  }

  /// 当前上下文的缓存键字节串（cpp `getCacheKey()`）：可能含非 UTF-8 字节。
  pub(crate) fn get_cache_key(&self) -> Option<Vec<u8>> {
    let writer = self.config.get_cache_key?;
    self.get_string_from_c_writer(writer, INITIAL_IDENTIFIER_BUFFER_SIZE)
  }

  /// 回调缺失（`validate_config` 之后不可达）时按 cpp 的缺省给 `Absent`。
  pub(crate) fn get_config_status(&self) -> ConfigStatus {
    self
      .query(self.config.get_config_status)
      .map_or(ConfigStatus::Absent, convert_config_status)
  }

  pub(crate) fn get_config_behavior(&self) -> ConfigBehavior {
    if self.config.get_alias.is_some() {
      ConfigBehavior::GetAlias
    } else {
      ConfigBehavior::GetConfig
    }
  }

  /// 按别名字节查询其映射路径（cpp `getAlias(alias)`）：入参与返回值都是字节串。
  pub(crate) fn get_alias(&self, alias: &[u8]) -> Option<Vec<u8>> {
    let writer = self.config.get_alias?;
    self.get_string_from_c_writer_with_input(writer, alias, INITIAL_IDENTIFIER_BUFFER_SIZE)
  }

  /// 配置文件原始字节（cpp `getConfig()` 返回 `std::string`）：
  /// 交给 ulua-config 解析前才在边界做 UTF-8 转换。
  pub(crate) fn get_config(&self) -> Option<Vec<u8>> {
    let writer = self.config.get_config?;
    self.get_string_from_c_writer(writer, INITIAL_FILE_BUFFER_SIZE)
  }

  fn get_string_from_c_writer_with_input(
    &self,
    writer: AliasWriterFn,
    input: &[u8],
    initial_buffer_size: usize,
  ) -> Option<Vec<u8>> {
    self.write_bytes(Writer::WithInput(writer, input), initial_buffer_size)
  }
}
