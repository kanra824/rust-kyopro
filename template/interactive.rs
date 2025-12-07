#![allow(unused)]

fn main() {
    // let mut s = String::new();
    // let stdin = stdin();
    // let mut re = Reader::new(&mut s, stdin);
    
}

fn take_string() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

