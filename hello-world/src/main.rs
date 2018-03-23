// Writing a plain main function
// println! is not a function, it is a macro. Macros use ! to distinguish them from normal method calls.
 fn main() {
    println!("Hello, world!");
}


//use std::env;
//use std::str;
// Main function takes no arguments and if you want to take params use std::env
//fn main() {
//    let args: Vec<_> = env::args().collect();
//    if args.len() > 1 {
//        println!("The first argument is {}", args[1]);
//    }
//}



// What if I can map over cli args to do something
//fn main() {
//    let args: Vec<_> = env::args().collect();
//    args.iter().map(|x| {("Argument: {}", x)});
//}
// On build notice the error. It seems like they are lazy https://doc.rust-lang.org/std/iter/trait.Iterator.html



// Okay call collect just like java8 to eval the lazy vector
// How are variables named in rust?
// https://doc.rust-lang.org/1.0.0/style/style/naming/README.html
//fn main() {
//    let args: Vec<String> = env::args().collect();
//    let argument_vector: Vec<_> = args.iter().map(|x| {("Argument: {}", x)}).collect();
//}



// A Rust String is like a std::string; it owns the memory and does the dirty job of managing memory.
// A Rust &str is like a char* (but a little more sophisticated); it points us to the beginning of a chunk in the same way you can get a pointer to the contents of std::string.
// str, only used as &str, is a string slice, a reference to a UTF-8 byte array.
// String is what used to be ~str, a growable, owned UTF-8 byte array.
//

// By default when "" are used it is String still it is good to be aware of str
//fn main() {
//    let a: Vec<&str> = vec!["hello", "bye"];
//    let argument_vector: Vec<String> = a.iter().map(|x| {str::to_string(x)}).collect();
//}

// Strings are default UTF-8
// A lot languages say that but they still allow blunders like using indexed string
//fn main(){
//    let s = "hello";
//    println!("The first letter of s is {}", s[0]);
//}

//fn main(){
//    let s = "hello";
//    println!("The first letter of s is {}", s.chars().nth(0).get_or_insert('').to_string());
//    println!("The first letter of s is {}", s.chars().next().unwrap());
//}

// In build message Finished dev [unoptimized + debuginfo] target(s)
// Cargo by default does not optimise the build because it is slow and you cannot
// run debugger on the optimised version.
// To enable debugging debug=true
// https://doc.rust-lang.org/cargo/reference/manifest.html


