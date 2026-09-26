use ulua_ast::records::ast_array::AstArray;

#[test]
fn test_ast_array_empty() {
  let empty: AstArray<i32> = AstArray::EMPTY;
  assert_eq!(empty.len(), 0);
  assert!(empty.is_empty());
  assert_eq!(empty.as_slice(), &[]);
  assert_eq!(&*empty, &[]);
  assert_eq!(empty.first(), None);
  assert_eq!(empty.last(), None);
}

#[test]
fn test_ast_array_deref_and_index() {
  let mut data = [10, 20, 30, 40, 50];
  let arr = AstArray::from_slice(&data);

  // Deref to slice: 调用切片方法
  assert_eq!(arr.len(), 5);
  assert!(!arr.is_empty());
  assert_eq!(arr.first(), Some(&10));
  assert_eq!(arr.last(), Some(&50));
  assert_eq!(arr.split_at(2), (&[10, 20][..], &[30, 40, 50][..]));

  // Index<usize>
  assert_eq!(arr[0], 10);
  assert_eq!(arr[2], 30);
  assert_eq!(arr[4], 50);

  // Index<Range>
  assert_eq!(&arr[1..4], &[20, 30, 40]);
  assert_eq!(&arr[..2], &[10, 20]);
  assert_eq!(&arr[3..], &[40, 50]);
  assert_eq!(&arr[..], &[10, 20, 30, 40, 50]);

  // IntoIterator for &AstArray
  let collected: Vec<_> = (&arr).into_iter().copied().collect();
  assert_eq!(collected, vec![10, 20, 30, 40, 50]);

  // for loop over &arr
  let mut sum = 0;
  for &x in &arr {
    sum += x;
  }
  assert_eq!(sum, 150);

  // 可变引用 DerefMut 与 IndexMut
  let mut mut_arr = AstArray::from_mut_slice(&mut data);
  mut_arr[0] = 99;
  mut_arr[2] = 88;
  assert_eq!(mut_arr[0], 99);
  assert_eq!(mut_arr[2], 88);

  // 切片可变方法（DerefMut）
  mut_arr[1..4].reverse();
  assert_eq!(&mut_arr[..], &[99, 40, 88, 20, 50]);
}

#[test]
fn test_ast_array_from_and_default() {
  let slice = [1, 2, 3];
  let arr: AstArray<i32> = AstArray::from(&slice[..]);
  assert_eq!(&arr[..], &[1, 2, 3]);

  let default_arr: AstArray<i32> = AstArray::default();
  assert!(default_arr.is_empty());
}
