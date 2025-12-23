use crate::library::number::mint::*;
use crate::library::fps::{bostan_mori::*, fps::Fps};

#[test]
fn test_bostan_mori() {
    let p = Fps::from_i64_vec(vec![1]);
    let q = Fps::from_i64_vec(vec![1, -1, -1]);
    let res = bostan_mori(10, p.clone(), q.clone());
    assert_eq!(res.x, 89);
}

#[test]
fn test_bostan_mori_from_linear_recurrence_relation() {
    let c = vec![1, 2];
    let a = vec![3, 1];

    // a_n = c_0 * a_n-2 + c_1 * a_n-1
    // a_n = 1 * a_n-2 + 2 * a_n-1
    // a_0 = 3
    // a_1 = 1
    // a_2 = 5
    // a_3 = 11
    // a_4 = 27
    // a_5 = 65
    
    let res = bostan_mori_from_linear_recurrence_relation(5, c.clone(), a.clone());
    assert_eq!(res.x, 65);
}