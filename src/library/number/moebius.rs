fn moebius_table(n: usize) -> Vec<i64> {
    let mut mu = vec![1; n+1];
    let mut p = vec![1; n + 1];
    let mut i = 2;
    while i <= n {
        if p[i] == 1 {
            let mut j = i;
            while j <= n {
                p[j] = i;
                j += i;
            }
        }
        if ((i / p[i]) % p[i] == 0) {
            mu[i] = 0;
        } else {
            mu[i] = -mu[i / p[i]];
        }
        i += 1;
    }

    mu
}
