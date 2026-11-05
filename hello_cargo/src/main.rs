

// fn add(x:&mut i32, y:&mut i32, z:&mut i32){
//     *z = *x + *y;
// }

// fn do_something(text:&mut String) {
//     println!("Doing something with a {text}");
//     text.push_str("now");
// }

fn main() {
    let mut x = 0;
    let mut y  = &mut x;
    let z= &mut y;
    *y = *y + 1;

    println!("Value {x}");
}
