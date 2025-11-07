pub fn z_algorithm(s: &Vec<char>) -> Vec<usize> {
    let n = s.len();
    let mut res = vec![0; n];
    res[0] = s.len();
    let mut i = 1;
    let mut j = 0;
    while i < s.len() {
        while i + j < s.len() && s[j] == s[i+j] {
            j += 1;
        }

        res[i] = j;
        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while i + k < s.len() && k + res[k] < j {
            res[i+k] = res[k];
            k += 1;
        }

        i += k;
        j -= k;
    }

    res
}