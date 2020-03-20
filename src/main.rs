use std::collections::HashSet;
use std::collections::VecDeque;

// representation of an unsolved cell
struct Cell {
  // set of possible values this cell could be
  possibilities: HashSet<u32>, 
}

// representation of a solved cell
struct SolvedCell {
  // need to keep track of the solved cell's coordinates since 
  // we aren't storing them in a matrix
  x: u32,
  y: u32,
  value: u32,
}

struct Board {
  cells: Vec<Vec<Cell>>,
  // keep solved cells in a queue so we process them in FIFO order
  solved_cells: VecDeque<SolvedCell>,
}

impl Cell {
  fn new(v: u32) -> Self {
    Cell {
      // init a set contain 1..9 as possibilities is v is 0
      // otherwise, init a set with just v 
      possibilities: if v == 0 { (1..=9).collect() } else { [v].iter().cloned().collect() }
    }
  }

  // resolve the cell's value to the only possibility when len(possibilities) == 1
  fn resolve(&self) -> u32 {
    match self.possibilities.len() {
      1 => *self.possibilities.iter().next().unwrap(),
      _ => 0,
    }
  }

  fn remove_possibility(&mut self, v: u32) {
    self.possibilities.remove(&v);
  }

  fn is_solved(&self) -> bool {
    self.possibilities.len() == 1
  }
}

impl Board {
  fn new(rows: [[u32; 9]; 9]) -> Self {
    let mut cells: Vec<Vec<Cell>> = vec![];
    let mut solved_cells: VecDeque<SolvedCell> = VecDeque::new();

    // create rows for the board
    for x in 0..rows.len() {
      let mut new_row: Vec<Cell> = vec![];

      // create columns for the board
      for y in 0..rows[x].len() {
        let v = rows[x][y];
        let cell = Cell::new(v);
        new_row.push(cell);

        // this cell has already been solved
        if v != 0 {
          solved_cells.push_back(SolvedCell { x: x as u32, y: y as u32, value: v });
        }
      }

      cells.push(new_row);
    }

    Board {
      cells,
      solved_cells,
    }
  }

  fn print(&self) {
    for x in 0..self.cells.len() {
      for y in 0..self.cells[x].len() {
        let buf = if y % 3 == 0 { " " } else { "" };
        print!("{}{}", buf, self.cells[x][y].resolve());
      }

      print!("\n");

      if (x + 1) % 3 == 0 {
        print!("\n");
      }
    }

    println!("*************\n");
  }

  // dequeues the next solved cell and checks its corresponding row, column, and block,
  // reducing any possibilities in its neighboring cells and adding the newly solved 
  // cells to the queue 
  fn solve(&mut self) {
    while let Some(solved) = self.solved_cells.pop_front() {
      self.reduce_possibilities(solved);
    }
  }

  // loops through the row, column, and block of the solved cell and removes the solved cell's
  // value as a possibility from all its neighboring cells 
  fn reduce_possibilities(&mut self, solved: SolvedCell) {
    // narrow down horizontal possibilities
    for x in 0..9 {
      self.reduce_cell_possibilities(x, solved.y, solved.value);
    }

    // narrow down vertical possibilities
    for y in 0..9 {
      self.reduce_cell_possibilities(solved.x, y, solved.value);
    }

    // narrow down possibilities in the 3x3 block this cell belongs to
    let sqx: u32 = (solved.x / 3) * 3;
    let sqy: u32 = (solved.y / 3) * 3;

    for x in 0..3 {
      for y in 0..3 {
        self.reduce_cell_possibilities(x + sqx, y + sqy, solved.value);
      }
    }
  }

  // remove the value as a possibility from the cell at the given coordinates 
  // if the cell becomes solved, add it to the queue of solved cells 
  fn reduce_cell_possibilities(&mut self, x: u32, y: u32, v: u32) {
    let cell: &mut Cell = &mut self.cells[x as usize][y as usize];

    if !cell.is_solved() {
      cell.remove_possibility(v);

      // check if the above operation solved the cell
      if cell.is_solved() {
        self.solved_cells.push_back(SolvedCell {
          x: x as u32,
          y: y as u32,
          value: cell.resolve(),
        });
      }
    }
  }
}

fn main() {
  let config1 = [
    [7, 0, 6, 0, 4, 0, 9, 0, 0],
    [0, 0, 0, 1, 6, 2, 0, 7, 0],
    [5, 0, 3, 0, 0, 0, 1, 0, 4],
    [0, 5, 0, 6, 0, 4, 0, 1, 0],
    [4, 3, 0, 0, 0, 0, 0, 2, 6],
    [0, 6, 0, 3, 0, 9, 0, 4, 0],
    [3, 0, 4, 0, 0, 0, 6, 0, 8],
    [0, 7, 0, 8, 3, 6, 0, 0, 0],
    [0, 0, 1, 0, 9, 0, 2, 0, 7],
  ];
  let config2 = [
    [5, 3, 0, 0, 7, 0, 0, 0, 0],
    [6, 0, 0, 1, 9, 5, 0, 0, 0],
    [0, 9, 8, 0, 0, 0, 0, 6, 0],
    [8, 0, 0, 0, 6, 0, 0, 0, 3],
    [4, 0, 0, 8, 0, 3, 0, 0, 1],
    [7, 0, 0, 0, 2, 0, 0, 0, 6],
    [0, 6, 0, 0, 0, 0, 2, 8, 0],
    [0, 0, 0, 4, 1, 9, 0, 0, 5],
    [0, 0, 0, 0, 8, 0, 0, 7, 9]
  ];

  let mut board1 = Board::new(config1);

  board1.print();
  board1.solve();
  board1.print();

  let mut board2 = Board::new(config2);

  board2.print();
  board2.solve();
  board2.print();
}