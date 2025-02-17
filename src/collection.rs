/// This the demo for some common collection in rust 
pub fn vector_demo() {
    let vector1 = [1, 2, 3];
    println!("{:?}", vector1);
    let mut vector2: Vec<i32> = Vec::new();
    vector2.push(5);
    vector2.push(6);
    vector2.push(7);
    vector2.push(8);
    println!("{}", vector2[0]);
}
