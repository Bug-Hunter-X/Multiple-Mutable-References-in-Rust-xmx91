fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y; // z is an immutable reference to y
    *y = 10; // Modifying x through y is fine
    println!("{}", *y); // Prints 10
    println!("{}", *z); // Prints 10

    let mut a = 5;
    let b = &mut a; // b is a mutable reference to a
    let c = &mut a; // c is also a mutable reference to a - this is allowed
    *b = 10; // Modifying a through b
    *c = 12; // Modifying a through c - this is UB. The compiler can't guarantee which modification comes first.
    println!("{}, {}", *b, *c); //The output is indeterminate.
}