#![allow(unused)]

fn solve(re: &mut Reader) {

    true
}

fn main() {
    let stdin = stdin();
    let mut re = Reader::new(stdin);

    loop {
        let res = solve(&mut re);
        if !res {
            break;
        }
    }
}

