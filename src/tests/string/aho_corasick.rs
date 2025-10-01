use crate::library::string::aho_corasick::*;

#[test]
fn use_aho_corasick() {
    let patterns = vec![
        vec![0, 1, 2],
        vec![2, 3, 4],
    ];
    let ahocora = AhoCorasick::new(&patterns);

    println!("{:?}", ahocora);
    println!("{:?}", ahocora.query(&vec![0, 1, 2, 3, 4]));
}