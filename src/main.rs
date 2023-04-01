use std::io;

const BOARD_SIZE:usize = 8;

#[derive(PartialEq,Debug, Clone, Copy)]
enum Cell{
    EMPTY,
    BLUCK,
    WHITE,
    OUTSIDE,
}

struct Board{
    cells:[[Cell;BOARD_SIZE+2];BOARD_SIZE+2],
}

struct Game{
    board : Board,
    current_player: Cell
}

impl Game {
    fn new() -> Self{
        let mut tmp_board = [[Cell::EMPTY;BOARD_SIZE+2];BOARD_SIZE+2];
        tmp_board[5][4] = Cell::BLUCK;
        tmp_board[4][4] = Cell::WHITE;
        tmp_board[4][5] = Cell::BLUCK;
        tmp_board[5][5] = Cell::WHITE;
        for i in 0..BOARD_SIZE+2{
            tmp_board[i][0] = Cell::OUTSIDE;
            tmp_board[i][BOARD_SIZE+1] = Cell::OUTSIDE;
            tmp_board[0][i] = Cell::OUTSIDE;
            tmp_board[BOARD_SIZE+1][i] = Cell::OUTSIDE;
        }

        Game { 
            board : Board{cells : tmp_board},
            current_player: Cell::BLUCK
        }
    }

    fn display(&self){
        println!("   0 1 2 3 4 5 6 7");
        println!(" +-----------------+");
        for y in 1..BOARD_SIZE+1{
            print!("{}| ",y-1);
            for x in 1..BOARD_SIZE+1{
                match self.board.cells[x][y]{
                    Cell::EMPTY => print!("/ "),
                    Cell::BLUCK => print!("B "),
                    Cell::WHITE => print!("W "),
                    Cell::OUTSIDE =>continue
                };
            }
            println!("|")
        }
        println!(" +-----------------+");
    }
    
    fn chack(&mut self, p :usize, q :usize, d:isize,e:isize)-> usize{
        let mut i = 1;

        while self.board.cells[(p as isize +i*d )as usize][(q as isize + i*e) as usize] as usize == (3-self.current_player as usize){
            i+=1;
        }

        if &self.board.cells[(p as isize +i*d )as usize][(q as isize + i*e) as usize] == &self.current_player{
                return i as usize - 1
        }else {
            return 0;
        }
    }

    fn suitability(&mut self, x :usize, y :usize)-> bool{
        if self.board.cells[x][y] == Cell::EMPTY{
            for d in -1..=1{
                for e in -1..=1{
                    if self.chack(x, y, d, e) > 0{
                        return true
                    }else {
                        continue
                    }
                }
            }
            return false
        }
        else {
            return false;
        }
    }

    fn put(&mut self, x :usize, y :usize){
        if self.suitability(x, y){
            for d in -1..=1{
                for e in -1..=1{
                    let count = self.chack(x, y, d, e);
                    let mut i:isize = 1;
                    while i  <= count as isize{
                        self.board.cells[(x as isize + i*d)as usize][(y as isize + i*e)as usize] = self.current_player;
                        i+=1;
                    }
                }
            }
            self.board.cells[x][y] = self.current_player;
            self.change_player()
        }
        else{
            println!{"illegal!"};
        }
    }

    fn change_player(&mut self){
        if self.current_player == Cell::WHITE{
            self.current_player = Cell::BLUCK
        }
        else{
            self.current_player = Cell::WHITE
        }
    }

    fn main_loop(&mut self){
        loop{
            self.display();
            match self.current_player{
                Cell::BLUCK => println!("BLUCK have a turn"),
                Cell::WHITE => println!("WHITE have a turn"),
                _ => {println!("Err");break}
            };
            println!("where do you put?");
            println!("Vertical:");
            let vertical = std_imput();
            println!("horizontal:");
            let horizontal = std_imput();
            self.put(vertical+1,horizontal+1);
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
    new_game.main_loop();
}