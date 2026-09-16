use alloc::{rc::Rc, string::String};

use ulua_config::type_aliases::module_name::ModuleName;

use crate::{
  records::{build_queue_work_state::Task, frontend::Frontend, global_types::GlobalTypes},
  type_aliases::{collections::HashMap, scope_ptr_type::ScopePtr},
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
/// 任务队列闭包。
pub type TaskQueue = Box<dyn Fn(Vec<Task>)>;
