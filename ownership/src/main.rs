fn main() {

    // Scope of variables is within here
    {
        let s: &str = "Hello S";
        println!("{}", s);

        let t: String = String::from("Hello T");
        println!("{}", t);
    }

    // s and t are out of scope here, memory is deallocated

    // memory and allocation
    {

        let x: i32 = 5;
        let y: i32 = x;

        println!("x = {}, y = {}", x, y);

        let s1: String = String::from("Hello S1");
        // let s2: String = s1; // move, s1 is invalid
        let s2: String = s1.clone(); // deep copy

        println!("s1 = {}", s1); 
        println!("s2 = {}", s2);    

        // defaults to move, but can be overridden with clone
    }

    // references

    {
        let s1: String = String::from("Hello S1");
        let len: usize = calculate_length(&s1);

        println!("The length of '{}' is {}.", s1, len);

        fn calculate_length(s: &String) -> usize {
            s.len()
        }

        // amperstand is a reference, allows you to refer to some value without taking ownership of it
    }

    // mutable references

    {
        let mut s: String = String::from("Hello");
        change(&mut s);

        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }

        println!("{}", s);
    }

    // mutable references have one big restriction: you can have only one mutable reference to a particular piece of data in a particular scope 

    
    {
        let s: String = String::from("Hello");
        let r1: &String = &s;
        let r2: &String = &s; 
        let r3: &String = &s;
        let r4: &String = &s;

        println!("{}, {}, {}, and {}.", r1, r2, r3, r4);
    }

    // immutable references are allowed to have multiple references to the same data
}
