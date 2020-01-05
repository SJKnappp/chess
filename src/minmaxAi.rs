use std::fs::File;
use ansi_term::Colour::{Red, White};
use std::{io, thread, time::Duration};

pub mod dataStore;

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
        let mut displace = Displace { peice : ' ', x : 8, y : 8, ambigX : 0, ambigY : 0, moveStr : [' '; 5]};

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
    pub fn Swap(mut self, Final : &Displace, intial : &Displace, Ai : bool) -> Board{
        

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
            if Ai == false{ //check not ai function
                        print!("{}, ", Ai);
                        

            println!("pawn premotted please select replacement");
            let sucsses = true;
            while sucsses == true { //waits till pawn prommotion sucseful before moving on
                let mut select = String::new();
                io::stdin().read_line(&mut select).unwrap(); //takes an input
                select = select.trim().to_string();
                if select.len() == 1{
                    match select.as_ref(){ //swaps peice
                        "n" | "N" => self.tile[intial.x as usize][intial.y as usize].peice = 'N',
                        "r" | "R" => self.tile[intial.x as usize][intial.y as usize].peice = 'R',
                        "b" | "B" => self.tile[intial.x as usize][intial.y as usize].peice = 'B',
                        "q" | "Q" => self.tile[intial.x as usize][intial.y as usize].peice = 'Q',
                        _ => println!("please select ether q r b q"),
                    }
                }
                if self.tile[intial.x as usize][intial.y as usize].peice != 'p' { break; } //if peices changed break out of loop
            }
            }else {self.tile[intial.x as usize][intial.y as usize].peice = 'Q';}
        }

        if self.tile[Final.x as usize][Final.y as usize].peice == 'K' {println!("attempt to remove king from {}{}", intial.x, intial.y);}

        self.tile[Final.x as usize][Final.y as usize].peice = self.tile[intial.x as usize][intial.y as usize].peice; //moves peice
        self.tile[Final.x as usize][Final.y as usize].colour = self.tile[intial.x as usize][intial.y as usize].colour;
        self.tile[Final.x as usize][Final.y as usize].moved = true;
        self.tile[intial.x as usize][intial.y as usize].peice = ' '; //resets original point
        self.tile[intial.x as usize][intial.y as usize].colour = 0 ;

        return self;
    }
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
pub fn nextTake(board : &Board, debug : bool) -> [[Sphere;8];8]{

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
                    if i < 7 {sphere[i+1][j].blackSphere += 1;}
                    if i > 0 && j > 0 {sphere[i-1][j-1].blackSphere += 1;}
                    if i > 0 && j < 7 {sphere[i-1][j+1].blackSphere += 1;}
                    if i < 7 && j > 0 {sphere[i+1][j-1].blackSphere += 1;}
                    if i < 7 && j < 7 {sphere[i+1][j+1].blackSphere += 1;}
                    if j > 0 {sphere[i][j-1].blackSphere += 1;}
                    if j < 7 {sphere[i][j+1].blackSphere += 1;}
                }else if board.tile[i][j].colour == 2 {
                    if i > 0 {sphere[i-1][j].whiteSphere += 1;}
                    if i < 7 {sphere[i+1][j].whiteSphere += 1;}
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

            if debug == true{
                print!("{} ", sphere[i][j].blackSphere);
            }
        }

        if debug == true {
        print!("   ");

        for i in 0..8{
            print!("{} ", sphere[i][j].whiteSphere);
        }
            println!("")
        }
    }

    return sphere;
}

pub fn CheckDetc(board : &Board, sphere : &[[Sphere;8];8], isAi : bool) -> Check{
    let mut check = Check{white: false, black : false};
    let mut kingCount = 0;
    for j in 0..8{
        for i in 0..8{
            if board.tile[i][j].peice == 'K'{
                kingCount += 1;
                if board.tile[i][j].colour == 1 && sphere[i][j].whiteSphere > 0 {
                    if isAi == false{println!("black check");}
                    check.black = true;
                   
                }else if board.tile[i][j].colour == 2 && sphere[i][j].blackSphere > 0 {
                    if isAi == false{println!("white check");}
                    check.white = true;
                   
                }
            }
        }
        if kingCount == 2{return check;}
    }
    return check;
}


