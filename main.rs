use colored::ColoredString;
use rand; // 0.8.5
use rand::Rng;
use std::fmt;
use std::io;
use colored::Colorize;

struct Board<const WIDTH: usize, const HEIGHT: usize> {
    board_data: [[Tile; HEIGHT]; WIDTH],
    game_over: bool,
    first_reveal: bool
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
        Board {board_data, game_over: false, first_reveal: true}
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
        if self.first_reveal {
            self.board_data[x][y] = Tile::Safe;
            for (xs, ys) in self.get_adjacent(x, y) {
                self.board_data[xs][ys] = Tile::Safe;
            }
            self.first_reveal = false;
        }
        match self.board_data[x][y] {
            Tile::Mine => true,
            Tile::Safe => {
                let mut to_reveal = vec![(x, y)];
                while let Some((x, y)) = to_reveal.pop() {
                    let mut s = 0;
                    for (xs, ys) in self.get_adjacent(x, y) {
                        if let Tile::Mine | Tile::FlaggedMine = self.board_data[xs][ys] {
                            s += 1;
                        }
                    }
                    self.board_data[x][y] = Tile::Revealed(s);
                    if s == 0 {
                        for l @ (xs, ys) in self.get_adjacent(x, y) {
                            if let Tile::Mine | Tile::Safe = self.board_data[xs][ys] {
                                to_reveal.push(l);
                            }
                        }
                    }
                }
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
            write!(f, "{:<2} ", (x+1).to_string().bright_black())?;
        }
        writeln!(f, "")?;
        write!(f, "   {}", "+".bright_black())?;
        for _ in 0..W {
            write!(f, "{}", "---".bright_black())?;
        }
        writeln!(f, "")?;
        for y in 0..H {
            write!(f, "{:>2} {} ", (y+1).to_string().bright_black(), "|".bright_black())?;
            for x in 0..W {
                let tile = self.board_data[x][y];
                write!(f, "{}  ", tile.get_char(self.game_over))?;
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
    fn get_char(&self, game_over: bool) -> ColoredString {
        if game_over {
            match self {
                Self::Mine => "X".red(),
                Self::Safe => "-".bright_white(),
                Self::FlaggedSafe => "F".yellow(),
                Self::FlaggedMine => "!".yellow(),
                Self::Revealed(1) => "1".blue(),
                Self::Revealed(2) => "2".green(),
                Self::Revealed(3) => "3".bright_red(),
                Self::Revealed(4) => "4".purple(),
                Self::Revealed(5) => "5".red(),
                Self::Revealed(6) => "6".cyan(),
                Self::Revealed(7) => "7".white(),
                Self::Revealed(8) => "8".bright_black(),
                _ => " ".white(),
            }
        } else {
            match self {
                Self::Mine => "-".white(),
                Self::Safe => "-".white(),
                Self::FlaggedSafe => "F".yellow(),
                Self::FlaggedMine => "F".yellow(),
                Self::Revealed(1) => "1".blue(),
                Self::Revealed(2) => "2".green(),
                Self::Revealed(3) => "3".bright_red(),
                Self::Revealed(4) => "4".purple(),
                Self::Revealed(5) => "5".red(),
                Self::Revealed(6) => "6".cyan(),
                Self::Revealed(7) => "7".white(),
                Self::Revealed(8) => "8".bright_black(),
                _ => " ".white(),
            }
        }
    }
}
enum EndState {
    WIN, LOSS, QUIT
}
fn main() {
    const WIDTH: usize = 8;
    const HEIGHT: usize = 8;
    let mut board: Board<WIDTH,HEIGHT> = Board::new(10);
    let state = loop {
        clear_screen();
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
        input.push(' ');
        let mut sp = input.split(" ");
        match sp.next() {
            Some("flag") | Some("f") => {
                if let Some(xs) = sp.next() {
                    if let Some(ys) = sp.next() {
                        if let Ok(x) = xs.trim().parse::<usize>() {
                            if let Ok(y) = ys.trim().parse::<usize>() {
                                if x > WIDTH || y > HEIGHT {
                                    println!("Location not on board.");
                                    continue;
                                }
                                board.flag(x-1usize, y-1usize);
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
                                if x > WIDTH || y > HEIGHT {
                                    println!("Location not on board.");
                                    continue;
                                }
                                let r = board.reveal(x-1usize, y-1usize);
                                if r {
                                    break EndState::LOSS;
                                }
                            }
                        }
                    }
                }
                println!("Malformed command.");
            }
            Some("quit") => {
                break EndState::QUIT;
            }
            Some(_) => {
                println!("Unrecognized command.");
            }
            _ => {
                println!("Please input a command.");
            }
        }
        if board.is_solved() {
            break EndState::WIN;
        }
    };
    board.game_over = true;
    match state {
        EndState::WIN => {
            clear_screen();
            println!("{}", board);
            println!("YOU WIN!");
        },
        EndState::LOSS => {
            clear_screen();
            println!("{}", board);
            println!("YOU LOSE");
        },
        EndState::QUIT => {
            clear_screen();
            println!("Quitting!");
        },
    }

}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}