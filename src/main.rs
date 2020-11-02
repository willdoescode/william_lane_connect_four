#[allow(dead_code)]

use log_update::LogUpdate;
use std::io::prelude::*;
use std::io::stdout;
mod time;

struct Board {
  board: [[char; 7]; 6],
  log_update: LogUpdate<std::io::Stdout>
}

impl Board {
  fn new(board: [[char; 7]; 6]) -> Self {
    Board {
      board,
      log_update: LogUpdate::new(stdout()).unwrap() 
    }
  }

  fn display_board(&self) -> String {
    let mut res = String::new();
    res.push_str(" ");
    for i in 0..7 {
      res.push_str(&format!(" {} ", i)[..]);
      if i == 6 {
        res.push('\n');
      }
    }

    for (index, row) in self.board.iter().enumerate() {
      for (slot_index, slot) in row.iter().enumerate() {
        if slot_index == 0 {
          res.push_str(&format!("{}", index)[..]);
        }
        res.push_str(&format!(" {} ", slot)[..]);
        if slot_index == 6 {
          res.push('\n');
        }
      }
    }
    res
  }

  fn change_slot(&mut self, x: usize, y: usize, new: char)  {
    self.board[y][x] = new;
  }

  fn animate_down(&mut self, col: usize, down_char: char) {
    for x in 0..6 {
      if x > 0 {
        self.change_slot(col, x - 1, '-');
      }
        self.change_slot(col, x, down_char);
        self.log_update.render(&self.display_board()).unwrap();
        time::sleep_ms(400);
    }
  }

  fn input(&mut self) {
    let mut input_text = String::new();
    std::io::stdin()
      .read_line(&mut input_text)
      .expect("failed to read from stdin");



      let trimmed = input_text.trim();
      match trimmed.parse::<u32>() {
        Ok(i) =>{
          &self.animate_down(i as usize, 'h');
          stdout().flush().ok().expect("could not flush");
        }, 
        Err(..) => println!("this was not an integer: {}", trimmed),
      };
      let _ = self.log_update.done();
  }

}


fn main() {
  let mut board = Board::new([
    ['-', '-', '-', '-', '-', '-', '-'],
    ['-', '-', '-', '-', '-', '-', '-'],
    ['-', '-', '-', '-', '-', '-', '-'],
    ['-', '-', '-', '-', '-', '-', '-'],
    ['-', '-', '-', '-', '-', '-', '-'],
    ['-', '-', '-', '-', '-', '-', '-'],
  ]);
  board.input();
  board.input();
  board.input();
}

#[cfg(test)]
mod test;