#[derive(Copy, Clone)]
pub struct Ai_return{
    pub play : Displace,
    result : isize,
}
impl Ai_return{
    fn new() -> Ai_return{
        let Ai = Ai_return{play : Displace::new(), result : 0};
        return Ai;
    }
}

pub struct AiScoreTrack {
    pub reward : isize,
    pub risk : isize,
    pub protected : isize,
}
impl AiScoreTrack{
    fn new() -> AiScoreTrack{
        let aiScoreTrack = AiScoreTrack{reward : 0, risk :0, protected : 0};
        return aiScoreTrack;
    }
}


pub fn AiCall(board : Board, colour : u8, check : Check, sphere : [[Sphere;8];8], debug : bool) -> Displace{
    let mut ai_return = Ai_return::new();

    //dataStore::main();
    ai_return = possibleMoves(&board, colour, colour, debug, 0, 7, 1); //for ai debth change 6 value
    
    println!("returned saftley");
    calcScore(board.clone(), colour, check, sphere, debug);
        
    return ai_return.play.clone();

    //handle.join().unwrap();
}

fn scores(peice : char) -> i16 {
    match peice{
        'p' => return 30,
        'R' => return 50,
        'N' => return 30,
        'B' => return 40,
        'Q' => return 100,
        'K' => return 1000,
        _ => return 0,
    }
}

fn turn_Score(board : &Board, sphere : &[[Sphere;8];8], turn : &Displace, colour : u8, openent : u8) -> isize{
    
    let mut gain = 0;
    let mut risk = 0;

    let mut selfSphere : [[i32;8];8] = [[0;8];8];
    let mut opSphere : [[i32;8];8] = [[0;8];8];
    
    if colour == 2{
        for j in 0..8{
            for i in 0..8{
                selfSphere[i][j] = sphere[i][j].whiteSphere as i32; 
                opSphere[i][j] = sphere[i][j].blackSphere as i32;
            }
        }
    }else {
        for j in 0..8{
            for i in 0..8{
                selfSphere[i][j] = sphere[i][j].blackSphere as i32; 
                opSphere[i][j] = sphere[i][j].whiteSphere as i32;
            }
        }
    }
    
    gain += scores(turn.peice) * selfSphere[turn.x as usize][turn.y as usize] as i16;
    if board.tile[turn.x as usize][turn.y as usize].colour == openent {gain += scores(board.tile[turn.x as usize][turn.y as usize].peice);}
    risk += scores(turn.peice) * opSphere[turn.x as usize][turn.y as usize] as i16;
    return (gain - risk) as isize;
}

