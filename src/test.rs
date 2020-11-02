use super::*;
use std::io::stdout;

// Test pass

#[test]
fn test_new() {
  let board = Board::new();
  let normal = Board {
    board: [
      ['-', '-', '-', '-', '-', '-', '-'],
      ['-', '-', '-', '-', '-', '-', '-'],
      ['-', '-', '-', '-', '-', '-', '-'],
      ['-', '-', '-', '-', '-', '-', '-'],
      ['-', '-', '-', '-', '-', '-', '-'],
      ['-', '-', '-', '-', '-', '-', '-'],
    ],
    log_update: LogUpdate::new(stdout()).unwrap()
  };

  assert_eq!(board.display_board(), normal.display_board());
}

#[test]
fn test_display() {
  assert_eq!(Board::new().display_board(),
  "  0  1  2  3  4  5  6 
0 -  -  -  -  -  -  - 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 -  -  -  -  -  -  - 
4 -  -  -  -  -  -  - 
5 -  -  -  -  -  -  - 
");
}

#[test]
fn test_slot_change() {
  let mut board = Board::new();
  board.change_slot(0, 3, 'e');
  assert_eq!(board.display_board(),
  "  0  1  2  3  4  5  6 
0 -  -  -  -  -  -  - 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 e  -  -  -  -  -  - 
4 -  -  -  -  -  -  - 
5 -  -  -  -  -  -  - 
");

  board.change_slot(3, 4, 'w');
  assert_eq!(board.display_board(),
  "  0  1  2  3  4  5  6 
0 -  -  -  -  -  -  - 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 e  -  -  -  -  -  - 
4 -  -  -  w  -  -  - 
5 -  -  -  -  -  -  - 
");

  board.change_slot(3, 5, 'j');
  assert_eq!(board.display_board(),
  "  0  1  2  3  4  5  6 
0 -  -  -  -  -  -  - 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 e  -  -  -  -  -  - 
4 -  -  -  w  -  -  - 
5 -  -  -  j  -  -  - 
");


  board.change_slot(4, 5, 'h');
  assert_eq!(board.display_board(),
  "  0  1  2  3  4  5  6 
0 -  -  -  -  -  -  - 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 e  -  -  -  -  -  - 
4 -  -  -  w  -  -  - 
5 -  -  -  j  h  -  - 
");
}
