use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub struct Peice{
    id : i32,
    king : i32,
    queen : i32,
    knight : i32,
    bishop : i32,
    rook : i32,
    pawn : i32,
}
impl Peice{
    fn new() -> Peice{
        let mut peice = Peice{id : 48, king: 49, queen : 49, knight : 49, bishop : 49, rook : 49, pawn : 49};
        return peice;
    }
}

pub fn main() {
    let peice = Peice::new();
    if Path::new("data.txt").exists() == false{
        println!("file store created");
        write(peice);
    }
    
    read();
    

    
}

pub fn write(mut peice : Peice) -> std::io::Result<()>{
    let mut file = File::create("data.txt")?;
    
    for _i in  0..64{
        //let strin = {"Hello, world! {}{}{}{}{}{}{}", peice.id, peice.king, peice.queen, peice.knight, peice.bishop, peice.rook, peice.pawn};
        write!(file, "{}{}{}{}{}{}{}\n", (peice.id as u8) as char, (peice.king as u8) as char, (peice.queen as u8) as char, (peice.knight as u8) as char, 
            (peice.bishop as u8) as char, (peice.rook as u8) as char, (peice.pawn as u8) as char)?;
        peice.id += 1;  
    }
        
    Ok(())
}//creates a file if it dose not exist

pub fn read() -> std::io::Result<()>{
    let mut file = File::open("data.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(())

}
