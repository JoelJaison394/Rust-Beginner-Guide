fn main(){
    let x=45; 
    //i32 means a signed 32 bit  integer , rust by deafult asssumes it when we type the number 45 
    //we can change it into our own datatype 

    //This is done by putting : after variable name then the type ex:
    let y:i64 = 65;
    //This makes y a 64 bit signed integer , so we can store large numbers 

    //so if you know your variable cannot be a negative integer so you can use unsigned integer
    //ex

    let z: u64 = 33; //this doest support negative numbers

    //other similar ex:

    let f = 6.7; // f32 for storing floating point numbers
    let b: bool = false;
    let result = 10;    // i32 by default
   let age:u32 = 20;
   let sum:i32 = 5-15;
   let mark:isize = 10;
   let count:usize = 30;
   println!("result value is {}",result);
   println!("sum is {} and age is {}",sum,age);
   println!("mark is {} and count is {}",mark,count);


   let age:u8 = 255;

   //An integer overflow occurs when the value assigned to an integer variable exceeds the Rust defined range for the data type. Let us understand this with an example

   // 0 to 255 only allowed for u8
  // let weight:u8 = 256;   //overflow value is 0
  // let height:u8 = 257;   //overflow value is 1
  // let score:u8 = 258;    //overflow value is 2

   println!("age is {} ",age);
//    println!("weight is {}",weight);
//    println!("height is {}",height);
//    println!("score is {}",score);

let special_character = '@'; //default
let alphabet:char = 'A';
let emoji:char = '😁';

println!("special character is {}",special_character);
println!("alphabet is {}",alphabet);
println!("emoji is {}",emoji);
}
