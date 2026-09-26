use crate::records::{block_iterator_wrapper::BlockIteratorWrapper, cfg_info::CfgInfo};

pub fn successors(cfg: &CfgInfo, block_idx: u32) -> BlockIteratorWrapper<'_> {
  // 行为与 C++ CODEGEN_ASSERT 保持一致，同时避免当前
  // 宏在 ulua_common::assert_call_handler 上的类型不匹配。
  assert!(block_idx < cfg.successors_offsets.len() as u32);

  let start = cfg.successors_offsets[block_idx as usize];
  let end = if block_idx + 1 < cfg.successors_offsets.len() as u32 {
    cfg.successors_offsets[(block_idx + 1) as usize]
  } else {
    cfg.successors.len() as u32
  };

  BlockIteratorWrapper::new(&cfg.successors[start as usize..end as usize])
}
