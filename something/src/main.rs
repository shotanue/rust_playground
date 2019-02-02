fn main() {
    let mut v = vec![1, 2, 3];
    v.push(1);
    v.push(2);

    let mut xs = Vec::new();

    // 各要素に1を足す
    for x in &v {
        let x = x + 1;
        xs.push(x);
    }
    dbg!(xs);
}