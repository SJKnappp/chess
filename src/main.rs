//use std::io;
use std::{io, mem};


struct Piece{
    peice : char
}

impl Default for Piece {
    #[inline]
    fn default() -> Piece {
        Piece{
            peice: ' ',
        }
    }
}

mut struct Board{
    tile :  Piece, //[[Piece; 8];8],
    count : u32,
}

impl Board {
    fn initate(mut self) -> Board{
        self.tile = Piece {peice: ' '};
        self.count = 0;
        return self;
    }
}

fn output(board: Board) {
    for i in 0..8 {
        for j in 0..8 {
            //println!("{:?}", board.board[i][j).peice;
            println!("board print");
        }
    }
}

unsafe fn initials() -> Board{

       // let board: Board = mem::uninitialized();
    
    
    let mut board : Board;
    board.tile.peice = ' ';
    //println!("{:?}", board.tile.peice);
    for i in 0..8 {
        for j in 0..8 {
           // board[i].peice = ' ';
        }
    }
    
    return board;
    
}

fn main() {
    
    let running = true;
    let board: Board;    
    unsafe{
    board = initials();
    }
   // output(board);
    
    while running {
        //output;
        
        println!("name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        
        println!("{}", name);
        
        
    }
    
    
    println!("Hello, world!");
}
