pub fn is_unwind_supported() -> bool {
  #[cfg(all(
    target_os = "windows",
    any(target_arch = "x86_64", target_arch = "x86")
  ))]
  {
    true
  }

  #[cfg(all(
    target_os = "macos",
    target_arch = "aarch64",
    not(target_os = "windows")
  ))]
  {
    // libunwind on macOS 12 and earlier (which maps to osrelease 21) assumes JIT
    // frames use pointer authentication without a way to override that.
    // Check kern.osrelease >= 22.
    use std::process::Command;

    let output = Command::new("sysctl")
      .arg("-n")
      .arg("kern.osrelease")
      .output();

    match output {
      Ok(out) if out.status.success() => {
        let ver = String::from_utf8_lossy(&out.stdout);
        let ver = ver.trim();
        matches!(ver.parse::<u32>(), Ok(n) if n >= 22)
      }
      _ => false,
    }
  }

  #[cfg(all(
    any(target_os = "linux", target_os = "macos"),
    any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64"),
    not(target_os = "windows"),
    not(all(target_os = "macos", target_arch = "aarch64"))
  ))]
  {
    true
  }

  #[cfg(not(any(
    all(
      target_os = "windows",
      any(target_arch = "x86_64", target_arch = "x86")
    ),
    all(target_os = "macos", target_arch = "aarch64"),
    all(
      any(target_os = "linux", target_os = "macos"),
      any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64"),
      not(all(target_os = "macos", target_arch = "aarch64"))
    )
  )))]
  {
    false
  }
}
