use alloc::vec;
use core::cmp::min;

pub fn edit_distance(mut a: &[u8], mut b: &[u8]) -> usize {
  // 前缀/后缀逐字节相等的部分是零代价对齐，先剥离（cpp 同款预处理）：
  // 公共前缀取 zip 对齐长度，公共后缀取反向 zip 对齐长度。
  let prefix = a.iter().zip(b).take_while(|(x, y)| x == y).count();
  a = &a[prefix..];
  b = &b[prefix..];
  let suffix = a
    .iter()
    .rev()
    .zip(b.iter().rev())
    .take_while(|(x, y)| x == y)
    .count();
  a = &a[..a.len() - suffix];
  b = &b[..b.len() - suffix];

  if a.is_empty() {
    return b.len();
  }
  if b.is_empty() {
    return a.len();
  }

  let max_distance = a.len() + b.len();
  let b_stride = b.len() + 2;
  let mut distances = vec![0; (a.len() + 2) * b_stride];

  distances[0] = max_distance;

  // 边界带初始化：`distances` 是 (a.len()+2) × b_stride 的 DP 矩阵（行主序展平），
  // 故按行切片写入就是迭代器形态，无需手推行列号：
  // - 第 0 行：除 `[0,0]` 哨兵外整行是「不可达」哨兵值；
  // - 第 1 行：`[1,0]` 哨兵，`[1, y+1] = y`（纯插入代价）；
  // - 第 x+1 行（x ≥ 1）：`[x+1, 0]` 哨兵，`[x+1, 1] = x`（纯删除代价）。
  for (x, row) in distances.chunks_mut(b_stride).enumerate() {
    match x {
      0 => row[1..].fill(max_distance),
      1 => {
        row[0] = max_distance;
        for (y, cell) in row[1..].iter_mut().enumerate() {
          *cell = y;
        }
      }
      _ => {
        row[0] = max_distance;
        row[1] = x - 1;
      }
    }
  }

  let mut seen_char_to_row = [0usize; 256];

  // 主 DP：Damerau-Levenshtein 最优串算法。下标 x/y 是矩阵坐标且转移要按
  // `seen_char_to_row` 回看任意历史行（`x1`），索引即数据（§3 例外），故保留
  // 0..=len 形态；`a[x - 1]`/`b[y - 1]` 的偏移在构造上严格落在切片内。
  for x in 1..=a.len() {
    let mut last_matched_y = 0;
    // 行主序展平下，当前行与下一行的基址对整列 y 恒定，提到内层之外；
    // 只有转置回看的历史行 `x1` 随 y 变化，仍需现算。
    let row_x = x * b_stride;
    let row_next = row_x + b_stride;

    for y in 1..=b.len() {
      let b_seen_char_index = b[y - 1] as usize;
      let x1 = seen_char_to_row[b_seen_char_index];
      let y1 = last_matched_y;

      let mut cost = 1;
      if a[x - 1] == b[y - 1] {
        cost = 0;
        last_matched_y = y;
      }

      let transposition = distances[x1 * b_stride + y1] + (x - x1 - 1) + 1 + (y - y1 - 1);
      let substitution = distances[row_x + y] + cost;
      let insertion = distances[row_x + y + 1] + 1;
      let deletion = distances[row_next + y] + 1;

      distances[row_next + y + 1] = min(min(insertion, deletion), min(substitution, transposition));
    }

    let a_seen_char_index = a[x - 1] as usize;
    seen_char_to_row[a_seen_char_index] = x;
  }

  distances[(a.len() + 1) * b_stride + b.len() + 1]
}
