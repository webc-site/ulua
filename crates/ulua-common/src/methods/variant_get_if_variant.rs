use crate::records::variant::{
  Variant1, Variant2, Variant3, Variant4, Variant5, Variant6, Variant7,
};

/// Returns `Some(&T)` if the active alternative has type `T`, else `None`.
impl<T0: 'static> Variant1<T0> {
  pub fn get_if<T: 'static>(&self) -> Option<&T> {
    let tid = Self::get_type_id::<T>();
    if tid < 0 {
      return None;
    }

    if self.index() as i32 == tid {
      // SAFETY:
      // - In this port, VariantN stores the active alternative as a real value inside the enum.
      // - `get_type_id::<T>()` guarantees the active variant matches `T`.
      // - We can safely take a reference to the correct payload.
      if tid == 0 {
        self.get_if_0().map(|v| {
          let r: &T = unsafe { &*(v as *const T0 as *const T) };
          r
        })
      } else {
        None
      }
    } else {
      None
    }
  }
}

/// Returns `Some(&T)` if the active alternative has type `T`, else `None`.
impl<T0: 'static, T1: 'static> Variant2<T0, T1> {
  pub fn get_if<T: 'static>(&self) -> Option<&T> {
    let tid = Self::get_type_id::<T>();
    if tid < 0 {
      return None;
    }

    if self.index() as i32 == tid {
      match tid {
        0 => self.get_if_0().map(|v0| {
          let r: &T = unsafe { &*(v0 as *const T0 as *const T) };
          r
        }),
        1 => self.get_if_1().map(|v1| {
          let r: &T = unsafe { &*(v1 as *const T1 as *const T) };
          r
        }),
        _ => None,
      }
    } else {
      None
    }
  }
}

/// Returns `Some(&T)` if the active alternative has type `T`, else `None`.
impl<T0: 'static, T1: 'static, T2: 'static> Variant3<T0, T1, T2> {
  pub fn get_if<T: 'static>(&self) -> Option<&T> {
    let tid = Self::get_type_id::<T>();
    if tid < 0 {
      return None;
    }

    if self.index() as i32 == tid {
      match tid {
        0 => self.get_if_0().map(|v0| {
          let r: &T = unsafe { &*(v0 as *const T0 as *const T) };
          r
        }),
        1 => self.get_if_1().map(|v1| {
          let r: &T = unsafe { &*(v1 as *const T1 as *const T) };
          r
        }),
        2 => self.get_if_2().map(|v2| {
          let r: &T = unsafe { &*(v2 as *const T2 as *const T) };
          r
        }),
        _ => None,
      }
    } else {
      None
    }
  }
}

/// Returns `Some(&T)` if the active alternative has type `T`, else `None`.
impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static> Variant4<T0, T1, T2, T3> {
  pub fn get_if<T: 'static>(&self) -> Option<&T> {
    let tid = Self::get_type_id::<T>();
    if tid < 0 {
      return None;
    }

    if self.index() as i32 == tid {
      match tid {
        0 => self.get_if_0().map(|v0| {
          let r: &T = unsafe { &*(v0 as *const T0 as *const T) };
          r
        }),
        1 => self.get_if_1().map(|v1| {
          let r: &T = unsafe { &*(v1 as *const T1 as *const T) };
          r
        }),
        2 => self.get_if_2().map(|v2| {
          let r: &T = unsafe { &*(v2 as *const T2 as *const T) };
          r
        }),
        3 => self.get_if_3().map(|v3| {
          let r: &T = unsafe { &*(v3 as *const T3 as *const T) };
          r
        }),
        _ => None,
      }
    } else {
      None
    }
  }
}

/// Returns `Some(&T)` if the active alternative has type `T`, else `None`.
impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static> Variant5<T0, T1, T2, T3, T4> {
  pub fn get_if<T: 'static>(&self) -> Option<&T> {
    let tid = Self::get_type_id::<T>();
    if tid < 0 {
      return None;
    }

    if self.index() as i32 == tid {
      match tid {
        0 => self.get_if_0().map(|v0| {
          let r: &T = unsafe { &*(v0 as *const T0 as *const T) };
          r
        }),
        1 => self.get_if_1().map(|v1| {
          let r: &T = unsafe { &*(v1 as *const T1 as *const T) };
          r
        }),
        2 => self.get_if_2().map(|v2| {
          let r: &T = unsafe { &*(v2 as *const T2 as *const T) };
          r
        }),
        3 => self.get_if_3().map(|v3| {
          let r: &T = unsafe { &*(v3 as *const T3 as *const T) };
          r
        }),
        4 => self.get_if_4().map(|v4| {
          let r: &T = unsafe { &*(v4 as *const T4 as *const T) };
          r
        }),
        _ => None,
      }
    } else {
      None
    }
  }
}

/// Returns `Some(&T)` if the active alternative has type `T`, else `None`.
impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static, T5: 'static>
  Variant6<T0, T1, T2, T3, T4, T5>
{
  pub fn get_if<T: 'static>(&self) -> Option<&T> {
    let tid = Self::get_type_id::<T>();
    if tid < 0 {
      return None;
    }

    if self.index() as i32 == tid {
      match tid {
        0 => self.get_if_0().map(|v0| {
          let r: &T = unsafe { &*(v0 as *const T0 as *const T) };
          r
        }),
        1 => self.get_if_1().map(|v1| {
          let r: &T = unsafe { &*(v1 as *const T1 as *const T) };
          r
        }),
        2 => self.get_if_2().map(|v2| {
          let r: &T = unsafe { &*(v2 as *const T2 as *const T) };
          r
        }),
        3 => self.get_if_3().map(|v3| {
          let r: &T = unsafe { &*(v3 as *const T3 as *const T) };
          r
        }),
        4 => self.get_if_4().map(|v4| {
          let r: &T = unsafe { &*(v4 as *const T4 as *const T) };
          r
        }),
        5 => self.get_if_5().map(|v5| {
          let r: &T = unsafe { &*(v5 as *const T5 as *const T) };
          r
        }),
        _ => None,
      }
    } else {
      None
    }
  }
}

/// Returns `Some(&T)` if the active alternative has type `T`, else `None`.
impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static, T5: 'static, T6: 'static>
  Variant7<T0, T1, T2, T3, T4, T5, T6>
{
  pub fn get_if<T: 'static>(&self) -> Option<&T> {
    let tid = Self::get_type_id::<T>();
    if tid < 0 {
      return None;
    }

    if self.index() as i32 == tid {
      match tid {
        0 => self.get_if_0().map(|v0| {
          let r: &T = unsafe { &*(v0 as *const T0 as *const T) };
          r
        }),
        1 => self.get_if_1().map(|v1| {
          let r: &T = unsafe { &*(v1 as *const T1 as *const T) };
          r
        }),
        2 => self.get_if_2().map(|v2| {
          let r: &T = unsafe { &*(v2 as *const T2 as *const T) };
          r
        }),
        3 => self.get_if_3().map(|v3| {
          let r: &T = unsafe { &*(v3 as *const T3 as *const T) };
          r
        }),
        4 => self.get_if_4().map(|v4| {
          let r: &T = unsafe { &*(v4 as *const T4 as *const T) };
          r
        }),
        5 => self.get_if_5().map(|v5| {
          let r: &T = unsafe { &*(v5 as *const T5 as *const T) };
          r
        }),
        6 => self.get_if_6().map(|v6| {
          let r: &T = unsafe { &*(v6 as *const T6 as *const T) };
          r
        }),
        _ => None,
      }
    } else {
      None
    }
  }
}
