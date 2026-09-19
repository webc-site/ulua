use alloc::{rc::Rc, string::String};

use crate::{
  records::{build_queue_work_state::Task, frontend::Frontend, global_types::GlobalTypes},
  type_aliases::{collections::HashMap, module_name_type::ModuleName, scope_ptr_type::ScopePtr},
};
/// 模块作用域准备回调。
pub type ModuleScopeCallback = Rc<dyn Fn(&ModuleName, &ScopePtr)>;
/// 带严格标记的模块作用域准备回调。
pub type ModuleScopeBoolCallback = Rc<dyn Fn(&ModuleName, &ScopePtr, bool)>;
/// JSON 日志回调。
pub type JsonLogCallback = Rc<dyn Fn(&ModuleName, String)>;
/// 内部错误回调。
pub type InternalErrorCallback = Rc<dyn Fn(&str)>;
/// 内置类型定义注册表。
pub type BuiltinDefinitionsMap =
  HashMap<String, Rc<dyn Fn(&mut Frontend, &mut GlobalTypes, ScopePtr)>>;
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
