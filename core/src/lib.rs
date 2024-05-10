use std::{
  cmp::Ordering,
  io::{Read, Write},
};

const INVALID_PACKET: &str = "Invalid packet type";
const INVALID_ORDER: &str = "Invalid order";

pub enum Packet {
  Guess { guess: u16 },
  Order { order: Ordering },
}

impl Packet {
  pub fn to_bytes(&self) -> Vec<u8> {
    match self {
      Packet::Guess { guess } => {
        vec![0, ((guess >> 8) & 0xFF) as u8, (guess & 0xFF) as u8]
      }
      Packet::Order { order } => {
        vec![
          1,
          match order {
            Ordering::Equal => 0,
            Ordering::Greater => 1,
            Ordering::Less => 2,
          },
        ]
      }
    }
  }

  pub const fn size(&self) -> usize {
    match self {
      Packet::Guess { .. } => 3,
      Packet::Order { .. } => 2,
    }
  }

  pub const fn packet_type(&self) -> u8 {
    match self {
      Packet::Guess { .. } => 0,
      Packet::Order { .. } => 1,
    }
  }

  pub const fn size_from_type(packet_type: u8) -> usize {
    match packet_type {
      0 => 3,
      1 => 2,
      _ => panic!("{}", INVALID_PACKET),
    }
  }

  pub fn from_bytes(bytes: Vec<u8>) -> Packet {
    match bytes.first().unwrap() {
      0 => {
        let guess: u16 = ((*bytes.get(1).unwrap() as u16) << 8) | *bytes.get(2).unwrap() as u16;
        Packet::Guess { guess }
      }
      1 => {
        let order: Ordering = match bytes.get(1).unwrap() {
          0 => Ordering::Equal,
          1 => Ordering::Greater,
          2 => Ordering::Less,
          _ => panic!("{}", INVALID_ORDER),
        };
        Packet::Order { order }
      }
      _ => panic!("{}", INVALID_PACKET),
    }
  }

  pub fn send_packet(packet: Packet, stream: &mut impl Write) {
    let _bytes: usize = stream.write(&packet.to_bytes()).unwrap();
    // println!("Wrote {} bytes", bytes);
  }

  pub fn get_packet(stream: &mut impl Read) -> Option<Packet> {
    let mut packet_type: [u8; 1] = [0; 1];
    stream.read_exact(&mut packet_type).ok()?;
    // println!("Read {} bytes as packet_type", 1);

    match packet_type[0] {
      0 => {
        let mut buf: [u8; 2] = [0; 2];
        stream.read_exact(&mut buf).ok()?;
        Some(Packet::from_bytes(vec![0, buf[0], buf[1]]))
      }
      1 => {
        let mut buf: [u8; 1] = [0; 1];
        stream.read_exact(&mut buf).ok()?;
        Some(Packet::from_bytes(vec![1, buf[0]]))
      }
      _ => None,
    }
  }
}
