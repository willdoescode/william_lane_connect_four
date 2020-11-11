#![allow(dead_code)]
#![allow(clippy::collapsible_if)]
use log_update::LogUpdate;
use std::io::{prelude::*, stdout};
mod time;

// Create a struct to contain the game state
struct Game {
  // Create a count to know how far down slots can animate
  count: [u8; 7],
  // Create an array of 6 other arrays that contain 7 chars to act as the board
  board: [[char; 7]; 6],
  // Define a player char
  player: char,
  // Track if the board is full based on the amount of moves
  // If moves are more than or equal to 42 than the board is full (6 * 7 = 42)
  moves: u32,
  // Create a log update state to manipulate within functions
  log_update: LogUpdate<std::io::Stdout>,
}

impl Game {
  // Helper function that returns a new Game
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
    // Define a return string
    let mut board = String::new();
    // Push an initial space for the first number on the top row
    board.push(' ');
    for i in 0..7 {
      // I have to convert the String to a &str using a reference to a slice
      board.push_str(&format!(" {} ", i + 1)[..]);
      if i == 6 {
        // Pushing new line at end of the row
        board.push('\n');
      }
    }

    for (index, row) in self.board.iter().enumerate() {
      for (slot_index, slot) in row.iter().enumerate() {
        if slot_index == 0 {
          // I have to convert the String to a &str using a reference to a slice
          board.push_str(&format!("{}", index + 1)[..]);
        }
        board.push_str(&format!(" {} ", slot)[..]);
        if slot_index == 6 {
          board.push('\n');
        }
      }
    }
    // Return the board string to whatever needs it
    board
  }

  // Helper function to change a position in the board to a character
  fn change_slot(&mut self, c: usize, r: usize, new: char) {
    self.board[r][c] = new;
  }

  fn animate_down(&mut self, col: usize, down_char: char) {
    // Make sure the player doesnt animate over the other peices
    for x in 0..self.count[col] {
      if x > 0 {
        self.change_slot(col, x as usize - 1, '-');
      }
      self.change_slot(col, x as usize, down_char);
      // Render the board
      self.log_update.render(&self.display_board()).unwrap();
      // Sleep function written in other file
      time::sleep_ms(200);
    }
    self.count[col] -= 1;
  }

  fn input(&mut self) {
    // Define input string
    let mut input_text = String::new();
    std::io::stdin()
      .read_line(&mut input_text)
      .expect("failed to read from stdin");

      // Trim the string to get rid of trailing whitespaces 
      let trimmed = input_text.trim();
      match trimmed.parse::<u32>() {
        Ok(i) => {
          // Account for edge cases and error handling
          if i != 0 && i <= 7 {
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

  // Win helper function that displays a win message and exits
  fn win(&mut self) {
    self.log_update.done().unwrap();
    println!("{} has won!", self.player);
    std::process::exit(0);
  }

  // Tie helper function that displays a tie message and exits
  fn tie(&mut self) {
    self.log_update.done().unwrap();
    println!("No one won (tie)");
    std::process::exit(0);
  }

  // To check for a tie you simply have to see if more than 42 moves have been made
  fn check_tie(&self) -> bool {
    self.moves >= 42
  }

  fn check_left(&self, c: usize, r: usize) -> bool {
    // Check left relative to the most recent move
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
    // Check right relative to the most recent move
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
    // Check up relative to the most recent move
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
    // Check down relative to the most recent move
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
    // Check diag down right relative to the most recent move
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
    // Check diag up right relative to the most recent move
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
    // Check diag up left relative to the most recent move
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
    // Check diag down left relative to the most recent move
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
    // If there is a tie display the tie message
    if self.check_tie() {
      self.tie()
    }

    // If any of the functions return true there is a win
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
      // If there is a win display the win message
      self.win()
    }
    
    // Change the player char relative to the current 
    if self.player == 'O' {
      self.player = '0'
    } else {
      self.player = 'O'
    }
    // Increment the move count 
    self.moves += 1;
  }

  fn play(&mut self) {
    // The win and tie function stop the process with std::process::exit(0) so it is ok to have an infinite loop
    loop {
      self.input();
    }
  }
}

// Helper function to check if all items in array are equal to a char/player
fn check_arr(a: [char; 4], player: char) -> bool {
  a.iter().all(|&i| i == player)
}

fn main() {
  // Define a board
  let mut board = Game::new();
  // Let the games begin
  board.play();
}

// Define a test cfg to store in other files
#[cfg(test)]
mod tests;
