use std::io;

use rmp::decode::{ValueReadError, MarkerReadError};

use super::Category;

// ==== Packet Decode Error
#[derive(Debug)]
pub enum PacketDecodeError {
    InvalidStructure,
    UnknownOpcode { category: Category, opcode: u16 },
    UnknownCategory { given_category: u16 },
    InvalidPayload { category: Category, opcode: u16 },
    Msgpack(ValueReadError),
}

impl From<ValueReadError> for PacketDecodeError {
    fn from(value: ValueReadError) -> Self {
        Self::Msgpack(value)
    }
}

impl From<MarkerReadError> for PacketDecodeError {
    fn from(value: MarkerReadError) -> Self {
        Self::Msgpack(ValueReadError::InvalidMarkerRead(value.0))
    }
}

impl<Opcode: Into<u16> + TryFrom<u16>> From<(Category, PacketCategoryDecodeError<Opcode>)> for PacketDecodeError {
    fn from((category, value): (Category, PacketCategoryDecodeError<Opcode>)) -> Self {
        match value {
            PacketCategoryDecodeError::UnknownOpcode { opcode }
                => PacketDecodeError::UnknownOpcode { category, opcode },

            PacketCategoryDecodeError::InvalidPayload { opcode }
                => PacketDecodeError::InvalidPayload { category, opcode: opcode.into() },
            
            PacketCategoryDecodeError::Msgpack(err)
                => PacketDecodeError::Msgpack(err),
        }
    }
}

// === Packet Category Decode Error
#[derive(Debug)]
pub enum PacketCategoryDecodeError<Opcode>
where Opcode: Into<u16> + TryFrom<u16>
{
    UnknownOpcode { opcode: u16 },
    InvalidPayload { opcode: Opcode },
    Msgpack(ValueReadError),
}

impl<Opcode> From<ValueReadError> for PacketCategoryDecodeError<Opcode>
where Opcode: Into<u16> + TryFrom<u16>
{
    fn from(value: ValueReadError) -> Self {
        PacketCategoryDecodeError::Msgpack(value)
    }
}

impl<Opcode> From<rmpv::decode::Error> for PacketCategoryDecodeError<Opcode>
where Opcode: Into<u16> + TryFrom<u16>
{
    fn from(value: rmpv::decode::Error) -> Self {
        match value {
            rmpv::decode::Error::InvalidMarkerRead(err)
                => PacketCategoryDecodeError::Msgpack(ValueReadError::InvalidMarkerRead(err)),
            rmpv::decode::Error::InvalidDataRead(err)
                => PacketCategoryDecodeError::Msgpack(ValueReadError::InvalidDataRead(err)),
            rmpv::decode::Error::DepthLimitExceeded
                => PacketCategoryDecodeError::Msgpack(ValueReadError::InvalidDataRead(
                    io::Error::new(value.kind(), value)
                ))
        }
    }
}

impl<Opcode> From<(Opcode, PayloadDecodeError)> for PacketCategoryDecodeError<Opcode>
where Opcode: Into<u16> + TryFrom<u16>
{
    fn from((opcode, err): (Opcode, PayloadDecodeError)) -> Self {
        match err {
            PayloadDecodeError::InvalidPayload
                => PacketCategoryDecodeError::InvalidPayload { opcode },
            PayloadDecodeError::Msgpack(mp)
                => PacketCategoryDecodeError::Msgpack(mp),
        }
    }
}

// === Packet Payload Decode Error
#[derive(Debug)]
pub enum PayloadDecodeError {
    InvalidPayload,
    Msgpack(ValueReadError)
}

impl From<ValueReadError> for PayloadDecodeError {
    fn from(value: ValueReadError) -> Self { PayloadDecodeError::Msgpack(value) }
}

impl From<MarkerReadError> for PayloadDecodeError {
    fn from(value: MarkerReadError) -> Self { PayloadDecodeError::Msgpack(value.into()) }
}
