use std::fmt;
use std::iter::FromIterator;
use std::collections::BTreeSet;

const DIMS: usize = 9;
const AREA: usize = DIMS * DIMS;

struct Sudoku {
  board: [u32; AREA], 
  unsolved_cells: Vec<usize>,
  coords: [(usize, usize); AREA], 
  cols: [[u32; DIMS]; DIMS], 
  rows: [[u32; DIMS]; DIMS], 
  blocks: [[u32; DIMS]; DIMS], 
}

impl fmt::Display for Sudoku {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for i in 0..AREA {
      let space = if i % 3 == 0 { " " } else { "" };
      write!(f, "{}{}", space, self.board[i]);
      if (i + 1) % 9 == 0 { write!(f, "\n"); }
      if (i + 1) % 27 == 0 { write!(f, "\n"); }
    }

    write!(f, "\n")
  }
}

impl Sudoku {
  fn get_cell(&self, row: usize, col: usize) -> u32 {
    let idx = row * 9 + col;
    self.board[idx]
  }

  fn set_cell(&mut self, row: usize, col: usize, value: u32) {
    let idx = row * 9 + col;
    self.board[idx] = value;
  }

  fn is_valid_cell(&self, row: usize, col: usize, value: u32) -> bool {
    let idx = (row * 9 + col) as u32;

    // check row
    for r in self.rows[row].iter() {
      if *r == idx { continue; }
      if value == self.board[*r as usize] { return false; }
    }

    // check column
    for c in self.cols[col].iter() {
      if *c == idx { continue; }
      if value == self.board[*c as usize] { return false; }
    }

    // check local box
    let block = (row / 3) * 3 + (col / 3);
    for b in self.blocks[block].iter() {
      if *b == idx { continue; }
      if value == self.board[*b as usize] { return false; }
    }

    true
  } 

  fn find_solution(&self, row: usize, col: usize) -> Option<u32> {
    let cell = self.get_cell(row, col) + 1;
    
    for i in cell..=9 {
      if self.is_valid_cell(row, col, i) { return Some(i); }
    }

    None
  }

  fn solve(&mut self) {
    let mut backtrack_idx = 0;

    while backtrack_idx < self.unsolved_cells.len() {
      let (row, col) = self.coords[self.unsolved_cells[backtrack_idx]];

      match self.find_solution(row, col) {
        Some(s) => {
          self.set_cell(row, col, s);
          backtrack_idx += 1;
        },
        None => {
          self.set_cell(row, col, 0);
          backtrack_idx -= 1;
        }
      }
    }
  }
}

impl FromIterator<(usize, u32)> for Sudoku {
  fn from_iter<T>(iter: T) -> Self 
    where T: IntoIterator<Item=(usize, u32)> 
  {
    let mut board = [0; AREA];
    let mut filled_slots = BTreeSet::new();

    for (i, n) in iter {
      board[i] = n; 
      filled_slots.insert(i);
    }

    let mut unsolved_cells: Vec<_> = (0..AREA).collect();
    unsolved_cells.retain(|x| !filled_slots.contains(x));

    let mut board_coords:[(usize, usize); AREA] = [(0, 0); AREA];
    board_coords.copy_from_slice((0..AREA).map(|i| (i / 9, i % 9)).collect::<Vec<_>>().as_slice());

    let board_rows: [[u32; DIMS]; DIMS] = {
      let mut rows = [[0; DIMS]; DIMS];
      for x in 0..DIMS {
        for y in 0..DIMS {
          rows[x][y] = (x * DIMS + y) as u32;
        }
      }
      rows
    };

    let board_cols: [[u32; DIMS]; DIMS] = {
      let mut cols = [[0; DIMS]; DIMS];
      for x in 0..DIMS { 
        for y in 0..DIMS {
          cols[y][x] = (x * DIMS + y) as u32;
        }
      }
      cols
    };

    let board_blocks: [[u32; DIMS]; DIMS] = {
      let mut blocks = [[0; DIMS]; DIMS];
      for x in 0..DIMS {
        for y in 0..DIMS {
          let idx = x * DIMS + y;
          let block = (x / 3) * 3 + (y / 3);
          let idx_in_block = (x % 3) * 3 + (y % 3);
          blocks[block][idx_in_block] = idx as u32;
        }
      }
      blocks
    };

    Sudoku { 
      board, 
      unsolved_cells,
      cols: board_cols, 
      rows: board_rows, 
      coords: board_coords, 
      blocks: board_blocks,
    }
  }
}

fn main() {
  let mut sudoku1 = Sudoku::from_iter(vec![
    (2, 7),(6, 3),(7, 1),(9, 6),(13, 9),(15, 7),(19, 1),(23, 8),(27, 2),(29, 6),(30, 8),
    (32, 9),(37, 4),(39, 6),(41, 1),(43, 9),(48, 3),(50, 7),(51, 8),(53, 6),(57, 7),
    (61, 3),(65, 1),(67, 8),(71, 2),(73, 2),(74, 5),(78, 6)
  ]);
  println!("{}", sudoku1);
  sudoku1.solve();
  println!("{}", sudoku1);

  let mut sudoku2 = Sudoku::from_iter(vec![
    (2,4),(9,9),(10,5),(12,4),(17,8),(22,1),(24,5),(26,6),(28,3),(30,6),
    (35,5),(37,1),(39,3),(41,8),(43,6),(45,4),(50,5),(52,7),(54,8),(56,9),
    (58,4),(63,3),(68,2),(70,5),(71,4),(78,2)
  ]);
  println!("{}", sudoku2);
  sudoku2.solve();
  println!("{}", sudoku2);
}