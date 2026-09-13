#[cfg(target_os = "windows")]
unsafe extern "system" {
  fn IsDebuggerPresent() -> i32;
}

#[cfg(target_os = "windows")]
pub fn debugger_present() -> bool {
  unsafe { IsDebuggerPresent() != 0 }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
pub fn debugger_present() -> bool {
  macos::debugger_present()
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
mod macos {
  use core::{
    ffi::{c_int, c_void},
    mem::{size_of, zeroed},
    ptr::null_mut,
  };

  unsafe extern "C" {
    fn sysctl(
      name: *const c_int,
      namelen: u32,
      oldp: *mut c_void,
      oldlenp: *mut usize,
      newp: *mut c_void,
      newlen: usize,
    ) -> c_int;
    fn getpid() -> c_int;
  }

  const CTL_KERN: c_int = 1;
  const KERN_PROC: c_int = 14;
  const KERN_PROC_PID: c_int = 1;
  const P_TRACED: c_int = 0x00000800;

  #[repr(C)]
  struct KinfoProc {
    padding: [u8; 32],
    p_flag: i32,
    padding2: [u8; 1000],
  }

  pub fn debugger_present() -> bool {
    unsafe {
      let mut mib = [CTL_KERN, KERN_PROC, KERN_PROC_PID, getpid()];
      let mut info: KinfoProc = zeroed();
      let mut size = size_of::<KinfoProc>();
      let ret = sysctl(
        mib.as_mut_ptr(),
        mib.len() as u32,
        &mut info as *mut _ as *mut c_void,
        &mut size,
        null_mut(),
        0,
      );
      ret == 0 && (info.p_flag & P_TRACED) != 0
    }
  }
}

#[cfg(target_os = "linux")]
pub fn debugger_present() -> bool {
  use std::{
    fs::File,
    io::{BufRead, BufReader},
  };

  let file = match File::open("/proc/self/status") {
    Ok(f) => f,
    Err(_) => return false,
  };
  let reader = BufReader::new(file);
  for line in reader.lines() {
    if let Ok(line) = line {
      if line.starts_with("TracerPid:\t") {
        if let Some(pid_str) = line.get(11..) {
          if let Ok(pid) = pid_str.trim().parse::<i32>() {
            return pid != 0;
          }
        }
        break;
      }
    }
  }
  false
}

#[cfg(not(any(
  target_os = "windows",
  target_os = "macos",
  target_os = "ios",
  target_os = "linux"
)))]
pub fn debugger_present() -> bool {
  false
}
