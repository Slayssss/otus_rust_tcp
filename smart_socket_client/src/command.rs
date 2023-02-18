#[derive(Debug)]
pub enum Command {
    GetDeviceInfo,
    TurnOff(u8),
    TurnOn(u8),
    GetPower(u8),
    TurnOffAll,
    TurnOnAll,
    Undefined,
}

impl Command {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            Command::GetDeviceInfo => vec![0x01],
            Command::TurnOff(id) => vec![0x02, *id],
            Command::TurnOn(id) => vec![0x03, *id],
            Command::GetPower(id) => vec![0x04, *id],
            Command::TurnOffAll => vec![0x05],
            Command::TurnOnAll => vec![0x06],
            Command::Undefined => vec![],
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        match bytes[0] {
            0x01 => Some(Command::GetDeviceInfo),
            0x02 => Some(Command::TurnOff(bytes[1])),
            0x03 => Some(Command::TurnOn(bytes[1])),
            0x04 => Some(Command::GetPower(bytes[1])),
            0x05 => Some(Command::TurnOffAll),
            0x06 => Some(Command::TurnOnAll),
            _ => None,
        }
    }
}
