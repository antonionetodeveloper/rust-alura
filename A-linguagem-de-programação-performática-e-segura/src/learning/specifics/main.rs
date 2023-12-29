fn main() {
    hello_world();
}

fn hello_world() {
    println!("Hello there!");

    let name = "Antônio";
    {
        println!("Name: {}", name);

        let name = "Valéria";
        println!("New name: {}", name);
    }
}