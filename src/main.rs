use std::io::Write;

use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Game {
    board: Board,
    player: Player,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Board {
    slots: [[Slot; 6]; 7],
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Slot(Option<Player>);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Player {
    Red,
    Yellow,
}

impl Player {
    fn get_char(self) -> char {
        match self {
            Player::Red => '🔴',
            Player::Yellow => '🟡',
        }
    }
}

impl Slot {
    fn get_char(self) -> char {
        match self.0 {
            Some(player) => player.get_char(),
            None => '⚪',
        }

        // self.0.map_or('⚪', Player::get_char)
    }
}

impl Board {
    fn print(self) {
        println!("╔═1═2═3═4═5═6═7═╗");
        for row in 0..6 {
            print!("║");
            for col in 0..7 {
                print!("{}", self.slots[col][row].get_char());
            }
            println!(" ║");
        }
        println!("╚═══════════════╝");
    }
}

impl Game {
    fn new() -> Self {
        Self {
            board: Board::default(),
            player: Player::Red,
        }
    }

    fn print(self) {
        self.board.print();
        println!();
    }

    fn get_input(self) -> i32 {
        let mut input = String::new();
        loop {
            print!("{} am Zug: ", self.player.get_char());
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();

            if let Ok(col) = input.parse() {
                break col;
            }

            match rand::thread_rng().gen_range(0..=9) {
                0..=8 => println!("Bitte eine Spalte zwischen 1-7 eingeben!"),
                9..=9 => println!("Sei doch keine HS"),
                _ => panic!(),
            }

            input.clear();
        }
    }
}

fn main() {
    loop {
        let game = Game::new();

        game.print();
        let col = game.get_input();

        // TODO: Process input
    }
}
