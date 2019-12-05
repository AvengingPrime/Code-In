use std::io;

fn main() {
 let mut num : i32 = 1;
 while num != 0 {
  num = input();
 if num == 0 {
  break;
 }
 armstrong(num);
 }
 
 println!("Thank you!");
}

fn armstrong(number : i32) {
  let mut x = number;
  let mut i : u32 = 0;
  let mut vec: Vec<i32> = vec![];
  while x > 0 {
    vec.push(x % 10);
    x = x/10;
    i += 1;
  }
  
  let mut sum : i32 = 0;

  for j in vec.iter() {
    sum += j.pow(i);
  }

  if sum == number {
    println!("Armstrong Number");
  }
  else {
    println!("Not an Armstrong Number");
  }
}
  
  fn input() -> i32 {
   println!("Enter in a number to check or 0 to quit");
   let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i32>() {
        Ok(i) => i,
        Err(..) => 0,
    }
}
