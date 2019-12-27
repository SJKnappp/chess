
extern crate ansi_term;

use ansi_term::Colour::{Red, White};
use std::{io};
 
#[derive(Copy, Clone)]
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
    History : Vec<Displace>, //move histroy
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
        
        
        let board = Board { tile: temp, player : false, takenWhite : Vec::new(), takenBlack : Vec::new(), History : Vec::new(), };
        return board;
    }
    //prints out the board
    fn print(&self){
        
        print!("Black taken: ");
        for i in 0..self.takenBlack.len(){
            print!("{}", self.takenBlack[i]);
        }
        print!("\nWhite taken: ");
        for i in 0..self.takenWhite.len(){
            print!("{}", self.takenWhite[i]);
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
        } else if intial.peice == 'S' {//self.tile[Final.x as usize][Final.y as usize].colour == 0 {
            if self.tile[intial.x as usize][intial.y as usize].colour == 2{
                self.takenWhite.push( self.tile[Final.x as usize][(Final.y as i8 - 1) as usize].peice );
                self.tile[Final.x as usize][(Final.y as i8 -1) as usize].peice = ' ';
                self.tile[Final.x as usize][(Final.y as i8 -1) as usize].colour = 0 ;
            }else if  self.tile[intial.x as usize][intial.y as usize].colour == 1 {
                self.takenBlack.push( self.tile[Final.x as usize][(Final.y as i8 + 1) as usize].peice );
                self.tile[Final.x as usize][(Final.y as i8 +1) as usize].peice = ' ';
                self.tile[Final.x as usize][(Final.y as i8 +1) as usize].colour = 0 ;
            }
        }
        
        if Final.y == 7 && Final.peice =='p'{
            println!("pawn premotted please select replacement");
            let mut sucsses = true;
            while(sucsses == true) {
            let mut select = String::new();
            io::stdin().read_line(&mut select).unwrap();
            select = select.trim().to_string();
            if select.len() == 1{ 
                match select.as_ref(){
                    "n" => self.tile[intial.x as usize][intial.y as usize].peice = 'n',
                    "r" => self.tile[intial.x as usize][intial.y as usize].peice = 'r',
                    "b" => self.tile[intial.x as usize][intial.y as usize].peice = 'b',
                    "q" => self.tile[intial.x as usize][intial.y as usize].peice = 'q',
                    _ => println!("please select ether q r b q"),
                }
            }
            if self.tile[intial.x as usize][intial.y as usize].peice != 'p' { break; }
            }
        }
    
        self.tile[Final.x as usize][Final.y as usize].peice = self.tile[intial.x as usize][intial.y as usize].peice;
        self.tile[Final.x as usize][Final.y as usize].colour = self.tile[intial.x as usize][intial.y as usize].colour;
        self.tile[intial.x as usize][intial.y as usize].peice = ' ';
        self.tile[intial.x as usize][intial.y as usize].colour = 0 ;
        
        return self;
    }
}

