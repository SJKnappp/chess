#![allow(non_snake_case)]

extern crate ansi_term;

use std::fs::File;
use std::io::prelude::*;
use ansi_term::Colour::{Red, White};
use std::{io};

mod minmaxAi;

#[derive(Copy, Clone)]
struct Check{
    white : bool,
    black : bool,
}

#[derive(Copy, Clone)]
struct Sphere{
    whiteSphere : i8, //white can move here next turn
    blackSphere : i8, //black can move here next turn
}

#[derive(Copy, Clone)]
struct Displace{
    peice : char, //stores the peice being moved and the susses of the return value
    x : u8, //stores coords
    y : u8, //stores coords
    ambigX : u8, //used to resolve abisuas moves
    ambigY : u8,
    moveStr : [char; 5],
}

impl Displace{

}

#[derive(Copy, Clone)]
struct Piece{
    peice : char, //stores the peice r rock n knight b bishop q queen k king p pawn
    colour : u8, //0 no specify 1 black 2 white
    moved : bool, //detects peices first move
}

#[derive(Copy, Clone)]
struct Player{
    WhiteAi : bool,
    BlackAi : bool,
}

#[derive(Clone)]
struct Board{
    tile :  [[Piece; 8]; 8], //stores board state
    player : bool, //false black true whtie
    takenWhite : Vec<char>, //holds pieces taken by black
    takenBlack : Vec<char>, //hold pieces taken by white
    History : Vec<Displace>, //move histroy
    playerAi : Player,
}

impl Board {

