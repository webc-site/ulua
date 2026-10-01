//! 指向 `SharedCodeGenContext` 的唯一指针的 native-only 类型别名，带自定义 deleter。
//! 用于在 native 构建中管理共享代码生成上下文的生命周期。
//! 由于 Rust 的 `Box<T>` 不支持无自定义分配器的自定义 deleter，且此类型
//! 仅 native 使用，我们将其表示为裸指针，由下游代码手动管理生命周期。

use core::ptr::NonNull;

use crate::records::shared_code_gen_context::SharedCodeGenContext;
pub type UniqueSharedCodeGenContext = NonNull<SharedCodeGenContext>;
