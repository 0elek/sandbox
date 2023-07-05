use rand::Rng;
use std::thread::sleep;
use std::time::{Duration, Instant};

const WIDTH: usize = 260;
const HEIGHT: usize = 60;
static NEIGHBOURS: [(i32, i32); 8] = [
    (-1, 1),
    (0, 1),
    (1, 1),
    (-1, 0),
    (1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

#[derive(Debug, Clone, Copy)]
struct Sandbox {
    cells: [[Cell; WIDTH]; HEIGHT],
}

impl Sandbox {
    fn new() -> Sandbox {
        Sandbox {
            cells: ([[Cell::new(); WIDTH]; HEIGHT]),
        }
    }
    fn neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;

        for (dx, dy) in NEIGHBOURS.iter() {
            let x2 = x as i32 + dx;
            let y2 = y as i32 + dy;

            if x2 >= 0 && x2 < WIDTH as i32 && y2 >= 0 && y2 < HEIGHT as i32 {
                if self.cells[y2 as usize][x2 as usize].alive {
                    if count > 4 {
                        return count;
                    }
                    count += 1;
                }
            }
        }
        count
    }

    fn print(&self) {
        print!("{}[2J", 27 as char);
        for rows in self.cells.iter() {
            for cell in rows.iter() {
                match cell.alive {
                    true => {
                        print!("{}[0;31m█{}[0;37m", 27 as char, 27 as char);
                        //{}", self.neighbours(count2 as usize - 1, count as usize - 1));
                    }
                    false => {
                        print!("░");
                    }
                }
            }
            println!();
        }
    }
    fn update(&mut self) {
        let mut new_cells = self.cells;

        for (y, rows) in self.cells.iter().enumerate() {
            for (x, cell) in rows.iter().enumerate() {
                let n = self.neighbours(x, y);

                if cell.alive {
                    if n < 2 || n > 3 {
                        new_cells[y][x].alive = false;
                    }
                } else if n == 3 {
                    new_cells[y][x].alive = true;
                }
            }
        }

        self.cells = new_cells;
    }
}

#[derive(Debug, Clone, Copy)]
struct Cell {
    alive: bool,
}

impl Cell {
    fn new() -> Cell {
        Cell { alive: false }
    }
}

fn main() {
    let mut sandbox: Sandbox = Sandbox::new();

    let mut i: i32 = 6000;
    while i > 1 {
        let mut rng = rand::thread_rng();
        let x: usize = rng.gen_range(0..WIDTH);
        let y: usize = rng.gen_range(0..HEIGHT);
        sandbox.cells[y][x].alive = true;
        i -= 1;
    }
    sandbox.print();
    sleep(Duration::from_millis(1000));

    loop {
        let time = Instant::now();

        sandbox.update();
        sandbox.print();

        println!("{:?}", time.elapsed());
        sleep(Duration::from_millis(50))
    }
}
