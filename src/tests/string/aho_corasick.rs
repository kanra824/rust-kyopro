use crate::library::string::aho_corasick::*;

#[test]
fn use_aho_corasick() {
    let patterns = vec![
        vec![0, 1, 2],
        vec![2, 3, 4],
    ];
    let mut ahocora = AhoCorasick::new(&patterns);

    eprintln!("{:?}", ahocora);
    eprintln!("{:?}", ahocora.query(&vec![0, 1, 2, 3, 4]));
}