    //creats a standards board layout
    fn new() -> Board{

        //creates an array of the board
        let mut temp = [[Piece{peice: ' ', colour : 0, moved : false};8];8];

        temp[0][0].peice = 'R';
        temp[1][0].peice = 'N';
        temp[2][0].peice = 'B';
        temp[3][0].peice = 'Q';
        temp[4][0].peice = 'K';
        temp[5][0].peice = 'B';
        temp[6][0].peice = 'N';
        temp[7][0].peice = 'R';
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
        temp[0][7].peice = 'R';
        temp[1][7].peice = 'N';
        temp[2][7].peice = 'B';
        temp[3][7].peice = 'Q';
        temp[4][7].peice = 'K';
        temp[5][7].peice = 'B';
        temp[6][7].peice = 'N';
        temp[7][7].peice = 'R';

        //loops and set colour
        for j in 0..2{
            for i in 0..8{
                temp[i][j].colour = 2;
                temp[i][7-j].colour = 1;
            }
        }
        let mut accepted = 0;
        let mut players = Player{WhiteAi : false, BlackAi : false};

        while accepted < 2{
            accepted = 0;
            println!("Ai white enter 'y' human enter 'n'");
            let mut white = String::new();
            io::stdin().read_line(&mut white).unwrap(); //input move
            white = white.trim().to_string();
            if white == "y"{
                players.WhiteAi = true;
                accepted += 1;
            }else if white == "n"{
                players.WhiteAi = false;
                accepted += 1;
            }

            println!("Ai black enter 'y' human enter 'n'");
            let mut black = String::new();
            io::stdin().read_line(&mut black).unwrap(); //input move
            black = black.trim().to_string();
            if black == "y"{
                players.BlackAi = true;
                accepted += 1;
            }else if white == "n"{
                players.BlackAi = false;
                accepted += 1;
            }

            if accepted < 2 {
                print!("input failed please retry");
            }
        }
        //creats the board
        let board = Board { tile: temp, player : false, takenWhite : Vec::new(), takenBlack : Vec::new(), History : Vec::new(), playerAi : players};
        return board;
    }
    //prints out the board
    fn print(&self){

        print!("Black taken: ");
        for i in 0..self.takenBlack.len(){
            print!("{}", self.takenBlack[i]); //list the peices black has taken
        }
        print!("\nWhite taken: ");
        for i in 0..self.takenWhite.len(){
            print!("{}", self.takenWhite[i]); //lists the peices white has taken
        }

        print!("\n    "); //sets intial displacment of the coords print out

        let mut a = 'a' as u8; //stores interger value of a to loop through
        for _i in 0..8{
            print!("{}   ", a as char); // prints the letter
            a+=1;
        }

        println!("\n");

        for j in 0..8{
            print!(" {}  ", j+1);

            for i in 0..8{
               print!("{} | ", if self.tile[i][j].colour == 1 { Red.paint(self.tile[i][j].peice.to_string()) } else { White.paint(self.tile[i][j].peice.to_string()) }); //prints out board state
            }
            print!("  {} \n", j+1 );

            print!("    --------------------------------"); //seperates the board with a line
            println!("")
        }

        print!("    ");
        a = 'a' as u8;

        for _i in 0..8{
            print!("{}   ", a as char); // prints bottom row coordinates
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

        if self.tile[Final.x as usize][Final.y as usize].colour == 1{ //adds taken peices to taken history
                self.takenWhite.push( self.tile[Final.x as usize][Final.y as usize].peice );
        } else if self.tile[Final.x as usize][Final.y as usize].colour == 2{
                self.takenBlack.push( self.tile[Final.x as usize][Final.y as usize].peice );
        } else if intial.peice == 'S' {//detects en passont to take peice thats not being landed on
            if self.tile[intial.x as usize][intial.y as usize].colour == 2{
                self.takenWhite.push( self.tile[Final.x as usize][(Final.y as i8 - 1) as usize].peice );
                self.tile[Final.x as usize][(Final.y as i8 -1) as usize].peice = ' '; //removes peice
                self.tile[Final.x as usize][(Final.y as i8 -1) as usize].colour = 0 ;
            }else if  self.tile[intial.x as usize][intial.y as usize].colour == 1 {
                self.takenBlack.push( self.tile[Final.x as usize][(Final.y as i8 + 1) as usize].peice );
                self.tile[Final.x as usize][(Final.y as i8 +1) as usize].peice = ' ';
                self.tile[Final.x as usize][(Final.y as i8 +1) as usize].colour = 0 ;
            }
        }

        if (Final.y == 7 || Final.y == 0 )&& Final.peice =='p'{ //premottes pawn
            println!("pawn premotted please select replacement");
            let mut sucsses = true;
            while sucsses == true { //waits till pawn prommotion sucseful before moving on
            let mut select = String::new();
            io::stdin().read_line(&mut select).unwrap(); //takes an input
            select = select.trim().to_string();
            if select.len() == 1{
                match select.as_ref(){ //swaps peice
                    "n" => self.tile[intial.x as usize][intial.y as usize].peice = 'N',
                    "r" => self.tile[intial.x as usize][intial.y as usize].peice = 'R',
                    "b" => self.tile[intial.x as usize][intial.y as usize].peice = 'B',
                    "q" => self.tile[intial.x as usize][intial.y as usize].peice = 'Q',
                    _ => println!("please select ether q r b q"),
                }
            }
            if self.tile[intial.x as usize][intial.y as usize].peice != 'p' { break; } //if peices changed break out of loop
            }
        }

        self.tile[Final.x as usize][Final.y as usize].peice = self.tile[intial.x as usize][intial.y as usize].peice; //moves peice
        self.tile[Final.x as usize][Final.y as usize].colour = self.tile[intial.x as usize][intial.y as usize].colour;
        self.tile[intial.x as usize][intial.y as usize].peice = ' '; //resets original point
        self.tile[intial.x as usize][intial.y as usize].colour = 0 ;

        return self;
    }
}

//checks allowed move for queen bishop and rook
fn checkQMB(board : &Board, mut startPos : Displace, endPos : &Displace, mut player : u8, mut peice : char, mut checkU : bool, mut checkD : bool, mut checkL : bool, mut checkR : bool, mut checkNE : bool, mut checkSE : bool, mut checkSW : bool, mut checkNW : bool) -> Displace {
    for i in 0..8{
            if checkU == true && endPos.y + i < 8 {
                if board.tile[endPos.x as usize][(endPos.y+i) as usize].peice == ' ' {} //pass on if empty tile
                else if board.tile[endPos.x as usize][(endPos.y+i )as usize].peice == peice && board.tile[endPos.x as usize][(endPos.y+i) as usize].colour == player { //checks that direction equual to peice and colour
                     startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y + i) as u8; //intial location of peice
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

            if checkNE == true && endPos.x + i < 8 && endPos.y + i < 8 {
                if board.tile[(endPos.x + i) as usize][(endPos.y + i) as usize].peice == ' ' {}
                else if board.tile[(endPos.x + i) as usize][(endPos.y + i) as usize].peice == peice && board.tile[(endPos.x + i) as usize][(endPos.y + i ) as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x + i) as u8; startPos.y = (endPos.y + i ) as u8;
                }else {checkNE = false}
            }else{checkNE = false}//NE

            if checkSE == true && endPos.x + i < 8 && endPos.y as i8 - i as i8 >= 0 {
                if board.tile[(endPos.x + i) as usize][(endPos.y - i) as usize].peice == ' ' {}
                else if board.tile[(endPos.x + i) as usize][(endPos.y - i) as usize].peice == peice && board.tile[(endPos.x + i) as usize][(endPos.y - i) as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x + i) as u8; startPos.y = (endPos.y - i) as u8;
                }else {checkSE = false}
            }else{checkSE = false}//SE

            if checkSW == true && endPos.x as i8 - i as i8 >= 0 && endPos.y as i8 - i as i8 >= 0 {
                if board.tile[(endPos.x - i) as usize][(endPos.y - i ) as usize].peice == ' ' {}
                else if board.tile[(endPos.x - i) as usize][(endPos.y - i) as usize].peice == peice && board.tile[(endPos.x - i) as usize][(endPos.y - i) as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x - i) as u8; startPos.y = (endPos.y - i) as u8;
                }else {checkSW = false}
            }else{checkSW = false}//SW

