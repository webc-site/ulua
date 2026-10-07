use crate::records::base_code_gen_context::BaseCodeGenContext;

/// cpp 原 `SharedCodeGenContext` 派生类的路径占位。
///
/// 其全部行为差异（带 module_id 的绑定走 `get_or_insert_native_module`、
/// `tryBindExistingModule` 真实查找、状态关闭 no-op）已收口为
/// `BaseCodeGenContext` 内 `CodeGenContextKind::Shared` 的静态分支，
/// 不再有独立的壳结构、`*mut BaseCodeGenContext` 下行转换的 fn 槽垫片与
/// 构造期占位覆盖把戏。本路径供创建/销毁/注册接口
/// （`create_shared_code_gen_context*` / `destroy_shared_code_gen_context` /
/// `create_code_gen_context::create`）的签名继续使用。
pub type SharedCodeGenContext = BaseCodeGenContext;
