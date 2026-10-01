use alloc::{boxed::Box, vec::Vec};

use ulua_config::records::interrupt_callbacks::ConfigInitCallback;
use ulua_vm::records::lua_state::LuaState;

use crate::enums::{
  config_behavior::ConfigBehavior, config_status::ConfigStatus, navigate_result::NavigateResult,
};

/// C++ 中配置回调缺省时的 Luau 配置执行超时（毫秒，`Require.h` 注释定死 2000）。
pub const DEFAULT_LUAU_CONFIG_TIMEOUT_MS: i32 = 2000;

/// 导航上下文接口，对应 cpp `RequireNavigator.h` 的纯虚基类 `NavigationContext`：
/// [`crate::records::navigator::Navigator`] 沿宿主层级遍历 require 路径时调用的
/// 全部能力。由注入方实现（静态工具直接实现此面；require 运行时经
/// [`crate::records::runtime_navigation_context::RuntimeNavigationContext`]
/// 适配 [`RequireHost`]）。
///
/// 路径 / 别名 / 组件一律是字节串（cpp 为 `std::string`），实现方不得做 UTF-8
/// 校验或替换；`get_alias`/`get_config` 的返回值同样是原始字节。
///
/// 接收者统一为 `&self`：导航状态的可变性由实现方按内部可变性自持
/// （`Cell`/`RefCell`），使共享引用即可驱动导航——require 链路允许重入
/// （循环 require 时一个 `load` 尚未返回、另一个 require 已在同一上下文上
/// 导航），`&mut self` 形态在这种嵌套窗口里无法合法重建。
pub trait NavigationContext {
  fn reset_to_requirer(&self) -> NavigateResult;
  fn jump_to_alias(&self, alias_path: &[u8]) -> NavigateResult;

  fn to_alias_override(&self, _alias_unprefixed: &[u8]) -> NavigateResult {
    NavigateResult::NotFound
  }

  fn to_alias_fallback(&self, _alias_unprefixed: &[u8]) -> NavigateResult {
    NavigateResult::NotFound
  }

  fn to_parent(&self) -> NavigateResult;
  fn to_child(&self, component: &[u8]) -> NavigateResult;

  fn get_config_status(&self) -> ConfigStatus {
    ConfigStatus::Absent
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    ConfigBehavior::GetAlias
  }

  fn get_alias(&self, _alias: &[u8]) -> Option<Vec<u8>> {
    None
  }

  fn get_config(&self) -> Option<Vec<u8>> {
    None
  }

  /// Luau 配置执行前的线程数据初始化回调。
  ///
  /// 返回 [`ConfigInitCallback`]（函数指针 + 转手 userdata 的静态分派对，
  /// 零分配零虚分派）：实现方给出的 `callback` 只会在配置执行的同步窗口内
  /// 被调用一次，`userdata` 指向该窗口内存活的数据即可。
  fn luau_config_init(&self) -> Option<ConfigInitCallback> {
    None
  }

  fn luau_config_interrupt(
    &self,
  ) -> Option<unsafe extern "C-unwind" fn(l: *mut LuaState, gc: i32)> {
    None
  }
}

/// require 执行宿主机（require-by-string 运行库的宿主面），对应 cpp
/// `luarequire_Configuration` 函数指针表的 Rust 原生形态：导航面（cpp
/// `RuntimeNavigationContext` 转接 `NavigationContext` 所需的各项）、模块标识
/// 查询与装载入口。全部方法以字节串入参、以值返回（cpp 的 out 参缓冲协议
/// 是 C ABI 的产物，此处不存在）。
///
/// 与 cpp 的必填回调清单对应的必需方法（无缺省实现，缺实现即编译错误，
/// 替代 `validateConfig` 的运行期判空）：
/// `is_require_allowed`/`reset_to_requirer`/`jump_to_alias`/`to_parent`/
/// `to_child`/`is_module_present`/`get_chunkname`/`get_loadname`/
/// `get_cache_key`/`load`。`to_alias_override`/`to_alias_fallback`/
/// `get_config_status`/`get_config_behavior`/`get_alias`/`get_config`/
/// `get_luau_config_timeout` 对应 cpp 可留空的槽位，按 cpp 缺省语义给默认。
///
/// `get_alias` 与 `get_config` 二选一由 `get_config_behavior` 明示（cpp 用
/// 「哪枚指针已置」隐式判定，Rust 侧不存在该可观察量）。
pub trait RequireHost {
  /// 是否允许从该 requirer chunkname 发起 require（cpp `is_require_allowed`）。
  fn is_require_allowed(&self, requirer_chunkname: &[u8]) -> bool;

  /// 复位到 requirer（chunkname 所指模块）所在层级（cpp `reset`）。
  fn reset_to_requirer(&self, requirer_chunkname: &[u8]) -> NavigateResult;
  /// 跳到别名配置的绝对路径（cpp `jump_to_alias`）。
  fn jump_to_alias(&self, alias_path: &[u8]) -> NavigateResult;

