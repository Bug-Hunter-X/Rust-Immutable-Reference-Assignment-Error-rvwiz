fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    *y += 1; // this is ok
    let mut z = &mut x; // z is a mutable reference to x
    *z += 1; // this is ok now
    println!("x = {}", x);
}