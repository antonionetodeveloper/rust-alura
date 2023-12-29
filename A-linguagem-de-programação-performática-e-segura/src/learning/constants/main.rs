const PI:f32  = 3.14; // Bro...
static mut GLOBAL:u8 = 1;

fn main() {
    println!("PI = {}", PI);
    
    unsafe {
        println!("Global mutable 'variable' = {}", GLOBAL);
    }
}