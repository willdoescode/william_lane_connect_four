#![allow(dead_code)]
#![allow(clippy::collapsible_if)]
use log_update::LogUpdate;
use std::io::{prelude::*, stdout};
mod time;

struct Game {
  count: [u8; 7],
  board: [[char; 7]; 6],
  player: char,
  moves: u32,
  log_update: LogUpdate<std::io::Stdout>,
}

impl Game {
  fn new() -> Self {
    Game {
      count: [6; 7],
      board: [['-'; 7]; 6],
      player: 'O',
      moves: 1,
      log_update: LogUpdate::new(stdout()).unwrap(),
    }
  }

  fn display_board(&self) -> String {
    let mut board = String::new();
    board.push(' ');
    for i in 0..7 {
      board.push_str(&format!(" {} ", i + 1)[..]);
      if i == 6 {
        board.push('\n');
      }
    }

    for (index, row) in self.board.iter().enumerate() {
      for (slot_index, slot) in row.iter().enumerate() {
        if slot_index == 0 {
          board.push_str(&format!("{}", index + 1)[..]);
        }
        board.push_str(&format!(" {} ", slot)[..]);
        if slot_index == 6 {
          board.push('\n');
        }
      }
    }
    board
  }

  fn change_slot(&mut self, c: usize, r: usize, new: char) {
    self.board[r][c] = new;
  }

  fn animate_down(&mut self, col: usize, down_char: char) {
    for x in 0..self.count[col] {
      if x > 0 {
        self.change_slot(col, x as usize - 1, '-');
      }
      self.change_slot(col, x as usize, down_char);
      self.log_update.render(&self.display_board()).unwrap();
      time::sleep_ms(200);
    }
    self.count[col] -= 1;
  }

  fn input(&mut self) {
    let mut input_text = String::new();
    std::io::stdin()
      .read_line(&mut input_text)
      .expect("failed to read from stdin");

      let trimmed = input_text.trim();
      match trimmed.parse::<u32>() {
        Ok(i) => {
          if i != 0 {
            if self.count[i as usize - 1] == 0 {
              println!("Please choose an empty column");
              self.input();
            } else if i > 7 {
              println!("Please enter a number less than 7 and more than 0");
              self.input();
            } else {
              self.animate_down((i - 1) as usize, self.player);
              self.check_win(i as usize - 1);
              stdout().flush().expect("could not flush");
            }
          } else {

            println!("Please enter a number less than 7 and more than 0");
            self.input();
          }
        }
        Err(..) => {
          println!("this was not an positive integer: {}", trimmed);
          self.input();
        }
      };
      self.log_update.done().unwrap();
  }

  fn win(&mut self) {
    self.log_update.done().unwrap();
    println!("{} has won!", self.player);
    std::process::exit(0);
  }

  fn tie(&mut self) {
    self.log_update.done().unwrap();
    println!("No one won (tie)");
    std::process::exit(0);
  }

  fn check_tie(&self) -> bool {
    self.moves >= 42
  }

  fn check_left(&self, c: usize, r: usize) -> bool {
    if c > 2 {
      return check_arr(
        [
        self.board[r][c],
        self.board[r][c - 1],
        self.board[r][c - 2],
        self.board[r][c - 3],
        ],
        self.player,
      );
    }
    false
  }

  fn check_right(&self, c: usize, r: usize) -> bool {
    if c < 4 {
      return check_arr(
        [
        self.board[r][c],
        self.board[r][c + 1],
        self.board[r][c + 2],
        self.board[r][c + 3],
        ],
        self.player,
      );
    }
    false
  }

  fn check_vert(&self, c: usize, r: usize) -> bool {
    if r > 2 {
      return check_arr(
        [
        self.board[r][c],
        self.board[r - 1][c],
        self.board[r - 2][c],
        self.board[r - 3][c],
        ],
        self.player,
      );
    }
    false
  }

  fn check_down(&self, c: usize, r: usize) -> bool {
    if r < 3 {
      return check_arr(
        [
        self.board[r][c],
        self.board[r + 1][c],
        self.board[r + 2][c],
        self.board[r + 3][c],
        ],
        self.player,
      );
    }
    false
  }

  fn check_down_right(&self, c: usize, r: usize) -> bool {
    if r < 3 && c < 4 {
      return check_arr(
        [
        self.board[r][c],
        self.board[r + 1][c + 1],
        self.board[r + 2][c + 2],
        self.board[r + 3][c + 3],
        ],
        self.player,
      );
    }
    false
  }

  fn check_up_right(&self, c: usize, r: usize) -> bool {
    if r < 3 && c > 2 {
      return check_arr(
        [
        self.board[r][c],
        self.board[r + 1][c - 1],
        self.board[r + 2][c - 2],
        self.board[r + 3][c - 3],
        ],
        self.player,
      );
    }
    false
  }

  fn check_up_left(&self, c: usize, r: usize) -> bool {
    if r > 3 && c < 4 {
      return check_arr(
        [
        self.board[r][c],
        self.board[r - 1][c + 1],
        self.board[r - 2][c + 2],
        self.board[r - 3][c + 3],
        ],
        self.player,
      );
    }
    false
  }

  fn check_down_left(&self, c: usize, r: usize) -> bool {
    if r < 3 && c > 2 {
      return check_arr(
        [
        self.board[r][c],
        self.board[r + 1][c - 1],
        self.board[r + 2][c - 2],
        self.board[r + 3][c - 3],
        ],
        self.player,
      );
    }
    false
  }

  fn check_win(&mut self, c: usize) {
    let r = self.count[c] as usize;
    if self.check_tie() {
      self.tie()
    }

    if [
      self.check_left(c, r),
      self.check_right(c, r),
      self.check_vert(c, r),
      self.check_down(c, r),
      self.check_down_right(c, r),
      self.check_up_right(c, r),
      self.check_up_left(c, r),
      self.check_down_left(c, r),
    ]
      .iter()
      .any(|&b| b)
    {
      self.win()
    }

    if self.player == 'O' {
      self.player = '0'
    } else {
      self.player = 'O'
    }
    self.moves += 1;
  }

  fn play(&mut self) {
    loop {
      self.input();
    }
  }
}

fn check_arr(a: [char; 4], player: char) -> bool {
  a.iter().all(|&i| i == player)
}

fn main() {
  let mut board = Game::new();
  board.play();
}

#[cfg(test)]
mod tests;
