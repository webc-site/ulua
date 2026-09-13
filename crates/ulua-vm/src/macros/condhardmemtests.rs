macro_rules! condhardmemtests {
  ($x:expr, $l:expr) => {
    #[cfg(feature = "hard_mem_tests")]
    {
      // HARDMEMTESTS is defined via a cfg or a constant in luaconf.h.
      const HARD_MEM_TESTS: i32 = 1;
      if HARD_MEM_TESTS >= $l {
        $x;
      }
    }
    #[cfg(not(feature = "hard_mem_tests"))]
    {
      let _ = $l; // Silence unused variable warning for the level
    }
  };
}

pub(crate) use condhardmemtests;
