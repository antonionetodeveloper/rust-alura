fn main() {
    let num: f32 = 1.0;
    let value: &str = match num {
        3.14 => "pi",
        2.71 => "e",
        _ => "unknown",

    };

    println!("{} -> {}", num, value);
}
