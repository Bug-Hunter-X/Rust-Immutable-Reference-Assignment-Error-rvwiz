fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y += 1; // this is ok
    let z = &x; // z is an immutable reference to x
    *z += 1; // Error! cannot assign to immutable  reference 
}