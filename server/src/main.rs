use core::Packet;
use rand::Rng;
use std::{
  cmp::Ordering,
  marker::PhantomData,
  net::{TcpListener, TcpStream},
  thread::sleep,
  time::Duration,
};

fn main() {
  println!("Guess the number!");

  let secret_number: u16 = rand::thread_rng().gen_range(1..=100);

  let listener: TcpListener = TcpListener::bind("127.0.0.1:8010").unwrap();
  let mut stream: TcpStream = listener.accept().unwrap().0;

  // stream
  //   .set_read_timeout(Some(Duration::from_millis(5)))
  //   .unwrap();

  stream.set_nonblocking(true).unwrap();

  loop {
    let guess: Option<u16> = get_guess_from_client(&mut stream);

    match guess {
      Some(guess) => {
        println!("You guessed: {guess}");

        let order: Ordering = guess.cmp(&secret_number);
        let ord_packet: Packet = Packet::Order { order };
        Packet::send_packet(ord_packet, &mut stream);

        match order {
          Ordering::Less => {}
          Ordering::Greater => {}
          Ordering::Equal => {
            println!("You win!");
            break;
          }
        }
      }
      None => println!("tick"),
    }
    sleep(Duration::from_millis(100));
  }
}

fn get_guess_from_client(stream: &mut TcpStream) -> Option<u16> {
  Packet::get_packet(stream).and_then(|packet| match packet {
    Packet::Guess { guess } => Some(guess),
    _ => None,
  })
}

mod private {
  pub trait SealedGameState {}
}
pub trait GameState: private::SealedGameState {}

struct Disconnected;
struct Connected;

impl private::SealedGameState for Disconnected {}
impl GameState for Disconnected {}
impl private::SealedGameState for Connected {}
impl GameState for Connected {}

struct Game<GS: GameState> {
  state: PhantomData<GS>,
}

impl Game<Disconnected> {
  fn connect(self) -> Game<Connected> {
    Game { state: PhantomData }
  }
}

impl Game<Connected> {
  fn disconnect(self) -> Game<Disconnected> {
    Game { state: PhantomData }
  }
}
