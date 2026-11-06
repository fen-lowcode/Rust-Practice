


// Variables in rust are always immutable (cannot be changed)
#[allow(unused)]
fn immutable() {
    let x: i32 = 10;
    // x = 20 will not work
}

#[allow(unused)]
fn imutable() {
    let mut x: i32 = 10;
    x = 20; // This works
}

// -----------------

// variables in rust has lifetimes, the moment it is declared and the moment it is last used
#[allow(unused)]
fn lifetime() {
    let x: i32 = 10;
    println!("X value is {x}");
    let y = &x; // Y as a refence to y or called as "Borrowed"

    // x as a int bit 32 is used already, now i can change it's identity
    let x : String = String::from("Hello");     

    println!("X says {x}");
}

fn main() {
    lifetime();
}
