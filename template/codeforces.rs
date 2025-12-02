#![allow(unused)]

fn solve(re: &mut Reader) {

}

fn main() {
    let mut s = String::new();
    let stdin = stdin();
    let mut re = Reader::new(&mut s, stdin);
    
    let t: usize = re.r();
    for _ in 0..t {
        solve(&mut re);
    }
}

