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

  let xs: [i32; 5] = [1, 2,3,4,5];
  println!("{:?}", xs);
  println!("printing all elements in the array using a for loop");
  for i in 0..xs.len() + 1 { // Oops, one element too far!
      match xs.get(i) {
          Some(xval) => println!("{}: {}", i, xval),
          None => println!("Slow down! {} is too far!", i),
      }
  }
}