pub fn calcScore(board : Board, colour : u8, check : Check, sphere : [[Sphere;8];8], debug : bool) -> AiScoreTrack{

    let mut scoreTrackr = AiScoreTrack::new();

    let openent : u8;
    if  colour == 1 {openent = 2;} else {openent = 1;}
    
    let mut gain : [[i32; 8]; 8] = [[0;8];8];
    let mut risk : [[i32; 8]; 8] = [[0;8];8];

    let mut selfSphere : [[i32;8];8] = [[0;8];8];
    let mut opSphere : [[i32;8];8] = [[0;8];8];

    if colour == 2{
        for j in 0..8{
            for i in 0..8{
                selfSphere[i][j] = sphere[i][j].whiteSphere as i32; 
                opSphere[i][j] = sphere[i][j].blackSphere as i32;
            }
        }
    }else {
        for j in 0..8{
            for i in 0..8{
                selfSphere[i][j] = sphere[i][j].blackSphere as i32; 
                opSphere[i][j] = sphere[i][j].whiteSphere as i32;
            }
        }
    }

    if debug == true{
        println!("player colour: {} ", colour);
    }

    //risk current
    for j in 0..8{
        for i in 0..8 {

            if board.tile[i][j].colour == colour && opSphere[i][j] != 0 {
                risk[i][j] += opSphere[i][j] * scores(board.tile[i][j].peice) as i32;
                scoreTrackr.risk += risk[i][j] as isize;
            }
            if board.tile[i][j].colour == colour && selfSphere[i][j] != 0 {
                scoreTrackr.protected += (opSphere[i][j] * (scores(board.tile[i][j].peice) as i32)/2) as isize;
            }
            if board.tile[i][j].colour == openent && selfSphere[i][j] != 0 {
                gain[i][j] += selfSphere[i][j] * 2 * scores(board.tile[i][j].peice) as i32;
                scoreTrackr.reward += gain[i][j] as isize;
            }if board.tile[i][j].colour == openent && opSphere[i][j] == 0 && selfSphere[i][j] != 0{
                scoreTrackr.reward += 100;
            }

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

    return scoreTrackr;
    
}

fn possibleSwaper(board : &Board, highest : isize, x : usize, y : usize, i : usize, j : usize,player : u8, colour : u8, openent :u8, debug : bool, down :u8, depth:u8, swap : isize, piece : char) -> Ai_return{
    let mut trial = board.clone();
    let mut persistent = board.clone();
    let mut intial = Displace::new();
    let mut Final = Displace::new();
    intial.x = i as u8; intial.y = j as u8; Final.x = x as u8; Final.y = y as u8;
    trial = trial.Swap(&Final, &intial, true);
    let mut ai_return = Ai_return::new();
    ai_return.play.peice = piece; ai_return.play.x = Final.x as u8; ai_return.play.y = Final.y;ai_return.play.ambigX = intial.x; ai_return.play.ambigY = intial.y;
    
    let sphere = nextTake(&persistent, false);
    //let check = CheckDetc(&persistent, &sphere, true);
    let turn = turn_Score(&trial, &sphere, &ai_return.play, colour, openent);
    if turn != 0 {
        //print!("turn {} down {}", turn, down);
    }
    if turn > highest {
        //print!("turn: {} highest: {}", turn, highest);
        let temp = possibleMoves(&trial.clone(), player, openent, debug, down+1, depth, swap).result;
        ai_return.result = turn + swap * temp;
        //print!("result {}", ai_return.result);
    }
                                    
    
    return ai_return;
}

pub fn possibleMoves(board : &Board, player : u8, colour : u8, mut debug : bool, down : u8, depth : u8, mut swap : isize) -> Ai_return{

    let mut intial = Displace::new();
    let mut Final = Displace::new();
    let mut kingPos = Displace::new();
    let mut trial = board.clone();
    let mut persistent = board.clone();
    let mut aiScoreTrack : AiScoreTrack;
    let mut openent;
    let mut tempPiece = ' ';
    let mut turn = Displace::new();
    let mut Ai_return = Ai_return::new();
    let mut highest = -1000;
    let mut Results = Ai_return::new();

    let mut moveable = [[false; 8]; 8];
    let direc : isize;
    if colour == 1 {direc = -1; openent = 2;} else {direc = 1; openent = 1;}
    swap = swap * -1;

    if down == depth{
        return Ai_return;
    }

    for j in 0..8{
        for i in 0..8{
            if board.tile[i][j].colour == colour{
                if board.tile[i][j].peice == 'p' {
                    tempPiece = 'p';
                    if j != 7 && j != 0 && board.tile[i][(j as isize + direc) as usize].colour == 0 && board.tile[i][(j as isize + direc) as usize].peice != 'K' {
                        let x = i; let y = (j as isize + direc) as usize;
                        /*trial = board.clone();
                        intial.x = i as u8; intial.y = j as u8; Final.x = i as u8; Final.y = (j as isize + direc) as u8;
                        trial = trial.Swap( &Final, &intial, true);
                        if down == 0 {
                            Ai_return.result = swap * possibleMoves(&trial.clone(), player, openent, debug, down+1, depth, swap).result;   
                        }else{
                            Ai_return.result = swap * possibleMoves(&trial.clone(), player, openent, debug, down+1, depth, swap).result;
                        }if Ai_return.result > highest as isize{
                            
                            persistent = trial.clone();
                            highest = Ai_return.result;
                            turn.x = i as u8; turn.y = ( j as isize + direc)as u8; turn.peice = 'p'; turn.ambigX = intial.x; turn.ambigY = intial.y;
                        }*/
                        Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                        if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                    }
                    if board.tile[i][j].moved == false && board.tile[i][(j as isize + 2* direc) as usize].colour == 0 && board.tile[i][(j as isize + direc) as usize].colour == 0 && board.tile[i][(j as isize + 2* direc) as usize].peice != 'K'{
                        let x = i; let y = (j as isize + 2 * direc) as usize;
                        Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        
                        /*trial = board.clone();
                        intial.x = i as u8; intial.y = j as u8; Final.x = i as u8; Final.y = (j as isize + 2*direc) as u8;
                        trial = trial.Swap(&Final, &intial, true);
                        if down == 0{
                            Ai_return.result = swap * possibleMoves(&trial.clone(), player, openent, debug, down+1, depth, swap).result;
                            if Ai_return.result > highest as isize {
                                persistent = trial.clone();
                                highest = Ai_return.result;
                                turn.x = i as u8; turn.y =( j as isize + 2* direc)as u8;turn.peice = 'p'; turn.ambigX = intial.x; turn.ambigY = intial.y;
                            }
                        }else{
                            Ai_return.result += swap * possibleMoves(&trial.clone(), player, openent, debug, down+1, depth, swap).result;
                        }*/
                    }
                    if j != 7 && j != 0 && i < 7 && board.tile[i+1][(j as isize + direc) as usize].colour == openent && board.tile[i+1][(j as isize + direc) as usize].peice != 'K'{
                        let x = i + 1; let y = (j as isize + direc) as usize;
                        Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                        if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        /*trial = board.clone();
                        intial.x = i as u8; intial.y = j as u8; Final.x = (i+1) as u8; Final.y = (j as isize + direc) as u8;
                        trial = trial.Swap(&Final, &intial, true);
                        if down == 0{
                            Ai_return.result = swap * possibleMoves(&trial.clone(), player, openent, debug, down+1, depth, swap).result;   
                        }else{
                            Ai_return.result = swap * possibleMoves(&trial.clone(), player, openent, debug, down+1, depth, swap).result;
                        }
                        if Ai_return.result > highest as isize {
                            persistent = trial.clone();
                            highest = Ai_return.result;
                            turn.x = Final.x as u8; turn.y = Final.y;turn.peice = 'p'; turn.ambigX = intial.x; turn.ambigY = intial.y;
                        }*/
                    }
                    if j != 7 && j != 0 && i > 0 && board.tile[i-1][(j as isize + direc) as usize].colour == openent && board.tile[i-1][(j as isize + direc) as usize].peice != 'K'{
                        let x = i - 1; let y = (j as isize + direc) as usize;
                        Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                            if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        /*trial = board.clone();
                        intial.x = i as u8; intial.y = j as u8; Final.x = (i-1) as u8; Final.y = (j as isize + direc) as u8;
                        trial = trial.Swap(&Final, &intial, true);
                        if down == 0{
                            Ai_return.result = swap * possibleMoves(&trial.clone(), player, openent, debug, down+1, depth, swap).result;
                            }
                        }else{
                            Ai_return.result = swap * possibleMoves(&trial.clone(), player, openent, debug, down+1, depth, swap).result;
                        }
                        if Ai_return.result > highest as isize {
                            persistent = trial.clone();
                            highest = Ai_return.result;
                            turn.x = Final.x as u8; turn.y = Final.y;turn.peice = 'p'; turn.ambigX = intial.x; turn.ambigY = intial.y;
                    */
                        }
                    //if Ai_return.result < -50 && down != 0{return Ai_return;}
                }
                
                else if board.tile[i][j].peice == 'R' || board.tile[i][j].peice == 'Q' || board.tile[i][j].peice == 'B' {
                    let mut Up = false;let mut Do =false;let mut Le =false;let mut Ri=false;let mut NE=false;let mut SE=false;let mut SW=false;let mut NW=false;
                    
                    if board.tile[i][j].peice == 'R' {Up = true; Do = true; Le = true; Ri = true; tempPiece = 'R';}
                    if board.tile[i][j].peice == 'B' {NE = true; NW = true; SE =true; SW =true; tempPiece = 'B';}
                    if board.tile[i][j].peice == 'Q' {Up = true; Do = true; Le = true; Ri = true;NE = true; NW = true; SE =true; SW =true; tempPiece = 'Q';}

                    for dis in 1..8{
                        if Up == true && j + dis < 8{
                            let x = i; let y = j  + dis;
                            if board.tile[x][y].colour != colour && board.tile[x][y].peice != 'K'{
                                if board.tile[x][y].colour == openent{Up = false;}
                                Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                            }else {Up = false }

                        }else {Up = false }
            
                        if Do == true && j as i8 - dis as i8 >= 0{
                            let x = i ; let y = j - dis;
                            if board.tile[x][y].colour != colour && board.tile[x][y].peice != 'K'{
                                if board.tile[x][y].colour == openent {Do = false;}
                                Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                            }else {Do = false }
                        }else {Do = false }
            
                        if Le == true && i as i8 - dis as i8 >= 0{
                            
                            let x = i - dis; let y = j;
                            if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                                if board.tile[x][y].colour == openent{Le = false;}
                                Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                            }else {Le = false }
                        }else {Le = false }
            
                        if Ri == true && j + dis < 8 {
                            let x = i; let y = j  + dis;
                            if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                                if board.tile[x][y].colour == openent{Ri = false;}
                                Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                            }else {Ri = false }
                        }else {Ri = false }
            
                        if NE == true && i +dis < 8 && j + dis < 8 && board.tile[(i + dis) as usize][(j + dis) as usize].colour == 0{                                    
                         
                            let x = i + dis; let y = j + dis;
                            if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                                if board.tile[x][y].colour == openent{NE = false;}
                                Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                            }else {NE = false }
                        }else {NE = false }
            
                        if SE == true && i +dis < 8 && j as i8 - dis as i8 >= 0 && board.tile[(i + dis) as usize][(j - dis) as usize].colour == 0{
                            let x = i + dis; let y = j - dis;
                            if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                                if board.tile[x][y].colour == openent{SE = false;}
                                Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                            }else {SE = false }
                        }else {SE = false }
            
                        if SW == true && i as i8 - dis as i8 >= 0 && j as i8 - dis as i8 >= 0 && board.tile[(i - dis) as usize][(j - dis) as usize].colour == 0{
                          
                            let x = i - dis; let y = j - dis;
                            if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                                if board.tile[x][y].colour == openent{SW = false;}
                                Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                            }else {SW = false }
                        }else {SW = false }
            
                        if NW == true && i as i8 - dis as i8 >= 0 && j +dis < 8 && board.tile[(i - dis) as usize][(j +dis) as usize].colour == 0{
                          
                            let x = i - dis; let y = j + dis;
                            if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                                if board.tile[x][y].colour == openent{NW = false;}
                                Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                            }else {NW = false }
                        }else {NW = false }
                        //if Ai_return.result < -50 && down != 0{return Ai_return;}
            
                }
                }else if board.tile[i][j].peice == 'N' {
                    tempPiece = 'N';
                    if i < 6 && j < 7 {
                        let x = i + 2; let y = j + 1;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                                //print!("{}", down);
                        }
                    }if i < 6 && j > 0 {
                        let x = i + 2; let y = j - 1;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }if i < 7 && j < 6 {
                        let x = i + 1; let y = j + 2;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }if i < 7 && j > 1 {
                        let x = i + 1; let y = j - 2;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }if i > 0 && j < 6 {
                        let x = i - 1; let y = j + 2;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }if i > 0 && j > 1 {
                        let x = i - 1; let y = j - 2;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }if i > 1 && j < 7 {
                        let x = i - 2; let y = j + 1;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }if i > 1 && j > 0 {
                        let x = i - 2; let y = j - 1;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }
                    if Ai_return.result < -50 && down != 0{return Ai_return;}
                }
                else if board.tile[i][j].peice == 'K' {
                    tempPiece = 'K';
                    kingPos.x = i as u8;
                    kingPos.y = j as u8; 
                    
                    if i < 7 { 
                        let x = i + 1; let y = j;
                        if board.tile[x][y].colour != colour && board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }
                    
                    if i<7 &&j < 7 {   
                        let x = i+1; let y = j+1;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }
                    if j < 7 {
                          
                        let x = i; let y = j+1;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }
                    if i > 0 && j < 7 {
                          
                        let x = i - 1; let y = j + 1;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }
                    if i > 0 {
                          
                        let x = i -1 ; let y = j;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                            if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }
                    if i > 0 && j> 0 {
                          
                        let x = i -1; let y = j-1;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }
                    if j > 0 {
                          
                        let x = i; let y = j-1;
                        if board.tile[x][y].colour != colour&& board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }
                    
                    if i < 7 && j > 0  {
                          
                        let x = i + 1; let y = j -1;
                        if board.tile[x][y].colour != colour && board.tile[x][y].peice != 'K'{
                            Results = possibleSwaper(board, highest, x, y, i, j, player, colour, openent, debug, down, depth, swap, tempPiece);
                                if (Results.result > highest ){highest = Results.result; Ai_return.play = Results.play; }
                        }
                    }
                    
                    //if Ai_return.result < -50 && down != 0{return Ai_return;}

                }
                
            }
        }
    }

    //trial.print();

    let sphere = nextTake(&persistent, false);
    let check = CheckDetc(&persistent, &sphere, true);

    //print!("kingpos {}{}", kingPos.x, kingPos.y);

    if colour == 1  {if sphere[kingPos.x as usize][kingPos.y as usize].whiteSphere != 0 { Ai_return.result = -1000; return Ai_return;}}
    else { if sphere[kingPos.x as usize][kingPos.y as usize].blackSphere != 0 { Ai_return.result = -1000; return Ai_return; }}
        

    //aiScoreTrack = calcScore(persistent.clone(), colour, check, sphere, debug);

    //Ai_return.result += (2 * aiScoreTrack.reward + aiScoreTrack.protected - aiScoreTrack.risk)/(down + 1) as isize ^ 2;
    if turn.x != 8 && turn.y != 8{
        Ai_return.result += turn_Score(&persistent, &sphere, &turn, colour, openent);
        
    }
    
    if down == 0{
        turn = Ai_return.play;
        print!("move req {}{}{}{}{} ",turn.peice, (turn.ambigX + 97) as char, (turn.ambigY + 49) as char,(turn.x + 97) as char, (turn.y + 49) as char);
        //Ai_return.play = turn;
        Ai_return.result = highest;
    }

    if Ai_return.result != 0{
        println!("\nresult {} player {}", Ai_return.result, colour);
    }

    return Ai_return;
    
}

fn main() {}