            if checkNW == true && endPos.x as i8 - i as i8 >= 0 && endPos.y + i < 8{
                if board.tile[(endPos.x - i) as usize][(endPos.y + i) as usize].peice == ' ' {}
                else if board.tile[(endPos.x - i) as usize][(endPos.y + i) as usize].peice == peice && board.tile[(endPos.x - i) as usize][(endPos.y + i) as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x - i) as u8; startPos.y = (endPos.y + i);
                }else {checkNW = false}
            }else{checkNW = false}//NW
        }
    return startPos;
}

fn resolve_Ambig(startPos : &Vec<Displace>, endPos : &Displace) -> Displace{
    print!("amb {} ", startPos.len());
    let mut intial = Displace{peice : 'f', x  : 8, y : 8, ambigX : 8, ambigY : 8, moveStr : [' '; 5]};
    let mut temp = intial.clone();
    let mut allowed : Vec<Displace> = Vec::new();
    let mut ambigResolved = false; //solved ambiguatiy
    let mut ambigResolvedX = false;
    let mut ambigResolvedY = false;
    let mut count = 0;

    if startPos.len() == 0{return intial}

    if startPos.len() == 1{
        intial = startPos[0].clone();
        ambigResolved = true;
    }

    if ambigResolved == false{
    for i in 0..startPos.len() {
        print!("{} ", startPos[i].peice);
        if endPos.ambigX != 8 { //detects if move is allowed
            if endPos.ambigX == startPos[i].x {ambigResolvedX = true;}
        }else {ambigResolvedX = true;}
        if endPos.ambigY != 8 {
            if endPos.ambigY == startPos[i].x {ambigResolvedY = true;}
        }else {ambigResolvedY = true;}

        if ambigResolvedX == true && ambigResolvedY == true {count += 1; allowed.push(startPos[i]);}
        ambigResolvedX = false; ambigResolvedY = false; //resets values
        }



    if count == 1{intial = allowed[0];}
    if count > 1{
        println!("please select numbered option");
        for i in 0..allowed.len(){
            println!("{} : {}{} ", i+1,(allowed[i].x +97) as char, allowed[i].y + 1);
        }
    }

    let mut play = String::new();
    io::stdin().read_line(&mut play).unwrap(); //input from
    play = play.trim().to_string();

    let result = play.as_bytes()[0] - 49;
    intial = allowed[result as usize];
    }
    return intial;
}

