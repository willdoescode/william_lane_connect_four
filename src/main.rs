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
    //if self.player == 'O' {
    //self.player = '0'
    //} else {
    //self.player = 'O'
    //}
  }

  fn input(&mut self) {
    let mut input_text = String::new();
    std::io::stdin()
      .read_line(&mut input_text)
      .expect("failed to read from stdin");

      let trimmed = input_text.trim();
      match trimmed.parse::<u32>() {
        Ok(i) => {
          if i > 7 {
            println!("Please enter a number less than 7");
            self.input();
          } else {
            self.animate_down((i - 1) as usize, self.player);
            stdout().flush().expect("could not flush");
          }
        }
        Err(..) => {
          println!("this was not an integer: {}", trimmed);
          self.input();
        }
      };
      self.log_update.done().unwrap();
  }

  fn play(&mut self) {
    self.input();
  }
}

fn main() {
  let mut board = Game::new();
  board.input();
  board.input();
  board.input();
  board.input();
  board.input();
  board.input();
}

#[cfg(test)]
mod tests;
