fn main(){
  println!("Esto es Rust");
  print_sum(1,2);

  //Boolens
  let x = true;
  let y: bool = false;
  //Char
  let lettler = 'a';
  //Arrays
  let array = [1,2,3,4];
  let mut m = [5,6,7,8];
  println!("array has {} elements", array.len());
  println!("the second array poc 1 is {}", m[0]);

}

 fn print_sum(x: i32, y: i32) {
    println!("la suma es: {}", x + y);
  }
