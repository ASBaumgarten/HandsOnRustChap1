use std::io::stdin;

fn main() {
    println!("What is your name?"); // print line to standard output
    let name = what_is_your_name(); // declare 'name' and set its value equal to output of 'what_is_your_name'
                                    // old visitor list array
                                    // let visitor_list: [&str; 3] = ["bert", "alex", "steve"]; // declare array with names
    let mut allow = false; // create `allow' boolean set to false
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse"),
        Visitor::new("steve", "Hi steve, and welcome"),
        Visitor::new("Alex", "Hello you handsome bastard"),
    ];
    let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

    // old visitor list for loop
    /*     for visitor in &visitor_list {
        // goes through the contents of 'visitor_list'
        if visitor == &name {
            allow = true;
        }
    } */

    /* if allow == true {
        println!("Welcome {}", name);
    } else {
        println!("Get lost {}", name);
    } */
}

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

struct Visitor {
    // Create struct named Visitor
    name: String,
    greeting: String,
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
        }
    }
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}
