//! Where do the milliseconds actually go per input? Replicates the `structured`
//! harness (generate -> check -> compile -> run under the 1M step limit) but
//! times each phase separately and records how many VM steps each input burns —
//! to see whether the step limit (not check/setup) is the throughput killer.
//!   cargo run --release --no-default-features --bin bench_phases -- 4000
use std::{
  cell::{Cell, RefCell},
  env::{args, var},
  rc::Rc,
  time::{Duration, Instant},
};

use ulua_rt::{Checker, Error, Lua, Result, VmState};

thread_local! {
    static CHECKER: RefCell<ulua_rt::Checker> = RefCell::new(Checker::new());
}

fn main() {
  let iters: u64 = args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(4000);
  let limit: u64 = var("STEP_LIMIT")
    .ok()
    .and_then(|s| s.parse().ok())
    // 对齐 structured 目标的实际默认（vm_step_limit = 100k），否则相位占比不可换算
    .unwrap_or(100_000);

  let (mut t_gen, mut t_chk, mut t_comp, mut t_run) = (
    Duration::ZERO,
    Duration::ZERO,
    Duration::ZERO,
    Duration::ZERO,
  );
  let mut hit_limit = 0u64;
  let mut ran = 0u64;
  let mut total_steps = 0u128;
  let mut seed = 0xabcd_1234_dead_0001u64;

  let t_all = Instant::now();
  for _ in 0..iters {
    let mut data = [0u8; 96];
    ulua_fuzz::lcg_fill(&mut seed, &mut data);

    let t = Instant::now();
    let src = ulua_fuzz::generate(&data);
    t_gen += t.elapsed();

    let t = Instant::now();
    CHECKER.with(|c| {
      let _ = c.borrow_mut().check(&src);
    });
    t_chk += t.elapsed();

    let t = Instant::now();
    let lua = Lua::new();
    let steps = Rc::new(Cell::new(0u64));
    let counter = steps.clone();
    lua.set_interrupt(move |_| -> Result<VmState> {
      let c = counter.get() + 1;
      counter.set(c);
      if c > limit {
        Err(Error::runtime("limit"))
      } else {
        Ok(VmState::Continue)
      }
    });
    let f = lua.load(&src).set_name("fuzz").into_function();
    t_comp += t.elapsed();

    if let Ok(f) = f {
      let t = Instant::now();
      let _ = f.call::<()>(());
      t_run += t.elapsed();
      ran += 1;
      let used = steps.get();
      total_steps += used as u128;
      if used >= limit {
        hit_limit += 1;
      }
    }
  }
  let wall = t_all.elapsed();

  eprintln!("== {} inputs, step limit {} ==", iters, limit);
  eprintln!(
    "  throughput      {:>8.0}/s",
    iters as f64 / wall.as_secs_f64()
  );
  for (name, t) in [
    ("generate", t_gen),
    ("check", t_chk),
    ("compile+setup", t_comp),
    ("VM run", t_run),
  ] {
    let secs = t.as_secs_f64();
    eprintln!(
      "  {name:<16}{:>6.1}%  ({:.1} us/in)",
      100.0 * secs / wall.as_secs_f64(),
      secs * 1e6 / iters as f64
    );
  }
  eprintln!(
    "  ran {}/{} compiled; {} hit step limit ({:.1}%); avg {} steps/run",
    ran,
    iters,
    hit_limit,
    100.0 * hit_limit as f64 / ran.max(1) as f64,
    (total_steps / ran.max(1) as u128)
  );
}
