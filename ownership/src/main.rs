fn main() {
    println!("Hello, world!");
    foo();
}

// Why ownership is needed ?

// Variable bindings have a property in Rust: they ‘have ownership’ of what they’re bound to.
fn foo() {
    // Pointer for the vector is saved on the stack and the elements are stored on the heap
    let v = vec![1, 2, 3];
} // v we will be garbage collected when went out of scope.

// Major problem in garbage collecting in c/c++ is that using passing pointers to various parts
// of the project causes leaking of the scope.

fn foo1() {
    let v = vec![1, 2, 3];

    let v2 = v; // deep copy or shallow copy? shallow copy

    println!("v[0] is: {}", v[0]);
    // Compilation error : move occurs because `v` has type `std::vec::Vec<i32>`, which does not implement the `Copy` trait
    // What is Copy trait? Later
    // What moved? v moved, in ownership system if you assign your variable to another variable
    // or pass it to some function ownership transfer happens.
}

fn foo2 (){
    fn take(v: Vec<i32>) {
        // What happens here isn’t important.
    }

    let v = vec![1, 2, 3]; // notice it is created without mut. It is immutable
    // you cannot change the reference to the vector but sill you can push, pop elements

    take(v); // so elements in v might change when function is called

    println!("v[0] is: {}", v[0]); // move error?
}

// in summary if you assign your variable to other variable or pass it to some other function
// ownership is transferred

// What can we move?
fn foo3() {
    fn some_fn(a: i32){

    }
    let x = 5;
    some_fn(x);
    println!("{}", x);
} // This is will compile.
// Primitive types implement Copy trait which means assigning it to a variable
// use std::marker::Copy;
// or passing it to a function creates a Copy

// The ownership system is a zero-cost abstraction.
// All of the analysis we talked about are done at compile time.
// You do not pay any run-time cost for any of these features.


// How to give back ownership?

fn foo4() {
    fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
        // Do stuff with `v1` and `v2`.

        // Hand back ownership, and the result of our function.
        (v1, v2, 42)
    }

    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let (v1, v2, answer) = foo(v1, v2);
}