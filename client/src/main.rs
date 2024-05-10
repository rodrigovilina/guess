use core::Packet;
use std::cmp::Ordering;
use std::io::{self};
use std::net::TcpStream;

const WELCOME: &str = "Guess the number!";
const ADDRESS: &str = "127.0.0.1:8010";
const GUESS: &str = "Please input your guess.";
const FAILED_READ: &str = "Failed to read line";
const TOO_SMALL: &str = "Too small!";
const TOO_BIG: &str = "Too big!";
const YOU_WIN: &str = "You win!";

fn main() {
  println!("{}", WELCOME);
  let mut stream: TcpStream = TcpStream::connect(ADDRESS).unwrap();

  loop {
    println!("{}", GUESS);

    let mut guess: String = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect(FAILED_READ);

    let guess: u16 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed: {guess}");

    let packet: Packet = Packet::Guess { guess };
    Packet::send_packet(packet, &mut stream);
    let cmp: Option<Ordering> = get_cmp(&mut stream);

    match cmp {
      Some(Ordering::Less) => println!("{}", TOO_SMALL),
      Some(Ordering::Greater) => println!("{}", TOO_BIG),
      Some(Ordering::Equal) => {
        println!("{}", YOU_WIN);
        break;
      }
      None => {}
    }
  }
}

fn get_cmp(stream: &mut TcpStream) -> Option<Ordering> {
  Packet::get_packet(stream).and_then(|packet| match packet {
    Packet::Order { order } => Some(order),
    _ => None,
  })
}
