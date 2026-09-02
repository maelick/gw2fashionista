use std::fmt::Display;
use std::io;
use std::str::FromStr;

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use byteorder::WriteBytesExt;
use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::domain::error::ChatLinkError;
use crate::domain::templates::travel::TravelTemplate;
use crate::domain::templates::wardrobe::WardrobeTemplate;
use crate::domain::templates::{FashionSlot, Template};

#[derive(IntoPrimitive, TryFromPrimitive, Debug, Copy, Clone)]
#[num_enum(error_type(name = ChatLinkError, constructor = ChatLinkError::UnknownType))]
#[repr(u8)]
pub enum ChatLinkType {
    Coin = 0x01,
    Item = 0x02,
    NPCText = 0x03,
    MapLink = 0x04,
    PvPGame = 0x05,
    Skill = 0x06,
    Trait = 0x07,
    User = 0x08,
    Recipe = 0x09,
    Wardrobe = 0x0A,
    Outfit = 0x0B,
    WvWObjective = 0x0C,
    BuildTemplate = 0x0D,
    Achievement = 0x0E,
    WardrobeTemplate = 0x0F,
    TravelTemplate = 0x10,
}

#[derive(Debug)]
pub enum ChatLink {
    WardrobeTemplate(WardrobeTemplate),
    TravelTemplate(TravelTemplate),
    Unparsed {
        link_type: ChatLinkType,
        bytes: Vec<u8>,
    },
}

impl ChatLink {
    pub fn link_type(&self) -> ChatLinkType {
        match self {
            ChatLink::WardrobeTemplate(_) => ChatLinkType::WardrobeTemplate,
            ChatLink::TravelTemplate(_) => ChatLinkType::TravelTemplate,
            ChatLink::Unparsed {
                link_type,
                bytes: _,
            } => *link_type,
        }
    }

    pub fn encode<W: io::Write + ?Sized>(&self, w: &mut W) -> io::Result<()> {
        w.write_u8(self.link_type().into())?;
        self.encode_payload(w)
    }

    fn encode_payload<W: io::Write + ?Sized>(&self, w: &mut W) -> io::Result<()> {
        match self {
            Self::WardrobeTemplate(t) => t.encode(w),
            Self::TravelTemplate(t) => t.encode(w),
            Self::Unparsed { bytes, .. } => w.write_all(bytes),
        }
    }
}

impl Display for ChatLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        encode_base64(self.link_type(), f, |w| self.encode_payload(w))
    }
}

impl Display for WardrobeTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        encode_base64(ChatLinkType::WardrobeTemplate, f, |w| self.encode(w))
    }
}

impl Display for TravelTemplate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        encode_base64(ChatLinkType::TravelTemplate, f, |w| self.encode(w))
    }
}

impl TryFrom<ChatLink> for WardrobeTemplate {
    type Error = ChatLinkError;

    fn try_from(link: ChatLink) -> Result<Self, Self::Error> {
        match link {
            ChatLink::WardrobeTemplate(template) => Ok(template),
            _ => Err(ChatLinkError::UnexpectedType {
                expected: ChatLinkType::WardrobeTemplate,
                found: link.link_type(),
            }),
        }
    }
}

impl TryFrom<ChatLink> for TravelTemplate {
    type Error = ChatLinkError;

    fn try_from(link: ChatLink) -> Result<Self, Self::Error> {
        match link {
            ChatLink::TravelTemplate(template) => Ok(template),
            _ => Err(ChatLinkError::UnexpectedType {
                expected: ChatLinkType::TravelTemplate,
                found: link.link_type(),
            }),
        }
    }
}

impl<S: FashionSlot> FromStr for Template<S>
where
    Template<S>: TryFrom<ChatLink, Error = ChatLinkError>,
{
    type Err = ChatLinkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<ChatLink>()?.try_into()
    }
}

impl FromStr for ChatLink {
    type Err = ChatLinkError;

    fn from_str(raw_chat_link: &str) -> Result<Self, Self::Err> {
        let decoded = decode_base64(raw_chat_link)?;
        let (link_type, payload) = decode_header(&decoded)?;

        Ok(match link_type {
            ChatLinkType::WardrobeTemplate => {
                let template = WardrobeTemplate::try_from(payload)?;
                Self::WardrobeTemplate(template)
            }
            ChatLinkType::TravelTemplate => {
                let template = TravelTemplate::try_from(payload)?;
                Self::TravelTemplate(template)
            }
            _ => Self::Unparsed {
                link_type,
                bytes: payload.to_vec(),
            },
        })
    }
}

fn decode_base64(raw_chat_link: &str) -> Result<Vec<u8>, ChatLinkError> {
    let base64_str = strip_delimiters(raw_chat_link)?;
    Ok(BASE64.decode(base64_str)?)
}

fn strip_delimiters(raw: &str) -> Result<&str, ChatLinkError> {
    let inner = match raw.strip_prefix('[') {
        Some(rest) => rest.strip_suffix(']').ok_or(ChatLinkError::InvalidString)?,
        None if raw.ends_with(']') => return Err(ChatLinkError::InvalidString),
        None => raw,
    };
    Ok(inner.strip_prefix('&').unwrap_or(inner))
}

fn decode_header(decoded: &[u8]) -> Result<(ChatLinkType, &[u8]), ChatLinkError> {
    let (header, payload) = decoded.split_first().ok_or(ChatLinkError::EmptyPayload)?;
    Ok((ChatLinkType::try_from(*header)?, payload))
}

fn encode_base64<F>(
    link_type: ChatLinkType,
    f: &mut std::fmt::Formatter<'_>,
    encode_payload: F,
) -> std::fmt::Result
where
    F: FnOnce(&mut dyn io::Write) -> io::Result<()>,
{
    let mut enc = base64::write::EncoderStringWriter::new(&BASE64);
    enc.write_u8(link_type.into())
        .and_then(|()| encode_payload(&mut enc))
        .expect("writing to a String cannot fail");
    write!(f, "[&{}]", enc.into_inner())
}
