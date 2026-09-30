//! C++ `Luau::AnnotationCheckMode`（`Analysis/include/Luau/TypeChecker2.h:70-74`）。
//!
//! `checkFunctionAnnotations` 的标注检查语境：普通函数（顶层 `function`/
//! `local function`）、类方法与类构造函数。`Method`/`Constructor` 语境下首个
//! 名为 `self` 的参数豁免标注（标注 `self` 本身就是语法错误）。

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationCheckMode {
  Function,
  Method,
  Constructor,
}
