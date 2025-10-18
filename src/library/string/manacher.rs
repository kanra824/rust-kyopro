pub fn manacher<T>(s: &Vec<T>) -> Vec<usize>
where T: PartialEq + Eq,
{
    let mut i = 0;
    let mut j = 0;
    let mut res = vec![0; s.len()];
    while i < s.len() {
        while i - j >= 0 && i + j < s.len() && s[i-j] == s[i+j] {
            j += 1;
        }
        r[i] = j;
        let mut k = 1;
        while i - k >= 0 && k + r[i-k] < j {
            r[i+k] = r[i-k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    res
}