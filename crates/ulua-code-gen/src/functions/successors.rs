use crate::records::{block_iterator_wrapper::BlockIteratorWrapper, cfg_info::CfgInfo};

pub fn successors(cfg: &CfgInfo, block_idx: u32) -> BlockIteratorWrapper {
  // Keep behavior consistent with the C++ CODEGEN_ASSERT without triggering the
  // current macro's type mismatch for ulua_common::assert_call_handler.
  assert!(block_idx < cfg.successors_offsets.len() as u32);

  let start = cfg.successors_offsets[block_idx as usize];
  let end = if block_idx + 1 < cfg.successors_offsets.len() as u32 {
    cfg.successors_offsets[(block_idx + 1) as usize]
  } else {
    cfg.successors.len() as u32
  };

  BlockIteratorWrapper {
    it_begin: unsafe { cfg.successors.as_ptr().add(start as usize) },
    it_end: unsafe { cfg.successors.as_ptr().add(end as usize) },
  }
}
