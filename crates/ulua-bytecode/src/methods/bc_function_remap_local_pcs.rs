use crate::records::bc_function::BcFunction;

/// 把 locals / local_types 的 PC 从指令索引映射为最终指令偏移；
/// 越界时回退到 fallback（通常为指令总数）。
pub(crate) fn bc_function_remap_local_pcs(f: &mut BcFunction, insns_pc: &[u32], fallback: u32) {
  let remap = |pc: u32| {
    if pc < insns_pc.len() as u32 {
      insns_pc[pc as usize]
    } else {
      fallback
    }
  };

  for l in &mut f.local_types {
    l.startpc = remap(l.startpc);
    l.endpc = remap(l.endpc);
  }

  for l in &mut f.locals {
    l.startpc = remap(l.startpc);
    l.endpc = remap(l.endpc);
  }
}
