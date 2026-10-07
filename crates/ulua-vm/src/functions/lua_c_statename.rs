pub fn lua_c_statename(gcstate: u8) -> &'static str {
  match gcstate {
    0 => "pause",
    1 => "propagate",
    2 => "atomic",
    3 => "sweepstring",
    4 => "sweep",
    5 => "finalize",
    _ => "idle",
  }
}
