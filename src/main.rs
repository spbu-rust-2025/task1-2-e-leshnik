use std::io;

fn main() {
    let mut sum = 0;
    let mut input = String::new();
    
    while let Ok(_) = io::stdin().read_line(&mut input) {
        let trimmed = input.trim();
        if trimmed == "-1" { break; }
        
        match trimmed.parse::<u32>() {
            Ok(num) => sum += num,
            Err(_) => {
                println!("NaN");
                return;
            }
        }
        input.clear();
    }
    
    println!("{}", sum);
}
