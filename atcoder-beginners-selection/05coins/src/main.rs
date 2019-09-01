use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();

    let mut ite = stdin.lock().lines();
    let c500 = ite.next().unwrap().unwrap().parse::<i32>().unwrap();
    let c100 = ite.next().unwrap().unwrap().parse::<i32>().unwrap();
    let c50 = ite.next().unwrap().unwrap().parse::<i32>().unwrap();
    let expected = ite.next().unwrap().unwrap().parse::<i32>().unwrap();

    println!(
        "500:{}, 100:{},50:{}, expected:{}",
        c500, c100, c50, expected
    );

    let mut matches = 0;

    let max500 = raise(expected, 500, c500);
    for n500 in (0..=max500).rev() {
        let total = totalize(n500, 0, 0);
        let shortage = expected - total;
        println!("{}, total:{}, shortage:{}", n500, total, shortage);
        if shortage == 0 {
            matches += 1;
            continue;
        }

        let max100 = raise(shortage, 100, c100);
        for n100 in (0..=max100).rev() {
            let total = totalize(n500, n100, 0);
            let shortage = expected - total;
            println!("{}, {}, total:{}, shortage:{}", n500, n100, total, shortage);
            if shortage == 0 {
                matches += 1;
                continue;
            }

            let max50 = raise(shortage, 50, c50);
            for n50 in (0..=max50).rev() {
                let total = totalize(n500, n100, n50);
                let shortage = expected - total;
                println!(
                    "{}, {}, {}, total:{}, shortage:{}",
                    n500, n100, n50, total, shortage
                );
                if shortage == 0 {
                    matches += 1;
                    continue;
                }
            }
        }
    }
    println!("{}", matches);
}

// 合計金額を算出する
// n500: 500円硬貨の枚数
// n100: 100円硬貨の枚数
// n50: 50円硬貨の枚数
fn totalize(n500: i32, n100: i32, n50: i32) -> i32 {
    500 * n500 + 100 * n100 + 50 * n50
}

// ceil: 求めたい上限値
// unit: 硬貨の単価
// max: 所持している硬貨の枚数
fn raise(ceil: i32, unit: i32, max: i32) -> i32 {
    for x in 0..=max {
        let total = unit * x;
        if ceil == total {
            return x;
        } else if ceil < total {
            return x - 1;
        }
    }
    0
}
