#[allow(dead_code)]
fn print_matrix<T: std::fmt::Display>(vec: &[T]) {
    for i in 0..8 {
        for j in 0..8 {
            print!("{:03} ", vec[i * 8 + j]);
        }
        print!("\n")
    }
}


fn main() {
    println!("Hello, world!");
}