fn checkallowed(board : &Board, endPos : &Displace) -> Displace{

    let mut startPos = Displace{peice : 'f', x  : 8, y : 8, ambigX : 8, ambigY : 8, moveStr : [' ';5]};
    let mut startVec : Vec<Displace> = Vec::new();
    let player : u8;
    let openent : u8;
    let direc : i8;
    if board.player == false {player = 1; openent = 2;} else { player = 2; openent = 1;}

    if endPos.peice == 'p' {
        if board.player == false{direc = 1;}else{direc = -1;} //direc looking backtowards start pos

        if board.tile[endPos.x as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].peice == ' ' && board.tile[endPos.x as usize][(endPos.y as i8 + direc)as usize].colour == player{
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 + direc) as u8;
            print!("test1");
            startVec.push(startPos);
        } //move one foward

        if board.tile[endPos.x as usize][(endPos.y as i8 + 2 * direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].peice == ' '  &&
        board.tile[endPos.x as usize][(endPos.y as i8 + direc) as usize].peice == ' ' && board.tile[endPos.x as usize][(endPos.y as i8 + 2 *  direc)as usize].colour == player
        && ((endPos.y == 3 && player == 2) || (endPos.y == 4 && player == 1)){
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 + 2 *  direc) as u8;
            print!("test2");
            startVec.push(startPos);
        } //move two foward

        if ( if endPos.x < 7 { board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].colour == openent && board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].colour == player} else {false == true}){
            startPos.peice = 's'; startPos.x = endPos.x + 1; startPos.y = (endPos.y as i8 + direc) as u8;
            println!("test3");
            startVec.push(startPos);
        } //takes left

        if ( if endPos.x > 0 { board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].colour == openent && board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].colour == player} else {false == true}){
            println!("test4");
            startPos.peice = 's'; startPos.x = endPos.x - 1; startPos.y = (endPos.y as i8 + direc) as u8;
            startVec.push(startPos);
        } //take right

        if ( if endPos.x < 7 { board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].peice == ' ' && board.History[board.History.len() - 1].peice == 'p'
        && board.History[board.History.len() - 1].x == endPos.x && board.History[board.History.len() - 1].y == (endPos.y as i8 + direc) as u8  && board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].colour == player}
        else {false == true}){

            startPos.peice = 'S'; startPos.x = endPos.x + 1; startPos.y = (endPos.y as i8 + direc) as u8;
            startVec.push(startPos);
        } //en possion left

        if ( if endPos.x > 0 { board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' &&
        (board.tile[endPos.x as usize][endPos.y as usize].peice == ' ' && board.History[board.History.len() - 1].peice == 'p' && board.History[board.History.len() - 1].x == endPos.x && board.History[board.History.len() - 1].y ==( endPos.y as i8 + direc ) as u8 ) && board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].colour == player} else {false == true}){

            startPos.peice = 'S'; startPos.x = endPos.x - 1; startPos.y = (endPos.y as i8 + direc) as u8;
            startVec.push(startPos);
        } //en possion right
    }//pawn allowed

    else if endPos.peice == 'R' { startPos = checkQMB(&board, startPos, &endPos, player, 'R', true, true, true, true, false, false, false, false); }//rook allowed

    else if endPos.peice == 'N' {
      if if endPos.x + 1 < 8 && endPos.y + 2 < 8 { board.tile[(endPos.x as i8 + 1) as usize][(endPos.y as i8 + 2)as usize].peice == 'N' && board.tile[(endPos.x as i8 + 1) as usize][(endPos.y as i8 + 2)as usize].colour == player} else {true == false} {
        startPos.peice = 'N'; startPos.x = (endPos.x as i8 + 1) as u8; startPos.y = (endPos.y as i8 + 2) as u8;
        startVec.push(startPos);
      }
      if if endPos.x as i8 - 1 >= 0 && endPos.y + 2 < 8 { board.tile[(endPos.x as i8 - 1) as usize][(endPos.y as i8 + 2)as usize].peice == 'N' && board.tile[(endPos.x as i8 - 1) as usize][(endPos.y as i8 + 2)as usize].colour == player} else {true == false} {
        startPos.peice = 'N'; startPos.x = (endPos.x as i8 - 1) as u8; startPos.y = (endPos.y as i8 + 2) as u8;
        startVec.push(startPos);
      }
      if if endPos.x + 1 < 8 && endPos.y as i8 - 2 >= 0 { board.tile[(endPos.x as i8 + 1) as usize][(endPos.y as i8 - 2)as usize].peice == 'N' && board.tile[(endPos.x as i8 + 1) as usize][(endPos.y as i8 - 2)as usize].colour == player} else {true == false} {
        startPos.peice = 'N'; startPos.x = (endPos.x as i8 + 1) as u8; startPos.y = (endPos.y as i8 - 2) as u8;
        startVec.push(startPos);
      }
      if if endPos.x as i8 - 1 >= 0 && endPos.y as i8 - 2 >= 0 { board.tile[(endPos.x as i8 - 1) as usize][(endPos.y as i8 - 2)as usize].peice == 'N' && board.tile[(endPos.x as i8 - 1) as usize][(endPos.y as i8 - 2)as usize].colour == player} else {true == false} {
        startPos.peice = 'N'; startPos.x = (endPos.x as i8 - 1) as u8; startPos.y = (endPos.y as i8 - 2) as u8 ;
        startVec.push(startPos);
      }
      if if endPos.x + 2 < 8 && endPos.y + 1 < 8 { board.tile[(endPos.x as i8 + 2) as usize][(endPos.y as i8 + 1)as usize].peice == 'N' && board.tile[(endPos.x as i8 + 2) as usize][(endPos.y as i8 + 1)as usize].colour == player} else {true == false} {
        startPos.peice = 'N'; startPos.x = (endPos.x as i8 + 2) as u8; startPos.y = (endPos.y as i8 + 1) as u8;
        startVec.push(startPos);
      }
      if if endPos.x + 2 < 8 && endPos.y as i8 - 1 >= 0 { board.tile[(endPos.x as i8 + 2) as usize][(endPos.y as i8 - 1)as usize].peice == 'N' && board.tile[(endPos.x as i8 + 2) as usize][(endPos.y as i8 - 1)as usize].colour == player} else {true == false} {
        startPos.peice = 'N'; startPos.x = (endPos.x as i8 + 2) as u8; startPos.y = (endPos.y as i8 - 1) as u8;
        startVec.push(startPos);
      }
      if if endPos.x as i8 - 2 >= 0 && endPos.y + 1 < 8 { board.tile[(endPos.x as i8 - 2) as usize][(endPos.y as i8 + 1)as usize].peice == 'N' && board.tile[(endPos.x as i8 - 2) as usize][(endPos.y as i8 + 1)as usize].colour == player} else {true == false} {
        startPos.peice = 'N'; startPos.x = (endPos.x as i8 - 2) as u8; startPos.y = (endPos.y as i8 + 1) as u8;
        startVec.push(startPos);
      }
      if if endPos.x as i8 - 2 >= 0 && endPos.y as i8 - 1 >= 0 { board.tile[(endPos.x as i8 - 2) as usize][(endPos.y as i8 - 1)as usize].peice == 'N' && board.tile[(endPos.x as i8 - 2) as usize][(endPos.y as i8 - 1)as usize].colour == player} else { true == false} {
        startPos.peice = 'N'; startPos.x = (endPos.x as i8 - 2) as u8; startPos.y = (endPos.y as i8 - 1) as u8;
        startVec.push(startPos);
      }
    }//knight allowed

    else if endPos.peice == 'B' { startPos = checkQMB(&board, startPos, &endPos, player, 'B', false, false, false, false, true, true, true, true); } //bishop
    else if endPos.peice == 'Q' { startPos = checkQMB(&board, startPos, &endPos, player, 'Q', true, true, true, true, true, true, true, true); } //queen
    else if endPos.peice == 'K' {
        if board.tile[endPos.x as usize][(endPos.y + 1) as usize].peice == 'K' && board.tile[endPos.x as usize][(endPos.y + 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 + 1) as u8;
        }
        else if board.tile[(endPos.x + 1) as usize][(endPos.y + 1) as usize].peice == 'K' && board.tile[(endPos.x + 1) as usize][(endPos.y + 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 + 1) as u8; startPos.y = (endPos.y as i8 + 1) as u8;
        }
        else if board.tile[(endPos.x + 1) as usize][endPos.y as usize].peice == 'K' && board.tile[(endPos.x + 1) as usize][endPos.y as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 + 1) as u8; startPos.y = endPos.y;
        }
        else if board.tile[(endPos.x + 1) as usize][(endPos.y - 1) as usize].peice == 'K' && board.tile[(endPos.x + 1) as usize][(endPos.y - 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 + 1) as u8; startPos.y = (endPos.y as i8 - 1) as u8;
        }
        else if board.tile[endPos.x as usize][(endPos.y - 1) as usize].peice == 'K' && board.tile[endPos.x as usize][(endPos.y - 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 - 1) as u8;
        }
        else if board.tile[(endPos.x - 1) as usize][(endPos.y - 1) as usize].peice == 'K' && board.tile[(endPos.x - 1) as usize][(endPos.y - 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 - 1) as u8; startPos.y = (endPos.y as i8 - 1) as u8;
        }
        else if board.tile[(endPos.x - 1) as usize][endPos.y as usize].peice == 'K' && board.tile[(endPos.x - 1) as usize][endPos.y as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 - 1) as u8; startPos.y = endPos.y;
        }
        else if board.tile[(endPos.x - 1) as usize][(endPos.y + 1) as usize].peice == 'K' && board.tile[(endPos.x - 1) as usize][(endPos.y + 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 - 1) as u8; startPos.y = (endPos.y as i8 + 1) as u8;
        }
    } // king does not detect ambuguate as only one king
    else {startPos.peice = 'f'}

    startPos = resolve_Ambig(&startVec, &endPos);

    return startPos;
}

