name : armstrong 

fn main() {
  let num : i32 = 9;
  armstrong(num);
  num = 153;
  armstrong(num);
  num = 154;
  armstrong(num);
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
