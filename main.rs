use rand; // 0.8.5
use rand::Rng;
use std::fmt;
use std::io;

struct Board<const WIDTH: usize, const HEIGHT: usize> {
    board_data: [[Tile; HEIGHT]; WIDTH],
    game_over: bool
}
impl<const WIDTH: usize, const HEIGHT: usize> Board<WIDTH, HEIGHT> {
    fn new(mine_count: usize) -> Board<WIDTH, HEIGHT> {
        let mut board_data = [[Tile::Safe; HEIGHT]; WIDTH];
        let mut rng = rand::thread_rng();
        for _ in 0..mine_count {
            let x = rng.gen_range(0..WIDTH);
            let y = rng.gen_range(0..HEIGHT);
            board_data[x][y] = Tile::Mine;
        }
        Board {board_data, game_over: false}
    }
    fn is_solved(&self) -> bool {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if let Tile::Mine | Tile::Safe = self.board_data[x][y] {
                    return false;
                }
            }
        }
        return true;
    }
    fn reveal(&mut self, x: usize, y: usize) -> bool {
        match self.board_data[x][y] {
            Tile::Mine => true,
            Tile::Safe => {
                let mut s = 0;
                for (xs, ys) in self.get_adjacent(x, y) {
                    if let Tile::Mine | Tile::FlaggedMine = self.board_data[xs][ys] {
                        s += 1;
                    }
                }
                self.board_data[x][y] = Tile::Revealed(s);
                false
            }
            _ => false
        }
    }
    fn flag(&mut self, x: usize, y: usize) {
        match self.board_data[x][y] {
            Tile::Mine => { self.board_data[x][y] = Tile::FlaggedMine }
            Tile::FlaggedMine => { self.board_data[x][y] = Tile::Mine }
            Tile::Safe => { self.board_data[x][y] = Tile::FlaggedSafe }
            Tile::FlaggedSafe => { self.board_data[x][y] = Tile::Safe }
            _ => return
        }
    }
    fn get_adjacent(&self, x: usize, y: usize) -> Box<dyn Iterator<Item = (usize, usize)>> {
        let v: Vec<(usize, usize)> = [(1isize,0isize),(-1,0),(0,1),(0,-1),(1,1),(1,-1),(-1,1),(-1,-1)]
        .into_iter().filter_map(|(xs, ys)| {
            let xs = xs.checked_add(x.try_into().ok()?)? as usize;
            let ys = ys.checked_add(y.try_into().ok()?)? as usize;
            if xs < WIDTH && ys < HEIGHT {
                Some((xs, ys))
            } else {
                None
            }
        }).collect();
        Box::new(v.into_iter())
    }
}
impl<const W: usize, const H: usize> fmt::Display for Board<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "     ")?;
        for x in 0..W {
            write!(f, "{:<2} ", x+1)?;
        }
        writeln!(f, "")?;
        write!(f, "   +")?;
        for _ in 0..W {
            write!(f, "---")?;
        }
        writeln!(f, "")?;
        for y in 0..H {
            write!(f, "{:>2} | ", y+1)?;
            for x in 0..W {
                let tile = self.board_data[x][y];
                write!(f, "{}  ", self.board_data[x][y].get_char(self.game_over))?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Copy,Clone,Debug)]
enum Tile {
    Mine,
    Safe,
    FlaggedSafe,
    FlaggedMine,
    Revealed(u8)
}
impl Tile {
    fn get_char(&self, game_over: bool) -> char {
        if game_over {
            match self {
                Self::Mine => 'X',
                Self::Safe => '-',
                Self::FlaggedSafe => 'F',
                Self::FlaggedMine => '!',
                Self::Revealed(n) => n.to_string().chars().next().unwrap()
            }
        } else {
            match self {
                Self::Mine => '-',
                Self::Safe => '-',
                Self::FlaggedSafe => 'F',
                Self::FlaggedMine => 'F',
                Self::Revealed(n) => n.to_string().chars().next().unwrap()
            }
        }
    }
}

fn main() {
    let mut board: Board<15,13> = Board::new(40);
    while !board.is_solved() {
        println!("{}", board);
        println!();
        println!("Enter Command: ");
        println!("- Flag (x) (y)");
        println!("- Reveal (x) (y)");
        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            eprintln!("Failed to read input!");
            return;
        }
        let mut sp = input.split(" ");
        match sp.next() {
            Some("flag") | Some("f") => {
                if let Some(xs) = sp.next() {
                    if let Some(ys) = sp.next() {
                        if let Ok(x) = xs.trim().parse::<usize>() {
                            if let Ok(y) = ys.trim().parse::<usize>() {
                                board.flag(x-1usize, y-1usize);
                                continue;
                            }
                        }
                    }
                }
                println!("Malformed command.");
            }
            Some("reveal") | Some("r") => {
                if let Some(xs) = sp.next() {
                    if let Some(ys) = sp.next() {
                        if let Ok(x) = xs.trim().parse::<usize>() {
                            if let Ok(y) = ys.trim().parse::<usize>() {
                                board.reveal(x-1usize, y-1usize);
                                continue;
                            }
                        }
                    }
                }
                println!("Malformed command.");
            }
            Some(_) => {
                println!("Unrecognized command.");
            }
            _ => {
                println!("Please input a command.");
            }
        }
    }
}