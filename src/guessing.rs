use std::io;

pub fn start() {
  println!("Welcome to the guessing game!");
  println!("Input guess:");

  let mut guess = String::new();
  io::stdin().read_line(&mut guess).unwrap();

  println!("You guessed: {}", guess);
}
