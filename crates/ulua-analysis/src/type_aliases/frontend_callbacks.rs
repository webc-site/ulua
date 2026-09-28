use alloc::{rc::Rc, string::String, vec::Vec};

use crate::{
  records::build_queue_work_state::Task,
  type_aliases::{module_name_type::ModuleName, scope_ptr_type::ScopePtr},
};
// 本文件的 dyn 均为 C++ `std::function` 成员/参数的直译：回调由宿主（含
// 其他 crate，如 ulua-unit-test 注入 prepare_module_scope、ulua-analyze-cli
// 注入 TaskQueue）在运行期闭盒构造，实现方无法穷举，泛型化会破坏跨 crate
// 公共 API，enum_dispatch 无适用空间，故 dyn 保留。
// 原 `BuiltinDefinitionsMap`（Frontend::builtin_definitions）已删：cpp
// Frontend 无此成员（BuiltinDefinitions 是 BuiltinDefinitions.h 的静态注册
// 表，Rust 侧由 add_global_binding_builtin_definitions* 函数承担），字段在
// 全仓从未插入或读取，属死代码。

/// 模块作用域准备回调。
pub type ModuleScopeCallback = Rc<dyn Fn(&ModuleName, &ScopePtr)>;
/// 带严格标记的模块作用域准备回调。
pub type ModuleScopeBoolCallback = Rc<dyn Fn(&ModuleName, &ScopePtr, bool)>;
/// JSON 日志回调。
pub type JsonLogCallback = Rc<dyn Fn(&ModuleName, String)>;
/// 内部错误回调。
pub type InternalErrorCallback = Rc<dyn Fn(&str)>;
/// 任务派发回调（C++: `std::function<void(std::vector<std::function<void()>> tasks)>`）。
///
/// 第一个参数是本次派发的构建队列下标（见 [`Task`]）；第二个参数是执行器，
/// 调用 `run(task)` 即在**当前线程**上完成该项检查（内部为
/// `Frontend::perform_queue_item_task`）。
///
/// 执行器是借自 `Frontend` 持有者的可变借用，既非 `'static` 也非 `Send`，所以派发方
/// 无法把任务挪到 worker 线程上跑——这是与 cpp `std::function` 版的唯一语义差别：cpp 端
/// `performQueueItemTask` 会原地改写同一个 `Frontend`（Rust 端口里 `Frontend` 还经
/// `wire_self_pointers` 自引用），跨线程共享它即构成数据竞争。默认派发方（cpp 亦同）
/// 就是顺序就地执行。
pub type TaskQueue = Box<dyn FnMut(Vec<Task>, &mut dyn FnMut(Task))>;
