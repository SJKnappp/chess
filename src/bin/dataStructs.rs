#[derive(Copy, Clone)]
pub struct Check{
    white : bool,
    black : bool,
}

#[derive(Copy, Clone)]
pub struct Sphere{
    whiteSphere : i8, //white can move here next turn
    blackSphere : i8, //black can move here next turn
}

#[derive(Copy, Clone)]
pub struct Displace{
    peice : char, //stores the peice being moved and the susses of the return value
    x : u8, //stores coords
    y : u8, //stores coords
    ambigX : u8, //used to resolve abisuas moves
    ambigY : u8,
    moveStr : [char; 5],
}

#[derive(Copy, Clone)]
pub struct Piece{
    pub peice : char, //stores the peice r rock n knight b bishop q queen k king p pawn
    pub colour : u8, //0 no specify 1 black 2 white
    pub moved : bool, //detects peices first move
}

#[derive(Copy, Clone)]
pub struct Player{
    WhiteAi : bool,
    BlackAi : bool,
}

#[derive(Clone)]
pub struct Board{
    pub tile :  [[Piece; 8]; 8], //stores board state
    player : bool, //false black true whtie
    takenWhite : Vec<char>, //holds pieces taken by black
    takenBlack : Vec<char>, //hold pieces taken by white
    History : Vec<Displace>, //move histroy
    pub playerAi : Player,
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