fn nextMovers(board : &Board, mut result : [[i8;8];8] ,xCord : usize, yCord : usize, colour : &u8 , mut Up : bool, mut Do :bool, mut Le : bool, mut Ri : bool, mut NE : bool, mut SE : bool, mut SW : bool, mut NW : bool) -> [[i8; 8]; 8]{

    for i in 1..8{
            if Up == true && xCord +i < 8 {
                if board.tile[(xCord + i) as usize][yCord].colour == 0 {}
                else {Up = false}
                result[(xCord+i)as usize][yCord] += 1;
            }
            else {Up = false }

            if Do == true && xCord as i8 -i as i8 >= 0 {
                if board.tile[(xCord - i) as usize][yCord].colour == 0 {}
                else {Do = false}
                result[(xCord - i)as usize][yCord] += 1;
            }
            else {Do = false }

            if Le == true && yCord as i8 - i as i8 >= 0 {
                if board.tile[xCord][yCord - i as usize].colour == 0 {}
                else {Le = false}
                result[xCord][(yCord - i)as usize] += 1;
            }
            else {Le = false }

            if Ri == true && yCord + i < 8 {
                if board.tile[xCord][(yCord +i ) as usize].colour == 0 {}
                else {Ri = false}
                result[xCord][(yCord+i)as usize] += 1;
            }
            else {Ri = false }

            if NE == true && xCord +i < 8 && yCord + i < 8 {
                if board.tile[(xCord + i) as usize][(yCord + i) as usize].colour == 0 {}
                else {NE = false}
                result[(xCord+i)as usize][(yCord+i)as usize] += 1;
            }
            else {NE = false }

            if SE == true && xCord +i < 8 && yCord as i8 - i as i8 >= 0 {
                if board.tile[(xCord + i) as usize][(yCord - i) as usize].colour == 0 {}
                else {SE = false}
                result[(xCord + i)as usize][(yCord - i)as usize] += 1;
            }
            else {SE = false }

            if SW == true && xCord as i8 - i as i8 >= 0 && yCord as i8 - i as i8 >= 0{
                if board.tile[(xCord - i) as usize][(yCord - i) as usize].colour == 0 {}
                else {SW = false}
                result[(xCord-i)as usize][(yCord-i)as usize] += 1;
            }
            else {SW = false }

            if NW == true && xCord as i8 - i as i8 >= 0 && yCord +i < 8 {
                if board.tile[(xCord - i) as usize][(yCord +i) as usize].colour == 0 {}
                else {NW = false}
                result[(xCord - i)as usize][(yCord + i)as usize] += 1;
            }
            else {NW = false }


    }
    return result;
}

