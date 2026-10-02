use crate::{
  functions::{getheapgrowth::getheapgrowth, getheaptriggererroroffset::getheaptriggererroroffset},
  records::global_state::global_State,
};

pub(crate) fn getheaptrigger(g: &mut global_State, heapgoal: usize) -> usize {
  const DURATION_THRESHOLD: f64 = 1e-3; // 避免测量小于 1ms 的时间间隔

  let gcstats = &g.gcstats;

  let allocationduration = gcstats.atomicstarttimestamp - gcstats.endtimestamp;

  if allocationduration < DURATION_THRESHOLD {
    return heapgoal;
  }

  let allocationrate = getheapgrowth(gcstats.atomicstarttotalsizebytes, gcstats.endtotalsizebytes)
    as f64
    / allocationduration;
  let markduration = gcstats.atomicstarttimestamp - gcstats.starttimestamp;

  let expectedgrowth = (markduration * allocationrate) as i64;
  let offset = getheaptriggererroroffset(g);
  let heaptrigger = heapgoal as i64 - (expectedgrowth + offset);

  let totalbytes = g.totalbytes as i64;

  if heaptrigger < totalbytes {
    g.totalbytes
  } else if heaptrigger > heapgoal as i64 {
    heapgoal
  } else {
    heaptrigger as usize
  }
}
