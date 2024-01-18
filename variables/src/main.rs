fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    let x = "6";
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {}", MAX_POINTS);

    let f: u8 = 255;
    println!("The value of f is: {}", f);

    let tup = (500, 120, "a");
    
    println!("The value of tup is: {:?}", tup);
    let x = tup.0;
    let y = tup.1;
    let z = tup.2;

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {:?}", a);

    let b = [3; 5];
    println!("The value of b is: {:?}", b);

    let sum = my_function(2,5);
    println!("The value of sum is: {}", sum);

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("The value of a is: {:?}", a);

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("the value is: {}", number);
    
    }

    {
        let s = 65;
        println!("The value of s is: {}", s);
    }

    // println!("The value of s is: {}", s);

}

fn my_function(x:i32, y:i32) -> i32{
    // println!("The value of x is: {}", x);
    // println!("The value of y is: {}", y);

    x+y
}