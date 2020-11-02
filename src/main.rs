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
}


fn main() {
  let board = Board::new([
    ['-', '-', '-', '-', '-', '-', '-'],
    ['-', '-', '-', '-', '-', '-', '-'],
    ['-', '-', '-', '-', '-', '-', '-'],
    ['-', '-', '-', '-', '-', '-', '-'],
    ['-', '-', '-', '-', '-', '-', '-'],
    ['-', '-', '-', '-', '-', '-', '-'],
  ]);

  println!("{}", board.display_board() )
}

#[cfg(test)]
mod test;
