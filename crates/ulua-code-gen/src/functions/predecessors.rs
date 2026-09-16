use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{block_iterator_wrapper::BlockIteratorWrapper, cfg_info::CfgInfo},
};

pub fn predecessors(cfg: &CfgInfo, block_idx: u32) -> BlockIteratorWrapper {
  CODEGEN_ASSERT!(block_idx < cfg.predecessors_offsets.len() as u32);

  let start = cfg.predecessors_offsets[block_idx as usize];
  let end = if block_idx + 1 < cfg.predecessors_offsets.len() as u32 {
    cfg.predecessors_offsets[(block_idx + 1) as usize]
  } else {
    cfg.predecessors.len() as u32
  };

  BlockIteratorWrapper {
    it_begin: unsafe { cfg.predecessors.as_ptr().add(start as usize) },
    it_end: unsafe { cfg.predecessors.as_ptr().add(end as usize) },
  }
}
