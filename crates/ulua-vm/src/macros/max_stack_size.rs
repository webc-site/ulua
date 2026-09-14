use core::mem::size_of;

use crate::type_aliases::t_value::TValue;

pub const MAX_STACK_SIZE: i32 = (1024 / size_of::<TValue>() as i32) * 1024 * 1024;
