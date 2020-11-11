use crate::*;

#[test]
fn test_vert() {
  let mut game = Game {
    count: [6; 7],
    board: [['-'; 7]; 6],
    player: 'O',
    moves: 0,
    log_update: LogUpdate::new(stdout()).unwrap(),
  };
  game.change_slot(3, 5, 'O');
  game.change_slot(3, 4, 'O');
  game.change_slot(3, 3, 'O');
  game.change_slot(3, 2, 'O');
  assert_eq!(game.check_vert(3, 5), true);
  game.change_slot(3, 2, '-');
  assert_ne!(game.check_vert(3, 5), true);
}

#[test]
fn test_down() {
  let mut game = Game {
    count: [6; 7],
    board: [['-'; 7]; 6],
    player: 'O',
    moves: 0,
    log_update: LogUpdate::new(stdout()).unwrap(),
  };
  game.change_slot(3, 5, 'O');
  game.change_slot(3, 4, 'O');
  game.change_slot(3, 3, 'O');
  game.change_slot(3, 2, 'O');
  assert_eq!(game.check_down(3, 2), true);
  game.change_slot(3, 3, '-');
  assert_ne!(game.check_down(3, 2), true);
}

#[test]
fn test_left() {
  let mut game = Game {
    count: [6; 7],
    board: [['-'; 7]; 6],
    player: 'O',
    moves: 0,
    log_update: LogUpdate::new(stdout()).unwrap(),
  };
  game.change_slot(3, 0, 'O');
  game.change_slot(2, 0, 'O');
  game.change_slot(1, 0, 'O');
  game.change_slot(0, 0, 'O');
  assert_eq!(game.check_left(3, 0), true);
  game.change_slot(2, 0, '-');
  assert_ne!(game.check_left(3, 0), true);
}

#[test]
fn test_right() {
  let mut game = Game {
    count: [6; 7],
    board: [['-'; 7]; 6],
    player: 'O',
    moves: 0,
    log_update: LogUpdate::new(stdout()).unwrap(),
  };
  game.change_slot(3, 0, 'O');
  game.change_slot(2, 0, 'O');
  game.change_slot(1, 0, 'O');
  game.change_slot(0, 0, 'O');
  assert_eq!(game.check_right(0, 0), true);
  game.change_slot(2, 0, '-');
  assert_ne!(game.check_right(0, 0), true);
}

#[test]
fn test_right_down_diag() {
  let mut game = Game {
    count: [6; 7],
    board: [['-'; 7]; 6],
    player: 'O',
    moves: 0,
    log_update: LogUpdate::new(stdout()).unwrap(),
  };
  game.change_slot(3, 3, 'O');
  game.change_slot(2, 2, 'O');
  game.change_slot(1, 1, 'O');
  game.change_slot(0, 0, 'O');
  assert_eq!(game.check_down_right(0, 0), true);
  game.change_slot(2, 2, '-');
  assert_ne!(game.check_down_right(0, 0), true);
}

#[test]
fn test_right_up_diag() {
  let mut game = Game {
    count: [6; 7],
    board: [['-'; 7]; 6],
    player: 'O',
    moves: 0,
    log_update: LogUpdate::new(stdout()).unwrap(),
  };
  game.change_slot(3, 3, 'O');
  game.change_slot(4, 2, 'O');
  game.change_slot(5, 1, 'O');
  game.change_slot(6, 0, 'O');
  assert_eq!(game.check_up_right(6, 0), true);
  game.change_slot(5, 1, '-');
  assert_ne!(game.check_up_right(3, 3), true);
}
