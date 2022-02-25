use std::io::stdin;

fn what_is_your_name() -> String {
    // '-> String' determines the return type
    let mut your_name = String::new(); // declare variable with 'let' and make it mutable with 'mut'
    stdin()
        .read_line(&mut your_name) // take standard input, real_line to store it in 'your_name'
        .expect("Failed to readline");
    your_name // return 'your_name'
        .trim() // trim off hidden characters such as '\n'
        .to_lowercase() // convert to all lowercase
}
fn main() {
    println!("What is your name?"); // print line to standard output
    let name = what_is_your_name(); // declare 'name' and set its value equal to output of 'what_is_your_name'
    println!("Hello, {:?}", name); // print line to output
}
