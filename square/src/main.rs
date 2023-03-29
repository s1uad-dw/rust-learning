fn main() {
    println!("Enter the length of the side of the square:");
    let mut input_number:String = String::new();
    std::io::stdin().read_line(&mut input_number)
        .expect("faied of read line");
    let input_number: u32 = input_number.trim().parse()
        .expect("type a number!");
    let result = square(input_number);
    println!("More info about your square:\n\t perimeter - {}\n\t area - {}\n\t diagonal - {}", result.0, result.1, result.2);
}

fn square(a: u32) -> (u32, u32, f32) {
    let p = a*4;
    let s = a*a;
    let d = ((a*a+a*a) as f32).sqrt();
    return(p,s,d);
}