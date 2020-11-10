use crate::*;
use std::io::stdout;

#[test]
fn test_new() {
  let board = Game::new();
  let normal = Game {
    count: [6; 7],
    board: [['-'; 7]; 6],
    player: 'O',
    moves: 0,
    log_update: LogUpdate::new(stdout()).unwrap(),
  };

  assert_eq!(board.display_board(), normal.display_board());
}
