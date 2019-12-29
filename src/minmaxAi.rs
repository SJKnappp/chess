use std::fs::File;
use ansi_term::Colour::{Red, White};
use std::{io};

#[derive(Copy, Clone)]
pub struct Check{
    pub white : bool,
    pub black : bool,
}

#[derive(Copy, Clone)]
pub struct Sphere{
    pub whiteSphere : i8, //white can move here next turn
    pub blackSphere : i8, //black can move here next turn
}

#[derive(Copy, Clone)]
pub struct Displace{
    pub peice : char, //stores the peice being moved and the susses of the return value
    pub x : u8, //stores coords
    pub y : u8, //stores coords
    pub ambigX : u8, //used to resolve abisuas moves
    pub ambigY : u8,
    pub moveStr : [char; 5],
}

impl Displace{
    pub fn new() -> Displace{
        let mut displace : Displace;

        displace.x = 0;
        displace.y = 0;
        displace.ambigX = 0;
        displace.ambigY = 0;
        displace.moveStr = [' ';5];

        return displace;
    }
}

#[derive(Copy, Clone)]
pub struct Piece{
    pub peice : char, //stores the peice r rock n knight b bishop q queen k king p pawn
    pub colour : u8, //0 no specify 1 black 2 white
    pub moved : bool, //detects peices first move
}

#[derive(Copy, Clone)]
pub struct Player{
    pub WhiteAi : bool,
    pub BlackAi : bool,
}

#[derive(Clone)]
pub struct Board{
    pub tile :  [[Piece; 8]; 8], //stores board state
    pub player : bool, //false black true whtie
    pub takenWhite : Vec<char>, //holds pieces taken by black
    pub takenBlack : Vec<char>, //hold pieces taken by white
    pub History : Vec<Displace>, //move histroy
    pub playerAi : Player,
}

impl Board {

    //creats a standards board layout
    pub fn new() -> Board{

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
    pub fn print(&self){

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
    pub fn checkColour(&self, Move : &Displace) -> bool{
        if self.player == true && self.tile[Move.x as usize][Move.y as usize].colour == 2 { return false; }
        else if self.player == false && self.tile[Move.x as usize][Move.y as usize].colour == 1 { return false; }
        return true;
    }
    //takes start and end point and kills and moves the peices
    pub fn Swap(mut self, Final : &Displace, intial : &Displace) -> Board{

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

fn scores(peice : char) -> i16 {
    match peice{
        'p' => return 1,
        'R' => return 50,
        'N' => return 25,
        'B' => return 30,
        'Q' => return 100,
        'K' => return 255,
        _ => return 0,
    }
}

pub fn calcScore(board : Board, colour : u8, check : Check, sphere : [[Sphere;8];8], debug : bool){
    let openent : u8;
    if  colour == 1 {openent = 2;} else {openent = 1;}
    
    let mut gain : [[i16; 8]; 8] = [[0;8];8];
    let mut risk : [[i16; 8]; 8] = [[0;8];8];

    let mut selfSphere : [[i8;8];8] = [[0;8];8];
    let mut opSphere : [[i8;8];8] = [[0;8];8];

    if colour == 2{
        for j in 0..8{
            for i in 0..8{
                selfSphere[i][j] = sphere[i][j].whiteSphere; 
                opSphere[i][j] = sphere[i][j].blackSphere;
            }
        }
    }else {
        for j in 0..8{
            for i in 0..8{
                selfSphere[i][j] = sphere[i][j].blackSphere; 
                opSphere[i][j] = sphere[i][j].whiteSphere;
            }
        }
    }

    println!("calc scores");

    if debug == true{
        println!("player colour: {} ", colour);
    }

    //risk current
    for j in 0..8{
        for i in 0..8 {

            if board.tile[i][j].colour == colour && opSphere[i][j] != 0 {risk[i][j] += opSphere[i][j] as i16 * scores(board.tile[i][j].peice);}
            if board.tile[i][j].colour == openent && selfSphere[i][j] != 0 {gain[i][j] += selfSphere[i][j] as i16 * scores(board.tile[i][j].peice);}

            if debug == true{
            print!(" {} ", risk[i][j]);
            }
        }
        if debug == true{
            print!(" <-risk gain-> ");
        }

        if debug == true{
        for i in 0..8{
            print!(" {} ", gain [i][j]);
        }
        println!("");
        }
    }
    
}
fn main() {}