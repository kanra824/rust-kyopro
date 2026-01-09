#![allow(unused)]

fn solve(re: &mut Reader) {

}

fn main() {
    let stdin = stdin();
    let mut re = Reader::new(stdin);
    
    let t: usize = re.r();
    for _ in 0..t {
        solve(&mut re);
    }
}

