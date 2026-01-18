use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::io::{Cursor, Read};
use std::net::ToSocketAddrs;

use crate::errors::Error;
use crate::utils::read_string::ReadString;
use crate::{A2S, HEADER_SINGLE_PACKET, S2C_CHALLENGE};

// A2S_INFO
const INFO_REQUEST_HEADER: u8 = 0x54;
const INFO_REQUEST_PAYLOAD: &str = "Source Engine Query\0";
const INFO_RESPONSE_HEADER: u8 = 0x49;

const THE_SHIP_APP_ID: i16 = 2400;

#[derive(Debug, Serialize, PartialEq)]
pub struct Info {
    /// Protocol version used by the server.
    pub protocol: u8,

    /// Name of the server
    pub name: String,

    /// Map the server has currently loaded
    pub map: String,

    /// Name of the folder containing the game files
    pub folder: String,

    /// Full name of the game
    pub game: String,

    /// Steam Application ID of game
    pub app_id: i16,

    /// Number of players on the server
    pub players: u8,

    /// Maximum number of players the server reports it can hold
    pub max_players: u8,

    /// Number of bots on the server
    pub bots: u8,

    /// Indicates the type of server
    pub server_type: ServerType,

    /// Indicates the operating system of the server
    pub environment: ServerEnvironment,

    /// Indicates whether the server requires a password
    pub password: bool,

    /// Specifies whether the server uses VAC
    pub vac: bool,

    #[serde(skip_serializing_if = "Option::is_none", flatten)]
    pub the_ship: Option<TheShipInfo>,

    /// Version of the game installed on the server
    pub version: String,

    /// If non-zero, this specifies which additional data fields will be included
    pub edf: u8,

    /// The server's game port number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i16>,

    /// Server's SteamID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub steam_id: Option<u64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sourcetv_info: Option<SourceTV>,

    /// Tags that describe the game according to the server
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<String>,

    /// The server's 64-bit GameID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_id: Option<u64>,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum ServerType {
    Dedicated,
    NonDedicated,
    SourceTVProxy,
}

impl ServerType {
    fn from_u8(byte: u8) -> Result<Self, Error> {
        match byte {
            b'd' => Ok(Self::Dedicated),
            b'l' => Ok(Self::NonDedicated),
            b'p' => Ok(Self::SourceTVProxy),
            _ => Err(Error::InvalidServerType),
        }
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub enum ServerEnvironment {
    Linux,
    Windows,
    Mac,
}

impl ServerEnvironment {
    fn from_u8(byte: u8) -> Result<Self, Error> {
        match byte {
            b'l' => Ok(Self::Linux),
            b'w' => Ok(Self::Windows),
            b'm' | b'o' => Ok(Self::Mac),
            _ => Err(Error::InvalidServerEnvironment),
        }
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub struct TheShipInfo {
    /// Indicates the game mode
    pub mode: TheShipMode,

    /// The number of witnesses necessary to have a player arrested
    pub witnesses: u8,

    /// Time (in seconds) before a player is arrested while being witnessed
    pub duration: u8,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum TheShipMode {
    Hunt,
    Elimination,
    Duel,
    Deathmatch,
    VIPTeam,
    TeamElimination,
}

impl TheShipMode {
    fn from_u8(byte: u8) -> Result<Self, Error> {
        match byte {
            0 => Ok(Self::Hunt),
            1 => Ok(Self::Elimination),
            2 => Ok(Self::Duel),
            3 => Ok(Self::Deathmatch),
            4 => Ok(Self::VIPTeam),
            5 => Ok(Self::TeamElimination),
            _ => Err(Error::InvalidResponse),
        }
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub struct SourceTV {
    /// Spectator port number for SourceTV
    #[serde(rename = "sourcetv_port")]
    pub port: i16,

    /// Name of the spectator server for SourceTV
    #[serde(rename = "sourcetv_name")]
    pub name: String,
}

impl Info {
    fn from_bytes(mut payload: Cursor<Vec<u8>>) -> Result<Self, Error> {
        let header = payload.read_u8()?;
        if header != INFO_RESPONSE_HEADER {
            return Err(Error::InvalidHeader {
                expected: INFO_RESPONSE_HEADER,
                found: header,
            });
        }

        let protocol = payload.read_u8()?;
        let name = payload.read_string()?;
        let map = payload.read_string()?;
        let folder = payload.read_string()?;
        let game = payload.read_string()?;
        let app_id = payload.read_i16::<LittleEndian>()?;
        let players = payload.read_u8()?;
        let max_players = payload.read_u8()?;
        let bots = payload.read_u8()?;
        let server_type = ServerType::from_u8(payload.read_u8()?)?;
        let environment = ServerEnvironment::from_u8(payload.read_u8()?)?;
        let password = payload.read_u8()? != 0;
        let vac = payload.read_u8()? != 0;
        let the_ship = if app_id == THE_SHIP_APP_ID {
            Some(TheShipInfo {
                mode: TheShipMode::from_u8(payload.read_u8()?)?,
                witnesses: payload.read_u8()?,
                duration: payload.read_u8()?,
            })
        } else {
            None
        };
        let version = payload.read_string()?;
        let edf = if (payload.position() as usize) < payload.get_ref().len() {
            payload.read_u8()?
        } else {
            0
        };

        let port = if edf & 0x80 != 0 {
            Some(payload.read_i16::<LittleEndian>()?)
        } else {
            None
        };

        let steam_id = if edf & 0x10 != 0 {
            Some(payload.read_u64::<LittleEndian>()?)
        } else {
            None
        };

        let sourcetv_info = if edf & 0x40 != 0 {
            Some(SourceTV {
                port: payload.read_i16::<LittleEndian>()?,
                name: payload.read_string()?,
            })
        } else {
            None
        };

        let keywords = if edf & 0x20 != 0 {
            Some(payload.read_string()?)
        } else {
            None
        };

        let game_id = if edf & 0x01 != 0 {
            Some(payload.read_u64::<LittleEndian>()?)
        } else {
            None
        };

        Ok(Info {
            protocol,
            name,
            map,
            folder,
            game,
            app_id,
            players,
            max_players,
            bots,
            server_type,
            environment,
            password,
            vac,
            the_ship,
            version,
            edf,
            port,
            steam_id,
            sourcetv_info,
            keywords,
            game_id,
        })
    }

    pub fn is_the_ship(&self) -> bool {
        self.app_id == THE_SHIP_APP_ID
    }
}

impl<A: ToSocketAddrs> A2S<A> {
    pub fn info(&self) -> Result<Info, Error> {
        let mut request = Vec::with_capacity(29);
        request.extend_from_slice(&HEADER_SINGLE_PACKET.to_le_bytes());
        request.push(INFO_REQUEST_HEADER);
        request.extend_from_slice(INFO_REQUEST_PAYLOAD.as_bytes());

        let mut data = self.send_and_recv(&request)?;

        let resp_header = data.read_u8()?;

        if resp_header == INFO_RESPONSE_HEADER {
            data.set_position(data.position() - 1);
            return Info::from_bytes(data);
        } else if resp_header == S2C_CHALLENGE {
            let mut challenge = [0u8; 4];
            data.read_exact(&mut challenge)?;
            request.extend_from_slice(&challenge);
            Info::from_bytes(self.send_and_recv(&request)?)
        } else {
            return Err(Error::InvalidHeader {
                expected: S2C_CHALLENGE,
                found: resp_header,
            });
        }
    }
}
