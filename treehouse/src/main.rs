#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use std::io::stdin;
fn main() {
    let mut visitor_list = vec![
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steave. Your milk is in the fridge."),
        Visitor::new("jane", "Hello Jane, enjoy your day."),
    ];

    loop {
        println!("Hello, whats your name?");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(|visitor| visitor.name == name);

        if let Some(visitor) = known_visitor {
            visitor.greet_visitor();
        } else {
            if name.is_empty() {
                break;
            }
            println!("{name} is not on the visitor list.");

            visitor_list.push(Visitor::new(&name, "Welcome to the treehouse."));
        }
    }
    println!("The final list of visitors is: ");
    println!("{visitor_list:#?}");
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read your name");

    your_name.trim().to_lowercase()
}

#[derive(Debug)]
struct Visitor {
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
