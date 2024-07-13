use std::cmp::Ordering;

pub fn suffix_array(s: &Vec<char>) -> Vec<usize> {
    let mut ma = 2000000;
    let n = s.len();
    let mut sa = vec![0; n + 1];
    let mut rank = vec![0; ma];
    let mut tmp = vec![0; ma];

    for i in 0..=n {
        sa[i] = i;
        if i < n {
            rank[i] = s[i] as i32;
        } else {
            rank[i] = -1;
        }
    }

    let mut k = 1;
    while k <= n {
        sa.sort_by(|&i, &j| {
            if (rank[i] != rank[j]) {
                rank[i].cmp(&rank[j])
            } else {
                let ri = if i + k <= n { rank[i + k] } else { -1 };
                let rj = if j + k <= n { rank[j + k] } else { -1 };
                ri.cmp(&rj)
            }
        });

        tmp[sa[0]] = 0;
        for i in 1..=n {

            let order = if (rank[sa[i-1]] != rank[sa[i]]) {
                rank[sa[i-1]].cmp(&rank[sa[i]])
            } else {
                let ri = if sa[i-1] + k <= n { rank[sa[i-1] + k] } else { -1 };
                let rj = if sa[i] + k <= n { rank[sa[i] + k] } else { -1 };
                ri.cmp(&rj)
            };
            let add = if let Ordering::Less = order {
                1
            } else {
                0
            };
            tmp[sa[i]] = tmp[sa[i - 1]] + add;
        }
        for i in 0..=n {
            rank[i] = tmp[i];
        }

        k *= 2;
    }

    sa
}
