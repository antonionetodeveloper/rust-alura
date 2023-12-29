fn main() {
    let mut counter: i32 = 1;
    loop {
        for i in 1..11 {
            println!("{} x {} = {}", counter, i, counter*i)
        }
        counter += 1;

        if counter > 10 {
            break;
        }
    }
}
