pub use main;
pub mod dataStructs;

fn scores(peice : char) -> i16 {
    match peice{
        'p' => return 1,
        'r' => return 50,
        'n' => return 25,
        'b' => return 30,
        'q' => return 100,
        'k' => return 255,
        _ => return 0,
    }
}

pub fn calcScore(board : dataStructs::Board, colour : u8, check : dataStructs::Check, sphere : [[dataStructs::Sphere;8];8]){
    let mut openent : u8;
    if  colour == 1 {openent = 2;} else {openent = 1;}
    
    let mut gain : [[i16; 8]; 8] = [[0;8];8];
    let mut risk : [[i16; 8]; 8] = [[0;8];8];

    //risk current
    for j in 0..8{
        for i in 0..8 {
            if board.tile[j][i].colour == colour{gain[i][j] += scores(board.tile[i][j].peice);}
            if board.tile[j][i].colour == openent{risk[i][j] += scores(board.tile[i][j].peice);}

            print!("{}", risk[i][j]);
        }
        println!("");
    }

    
}
