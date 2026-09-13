/// # Safety
///
/// Pointer arguments must be valid, aligned, and properly initialized.
use core::ffi::c_char;
use core::ptr::copy_nonoverlapping;
pub(crate) unsafe fn printunsignedrev(mut end: *mut c_char, mut num: u64) -> *mut c_char {
  const K_DIGIT_TABLE: [u8; 200] = *b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899";

  while num >= 10000 {
    let tail = (num % 10000) as u32;
    unsafe {
      let src0 = K_DIGIT_TABLE.as_ptr().add((tail / 100) as usize * 2);
      copy_nonoverlapping(src0, (end as *mut u8).sub(4), 2);
      let src1 = K_DIGIT_TABLE.as_ptr().add((tail % 100) as usize * 2);
      copy_nonoverlapping(src1, (end as *mut u8).sub(2), 2);
    }
    num /= 10000;
    unsafe {
      end = end.sub(4);
    }
  }

  let mut rest = num as u32;

  while rest >= 10 {
    unsafe {
      let src = K_DIGIT_TABLE.as_ptr().add((rest % 100) as usize * 2);
      copy_nonoverlapping(src, (end as *mut u8).sub(2), 2);
    }
    rest /= 100;
    unsafe {
      end = end.sub(2);
    }
  }

  if rest > 0 {
    unsafe {
      *end.sub(1) = (b'0' + rest as u8) as c_char;
      end = end.sub(1);
    }
  }

  end
}
