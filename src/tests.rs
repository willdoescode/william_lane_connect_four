use super::*;
use std::io::stdout;

// Test pass

#[test]
fn test_new() {
  let board = Game::new();
  let normal = Game {
    count: [6; 7],
    board: [['-'; 7]; 6],
    player: 'O',
    log_update: LogUpdate::new(stdout()).unwrap()
  };

  assert_eq!(board.display_board(), normal.display_board());
}

#[test]
fn test_display() {
  assert_eq!(Game::new().display_board(),
  "  1  2  3  4  5  6  7 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 -  -  -  -  -  -  - 
4 -  -  -  -  -  -  - 
5 -  -  -  -  -  -  - 
6 -  -  -  -  -  -  - 
");
}

#[test]
fn test_slot_change() {
  let mut board = Game::new();
  board.change_slot(0, 3, 'e');
  assert_eq!(board.display_board(),
  "  1  2  3  4  5  6  7 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 -  -  -  -  -  -  - 
4 e  -  -  -  -  -  - 
5 -  -  -  -  -  -  - 
6 -  -  -  -  -  -  - 
");

  board.change_slot(3, 4, 'w');
  assert_eq!(board.display_board(),
  "  1  2  3  4  5  6  7 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 -  -  -  -  -  -  - 
4 e  -  -  -  -  -  - 
5 -  -  -  w  -  -  - 
6 -  -  -  -  -  -  - 
");

  board.change_slot(3, 5, 'j');
  assert_eq!(board.display_board(),
  "  1  2  3  4  5  6  7 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 -  -  -  -  -  -  - 
4 e  -  -  -  -  -  - 
5 -  -  -  w  -  -  - 
6 -  -  -  j  -  -  - 
");


  board.change_slot(4, 5, 'h');
  assert_eq!(board.display_board(),
  "  1  2  3  4  5  6  7 
1 -  -  -  -  -  -  - 
2 -  -  -  -  -  -  - 
3 -  -  -  -  -  -  - 
4 e  -  -  -  -  -  - 
5 -  -  -  w  -  -  - 
6 -  -  -  j  h  -  - 
");
}
