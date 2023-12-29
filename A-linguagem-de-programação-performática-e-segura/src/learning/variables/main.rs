fn main() {
    let variable1:u8 = 128;
    println!("My first variable contains: {}", variable1);

    let name:&str = "Antônio Fernandes De Santana Neto";
    println!("And my name is {}", name);
    println!("My name contains {} bytes!", std::mem::size_of_val(name));
}