fn checkQMB(board : &Board, mut startPos : Displace, endPos : &Displace, mut player : u8, mut peice : char, mut checkU : bool, mut checkD : bool, mut checkL : bool, mut checkR : bool, mut checkNE : bool, mut checkSE : bool, mut checkSW : bool, mut checkNW : bool) -> Displace {
    for i in 0..8{
            if checkU == true && endPos.y + i < 8 {
                if board.tile[endPos.x as usize][(endPos.y+i) as usize].peice == ' ' {}
                else if board.tile[endPos.x as usize][(endPos.y+i )as usize].peice == peice && board.tile[endPos.x as usize][(endPos.y+i) as usize].colour == player {
                     startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y + i) as u8;
                }else {checkU = false}
            }else{checkU = false}//Up
            
            if checkD == true && endPos.y as i8 - i as i8>= 0 {
                if board.tile[endPos.x as usize][(endPos.y-i) as usize].peice == ' ' {}
                else if board.tile[endPos.x as usize][(endPos.y-i) as usize].peice == peice && board.tile[endPos.x as usize][(endPos.y-i) as usize].colour == player {
                     startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y - i) as u8;
                }else {checkD = false}
            }else{checkD = false}//Down
            
            if checkR == true && endPos.x + i < 8 {
                if board.tile[(endPos.x + i) as usize][endPos.y as usize].peice == ' ' {}
                else if board.tile[(endPos.x+i) as usize][endPos.y as usize].peice == peice && board.tile[(endPos.x - i) as usize][endPos.y as usize].colour == player {
                     startPos.peice = 's'; startPos.x = (endPos.x + i) as u8; startPos.y = endPos.y;
                }else {checkR = false}
            }else{checkR = false}//Right
            
            if checkL == true && endPos.x as i8 - i as i8 >= 0 {
                if board.tile[(endPos.x - i) as usize][endPos.y as usize].peice == ' ' {}
                else if board.tile[(endPos.x-i) as usize][endPos.y as usize].peice == peice && board.tile[(endPos.x - i) as usize][endPos.y as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x - i) as u8; startPos.y = endPos.y;
                }else {checkL = false}
            }else{checkL = false}//Left
            
            if checkNE == true && endPos.x as i8 + i as i8 >= 0 && endPos.y as i8 + i as i8 >= 0 {
                if board.tile[(endPos.x - i) as usize][endPos.y as usize].peice == ' ' {}
                else if board.tile[(endPos.x-i) as usize][endPos.y as usize].peice == peice && board.tile[(endPos.x - i) as usize][endPos.y as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x - i) as u8; startPos.y = endPos.y;
                }else {checkL = false}
            }else{checkNE = false}//NE
            
            if checkSE == true && endPos.x as i8 + i as i8 >= 0 && endPos.y as i8 - i as i8 >= 0 {
                if board.tile[(endPos.x - i) as usize][endPos.y as usize].peice == ' ' {}
                else if board.tile[(endPos.x-i) as usize][endPos.y as usize].peice == peice && board.tile[(endPos.x - i) as usize][endPos.y as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x - i) as u8; startPos.y = endPos.y;
                }else {checkL = false}
            }else{checkSE = false}//SE
            
            if checkSW == true && endPos.x as i8 - i as i8 >= 0 && endPos.y as i8 - i as i8 >= 0 {
                if board.tile[(endPos.x - i) as usize][endPos.y as usize].peice == ' ' {}
                else if board.tile[(endPos.x-i) as usize][endPos.y as usize].peice == peice && board.tile[(endPos.x - i) as usize][endPos.y as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x - i) as u8; startPos.y = endPos.y;
                }else {checkL = false}
            }else{checkSW = false}//SW
            
            if checkNW == true && endPos.x as i8 - i as i8 >= 0 && endPos.y as i8 + i as i8 >= 0{
                if board.tile[(endPos.x - i) as usize][endPos.y as usize].peice == ' ' {}
                else if board.tile[(endPos.x-i) as usize][endPos.y as usize].peice == peice && board.tile[(endPos.x - i) as usize][endPos.y as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x - i) as u8; startPos.y = endPos.y;
                }else {checkL = false}
            }else{checkNW = false}//NW
        }
    return startPos;
}

