fn main() {
    let mut s = "Hello".to_string();
    let t = s;
    s = "World".to_string();
    println!("s : {}, t: {}", s, t);

    let mut v = Vec::new();
    for i in 101..106 {
        v.push(i.to_string());
    }

    /* ------------------------- */
    let first = &v[1];
    println!("first: {}", first);

    /* -------------------------- */
    /// this is panic!
    /// まず、vがタプルである
    /// 次に、forのsはVec<String>の要素を所有するString型の可変束縛だが、 String 型の値そのものを所有しているため、その要素は可変ではない。
    // let v = vec![
    //     "text1".to_string(),
    //     "text2".to_string(),
    //     "text3".to_string(),
    // ];
    // for s in v {
    //     s.push_str("!");
    //     println!("{}", s);
    // }

    /// 変更後
    let mut v = vec![
        "text1".to_string(),
        "text2".to_string(),
        "text3".to_string(),
    ];
    for s in &mut v {
        s.push_str("!");
        println!("{}", s);
    }
}
