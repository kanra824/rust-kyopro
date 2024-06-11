use crate::library::structure::union_find::*;
use std::collections::btree_set::Union;

use super::*;
fn get_uf() -> UnionFind {
    let n = 10;
    let mut uf = UnionFind::new(n);

    uf.unite(1, 2);
    uf.unite(3, 4);
    uf.unite(2, 3);
    uf.unite(5, 6);
    uf.unite(7, 9);
    uf.unite(6, 7);
    uf
}

#[test]
fn test_uf_get_sz() {
    let mut uf = get_uf();
    assert_eq!(uf.get_sz(1), 4);
    assert_eq!(uf.get_sz(5), 4);
    assert_eq!(uf.get_sz(8), 1);
}

#[test]
fn test_uf_count() {
    let mut uf = get_uf();
    assert_eq!(uf.count(), 4);
}

#[test]
fn test_uf_unite() {
    let mut uf = get_uf();

    assert!(uf.same(1, 2));
    assert!(!uf.same(1, 5));
    assert!(uf.same(6, 7));
    assert!(uf.same(1, 1));
}
