use crate::{
  macros::codegen_assert::CODEGEN_ASSERT,
  records::{block_iterator_wrapper::BlockIteratorWrapper, cfg_info::CfgInfo},
};

pub fn dom_children(cfg: &CfgInfo, block_idx: u32) -> BlockIteratorWrapper {
  if !(block_idx < cfg.dom_children_offsets.len() as u32) {
    CODEGEN_ASSERT!(false);
  }

  let start = cfg.dom_children_offsets[block_idx as usize];
  let end = if (block_idx + 1) < cfg.dom_children_offsets.len() as u32 {
    cfg.dom_children_offsets[(block_idx + 1) as usize]
  } else {
    cfg.dom_children.len() as u32
  };

  BlockIteratorWrapper {
    it_begin: unsafe { cfg.dom_children.as_ptr().add(start as usize) },
    it_end: unsafe { cfg.dom_children.as_ptr().add(end as usize) },
  }
}
