## 変数、整数、シャドーイング、型

デフォが定数

```rust
// 定数
let x: i32 = 1;
const A: i32 = 100; // 多分環境変数とか

// 変数
let mut x: i32 = 5;
    println!("The value of x is: {}", x); //5
    x = 6;
    println!("The value of x is: {}", x); //6

```

## シャドーイング

ブロックという概念があり、`{}`内でスコープが分離している。スコープが違えば同じ名前の変数を宣言できる
ブロック内の変数はブロック内のみで使用可能

```rust
fn main() {
    let y: i32 = 6;
    {
        let y: i32 = y + 1;
        println!("The value of y is: {}", y); //7
    }
    println!("The value of y is: {}", y); //6
}
```

`Before`はエラーになる
-> 変数に再代入する場合、型を揃えなければいけない

`After`では定数として`some_strings`を再宣言しているので型を新しくしても OK

```rust
// Before
fn main() {
    let mut some_strings: &str = "aaa";
    println!("The value of: {}", some_strings); // aaa

    let some_strings: usize = some_strings.len();
    println!("The value of: {}", some_strings); // 3
}

// After
fn main() {
    let some_strings: &str = "aaa";
    println!("The value of: {}", some_strings); // aaa

    let some_strings: usize = some_strings.len();
    println!("The value of: {}", some_strings); // 3
}
```
