//! Native-only type alias for a unique pointer to `SharedCodeGenContext` with a custom deleter.
//! This type is used to manage the lifecycle of shared code generation contexts in native builds.
//! Since Rust's `Box<T>` does not support custom deleters without a custom allocator, and this type
//! is native-only, we represent it as a raw pointer with manual lifecycle management by downstream code.

use core::ptr::NonNull;

use crate::records::shared_code_gen_context::SharedCodeGenContext;
pub type UniqueSharedCodeGenContext = NonNull<SharedCodeGenContext>;
