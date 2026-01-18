use byteorder::{LittleEndian, ReadBytesExt};
use serde::Serialize;
use std::io::Cursor;
use std::net::ToSocketAddrs;

use crate::A2S;
use crate::errors::Error;
use crate::utils::read_string::ReadString;

// A2S_RULES
const RULES_REQUEST_HEADER: u8 = 0x56;
const RULES_RESPONSE_HEADER: u8 = 0x45;

#[derive(Debug, Serialize)]
pub struct Rule {
    pub name: String,

    pub value: String,
}

impl Rule {
    fn list_from_bytes(mut payload: Cursor<Vec<u8>>) -> Result<Vec<Rule>, Error> {
        let header = payload.read_u8()?;
        if header != RULES_RESPONSE_HEADER {
            return Err(Error::InvalidHeader {
                expected: RULES_RESPONSE_HEADER,
                found: header,
            });
        }

        let n_rules = payload.read_u16::<LittleEndian>()?;
        let mut rules: Vec<Rule> = Vec::with_capacity(n_rules as usize);

        for _ in 0..n_rules {
            rules.push(Rule {
                name: payload.read_string()?,
                value: payload.read_string()?,
            });
        }

        Ok(rules)
    }
}

impl<A: ToSocketAddrs> A2S<A> {
    pub fn rules(&self) -> Result<Vec<Rule>, Error> {
        let data = self.send_and_recv_with_challenge(RULES_REQUEST_HEADER)?;
        Rule::list_from_bytes(data)
    }
}
