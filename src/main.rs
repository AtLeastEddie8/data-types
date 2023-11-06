use std::io;

fn main() {
    {
        //guess in this context needs a data type otherwise error
        let guess: i32 = "42".parse().expect("Not a number!");
        println!("The guess is: {guess}");
        let num1 = 5;
        let num2 = 10;
        //this will overflow bc num3 is unsigned
        //let num3: u32 = num1 - num2;
        let num3 = num1 - num2;
        println!("num1 - num2 = {num3}");
    }
    //Numeric Operations
    {
        // addition
        let sum = 5 + 10;
        println!("sum = {sum}");
        // subtraction
        let difference = 95.5 - 4.3;
        println!("difference = {difference}");
        // multiplication
        let product = 4 * 30;
        println!("product = {product}");
        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1
        println!("quotient = {quotient}, truncated = {truncated}");
        // remainder
        let remainder = 43 % 5;
        println!("remainder = {remainder}");
    }
    //Floating-Point Types
    {
        let x = 2.0; // f64

        let y: f32 = 3.0; // f32
        println!("x = {x}, y = {y}");
    }
    //The Boolean Type
    {
        let t = true;

        let f: bool = false; // with explicit type annotation
        println!("t = {t}, f = {f}");
    }
    //The Character Type
    {
        let c = 'z';
        let z: char = 'ℤ'; // with explicit type annotation
        let heart_eyed_cat = '😻';
        println!("c = {c}, z = {z}, heart_eyed_cat = {heart_eyed_cat}");
    }
    //Compound Types
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
        //str did not work but char did 
        let tup2: (i32, u32, char) = (99, 23, '!');
        let (a, b, c) = tup2;

        println!("i32 = {x}, f64 = {y}, u8 = {z}");
        println!("tup2 = {a}, {b}, {c} ");
        //cant find a way to cycle through a tup using a loop without having the size defined 
        //can do with indexing such as in arrays like tup2.0
    }
    //The Array Type
    {
        let a = [1, 2, 3, 4, 5];
        println!("a = {:?}", a);
        let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
        let b = [3; 6];

        let a = [8;5];
        println!("months = {:?}", months);
        println!("a = {:?}", a);
        println!("b = {:?}", b);
    }
    // ... Invalid Array Element Access
    {
        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];
        //panics if the index is greater than the size of the array
        println!("The value of the element at index {index} is: {element}");
        }
}
