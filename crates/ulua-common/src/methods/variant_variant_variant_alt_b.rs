use core::{mem::forget, ptr::read};

use crate::records::variant::{
  Variant1, Variant2, Variant3, Variant4, Variant5, Variant6, Variant7,
};

impl<T0: 'static> Variant1<T0> {
  pub fn variant_t_enable_if_t_get_type_id_t<T: 'static>(value: T) -> Self {
    let tid = Self::get_type_id::<T>();
    unsafe {
      let ptr = &value as *const T;
      forget(value);
      match tid {
        0 => Self::V0(read(ptr as *const T0)),
        _ => panic!("Variant1: type not found in variant"),
      }
    }
  }
}

impl<T0: 'static, T1: 'static> Variant2<T0, T1> {
  pub fn variant_t_enable_if_t_get_type_id_t<T: 'static>(value: T) -> Self {
    let tid = Self::get_type_id::<T>();
    unsafe {
      let ptr = &value as *const T;
      forget(value);
      match tid {
        0 => Self::V0(read(ptr as *const T0)),
        1 => Self::V1(read(ptr as *const T1)),
        _ => panic!("Variant2: type not found in variant"),
      }
    }
  }
}

impl<T0: 'static, T1: 'static, T2: 'static> Variant3<T0, T1, T2> {
  pub fn variant_t_enable_if_t_get_type_id_t<T: 'static>(value: T) -> Self {
    let tid = Self::get_type_id::<T>();
    unsafe {
      let ptr = &value as *const T;
      forget(value);
      match tid {
        0 => Self::V0(read(ptr as *const T0)),
        1 => Self::V1(read(ptr as *const T1)),
        2 => Self::V2(read(ptr as *const T2)),
        _ => panic!("Variant3: type not found in variant"),
      }
    }
  }
}

impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static> Variant4<T0, T1, T2, T3> {
  pub fn variant_t_enable_if_t_get_type_id_t<T: 'static>(value: T) -> Self {
    let tid = Self::get_type_id::<T>();
    unsafe {
      let ptr = &value as *const T;
      forget(value);
      match tid {
        0 => Self::V0(read(ptr as *const T0)),
        1 => Self::V1(read(ptr as *const T1)),
        2 => Self::V2(read(ptr as *const T2)),
        3 => Self::V3(read(ptr as *const T3)),
        _ => panic!("Variant4: type not found in variant"),
      }
    }
  }
}

impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static> Variant5<T0, T1, T2, T3, T4> {
  pub fn variant_t_enable_if_t_get_type_id_t<T: 'static>(value: T) -> Self {
    let tid = Self::get_type_id::<T>();
    unsafe {
      let ptr = &value as *const T;
      forget(value);
      match tid {
        0 => Self::V0(read(ptr as *const T0)),
        1 => Self::V1(read(ptr as *const T1)),
        2 => Self::V2(read(ptr as *const T2)),
        3 => Self::V3(read(ptr as *const T3)),
        4 => Self::V4(read(ptr as *const T4)),
        _ => panic!("Variant5: type not found in variant"),
      }
    }
  }
}

impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static, T5: 'static>
  Variant6<T0, T1, T2, T3, T4, T5>
{
  pub fn variant_t_enable_if_t_get_type_id_t<T: 'static>(value: T) -> Self {
    let tid = Self::get_type_id::<T>();
    unsafe {
      let ptr = &value as *const T;
      forget(value);
      match tid {
        0 => Self::V0(read(ptr as *const T0)),
        1 => Self::V1(read(ptr as *const T1)),
        2 => Self::V2(read(ptr as *const T2)),
        3 => Self::V3(read(ptr as *const T3)),
        4 => Self::V4(read(ptr as *const T4)),
        5 => Self::V5(read(ptr as *const T5)),
        _ => panic!("Variant6: type not found in variant"),
      }
    }
  }
}

impl<T0: 'static, T1: 'static, T2: 'static, T3: 'static, T4: 'static, T5: 'static, T6: 'static>
  Variant7<T0, T1, T2, T3, T4, T5, T6>
{
  pub fn variant_t_enable_if_t_get_type_id_t<T: 'static>(value: T) -> Self {
    let tid = Self::get_type_id::<T>();
    unsafe {
      let ptr = &value as *const T;
      forget(value);
      match tid {
        0 => Self::V0(read(ptr as *const T0)),
        1 => Self::V1(read(ptr as *const T1)),
        2 => Self::V2(read(ptr as *const T2)),
        3 => Self::V3(read(ptr as *const T3)),
        4 => Self::V4(read(ptr as *const T4)),
        5 => Self::V5(read(ptr as *const T5)),
        6 => Self::V6(read(ptr as *const T6)),
        _ => panic!("Variant7: type not found in variant"),
      }
    }
  }
}
