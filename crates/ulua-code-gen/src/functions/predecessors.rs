use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{block_iterator_wrapper::BlockIteratorWrapper, cfg_info::CfgInfo},
};

pub fn predecessors(cfg: &CfgInfo, block_idx: u32) -> BlockIteratorWrapper<'_> {
  CODEGEN_ASSERT!(block_idx < cfg.predecessors_offsets.len() as u32);

  let start = cfg.predecessors_offsets[block_idx as usize];
  let end = if block_idx + 1 < cfg.predecessors_offsets.len() as u32 {
    cfg.predecessors_offsets[(block_idx + 1) as usize]
  } else {
    cfg.predecessors.len() as u32
  };

  BlockIteratorWrapper::new(&cfg.predecessors[start as usize..end as usize])
}
