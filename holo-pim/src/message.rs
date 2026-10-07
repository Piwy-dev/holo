use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum MessageType {
    Hello = 0,
    Register = 1,
    RegisterStop = 2,
    JoinPrune = 3,
    Bootstrap = 4,
    Assert = 5,
    Graft = 6,
    GraftAck = 7,
    CandidateRpAdvertisement = 8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnknownMessageType(pub u8);

impl MessageType {
    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

impl TryFrom<u8> for MessageType {
    type Error = UnknownMessageType;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let message_type = match value {
            0 => Self::Hello,
            1 => Self::Register,
            2 => Self::RegisterStop,
            3 => Self::JoinPrune,
            4 => Self::Bootstrap,
            5 => Self::Assert,
            6 => Self::Graft,
            7 => Self::GraftAck,
            8 => Self::CandidateRpAdvertisement,
            value => return Err(UnknownMessageType(value)),
        };
        Ok(message_type)
    }
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Hello => "Hello",
            Self::Register => "Register",
            Self::RegisterStop => "Register-Stop",
            Self::JoinPrune => "Join/Prune",
            Self::Bootstrap => "Bootstrap",
            Self::Assert => "Assert",
            Self::Graft => "Graft",
            Self::GraftAck => "Graft-Ack",
            Self::CandidateRpAdvertisement => "Candidate-RP-Advertisement",
        };
        f.write_str(name)
    }
}

impl fmt::Display for UnknownMessageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "unknown PIM message type: {}", self.0)
    }
}

impl std::error::Error for UnknownMessageType {}
