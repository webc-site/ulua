#[macro_export]
macro_rules! lua_m_arraysize {
    ($l:expr, $n:expr, $e:expr) => {
        if ($n as usize) <= usize::MAX / ($e as usize) {
            $n * $e
        } else {
            $crate::functions::lua_m_toobig::lua_m_toobig($l)
        }
    };
}

pub use lua_m_arraysize;
