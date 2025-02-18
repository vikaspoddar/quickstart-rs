pub fn iterator_demo() {
    let v1 = vec![1, 2, 3, 4, 5];
    let v2: Vec<i32> = v1.into_iter().filter(|&x| x != 2).collect();
    println!("{:?}", v2);
}