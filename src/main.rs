use log_update::LogUpdate;
use std::io::stdout;
mod time;

struct Board {
  board: [[char; 7]; 6],
}

impl Board {
  fn new(board: [[char; 7]; 6]) -> Self {
    Board { board }
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

  fn animate_down(&mut self, col: usize) {
    let mut log_update = LogUpdate::new(stdout()).unwrap();
    for x in 0..6 {
      if x > 0 {
        Board::change_slot(self, col, x - 1, '-');
      }
      Board::change_slot(self, col, x, 'h');
      log_update.render(&Board::display_board(self)).unwrap();
      time::sleep_ms(400);
    }
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
  board.animate_down(4);
}

#[cfg(test)]
mod test;
