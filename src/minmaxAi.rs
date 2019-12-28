mod main;

fn scores(peice : char) -> u8 {
    match peice{
        'p' => return 10,
        'r' => return 500,
        'n' => return 250,
        'b' => return 300,
        'q' => return 1000,
        'k' => return 10000,
    }
}


fn calcScore(baord : &Board, colour : u8, check : Check, sphere : [[Sphere;8];8]){
    let mut openent : u8;
    if  colour == 1 {openent = 2;} else {openent = 1;}
    
    let mut gain = [[i8; 8]; 8];
    let mut risk = [[i8; 8]; 8];

    //risk current
    for j in 0..8{
        for i in 0..8 {
            if board.tile[j][i].colour == colour{gain += scores(board.tile[i][j].colour);}
            if board.tile[j][i].colour == openent{risk += scores(board.tile[i][j].colour);}

            print!("{}", risk);
        }
        println("");
    }

}

