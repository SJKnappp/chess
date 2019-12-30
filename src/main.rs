#![allow(non_snake_case)]

extern crate ansi_term;


use std::fs::File;
use ansi_term::Colour::{Red, White};
use std::{io};


pub mod minmaxAi;


//checks allowed move for queen bishop and rook
fn checkQMB(board : &minmaxAi::Board, mut startVec : Vec<minmaxAi::Displace>, endPos : &minmaxAi::Displace, mut player : u8, mut peice : char, mut checkU : bool, mut checkD : bool, mut checkL : bool, mut checkR : bool, mut checkNE : bool, mut checkSE : bool, mut checkSW : bool, mut checkNW : bool) -> Vec<minmaxAi::Displace> {
    let mut startPos = minmaxAi::Displace::new();
    for i in 0..8{
            if checkU == true && endPos.y + i < 8 {
                if board.tile[endPos.x as usize][(endPos.y+i) as usize].peice == ' ' {} //pass on if empty tile
                else if board.tile[endPos.x as usize][(endPos.y+i )as usize].peice == peice && board.tile[endPos.x as usize][(endPos.y+i) as usize].colour == player { //checks that direction equual to peice and colour
                     startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y + i) as u8; //intial location of peice
                     startVec.push(startPos);
                }else {checkU = false}
            }else{checkU = false}//Up

            if checkD == true && endPos.y as i8 - i as i8>= 0 {
                if board.tile[endPos.x as usize][(endPos.y-i) as usize].peice == ' ' {}
                else if board.tile[endPos.x as usize][(endPos.y-i) as usize].peice == peice && board.tile[endPos.x as usize][(endPos.y-i) as usize].colour == player {
                     startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y - i) as u8;
                     startVec.push(startPos);
                }else {checkD = false}
            }else{checkD = false}//Down

            if checkR == true && endPos.x + i < 8 {
                if board.tile[(endPos.x + i) as usize][endPos.y as usize].peice == ' ' {}
                else if board.tile[(endPos.x+i) as usize][endPos.y as usize].peice == peice && board.tile[(endPos.x - i) as usize][endPos.y as usize].colour == player {
                     startPos.peice = 's'; startPos.x = (endPos.x + i) as u8; startPos.y = endPos.y;
                     startVec.push(startPos);
                }else {checkR = false}
            }else{checkR = false}//Right

            if checkL == true && endPos.x as i8 - i as i8 >= 0 {
                if board.tile[(endPos.x - i) as usize][endPos.y as usize].peice == ' ' {}
                else if board.tile[(endPos.x-i) as usize][endPos.y as usize].peice == peice && board.tile[(endPos.x - i) as usize][endPos.y as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x - i) as u8; startPos.y = endPos.y;
                     startVec.push(startPos);
                }else {checkL = false}
            }else{checkL = false}//Left

            if checkNE == true && endPos.x + i < 8 && endPos.y + i < 8 {
                if board.tile[(endPos.x + i) as usize][(endPos.y + i) as usize].peice == ' ' {}
                else if board.tile[(endPos.x + i) as usize][(endPos.y + i) as usize].peice == peice && board.tile[(endPos.x + i) as usize][(endPos.y + i ) as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x + i) as u8; startPos.y = (endPos.y + i ) as u8;
                     startVec.push(startPos);
                }else {checkNE = false}
            }else{checkNE = false}//NE

            if checkSE == true && endPos.x + i < 8 && endPos.y as i8 - i as i8 >= 0 {
                if board.tile[(endPos.x + i) as usize][(endPos.y - i) as usize].peice == ' ' {}
                else if board.tile[(endPos.x + i) as usize][(endPos.y - i) as usize].peice == peice && board.tile[(endPos.x + i) as usize][(endPos.y - i) as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x + i) as u8; startPos.y = (endPos.y - i) as u8;
                     startVec.push(startPos);
                }else {checkSE = false}
            }else{checkSE = false}//SE

            if checkSW == true && endPos.x as i8 - i as i8 >= 0 && endPos.y as i8 - i as i8 >= 0 {
                if board.tile[(endPos.x - i) as usize][(endPos.y - i ) as usize].peice == ' ' {}
                else if board.tile[(endPos.x - i) as usize][(endPos.y - i) as usize].peice == peice && board.tile[(endPos.x - i) as usize][(endPos.y - i) as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x - i) as u8; startPos.y = (endPos.y - i) as u8;
                     startVec.push(startPos);
                }else {checkSW = false}
            }else{checkSW = false}//SW

            if checkNW == true && endPos.x as i8 - i as i8 >= 0 && endPos.y + i < 8{
                if board.tile[(endPos.x - i) as usize][(endPos.y + i) as usize].peice == ' ' {}
                else if board.tile[(endPos.x - i) as usize][(endPos.y + i) as usize].peice == peice && board.tile[(endPos.x - i) as usize][(endPos.y + i) as usize].colour == player{
                     startPos.peice = 's'; startPos.x = (endPos.x - i) as u8; startPos.y = (endPos.y + i);
                     startVec.push(startPos);
                }else {checkNW = false}
            }else{checkNW = false}//NW
        }
    return startVec;
}

