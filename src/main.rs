
extern crate ansi_term;

use ansi_term::Colour::{Red, White};
use std::{io, mem, vec, write};
    
#[derive(Copy, Clone)]
struct Piece{
    peice : char, // 
    colour : u8, //0 no specify 1 black 2 white
    moved : bool,
}

struct Board{
    tile :  [[Piece; 8]; 8],
}

impl Board {
    fn new() -> Board{
        let mut temp = [[Piece{peice: ' ', colour : 0, moved : false};8];8];
        
        temp[0][0].peice = 'r';
        temp[1][0].peice = 'n';
        temp[2][0].peice = 'b';
        temp[3][0].peice = 'q';
        temp[4][0].peice = 'k';
        temp[5][0].peice = 'b';
        temp[6][0].peice = 'n';
        temp[7][0].peice = 'r';
        temp[0][1].peice = 'p';
        temp[1][1].peice = 'p';
        temp[2][1].peice = 'p';
        temp[3][1].peice = 'p';
        temp[4][1].peice = 'p';
        temp[5][1].peice = 'p';
        temp[6][1].peice = 'p';
        temp[7][1].peice = 'p';
        temp[0][6].peice = 'p';
        temp[1][6].peice = 'p';
        temp[2][6].peice = 'p';
        temp[3][6].peice = 'p';
        temp[4][6].peice = 'p';
        temp[5][6].peice = 'p';
        temp[6][6].peice = 'p';
        temp[7][6].peice = 'p';
        temp[0][7].peice = 'r';
        temp[1][7].peice = 'n';
        temp[2][7].peice = 'b';
        temp[3][7].peice = 'q';
        temp[4][7].peice = 'k';
        temp[5][7].peice = 'b';
        temp[6][7].peice = 'n';
        temp[7][7].peice = 'r';
        
        for j in 0..2{
            for i in 0..8{
                temp[i][j].colour = 2;
                temp[i][7-j].colour = 1;
            }
        }
        
        
        let board = Board { tile: temp,};
        return board;
    }
    fn print(&self){
        println!(" print line");
        print!("    ");
        let mut a = 'a' as u8;
        
        for i in 0..8{
            print!("{}   ", a as char);
            a+=1;
        }
        
        println!("\n");
        
        for j in 0..8{
            print!(" {}  ", j+1);
        
            for i in 0..8{
               print!("{} | ", if self.tile[i][j].colour == 1 { Red.paint(self.tile[i][j].peice.to_string()) } else { White.paint(self.tile[i][j].peice.to_string()) });                    
            }
            print!("  {} \n", j+1 );
            print!("    --------------------------------");
            println!("")
        }
        
        print!("    ");
        a = 'a' as u8;
        
        for i in 0..8{
            print!("{}   ", a as char);
            a+=1;
        }
        
        println!("\n")
        
    }
}

fn main() {
    
    let running = true;
    let board = Board::new();
    board.print();
    
    while running {
        //output;
        
        println!("name: ");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read line");
        
        println!("{}", name);
        
        
    }
    
    
    println!("Hello, world!");
}
