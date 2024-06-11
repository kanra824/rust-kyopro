pub fn factorization(mut n: i64) -> Vec<(i64, i64)> {
    let mut res = vec![];
    let mut i = 2;
    while i * i <= n {
        println!("{} {}", i, n);
        let mut cnt = 0;
        while n % i == 0 {
            cnt += 1;
            n /= i;
        }
        if cnt > 0 {
            res.push((i, cnt));
        }
        i += 1;
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}
