use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::io::Cursor;
use std::net::ToSocketAddrs;

use crate::A2S;
use crate::errors::Error;
use crate::utils::read_string::ReadString;

// A2S_PLAYER
const PLAYER_REQUEST_HEADER: u8 = 0x55;
const PLAYER_RESPONSE_HEADER: u8 = 0x44;

#[derive(Debug, Serialize, PartialEq)]
pub struct Player {
    /// Index of player chunk starting from 0
    pub index: u8,

    /// Name of the player
    pub name: String,

    /// Player's score (usually "frags" or "kills".)
    pub score: i32,

    /// Time (in seconds) player has been connected to the server
    pub duration: f32,

    /// The Ship additional player info
    #[serde(skip_serializing_if = "Option::is_none")]
    pub the_ship: Option<TheShipInfo>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct TheShipInfo {
    /// Player's deaths
    pub deaths: u32,

    /// Player's money
    pub money: u32,
}

impl Player {
    fn min_byte_size(is_the_ship: bool) -> usize {
        let base = size_of::<u8>() + 1 + size_of::<i32>() + size_of::<f32>();
        if is_the_ship {
            base + size_of::<u32>() + size_of::<u32>()
        } else {
            base
        }
    }

    fn list_from_bytes(
        mut payload: Cursor<Vec<u8>>,
        is_the_ship: bool,
    ) -> Result<Vec<Self>, Error> {
        let header = payload.read_u8()?;
        if header != PLAYER_RESPONSE_HEADER {
            return Err(Error::InvalidHeader {
                expected: PLAYER_RESPONSE_HEADER,
                found: header,
            });
        }

        let total_players = payload.read_u8()?;
        let mut players: Vec<Player> = Vec::with_capacity(total_players as usize);

        for _ in 0..total_players {
            players.push(Player {
                index: payload.read_u8()?,
                name: payload.read_string()?,
                score: payload.read_i32::<LittleEndian>()?,
                duration: payload.read_f32::<LittleEndian>()?,
                the_ship: if is_the_ship {
                    Some(TheShipInfo {
                        deaths: payload.read_u32::<LittleEndian>()?,
                        money: payload.read_u32::<LittleEndian>()?,
                    })
                } else {
                    None
                },
            });

            if (payload.position() as usize + Player::min_byte_size(is_the_ship))
                > payload.get_ref().len()
            {
                break;
            }
        }

        Ok(players)
    }
}

impl<A: ToSocketAddrs> A2S<A> {
    pub fn players(&self) -> Result<Vec<Player>, Error> {
        let is_the_ship = self.info()?.is_the_ship();
        let data = self.send_and_recv_with_challenge(PLAYER_REQUEST_HEADER)?;
        Player::list_from_bytes(data, is_the_ship)
    }
}
