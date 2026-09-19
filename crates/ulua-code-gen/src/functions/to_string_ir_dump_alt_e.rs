extern crate alloc;

use alloc::string::String;

use ulua_common::functions::format_g::format_g;
use ulua_vm::type_aliases::proto::Proto;

use crate::{
  functions::{append::append, append_vm_constant::append_vm_constant, get_tag_name::get_tag_name},
  records::ir_const::IrConst,
};

/// # Safety
/// 传入的指针必须有效且指向存活对象，调用方须满足 C++ 参考实现的前置条件。
pub unsafe fn to_string(result: &mut String, proto: *mut Proto, constant: IrConst) {
  // 载荷直配，替代原先「match kind + unsafe union 读」的两段式
  match constant {
    IrConst::Int(v) => append(result, format_args!("{}i", v)),
    IrConst::Int64(v) => append(result, format_args!("{}i", v)),
    IrConst::Uint(v) => append(result, format_args!("{}u", v)),
    IrConst::Double(d) => {
      if d.is_nan() {
        append(result, format_args!("nan"));
      } else {
        // C++ uses printf "%.17g"; Rust's `{}` prints the shortest
        // round-tripping form (e.g. `0.4` vs `0.40000000000000002`).
        result.push_str(&format_g(d, 17));
      }
    }
    IrConst::Tag(tag) => result.push_str(get_tag_name(tag)),
    IrConst::Import(value_uint) => {
      append(result, format_args!("{}u", value_uint));

      if !proto.is_null() {
        append(result, format_args!(" ("));

        let count = (value_uint >> 30) as i32;
        let id0 = if count > 0 {
          ((value_uint >> 20) & 1023) as i32
        } else {
          -1
        };
        let id1 = if count > 1 {
          ((value_uint >> 10) & 1023) as i32
        } else {
          -1
        };
        let id2 = if count > 2 {
          (value_uint & 1023) as i32
        } else {
          -1
        };

        if id0 != -1 {
          unsafe { append_vm_constant(result, proto, id0) };
        }

        if id1 != -1 {
          append(result, format_args!("."));
          unsafe { append_vm_constant(result, proto, id1) };
        }

        if id2 != -1 {
          append(result, format_args!("."));
          unsafe { append_vm_constant(result, proto, id2) };
        }

        append(result, format_args!(")"));
      }
    }
  }
}
