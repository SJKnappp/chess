
extern crate ansi_term;

use ansi_term::Colour::{Red, White};
use std::{io};
 
struct Displace{
    peice : char, //stores the peice being moved and the susses of the return value
    x : u8, //stores coords
    y : u8, //stores coords
}
 
#[derive(Copy, Clone)]
struct Piece{
    peice : char, //stores the peice r rock n knight b bishop q queen k king p pawn
    colour : u8, //0 no specify 1 black 2 white
    moved : bool, //detects peices first move
}

struct Board{
    tile :  [[Piece; 8]; 8], //stores board state
    player : bool, //false black true whtie
    takenWhite : Vec<char>, //holds pieces taken by black
    takenBlack : Vec<char>, //hold pieces taken by white
}

impl Board {

    //creats a standards board layout
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
        
        
        let board = Board { tile: temp, player : false, takenWhite : Vec::new(), takenBlack : Vec::new()};
        return board;
    }
    //prints out the board
    fn print(&self){
    
        for i in 0..self.takenBlack.len(){
            print!("Black taken: {}", self.takenBlack[i]);
        }
        println!("");
        for i in 0..self.takenWhite.len(){
            print!("white taken: {}", self.takenWhite[i]);
        }
        
        println!(" ");

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
    //checks if playing on own colour
    fn checkColour(&self, Move : &Displace) -> bool{
        if self.player == true && self.tile[Move.x as usize][Move.y as usize].colour == 2 { return false; }
        else if self.player == false && self.tile[Move.x as usize][Move.y as usize].colour == 1 { return false; }
        return true;
    }
    //takes start and end point and kills and moves the peices
    fn Swap(mut self, Final : &Displace, intial : &Displace) -> Board{
        if self.tile[Final.x as usize][Final.y as usize].colour == 1{
                self.takenWhite.push( self.tile[Final.x as usize][Final.y as usize].peice );
        } else if self.tile[Final.x as usize][Final.y as usize].colour == 2{
                self.takenBlack.push( self.tile[Final.x as usize][Final.y as usize].peice );
        }
    
        self.tile[Final.x as usize][Final.y as usize].peice = self.tile[intial.x as usize][intial.y as usize].peice;
        self.tile[Final.x as usize][Final.y as usize].colour = self.tile[intial.x as usize][intial.y as usize].colour;
        self.tile[intial.x as usize][intial.y as usize].peice = ' ';
        self.tile[intial.x as usize][intial.y as usize].colour = 0 ;
        return self;
    }
}

fn checkallowed(board : &Board, endPos : &Displace) -> Displace{
    let mut startPos = Displace{peice : 'f', x : 8, y : 8};
    let player : u8;
    let direc : i8;
    if board.player == false {player = 1;} else { player = 2;}
    
    if endPos.peice == 'p' {
        if board.player == false{direc = 1;}else{direc = -1;} //direc looking backtowards start pos
        
        if board.tile[endPos.x as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].peice == ' ' && board.tile[endPos.x as usize][(endPos.y as i8 + direc)as usize].colour == player{
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 + direc) as u8;
        }else if board.tile[endPos.x as usize][(endPos.y as i8 + 2 * direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].peice == ' '  && 
        board.tile[endPos.x as usize][(endPos.y as i8 + 2 *     direc)as usize].colour == player{
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 + 2 * direc) as u8;
        }
        
    }
    else if endPos.peice == 'r' {}
    else if endPos.peice == 'n' {}
    else if endPos.peice == 'b' {}
    else if endPos.peice == 'q' {}
    else if endPos.peice == 'k' {}
    else {startPos.peice = 'f'}
    
    return startPos;
}

fn main() {
    
    let running = true;
    let mut board = Board::new();
    let mut moveAccepted = true;
    let mut turn = Displace {peice : ' ', x : 9, y : 9};
    let mut end = Displace {peice: ' ', x:1, y: 1};
    
    while running {
        if moveAccepted == true{
            if board.player == true{board.player = false;}else{board.player = true;}
            board.print();
            println!("player: {} please make your move \n", if board.player == true { "white " }else {" black"});
        }else{println!("please reenter your move"); moveAccepted = true;}
        let mut play = String::new();
        io::stdin().read_line(&mut play).unwrap();
        play = play.trim().to_string();
            
        if play.len() == 2{
            turn.peice = 'p';
            turn.x = play.as_bytes()[0] as u8; turn.x -= 97;
            turn.y = play.as_bytes()[1] as u8; turn.y -= 49;
            
            if  turn.x <= 7 && turn.y <= 7 {
                moveAccepted = board.checkColour(&turn)
            }else{moveAccepted = false;}
            
        }else if play.len() == 3{
            turn.peice = play.as_bytes()[0] as char;
            turn.x = play.as_bytes()[1] as u8; turn.x -= 97;
            turn.y = play.as_bytes()[2] as u8; turn.y -= 49;
            
            if turn.x <= 7 && turn.y <= 7 && (turn.peice == 'r' || turn.peice == 'n' || turn.peice == 'b' || turn.peice == 'q' || turn.peice == 'k' ){
                moveAccepted = board.checkColour(&turn)
            }else{moveAccepted = false;}
        }else if play == "exit"{ break; }
        else{moveAccepted = false;}
        
        if moveAccepted == true {
            end = checkallowed(&board, &turn);
            if end.peice == 'f' {moveAccepted = false; println!("move not allowed");}
        }
        
        if moveAccepted == true {
            board = board.Swap(&turn, &end);
            
        }
              
        
    }

    println!("Hello, world!");
}
