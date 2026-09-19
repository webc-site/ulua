//! 本 crate 是 Luau `Analysis` 库的 Rust 移植：前端 `Frontend` + `TypeChecker` /
//! `ConstraintGenerator` / `Solver` 的类型检查器，**不是** FFI 封装层。全 crate
//! 约 6000 处 `unsafe`，绝大多数与 FFI 无关，而是 cpp 指针模型（`*mut Type` /
//! `*mut AstNode` / arena 句柄）的直译产物，其安全性由以下**本 crate 自有不变量**
//! 保证，不能笼统归给「FFI 契约」：
//!
//! 1. **单线程**：一个 `Frontend`/`TypeChecker` 实例只在其所属线程内被驱动，
//!    crate 内不含并发调度；`BuildQueueWorkState` 的锁仅用于跨线程**等待计数**，
//!    不保护类型数据。
//! 2. **arena 句柄的有效性**：`*mut Type` / `*mut Scope` / `*mut Module` 等等价于
//!    cpp `Arena` 分配的裸引用，其生命周期由 `Frontend` 持有的 arena 界定，
//!    不得跨 `Frontend` 存活，也不得在 `FreeTypeArena` 之后解引用。
//! 3. **RTTI 下转**：`&mut AstNode ↔ *mut AstExprLocal` 一类转换必须先由
//!    `rtti_` 判型，误判即 UB。
//! 4. **C-ABI 边界是局部的**：仅三处真实存在 OS 互操作——LuaState 类型函数注册表
//!    （`register_type_user_data`）、分页 arena（`paged_allocate` 的 `mmap`）、
//!    栈深度守卫（`native_stack_guard` 的 `pthread_*`）；共 27 个 `extern` 声明。
//!    其余 `unsafe` 均不属于 C-ABI。
//! 5. **`unsafe impl Send/Sync`（35 文件）** 是为适配通用容器签名而写的断言，
//!    依赖不变量 1 才成立；跨线程搬运这些类型即为 UB。
//!
//! 已知**尚未收口**的高风险点（改动时请勿当作既成事实）：
//! - `ScopePtr = Arc<Scope>` / `Arc<Module>` / `Arc<AstNameTable>` 无 `UnsafeCell`，
//!   但存在 `as_ref() as *const _ as *mut _` 式写穿（同源指针并存可变引用）。
//! - `get_mutable_type` 系列返回 `&'static mut T`，同一 `TypeId` 两次调用可得并存 `&mut`。
//! - 少量以 `NonNull::new_unchecked(&self.x as *const _ as *mut _)` 伪造独占句柄。

extern crate alloc;

pub mod enums;
pub mod functions;
pub mod macros;
pub mod methods;
pub mod records;
pub mod type_aliases;

extern crate self as ulua_analysis;

pub use ulua_ast::rtti;
pub use ulua_common::{fflag, fint};
