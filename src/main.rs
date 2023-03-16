use std::io;

const BOARD_SIZE:usize = 8;

#[derive(PartialEq,Debug, Clone, Copy)]
enum Player{
    WHITE,
    BLUCK,
}

#[derive(PartialEq,Debug, Clone, Copy)]
struct Piece{
    player:Player,
}

#[derive(PartialEq,Debug, Clone, Copy)]
enum Cell{
    Empty,
    Piece(Piece),
}

struct Board{
    cells:[[Cell;BOARD_SIZE];BOARD_SIZE],
}

struct Game{
    board : Board,
    current_player: Player
}

impl Game {
    fn new() -> Self{
        let mut tmp_board = [[Cell::Empty;BOARD_SIZE];BOARD_SIZE];
        tmp_board[4][3] = Cell::Piece(Piece{player:Player::BLUCK});
        tmp_board[3][3] = Cell::Piece(Piece{player:Player::WHITE});
        tmp_board[3][4] = Cell::Piece(Piece{player:Player::BLUCK});
        tmp_board[4][4] = Cell::Piece(Piece{player:Player::WHITE});

        Game { 
            board : Board{cells : tmp_board},
            current_player: Player::BLUCK
        }
    }

    fn display(&self){
        println!("   0 1 2 3 4 5 6 7");
        println!(" +----------------+");
        for y in 0..BOARD_SIZE{
            print!("{}|",y);
            for x in 0..BOARD_SIZE{
                let cell = match self.board.cells[x][y]{
                    Cell::Empty => "/",
                    Cell::Piece(Piece{player:Player::BLUCK}) => "B",
                    Cell::Piece(Piece{player:Player::WHITE}) => "W",
                };
                print!(" {}",cell);
            }
            println!("|")
        }
        println!(" +----------------+");
    }

    fn put(&mut self, x :usize, y :usize){
        if self.board.cells[x][y] == Cell::Empty{
            self.board.cells[x][y] = Cell::Piece(Piece{player:self.current_player});
        }
        else{
            println!{"Occupied"};
        }
    }

    fn change_player(&mut self){
        if self.current_player == Player::WHITE{
            self.current_player = Player::BLUCK
        }
        else{
            self.current_player = Player::WHITE
        }
    }

    fn main_loop(&mut self){
        loop{
            self.display();
            match self.current_player{
                Player::BLUCK => println!("BLUCK have a tuen"),
                Player::WHITE => println!("WHITE have a tuen"),
            };
            println!("where do you put?");
            println!("Vertical:");
            let vertical = std_imput();
            println!("horizontal:");
            let horizontal = std_imput();
            self.put(vertical,horizontal);
            self.change_player()
        }
    }
}

fn std_imput()-> usize {
    loop{
    let mut imput = String::new();
    io::stdin()
    .read_line(&mut imput)
    .expect("Failed to read line");
    let imput: usize = match imput.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
        };
        return imput
    }
}

fn main() {
    let mut new_game=Game::new();
    new_game.main_loop()    
}