fn checkallowed(board : &Board, endPos : &Displace) -> Displace{
    
    let mut startPos = Displace{peice : 'f', x : 8, y : 8};
    let player : u8;
    let openent : u8;
    let direc : i8;
    if board.player == false {player = 1; openent = 2;} else { player = 2; openent = 1;}
  
    if endPos.peice == 'p' {
        if board.player == false{direc = 1;}else{direc = -1;} //direc looking backtowards start pos
        print!("{}", direc);
        
        if board.tile[endPos.x as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].peice == ' ' && board.tile[endPos.x as usize][(endPos.y as i8 + direc)as usize].colour == player{
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 + direc) as u8;
        } //move one foward
        
        else if board.tile[endPos.x as usize][(endPos.y as i8 + 2 * direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].peice == ' '  && 
        board.tile[endPos.x as usize][(endPos.y as i8 + direc) as usize].peice == ' ' && board.tile[endPos.x as usize][(endPos.y as i8 + 2 *  direc)as usize].colour == player
        && ((endPos.y == 3 && player == 2) || (endPos.y == 4 && player == 1)){
        
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 + 2 *  direc) as u8;
        
        } //move two foward
        
        else if ( if endPos.x < 7 { board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].colour == openent && board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].colour == player} else {false == true}){
            
            startPos.peice = 's'; startPos.x = endPos.x + 1; startPos.y = (endPos.y as i8 + direc) as u8;
        
        } //takes left
        
        else if ( if endPos.x > 0 { board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].colour == openent && board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].colour == player} else {false == true}){
            
            startPos.peice = 's'; startPos.x = endPos.x - 1; startPos.y = (endPos.y as i8 + direc) as u8;
        
        } //take right
        
        else if ( if endPos.x < 7 { board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].peice == ' ' && board.History[board.History.len() - 1].peice == 'p'
        && board.History[board.History.len() - 1].x == endPos.x && board.History[board.History.len() - 1].y == (endPos.y as i8 + direc) as u8  && board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].colour == player} 
        else {false == true}){
            
            startPos.peice = 'S'; startPos.x = endPos.x + 1; startPos.y = (endPos.y as i8 + direc) as u8;
        
        } //en possion left
        
        else if ( if endPos.x > 0 { board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' &&
        (board.tile[endPos.x as usize][endPos.y as usize].peice == ' ' && board.History[board.History.len() - 1].peice == 'p' && board.History[board.History.len() - 1].x == endPos.x && board.History[board.History.len() - 1].y ==( endPos.y as i8 + direc ) as u8 ) && board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].colour == player} else {false == true}){
            
            startPos.peice = 'S'; startPos.x = endPos.x - 1; startPos.y = (endPos.y as i8 + direc) as u8;
        
        } //en possion right
    }//pawn allowed 
    
    else if endPos.peice == 'r' {
    let mut checkU = true; let mut checkD = true; let mut checkL = true; let mut checkR = true;
        startPos = checkQMB(&board, startPos, &endPos, player, 'r', true, true, true, true, false, false, false, false);
    }//rook allowed
    
    else if endPos.peice == 'n' {
      if if endPos.x + 1 < 8 && endPos.y + 2 < 8 { board.tile[(endPos.x as i8 + 1) as usize][(endPos.y as i8 + 2)as usize].peice == 'n'} else {true == false} {
        startPos.peice = 'n'; startPos.x = (endPos.x as i8 + 1) as u8; startPos.y = (endPos.y as i8 + 2) as u8;
      }
      else if if endPos.x as i8 - 1 >= 0 && endPos.y + 2 < 8 { board.tile[(endPos.x as i8 - 1) as usize][(endPos.y as i8 + 2)as usize].peice == 'n'} else {true == false} {
        startPos.peice = 'n'; startPos.x = (endPos.x as i8 - 1) as u8; startPos.y = (endPos.y as i8 + 2) as u8;
      }
      else if if endPos.x + 1 < 8 && endPos.y as i8 - 2 >= 0 { board.tile[(endPos.x as i8 + 1) as usize][(endPos.y as i8 - 2)as usize].peice == 'n'} else {true == false} {
        startPos.peice = 'n'; startPos.x = (endPos.x as i8 + 1) as u8; startPos.y = (endPos.y as i8 - 2) as u8;
      }
      else if if endPos.x as i8 - 1 >= 0 && endPos.y as i8 - 2 >= 0 { board.tile[(endPos.x as i8 - 1) as usize][(endPos.y as i8 - 2)as usize].peice == 'n'} else {true == false} {
        startPos.peice = 'n'; startPos.x = (endPos.x as i8 - 1) as u8; startPos.y = (endPos.y as i8 - 2) as u8 ;
      }
      else if if endPos.x + 2 < 8 && endPos.y + 1 < 8 { board.tile[(endPos.x as i8 + 2) as usize][(endPos.y as i8 + 1)as usize].peice == 'n'} else {true == false} {
        startPos.peice = 'n'; startPos.x = (endPos.x as i8 + 2) as u8; startPos.y = (endPos.y as i8 + 1) as u8;
      }
      else if if endPos.x + 2 < 8 && endPos.y as i8 - 1 >= 0 { board.tile[(endPos.x as i8 + 2) as usize][(endPos.y as i8 - 1)as usize].peice == 'n'} else {true == false} {
        startPos.peice = 'n'; startPos.x = (endPos.x as i8 + 2) as u8; startPos.y = (endPos.y as i8 - 1) as u8;
      }
      else if if endPos.x as i8 - 2 >= 0 && endPos.y + 1 < 8 { board.tile[(endPos.x as i8 - 2) as usize][(endPos.y as i8 + 1)as usize].peice == 'n'} else {true == false} {
        startPos.peice = 'n'; startPos.x = (endPos.x as i8 - 2) as u8; startPos.y = (endPos.y as i8 + 1) as u8;
      }
      else if if endPos.x as i8 - 2 >= 0 && endPos.y as i8 - 1 >= 0 { board.tile[(endPos.x as i8 - 2) as usize][(endPos.y as i8 - 1)as usize].peice == 'n'} else { true == false} {
        startPos.peice = 'n'; startPos.x = (endPos.x as i8 - 2) as u8; startPos.y = (endPos.y as i8 - 1) as u8;
      }
    }//knight allowed
   
    else if endPos.peice == 'b' {
        startPos = checkQMB(&board, startPos, &endPos, player, 'b', false, false, false, false, true, true, true, true);
    }
    else if endPos.peice == 'q' {
        startPos = checkQMB(&board, startPos, &endPos, player, 'q', true, true, true, true, true, true, true, true);
    }
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
            board.History.push(turn);
            
        }
              
        
    }

    println!("Hello, world!");
}