fn nextNight(board : &Board, mut result : [[i8;8];8], x : &usize, y : &usize) -> [[i8;8];8]{
    if *x + 2 < 8 && *y + 1 < 8 {result[x+2][y+1] +=1;}
    if *x + 2 < 8 && *y as i8 - 1 >= 0 {result[(x+2) as usize][(y-1) as usize] += 1;}
    if *x as i8 - 2 >= 0 && *y + 1 < 8 {result[(x-2) as usize][(y+1) as usize] += 1;}
    if *x as i8- 2 >= 0 && *y as i8 - 1 >= 0 {result[(x-2) as usize][(y-1) as usize] += 1;}
    if *x + 1 < 8 && *y + 2 < 8 {result[(x+1) as usize][(y+2) as usize]+= 1;}
    if *x as i8 - 1 >= 0 && *y + 2 < 8 {result[(x-1) as usize][(y+2) as usize]+=1;}
    if *x + 1 < 8 && *y as i8 - 2 >= 0 {result[(x+1) as usize][(y-2) as usize]+=1;}
    if *x as i8 - 1 >= 0 && *y as i8 - 2 >= 0 {result[(x-1) as usize][(y-2) as usize]+=1;}

    return result;
}

//array of all posible captures points in next turn note does not include all moves
fn nextTake(board : &Board) -> [[Sphere;8];8]{

    let mut sphere = [[Sphere {whiteSphere : 0, blackSphere : 0};8];8];
    let mut black = [[0 as i8; 8]; 8];
    let mut white = [[0 as i8; 8]; 8];

    for j in 0..8{
        for i in 0..8{
            if board.tile[i][j].peice == 'p'{
                if board.tile[i][j].colour == 1 {
                    if j > 0{
                        if i < 7{ sphere[i+1][j-1].blackSphere += 1; } // ways that pawn can take
                        if i > 0{ sphere[i-1][j-1].blackSphere += 1; }
                    }
                }else if board.tile[i][j].colour == 2 {
                    if j < 7{
                        if i < 7{ sphere[i+1][j+1].whiteSphere += 1; } // ways that pawn can take
                        if i > 0{ sphere[i-1][j+1].whiteSphere += 1; }
                    }
                }

            }
            else if board.tile[i][j].peice == 'R'{
                if board.tile[i][j].colour == 1{ black = nextMovers(board, black, i, j, &1, true, true, true, true, false, false, false, false); }
                else {white = nextMovers(board, white, i, j, &2, true, true, true, true, false, false, false, false); }

            }
            else if board.tile[i][j].peice == 'N'{
                if board.tile[i][j].colour == 1{ black = nextNight(board, black, &i, &j); }
                else { white = nextNight(board, white, &i, &j); }
            }
            else if board.tile[i][j].peice == 'B'{
                if board.tile[i][j].colour == 1{ black = nextMovers(board, black, i, j, &1, false, false, false, false, true, true, true, true); }
                else {white = nextMovers(board, white, i, j, &2, false, false, false, false, true, true, true, true); }
            }

            else if board.tile[i][j].peice == 'Q'{
                if board.tile[i][j].colour == 1{ black = nextMovers(board, black, i, j, &1, true, true, true, true, true, true, true, true); }
                else {white = nextMovers(board, white, i, j, &2, true, true, true, true, true, true, true, true); }
            }

            else if board.tile[i][j].peice == 'K'{
                if board.tile[i][j].colour == 1 {
                    if i > 0 {sphere[i-1][j].blackSphere += 1;}
                    if j < 7 {sphere[i+1][j].blackSphere += 1;}
                    if i > 0 && j > 0 {sphere[i-1][j-1].blackSphere += 1;}
                    if i > 0 && j < 7 {sphere[i-1][j+1].blackSphere += 1;}
                    if i < 7 && j > 0 {sphere[i+1][j-1].blackSphere += 1;}
                    if i < 7 && j < 7 {sphere[i+1][j+1].blackSphere += 1;}
                    if j > 0 {sphere[i][j-1].blackSphere += 1;}
                    if j < 7 {sphere[i][j+1].blackSphere += 1;}
                }else if board.tile[i][j].colour == 1 {
                    if i > 0 {sphere[i-1][j].whiteSphere += 1;}
                    if j < 7 {sphere[i+1][j].whiteSphere += 1;}
                    if i > 0 && j > 0 {sphere[i-1][j-1].whiteSphere += 1;}
                    if i > 0 && j < 7 {sphere[i-1][j+1].whiteSphere += 1;}
                    if i < 7 && j > 0 {sphere[i+1][j-1].whiteSphere += 1;}
                    if i < 7 && j < 7 {sphere[i+1][j+1].whiteSphere += 1;}
                    if j > 0 {sphere[i][j-1].whiteSphere += 1;}
                    if j < 7 {sphere[i][j+1].whiteSphere += 1;}
                }
            }
        }
    }

    for j in 0..8{
        for i in 0..8{
            sphere[i][j].whiteSphere += white[i][j];
            sphere[i][j].blackSphere += black[i][j];
            print!("{} ", sphere[i][j].blackSphere);
        }

        print!("   ");

        for i in 0..8{
            print!("{} ", sphere[i][j].whiteSphere);
        }

        println!("")
    }

    return sphere;
}