fn resolve_Ambig(startPos : &Vec<minmaxAi::Displace>, endPos : &minmaxAi::Displace) -> minmaxAi::Displace{
    let mut intial = minmaxAi::Displace{peice : 'f', x  : 8, y : 8, ambigX : 8, ambigY : 8, moveStr : [' '; 5]};
    let mut temp = intial.clone();
    let mut allowed : Vec<minmaxAi::Displace> = Vec::new();
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

fn checkallowed(board : &minmaxAi::Board, endPos : &minmaxAi::Displace) -> minmaxAi::Displace{

    println!("test");

    let mut startPos = minmaxAi::Displace{peice : 'f', x  : 8, y : 8, ambigX : 8, ambigY : 8, moveStr : [' ';5]};
    let mut startVec : Vec<minmaxAi::Displace> = Vec::new();
    let player : u8;
    let openent : u8;
    let direc : i8;
    if board.player == false {player = 1; openent = 2;} else { player = 2; openent = 1;}

    if endPos.peice == 'p' {

     
        if board.player == false{direc = 1;}else{direc = -1;} //direc looking backtowards start pos
    
        if board.tile[endPos.x as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].peice == ' ' && board.tile[endPos.x as usize][(endPos.y as i8 + direc)as usize].colour == player{
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 + direc) as u8;
            startVec.push(startPos);
        } //move one foward
        
        if board.tile[endPos.x as usize][(endPos.y as i8 + 2 * direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].peice == ' '  &&
        board.tile[endPos.x as usize][(endPos.y as i8 + direc) as usize].peice == ' ' && board.tile[endPos.x as usize][(endPos.y as i8 + 2 *  direc)as usize].colour == player
        && ((endPos.y == 3 && player == 2) || (endPos.y == 4 && player == 1)){
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 + 2 *  direc) as u8;
            startVec.push(startPos);
        } //move two foward
       
        if ( if endPos.x < 7 { board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].colour == openent && board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].colour == player} else {false == true}){
            startPos.peice = 's'; startPos.x = endPos.x + 1; startPos.y = (endPos.y as i8 + direc) as u8;
            startVec.push(startPos);
        } //takes left
        
        if ( if endPos.x > 0 { board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].colour == openent && board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].colour == player} else {false == true}){
            startPos.peice = 's'; startPos.x = endPos.x - 1; startPos.y = (endPos.y as i8 + direc) as u8;
            startVec.push(startPos);
        } //take right

        if ( if endPos.x < 7 && board.History.len() > 0 { board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' && board.tile[endPos.x as usize][endPos.y as usize].peice == ' ' && board.History[board.History.len() - 1].peice == 'p'
        && board.History[board.History.len() - 1].x == endPos.x && board.History[board.History.len() - 1].y == (endPos.y as i8 + direc) as u8  && board.tile[(endPos.x + 1) as usize][(endPos.y as i8 + direc)as usize].colour == player}
        else {false == true}){

            startPos.peice = 'S'; startPos.x = endPos.x + 1; startPos.y = (endPos.y as i8 + direc) as u8;
            startVec.push(startPos);
        } //en possion left

        if ( if endPos.x > 0 && board.History.len() > 0 { board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].peice == 'p' &&
        (board.tile[endPos.x as usize][endPos.y as usize].peice == ' ' && board.History[board.History.len() - 1].peice == 'p' && board.History[board.History.len() - 1].x == endPos.x && board.History[board.History.len() - 1].y ==( endPos.y as i8 + direc ) as u8 ) && board.tile[(endPos.x -1) as usize][(endPos.y as i8 + direc)as usize].colour == player} else {false == true}){

            startPos.peice = 'S'; startPos.x = endPos.x - 1; startPos.y = (endPos.y as i8 + direc) as u8;
            startVec.push(startPos);
        } //en possion right
    }//pawn allowed

    else if endPos.peice == 'R' { startVec = checkQMB(&board, startVec, &endPos, player, 'R', true, true, true, true, false, false, false, false); }//rook allowed

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

    else if endPos.peice == 'B' { startVec = checkQMB(&board, startVec, &endPos, player, 'B', false, false, false, false, true, true, true, true); } //bishop
    else if endPos.peice == 'Q' { startVec = checkQMB(&board, startVec, &endPos, player, 'Q', true, true, true, true, true, true, true, true); } //queen
    else if endPos.peice == 'K' {
        if board.tile[endPos.x as usize][(endPos.y + 1) as usize].peice == 'K' && board.tile[endPos.x as usize][(endPos.y + 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 + 1) as u8;
            startVec.push(startPos);
        }
        else if board.tile[(endPos.x + 1) as usize][(endPos.y + 1) as usize].peice == 'K' && board.tile[(endPos.x + 1) as usize][(endPos.y + 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 + 1) as u8; startPos.y = (endPos.y as i8 + 1) as u8;
            startVec.push(startPos);
        }
        else if board.tile[(endPos.x + 1) as usize][endPos.y as usize].peice == 'K' && board.tile[(endPos.x + 1) as usize][endPos.y as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 + 1) as u8; startPos.y = endPos.y;
            startVec.push(startPos);
        }
        else if board.tile[(endPos.x + 1) as usize][(endPos.y - 1) as usize].peice == 'K' && board.tile[(endPos.x + 1) as usize][(endPos.y - 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 + 1) as u8; startPos.y = (endPos.y as i8 - 1) as u8;
            startVec.push(startPos);
        }
        else if board.tile[endPos.x as usize][(endPos.y - 1) as usize].peice == 'K' && board.tile[endPos.x as usize][(endPos.y - 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = endPos.x; startPos.y = (endPos.y as i8 - 1) as u8;
            startVec.push(startPos);
        }
        else if board.tile[(endPos.x - 1) as usize][(endPos.y - 1) as usize].peice == 'K' && board.tile[(endPos.x - 1) as usize][(endPos.y - 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 - 1) as u8; startPos.y = (endPos.y as i8 - 1) as u8;
            startVec.push(startPos);
        }
        else if board.tile[(endPos.x - 1) as usize][endPos.y as usize].peice == 'K' && board.tile[(endPos.x - 1) as usize][endPos.y as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 - 1) as u8; startPos.y = endPos.y;
            startVec.push(startPos);
        }
        else if board.tile[(endPos.x - 1) as usize][(endPos.y + 1) as usize].peice == 'K' && board.tile[(endPos.x - 1) as usize][(endPos.y + 1) as usize].colour == player {
            startPos.peice = 's'; startPos.x = (endPos.x as i8 - 1) as u8; startPos.y = (endPos.y as i8 + 1) as u8;
            startVec.push(startPos);
        }
    } // king does not detect ambuguate as only one king
    else {startPos.peice = 'f'}

    println!("test2");

    startPos = resolve_Ambig(&startVec, &endPos);

    return startPos;
}





