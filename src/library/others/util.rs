// 部分集合列挙
mod subset {
    fn enumerate(s: u32) {
        let mut t = s;
        while t != 0 {
            // 処理
            t = (t - 1) & s;
        }
    }
}