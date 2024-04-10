// Ownership rules
// 1. Each value in rust has a variable that's called its owner.
// 2. there can only be one owner at a time.
// 3. when the owener goes out of scope, the value will be dropped.


pub fn run() {
    { // s is not valid here, it's not yet declared
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // this scope is now over, and s is no longer valid

    main();

    let get_ownership = gives_ownership();
    let take_and_get_ownership = takes_and_gives_back(String::from("HEllloooo"));

}

fn main() {
    let x: i32 = 5;
    let y: i32 = x; // Copy

    let s1 = String::from("Hello");
    let s2 = s1; // Move (not shallow copy) 

    println!("{}, world!", s2);
}

fn gives_ownership() -> String {
    let returned_string = String::from("Hell");
    returned_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
