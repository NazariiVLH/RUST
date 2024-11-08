
fn main() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x as u32;  

    let z = 10; 

    println!("Success!");
}



fn main() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}


fn main() {
    let x = 5u32; // Declare x as a u32 type
    assert_eq!("u32".to_string(), type_of(&x));

    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn main() {
    assert_eq!(i8::MAX, 127);  
    assert_eq!(u8::MAX, 255);  

    println!("Success!");
}

fn main() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1579);

    println!("Success!");
}

fn main() {
    let x = 1_000.000_1_f64; 
    let y: f32 = 0.12; 
    let z = 0.01_f64; 

    assert_eq!(type_of(&x), "f64".to_string()); 
    println!("Success!");
}

fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

fn main() {
    assert!((0.1_f64 + 0.2_f64 - 0.3_f64).abs() < 1e-9);

    println!("Success!");
}

use std::ops::{Range, RangeInclusive};

fn main() {
    assert_eq!((1..5), Range { start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));

    println!("Success!");
}

fn main() {
   
    assert!(1u32 + 2 == 3);

    
    assert!(1i32 - 2 == -1); 
    assert!(1u8.wrapping_sub(2) == 255);  
    
    assert!(3 * 50 == 150);

   
    assert!((9.6 / 3.2 - 3.0).abs() < 1e-9);  
    assert!(24 % 5 == 4);

   
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}


2.
use std::mem::size_of_val;

fn main() {
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);  

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);  

    println!("Success!");
}

fn main() {
    let _f: bool = false;

    let t = true;
    if t {  
        println!("Success!");
    }
}

fn main() {
    let _f: bool = false;

    let t = true;
    if t {  
        println!("Success!");
    }
}

fn main() {
    let f = true;
    let t = true && true; 
    assert_eq!(t, f);

    println!("Success!");
}

fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());  
    println!("Success!");
}

fn implicitly_ret_unit() -> (i32, i32) {
    println!("I will return a ()");
    (2, 3)  
}


fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

3

fn main() {
   let v = {
       let mut x = 1;
       x += 2;
       x  
   };

   assert_eq!(v, 3);

   println!("Success!");
}

fn main() {
   let v = {
       let x = 3;
       x
   };

   assert!(v == 3);

   println!("Success!");
}

fn main() {
    let s = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y 
}

4
fn main() {
   
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
   let result = print();
   println!("Result: {}", result);
}


fn print() -> i32 {
   println!("Success!");
   42 
}

fn main() {
    never_return();


    println!("Failed!"); 
}

fn never_return() -> ! {
    panic!("This function never returns!");
}

fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => Some(42), 
        _ => None,
    }
}

fn never_return_fn() -> ! {
    panic!("This function never returns!"); 
}

fn main() {
    
    let b = false;

    let _v = match b {
        true => 1,
      
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    
    println!("Exercise Failed if printing out this line!");
}



