/// Absolute value of a number
/// 型を完全に推定した後に、abs()等の関数を呼び出す必要がある
/// つまり、 -4.abs()はエラーになる。(-4).abs()は正しい。
/// 注意: 「1:」の出力は-4である。これはメソッドの呼び出しの優先順位が単位前置演算子よりも高いためである。
fn a() {
    println!("{}", -4_i32.abs()); // ...1: -4
    println!("{}", (-4_i32).abs()); // ...2: 4
    println!("{}", i32::abs(-4)); // ...3: 4
}

fn b() {
    let mut primes = vec![2, 3, 5, 7];
    println!("{}", primes.iter().product::<i32>()) // 210
}

/// Vec::with_capacity()は、指定された要素数を格納するためのメモリを確保。
/// capacity はベクターが現在保持できる要素数の最大値を示す。
/// 以下の例で3つ目の要素を追加する際には、panic!が発生することはない。
/// これは、要素数が容量に達した場合に自動的に内部の配列を拡張するため。
fn c() {
    let mut v = Vec::with_capacity(2); // 定義した段階での要素数を指定
    v.push(1);
    v.push(2);
    v.push(3); // not panic!
    println!("{:?}", v);
}

fn main() {
    // a();
    // b();
    // c()
}

/// 遊ぶよう

/// コマンド: cargo run Lips Scheme C C++
fn lang_bool() {
    let languages: Vec<String> = std::env::args().skip(1).collect();
    for l in languages {
        println!(
            "{}: {}",
            l,
            if l.len() % 2 == 0 {
                "functional"
            } else {
                "imperative"
            }
        );
    }
}
