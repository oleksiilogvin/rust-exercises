fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    inspect(&arg);
    change(&mut arg);
    println!("I have many {}", arg);
    if eat(arg) {
        println!("Might be bananas");
    } else {
        println!("Not bananas");
    }
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}

fn bedazzle(p0: &mut String) {
    *p0 = "sparkly".to_string()
}

fn eat(p0: String) -> bool {
    p0.starts_with("b") && p0.contains("a")
}

fn change(p0: &mut String) {
    if !p0.ends_with("s") {
        p0.push_str("s");
    }
}

fn inspect(arg: &String) {
    if arg.ends_with("s") {
        println!("{} is plural", arg);
    } else {
        println!("{} is singular", arg);
    }
}
