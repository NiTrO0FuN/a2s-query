pub mod errors;
pub mod info;
pub mod players;
pub mod rules;
mod utils;

use std::net::ToSocketAddrs;
use std::net::UdpSocket;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use errors::Error;
use std::io::{Cursor, Read};

const HEADER_SINGLE_PACKET: i32 = -1;
const HEADER_MULTI_PACKET: i32 = -2;
const S2C_CHALLENGE: u8 = 0x41;

pub struct A2S<A: ToSocketAddrs> {
    address: A,
}

struct Packet {
    number: u8,
    payload: Vec<u8>,
}

impl<A: ToSocketAddrs> A2S<A> {
    pub fn new(address: A) -> Self {
        A2S { address }
    }

    fn send_and_recv_with_challenge(&self, req_header: u8) -> Result<Cursor<Vec<u8>>, Error> {
        let data = Vec::with_capacity(9);
        let mut data = Cursor::new(data);

        data.write_i32::<LittleEndian>(HEADER_SINGLE_PACKET)?;
        data.write_u8(req_header)?;
        data.write_i32::<LittleEndian>(HEADER_SINGLE_PACKET)?;

        let mut res = self.send_and_recv(data.get_ref())?;

        let resp_header = res.read_u8()?;
        if resp_header != S2C_CHALLENGE {
            res.set_position(res.position() - 1);
            return Ok(res);
        }
        let challenge = res.read_i32::<LittleEndian>()?;

        data.set_position(5);
        data.write_i32::<LittleEndian>(challenge)?;
        self.send_and_recv(&data.get_ref())
    }

    fn send_and_recv(&self, data: &[u8]) -> Result<Cursor<Vec<u8>>, Error> {
        let socket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect(&self.address)?;
        socket.set_read_timeout(Some(std::time::Duration::from_secs(5)))?;

        socket.send(&data)?;

        let mut buf = [0u8; 1260];

        let n_received = socket.recv(&mut buf)?;
        if n_received < 5 {
            return Err(Error::InvalidResponse);
        }

        let mut data = buf.to_vec();
        data.truncate(n_received);
        let mut data = Cursor::new(data);

        let header = data.read_i32::<LittleEndian>()?;
        if header == HEADER_SINGLE_PACKET {
            Ok(data)
        } else if header == HEADER_MULTI_PACKET {
            let mut packets = Vec::new();

            let answer_id = data.read_i32::<LittleEndian>()?;

            if answer_id >> 31 == 1 {
                return Err(Error::NotImplemented {
                    feature: "gzip2 is not handled",
                });
            }

            data.set_position(data.position() - 4);

            loop {
                let id = data.read_i32::<LittleEndian>()?;
                if id != answer_id {
                    return Err(Error::UnexpectedAnswerID {
                        expected: answer_id,
                        found: id,
                    });
                };

                let (total_packets, packet_number) = (data.read_u8()?, data.read_u8()?);

                let payload_size = data.read_i16::<LittleEndian>()?;
                let mut payload = Vec::with_capacity(payload_size as usize);
                data.read_to_end(&mut payload)?;
                packets.push(Packet {
                    number: packet_number,
                    payload: payload,
                });

                if packets.len() == total_packets as usize {
                    break;
                }

                let n = socket.recv(&mut buf)?;
                let mut a: Vec<u8> = buf.to_vec();
                a.truncate(n);
                data = Cursor::new(a);
                data.read_i32::<LittleEndian>()?;
            }

            packets.sort_by_key(|p| p.number);

            let mut payload = Vec::new();
            for packet in packets {
                payload.extend(packet.payload);
            }
            let mut res = Cursor::new(payload);
            res.read_i32::<LittleEndian>()?;
            Ok(res)
        } else {
            return Err(Error::InvalidResponse);
        }
    }
}
