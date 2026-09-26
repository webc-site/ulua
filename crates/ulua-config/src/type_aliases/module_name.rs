/// 模块名称类型别名。
///
/// 使用 `hipstr::HipStr<'static>` 实现小模块名内联存储（零堆分配）与廉价引用计数克隆。
pub type ModuleName = hipstr::HipStr<'static>;
