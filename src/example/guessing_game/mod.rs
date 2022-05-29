// let, match, 메소드, 연관함수, 외부 크레이트 사용과 같은 많은 새로운 러스트 개념들을 소개하기 위한 실습
extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessing_game() {
  println!("Guess the number!");

  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("The secret number is: {}", secret_number);

  loop {
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");
      // expect 메소드 호출을 match 표현식으로 바꾸는 것은 에러 발생 시 종료에서 처리 로 바꾸는 일반적인 방법
      // let guess: u32 = guess.trim().parse().expect("Failed to read line");
      let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("It's not number!");
          continue
        },
    };    
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!");
        break;
      }
    }
  }
}