//save board state to file
fn save(board : &minmaxAi::Board, mut endPos : minmaxAi::Displace, mut intial : minmaxAi::Displace) -> std::io::Result<()>{

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
    let mut board = minmaxAi::Board::new();
    let mut moveAccepted = true;
    let mut turn = minmaxAi::Displace {peice : ' ', x : 8, y : 8, ambigX : 8, ambigY : 8, moveStr : [' '; 5]};
    let mut end = minmaxAi::Displace {peice: ' ', x:8, y: 8, ambigX : 8, ambigY : 8, moveStr : [' '; 5]};
    let mut oldstate = board.clone();
    let mut history =  String::new();
    let mut colour = 1;
    let mut debug = false;

    let mut sphere = minmaxAi::nextTake(&board, debug);

    let mut check = minmaxAi::Check{white: false, black : false};

    while running { //main loop
        if moveAccepted == true{
            if board.player == true{board.player = false; }else{board.player = true;} //player
            board.print();

            check = minmaxAi::CheckDetc(&board, &sphere, false);

            print!("player: {} please make your move:  \n", if board.player == true { "white" }else {"black"}); //player turn
            if board.player == false {colour = 1;} else {colour = 2;}

            if (board.playerAi.BlackAi == true && colour == 1) || (board.playerAi.WhiteAi  == true && colour == 2){
                minmaxAi::AiCall(board.clone(), colour, check, sphere, debug); //Ai test function
            }
        }else{println!("please reenter your move"); moveAccepted = true;} //reinput turn if not allowed
        
        let mut play = String::new();
        io::stdin().read_line(&mut play).unwrap(); //input move
        play = play.trim().to_string();
       




        if play == "exit"{ break; }
        else if play == "debug" {debug = true; moveAccepted = false; println!("debug mode on"); turn.x =8; turn.y = 8;}
        else if play.len() == 2{ //checks if pawn as input 2 long
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

        }
        else{moveAccepted = false;}

        if turn.x != 8 && turn.y != 8{

        if board.tile[turn.x.clone() as usize][turn.y.clone() as usize].peice == 'K'{ //checks if king is being taken
            println!("cannot take the king");
            moveAccepted = false;
        }

        if moveAccepted == true { //runs peices check
            end = checkallowed(&board, &turn);
            if end.peice == 'f' {moveAccepted = false; println!("move not allowed");}
        }

        if moveAccepted == true { //moves the peices
            board = board.Swap(&turn, &end, false);

            sphere = minmaxAi::nextTake(&board, debug);
            check = minmaxAi::CheckDetc(&board, &sphere, false);

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
                println!("test {} history [ ln 560]", history);
                oldstate = board.clone(); //updates old state
                board.History.push(turn); //creates a history in PGN notation
            }
        }
        }else {println!("move failed ingnore if debug mode just turned on"); moveAccepted = false;}
    }
    println!("Hello, world!");
}
