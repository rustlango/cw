// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    let answer = square(3);
    println!("The square of 3 is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num;
    // return expression needs to be speicified
    // can do it expliclity to like return num
    num
}
