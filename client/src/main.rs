use core::Packet;
use std::cmp::Ordering;
use std::io::{self};
use std::net::TcpStream;

fn main() {
  println!("Guess the number!");
  let mut stream: TcpStream = TcpStream::connect("127.0.0.1:8010").unwrap();

  loop {
    println!("Please input your guess.");

    let mut guess: String = String::new();

    io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u16 = match guess.trim().parse() {
      Ok(num) => num,
      Err(_) => continue,
    };

    println!("You guessed: {guess}");

    let packet: Packet = Packet::Guess { guess };
    Packet::send_packet(packet, &mut stream);
    let cmp: Option<Ordering> = get_cmp(&mut stream);

    match cmp {
      Some(Ordering::Less) => println!("Too small!"),
      Some(Ordering::Greater) => println!("Too big!"),
      Some(Ordering::Equal) => {
        println!("You win!");
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
