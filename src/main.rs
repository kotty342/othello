const BOARD_SIZE:usize = 8;

#[derive(Debug, Clone, Copy)]
enum Player{
    WHITE,
    BLUCK,
}

#[derive(Debug, Clone, Copy)]
struct Piece{
    player:Player,
}

#[derive(Debug, Clone, Copy)]
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
}

fn main() {
    let Game = Game::new();
    Game.display()
    
}
