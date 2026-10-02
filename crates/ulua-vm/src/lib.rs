extern crate alloc;

pub mod enums;
pub mod functions;
pub mod macros;
// 仅 crate 内消费的 trait/impl 集合，外部无 `ulua_vm::methods::` 路径消费
pub(crate) mod methods;
pub mod records;
pub mod type_aliases;
