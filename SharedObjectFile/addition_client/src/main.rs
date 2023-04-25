extern crate addition_library;
use addition_library::add;

fn main() {
    println!("");
    println!("--------------------------------------------------");
    println!("   .so file , The DLL of LINUX.");
    println!("");
    println!("   Calling Function from Shared Object (.so) file.");
    println!("");
    let result = add(2,2);
    println!("   Addition is : {}", result);
    println!("--------------------------------------------------");

}
