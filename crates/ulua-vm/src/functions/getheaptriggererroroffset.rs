//! Node: `cxx:Function:Luau.VM:VM/src/lgc.cpp:1100:getheaptriggererroroffset`
//! Source: `VM/src/lgc.cpp:1100-1130` (hand-ported)

use crate::type_aliases::global_state::global_State;

pub(crate) fn getheaptriggererroroffset(g: *mut global_State) -> i64 {
  let gcstats = unsafe { &mut (*g).gcstats };

  let atomicstarttotalsizebytes = gcstats.atomicstarttotalsizebytes;
  let heapgoalsizebytes = gcstats.heapgoalsizebytes;

  let error_kb = (atomicstarttotalsizebytes.wrapping_sub(heapgoalsizebytes) / 1024) as i32;

  const TRIGGERTERMCOUNT: usize = 32;

  let slot = &mut gcstats.triggerterms[gcstats.triggertermpos as usize % TRIGGERTERMCOUNT];
  let prev = *slot;
  *slot = error_kb;
  gcstats.triggerintegral += error_kb - prev;
  gcstats.triggertermpos += 1;

  const KU: f64 = 0.9;
  const TU: f64 = 2.5;

  const KP: f64 = 0.45 * KU;
  const TI: f64 = 0.8 * TU;
  const KI: f64 = 0.54 * KU / TI;

  let proportional_term = KP * error_kb as f64;
  let integral_term = KI * gcstats.triggerintegral as f64;

  let total_term = proportional_term + integral_term;

  (total_term * 1024.0) as i64
}
