#![allow(dead_code)]
use log_update::LogUpdate;
use std::io::prelude::*;
use std::io::stdout;
mod time;

struct Game {
  count: [u8; 7],
  board: [[char; 7]; 6],
  player: char,
  log_update: LogUpdate<std::io::Stdout>,
}

impl Game {
  fn new() -> Self {
    Game {
      count: [6; 7],
      board: [['-'; 7]; 6],
      player: 'O',
      log_update: LogUpdate::new(stdout()).unwrap(),
    }
  }

  fn display_board(&self) -> String {
    let mut res = String::new();
    res.push(' ');
    for i in 0..7 {
      res.push_str(&format!(" {} ", i + 1)[..]);
      if i == 6 {
        res.push('\n');
      }
    }

    for (index, row) in self.board.iter().enumerate() {
      for (slot_index, slot) in row.iter().enumerate() {
        if slot_index == 0 {
          res.push_str(&format!("{}", index + 1)[..]);
        }
        res.push_str(&format!(" {} ", slot)[..]);
        if slot_index == 6 {
          res.push('\n');
        }
      }
    }
    res
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
          if i > 7 || i <= 0 {
            println!("Please enter a number less than 7 and more than 0");
            self.input();
          } else {
            self.animate_down((i - 1) as usize, self.player);
            self.check_win(i as usize - 1);
            stdout().flush().expect("could not flush");
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

  fn check_win(&mut self, c: usize) {
    let r = self.count[c] as usize;
    if c > 2 {
      if check_arr([self.board[r][c], self.board[r][c - 1], self.board[r][c - 2], self.board[r][c - 3]], self.player) {
        self.win()
      }
    }
    if c < 4 {
      if check_arr([self.board[r][c], self.board[r][c + 1], self.board[r][c + 2], self.board[r][c + 3]], self.player) {
        self.win()
      }
    }
    if r > 2 {
      if check_arr([self.board[r][c], self.board[r - 1][c], self.board[r - 2][c], self.board[r - 3][c]], self.player) {
        self.win()
      }
    }
    if r < 3 {
      if check_arr([self.board[r][c], self.board[r + 1][c], self.board[r + 2][c], self.board[r + 3][c]], self.player) {
        self.win()
      }
    }
    if r < 3 && c < 4 {
      if check_arr([self.board[r][c], self.board[r + 1][c + 1], self.board[r + 2][c + 2], self.board[r + 3][c + 3]], self.player) {
        self.win()
      }
    }
    if r < 3 && c > 2 {
      if check_arr([self.board[r][c], self.board[r + 1][c - 1], self.board[r + 2][c - 2], self.board[r + 3][c - 3]], self.player) {
        self.win()
      }
    }
    if r > 3 && c < 4 {
      if check_arr([self.board[r][c], self.board[r - 1][c + 1], self.board[r - 2][c + 2], self.board[r - 3][c + 3]], self.player) {
        self.win()
      }
    }
    if r > 3 && c > 2 {
      if check_arr([self.board[r][c], self.board[r - 1][c - 1], self.board[r - 2][c - 2], self.board[r - 3][c - 3]], self.player) {
        self.win()
      }
    }
    if self.player == 'O' {
    self.player = '0'
    } else {
    self.player = 'O'
    }
  }

  fn play(&mut self) {
    self.input();
  }
}

fn check_arr(a: [char; 4], player: char) -> bool {
  a.iter().all(|&i| i == player) 
}


fn main() {
  let mut board = Game::new();
  loop {
    board.input();
  }
}

#[cfg(test)]
mod tests;
