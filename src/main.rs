#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Point{
  x: f32,
  y: f32,
}

struct Rectangle{
  top_left: Point,
  bottom_right: Point,
}

fn main() {
    println!("Hello, world!");
    println!("this is derrick shibero");
  println!("I am a rustician");
  //variable definition
  let x = 5+5;
  // various prints
  println!("is `x` equal to 10 or 1000: {}",x);
  println!("january has {} days", 31);
  println!("{0} this is {1} and {1} this is {0}", "Alice", "Bob");
  println!("{subject} {verb} {object}",
           object="the lazy dog",
           verb="jumps over ",
           subject="the quick brown fox");

  //arrays
  let xs: [i32; 5] = [1, 2,3,4,5];
  println!("{:?}", xs);
  println!("printing all elements in the array using a for loop");
  for i in 0..xs.len() + 1 { // Oops, one element too far!
      match xs.get(i) {
          Some(xval) => println!("{}: {}", i, xval),
          None => println!("Slow down! {} is too far!", i),
      }
  }

  //structs
  let name = String::from("derrick");
  let age: u8 = 21;
  let person = Person { name, age };
  println!("{:?}", person);
  let point: Point = Point { x: 10.3, y: 0.4 };
  let another_point: Point = Point { x: 5.2, y: 0.2 };
  let rect = Rectangle {
    top_left: point,
    bottom_right: another_point,
  };
  // println!("{:?}", rect);
}