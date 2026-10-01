// `become` 显式尾调用是 VM 派发层的骨架（每个 opcode 一个独立间接跳转 site，等价
// cpp computed goto），见 `functions/luau_execute.rs` 的 `vm_next!`。
// `incomplete_features` 是 nightly 对该特性的固定告警，仓库里唯一一处豁免。
#![feature(explicit_tail_calls)]
#![allow(incomplete_features)]

extern crate alloc;

pub mod enums;
pub mod functions;
pub mod macros;
pub mod methods;
pub mod records;
pub mod type_aliases;