  fn to_alias_override(&self, _alias_unprefixed: &[u8]) -> NavigateResult {
    NavigateResult::NotFound
  }

  fn to_alias_fallback(&self, _alias_unprefixed: &[u8]) -> NavigateResult {
    NavigateResult::NotFound
  }

  fn to_parent(&self) -> NavigateResult;
  fn to_child(&self, component: &[u8]) -> NavigateResult;

  /// 当前层级是否指向一个模块（cpp `is_module_present`）。
  fn is_module_present(&self) -> bool;

  /// 当前模块的 chunkname（cpp `get_chunkname` writer；可能含非 UTF-8 字节）。
  fn get_chunkname(&self) -> Option<Vec<u8>>;
  /// 当前模块的 loadname（cpp `get_loadname` writer）。
  fn get_loadname(&self) -> Option<Vec<u8>>;
  /// 当前模块的缓存键（cpp `get_cache_key` writer）。
  fn get_cache_key(&self) -> Option<Vec<u8>>;

  fn get_config_status(&self) -> ConfigStatus {
    ConfigStatus::Absent
  }

  fn get_config_behavior(&self) -> ConfigBehavior {
    ConfigBehavior::GetAlias
  }

  /// 按别名字节查询其映射路径（cpp `get_alias` writer；行为为
  /// `GetAlias` 时必填）。
  fn get_alias(&self, _alias: &[u8]) -> Option<Vec<u8>> {
    None
  }

  /// 配置文件原始字节（cpp `get_config` writer；行为为 `GetConfig` 时提供）。
  fn get_config(&self) -> Option<Vec<u8>> {
    None
  }

  /// Luau 语法配置的执行超时（毫秒）；负值按无限处理（cpp
  /// `get_luau_config_timeout`，缺省 2000）。
  fn get_luau_config_timeout(&self) -> i32 {
    DEFAULT_LUAU_CONFIG_TIMEOUT_MS
  }

  /// 执行模块并把结果压在 `l` 栈顶，返回压入的结果数；返回 `-1` 表示要求
  /// 本线程挂起（cpp `load`）。`path`/`chunkname`/`loadname` 为 require 链路
  /// 解析出的字节串。
  fn load(&self, l: *mut LuaState, path: &[u8], chunkname: &[u8], loadname: &[u8]) -> i32;
}

/// require 宿主机的**唯一**槽位类型：注入方在建立闭包时把宿主按值装箱移交，
/// 随 require 闭包的 userdata 存活，取回侧按同一 `C` 借出共享引用。
///
/// 零 `dyn`（review.md §4）：原先此处是 `Box<dyn RequireHost>`（本 crate 唯一擦除
/// 点），理由是「非泛型 C 闭包体取回时 `C` 已不存在」。该前提不成立——闭包体
/// `lua_require` / `lua_proxyrequire` 本身就是 `pub(crate) unsafe extern "C-unwind"
/// fn …<C>`，注入点 `luarequire_pushrequire<C>` / `luarequire_pushproxyrequire<C>` /
/// `luaopen_require<C>` 以 `Some(lua_require::<C>)` 具名实例化后 coerce 成
/// `LuaCFunction` 函数指针：擦除发生在**函数指针**层（每宿主类型一个单态化体），
/// 而非 trait-object 层。实现集合静态可举（repl-cli `ReplRequirer`、其不透明
/// `impl RequireHost` 门面、测试 `MemHost`），泛型贯穿 require 链
/// （`lua_requireinternal<C>` → `resolve_require<H>` → `RuntimeNavigationContext<'_, H>`
/// → 已泛型的 `Navigator`）代价为每宿主类型一份单态化代码，无自引用/混合持有，
/// 故收敛为零 `dyn`。
///
/// 收成别名而非各处手写该装箱类型：分配尺寸（`size_of`）、GC 析构
/// （`drop_in_place`）与读回（`cast`）三处必须指向同一布局，原先四处同形拼写
/// 只靠注释约束，任一处改型即 UB；共用此定义后改型即编译失败。
pub(crate) type HostSlot<C> = Box<C>;

/// 同名固有方法 → [`NavigationContext`] 的机械转发单源：实现方把方法体
/// 放在类型自己的 impl 块（一文件一函数、对应 cpp 文件），trait impl 只按
/// 「方法名(参数名) -> 返回类型」单行声明，展开体把 `self`/参数原样交给
/// `Self::$name` 固有关联函数（固有候选优先于 trait 候选，无递归）。
/// 参数一律 `&[u8]`（路径/别名/组件的字节串约定）。
#[macro_export]
macro_rules! forward_nav_trait {
  ($name:ident($($arg:ident),*) -> $ret:ty) => {
    fn $name(&self, $($arg: &[u8]),*) -> $ret {
      Self::$name(self, $($arg),*)
    }
  };
}
