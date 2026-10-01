use core::ops::{Deref, DerefMut};

use ulua_ast::records::temp_vector::TempVector;

#[test]
fn temp_vector_deref_and_slice_methods() {
  let mut scratch = Vec::new();
  {
    let mut tv = TempVector::new(&mut scratch);
    assert!(tv.is_empty());
    assert_eq!(tv.len(), 0);
    assert_eq!(tv.first(), None);
    assert_eq!(tv.last(), None);

    tv.push_back(10);
    tv.push_back(20);
    tv.push_back(30);

    assert!(!tv.is_empty());
    assert_eq!(tv.len(), 3);
    assert_eq!(tv.first(), Some(&10));
    assert_eq!(tv.last(), Some(&30));

    // Deref to slice
    let slice: &[i32] = tv.deref();
    assert_eq!(slice, &[10, 20, 30]);

    // Index & Range Index
    assert_eq!(tv[0], 10);
    assert_eq!(tv[1], 20);
    assert_eq!(tv[2], 30);
    assert_eq!(&tv[1..], &[20, 30]);
    assert_eq!(&tv[..2], &[10, 20]);
    assert_eq!(&tv[0..1], &[10]);

    // DerefMut & IndexMut
    tv[1] = 99;
    assert_eq!(tv[1], 99);
    assert_eq!(&tv[..], &[10, 99, 30]);

    let mut_slice: &mut [i32] = tv.deref_mut();
    mut_slice[2] = 100;
    assert_eq!(tv[2], 100);

    // IntoIterator for &TempVector
    let collected: Vec<i32> = (&tv).into_iter().copied().collect();
    assert_eq!(collected, [10, 99, 100]);

    let for_collected: Vec<i32> = tv.iter().copied().collect();
    assert_eq!(for_collected, [10, 99, 100]);

    // IntoIterator for &mut TempVector
    for x in &mut tv {
      *x += 1;
    }
    assert_eq!(&tv[..], &[11, 100, 101]);
  }

  // Drop 保证 scratch 恢复为空
  assert!(scratch.is_empty());
}

#[test]
fn temp_vector_multiple_lifetimes() {
  let mut scratch = Vec::new();
  {
    let mut tv1 = TempVector::new(&mut scratch);
    tv1.push_back(1);
    tv1.push_back(2);
    assert_eq!(tv1.len(), 2);
    assert_eq!(&tv1[..], &[1, 2]);
  }
  assert!(scratch.is_empty());

  {
    let mut tv2 = TempVector::new(&mut scratch);
    tv2.push_back(3);
    tv2.push_back(4);
    assert_eq!(tv2.len(), 2);
    assert_eq!(&tv2[..], &[3, 4]);
  }
  assert!(scratch.is_empty());
}
