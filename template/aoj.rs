#![allow(unused)]

fn solve(re: &mut Reader) {

    true
}

fn main() {
    let mut s = String::new();
    let stdin = stdin();
    let mut re = Reader::new(&mut s, stdin);

    loop {
        let res = solve(&mut re);
        if !res {
            break;
        }
    }
}

