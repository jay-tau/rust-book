fn return_a_string(output: &mut String) {
    output.replace_range(.., "Hello world");
}
fn main() {
    let mut fn_output: String = String::from("");
    return_a_string(&mut fn_output); // Similar to how file descriptors are recieved in C
}
