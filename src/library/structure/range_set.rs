#[derive(Debug, Clone)]
struct RangeSet {
    st: BTreeSet<(i64, i64)>
}

use std::collections::BTreeSet;

impl RangeSet {

    // [l, r) の区間を追加
    // 増えた要素の個数を返す
    fn add(&mut self, l: i64, r: i64) -> i64 {
        // l以上の左端

        let mut nl = l;
        let mut nr = r;

        let mut diff = 0;

        // 左側
        loop {
            let res = self.st.range(..(nl, 0)).next_back();
            match res {
                None => {
                    break;
                },
                Some(&(nowl, nowr)) => {
                    // println!("l: {} {}", nowl, nowr);
                    if nowr < nl {
                        break;
                    } else if nl <= nowr && nowr <= nr {
                        diff += nowr - nowl;
                        self.st.remove(&(nowl, nowr));
                        nl = nowl;
                    } else {
                        diff += nowr - nowl;
                        self.st.remove(&(nowl, nowr));
                        nl = nowl;
                        nr = nowr;
                    }
                }
            }
        }

        // 右側
        loop {
            let res = self.st.range((nl, 0)..).next();
            match res {
                None => {
                    break;
                },
                Some(&(nowl, nowr)) => {
                    // println!("r: {} {}", nowl, nowr);
                    if nr <= nowl {
                        break;
                    } else if nr <= nowr {
                        diff += nowr - nowl;
                        self.st.remove(&(nowl, nowr));
                        nr = nowr;
                    } else {
                        diff += nowr - nowl;
                        self.st.remove(&(nowl, nowr));
                    }
                }
            }
        }

        // println!("res: {} {}", nl, nr);
        diff -= nr - nl;
        self.st.insert((nl, nr));
    diff
}
}