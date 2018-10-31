fn five() -> i32 { // declare return value type after "->"
    5
} 
fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