fn CheckDetc(board : &Board, sphere : &[[Sphere;8];8]) -> Check{
    let mut check = Check{white: false, black : false};
    let mut kingCount = 0;
    for j in 0..8{
        for i in 0..8{
            if board.tile[i][j].peice == 'K'{
                kingCount += 1;
                if board.tile[i][j].colour == 1 && sphere[i][j].whiteSphere > 0 {
                    println!("black check");
                    check.black = true;
                }else if board.tile[i][j].colour == 2 && sphere[i][j].blackSphere > 0 {
                    println!("white check");
                    check.white = true;
                }
            }
        }
        if kingCount == 2{return check;}
    }
    return check;
}

//save board state to file
fn save(board : &Board, mut endPos : Displace, mut intial : Displace) -> std::io::Result<()>{

    let mut count = 0;
    if endPos.peice == 'p' {} else {intial.moveStr[0] = endPos.peice; count += 1;}
    if intial.ambigX == 8 {} else {intial.moveStr[count] = (intial.ambigX) as char; count += 1;}
    if intial.ambigY == 8 {} else {intial.moveStr[count] = (intial.ambigY) as char; count += 1;}
    intial.moveStr[count] = (endPos.x +97) as char;
    intial.moveStr[count+1] = (endPos.y+49) as char;

    println!(" test {}{}{}{}{} test1", intial.moveStr[0], intial.moveStr[1], intial.moveStr[2], intial.moveStr[3], intial.moveStr[4]);

    let mut file = File::create("Foo.pgn")?;

    for i in 0..board.History.len(){
        // /file.write_all(board.History[i])?;
    }
    Ok(())
}

