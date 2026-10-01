#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct GCCycleMetrics {
  pub starttimestamp: f64,
  pub endtimestamp: f64,

  pub marktime: f64,
  pub markassisttime: f64,
  pub markmaxexplicittime: f64,
  pub markexplicitsteps: usize,
  pub markwork: usize,

  pub atomicstarttimestamp: f64,
  pub atomicstarttotalsizebytes: usize,
  pub atomictime: f64,

  pub sweeptime: f64,
  pub sweepassisttime: f64,
  pub sweepmaxexplicittime: f64,
  pub sweepexplicitsteps: usize,
  pub sweepwork: usize,

  pub assistwork: usize,
  pub explicitwork: usize,

  pub endtotalsizebytes: usize,
}
