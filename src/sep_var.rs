// Fix the error below with least amount of modification
fn main() {
    let (mut x, y) = (1, 2);
    
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!");
}


// 2nd approach
// fn main() {
//     let (mut x, ..) = (3, 2);
//     let (.., y) = (1, 2);
//     x += 2;

//     assert_eq!(x, 5);
//     assert_eq!(y, 2);

//     println!("Success!");
// }