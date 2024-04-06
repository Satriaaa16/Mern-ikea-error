use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Tebak angka ");
   

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("angka tersembunyi adalah:{}", secret_number);
    
    loop{
        println!("masukan angka tebakan mu :) = ");
   
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect( "failde to read line");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_) => continue,
        };
    
        println!("kamu menebak : {}", guess);
    
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("terlalu kecil"),
            Ordering::Greater=> println!("kelebihan"),
            Ordering::Equal => {
                println!("selamat anda menang");
                break;
            },
        }
    }
}