//main function
fn main() {

    //intialise variables
    let running = true;
    let mut board = Board::new();
    let mut moveAccepted = true;
    let mut turn = Displace {peice : ' ', x : 8, y : 8, ambigX : 8, ambigY : 8, moveStr : [' '; 5]};
    let mut end = Displace {peice: ' ', x:8, y: 8, ambigX : 8, ambigY : 8, moveStr : [' '; 5]};
    let mut oldstate = board.clone();
    let mut history =  String::new();

    let mut sphere = nextTake(&board);

    let mut check = Check{white: false, black : false};

    while running { //main loop
        if moveAccepted == true{
            if board.player == true{board.player = false; }else{board.player = true;} //player
            board.print();

            check = CheckDetc(&board, &sphere);

            print!("player: {} please make your move:  \n", if board.player == true { "white" }else {"black"}); //player turn

        }else{println!("please reenter your move"); moveAccepted = true;} //reinput turn if not allowed
        let mut play = String::new();
        io::stdin().read_line(&mut play).unwrap(); //input move
        play = play.trim().to_string();

        if play.len() == 2{ //checks if pawn as input 2 long
            turn.peice = 'p';
            turn.x = play.as_bytes()[0] as u8; turn.x -= 97;
            turn.y = play.as_bytes()[1] as u8; turn.y -= 49;

            if  turn.x <= 7 && turn.y <= 7 { //checks played inside board
                moveAccepted = board.checkColour(&turn)
            }else{moveAccepted = false;}

        }else if play.len() == 3{ //checks play lengh
            turn.peice = play.as_bytes()[0] as char;
            turn.x = play.as_bytes()[1] as u8; turn.x -= 97;
            turn.y = play.as_bytes()[2] as u8; turn.y -= 49;

            if turn.x <= 7 && turn.y <= 7 && (turn.peice == 'R' || turn.peice == 'N' || turn.peice == 'B' || turn.peice == 'Q' || turn.peice == 'K' ){ //checks allowed
                moveAccepted = board.checkColour(&turn)
            }else{moveAccepted = false;}
        }else if play.len() == 4{
            turn.peice = play.as_bytes()[0] as char;
            turn.x = play.as_bytes()[2] as u8; turn.x -= 97;
            turn.y = play.as_bytes()[3] as u8; turn.y -= 49;

            if play.as_bytes()[1] as u8 - 97 >= 0 && play.as_bytes()[1] as u8 -97 < 8{
                turn.ambigX = play.as_bytes()[1] as u8 - 97;
            }else if play.as_bytes()[1] as u8 - 49 >= 0 && play.as_bytes()[1] as u8 - 49 < 8{
                turn.ambigY = play.as_bytes()[1] as u8 - 49;
            }
        }else if play.len() == 5{
            turn.peice = play.as_bytes()[0] as char;
            turn.ambigX = play.as_bytes()[1] as u8; turn.x -= 97;
            turn.ambigY = play.as_bytes()[2] as u8; turn.y -= 49;
            turn.x = play.as_bytes()[3] as u8; turn.x -= 97;
            turn.y = play.as_bytes()[4] as u8; turn.y -= 49;

        }else if play == "exit"{ break; }
        else{moveAccepted = false;}

        if board.tile[turn.x.clone() as usize][turn.y.clone() as usize].peice == 'K'{ //checks if king is being taken
            println!("cannot take the king");
            moveAccepted = false;
        }

        if moveAccepted == true { //runs peices check
            end = checkallowed(&board, &turn);
            if end.peice == 'f' {moveAccepted = false; println!("move not allowed");}
        }

        if moveAccepted == true { //moves the peices
            board = board.Swap(&turn, &end);

            sphere = nextTake(&board);
            check = CheckDetc(&board, &sphere);

            //detects in player is in check
            if board.player == true && check.white == true{
                print!("in check");
                board = oldstate.clone(); //puts back into old state if they are
            }else if board.player == true && check.black == true{
                print!("in check");
                board = oldstate.clone();
            }else {
                history = format!("{}{}{}{}{}", end.moveStr[0], end.moveStr[1], end.moveStr[2], end.moveStr[3], end.moveStr[4]);
                print!("{}{}{}{}{}", end.moveStr[0], end.moveStr[1], end.moveStr[2], end.moveStr[3], end.moveStr[4]);
                println!("test {} test", history);
                oldstate = board.clone(); //updates old state
                board.History.push(turn); //creates a history in PGN notation
            }
        }
    }
    println!("Hello, world!");
}
