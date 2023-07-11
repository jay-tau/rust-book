fn main() {
    let mut x: Box<i32> = Box::new(1); // Create a box
    let _a: i32 = *x; // Dereference the box
    *x += 1; // Increase the value inside the box to 2

    let r1: &Box<i32> = &x; // Point to x on the stack
    let _b: i32 = **r1; // Dereference the box twice to reach the data on the heap

    let r2: &i32 = &*x; // Directly point to the address of x on the heap
    let _c: i32 = *r2; // Dereference x on the heap
}
