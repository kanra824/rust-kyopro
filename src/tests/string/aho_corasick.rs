use crate::library::string::aho_corasick::*;

#[test]
fn use_aho_corasick() {
    let patterns = vec![
        vec![0, 1, 2],
        // vec![3, 4, 5],
    ];
    let ahocora = AhoCorasick::new(&patterns);

    println!("{:?}", ahocora);


}