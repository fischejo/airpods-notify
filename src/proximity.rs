use std::{collections::HashMap};
use std::fmt;

const AIRPODS_MANUFACTURER: u16 = 76;
const PROXIMITIY_PROTOCOL: u8 = 0x7;
const PROXIMITIY_PAIRING_MODE: u8 = 0x0;
const PROXIMITIY_PAIRED_MODE: u8 = 0x1;
const PROXIMITIY_PAIRING_PROTOCOL_LENGTH: u8 = 15;
const PROXIMITIY_PAIRED_PROTOCOL_LENGTH: u8 = 25;


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Part {
    LeftEarPlug,
    RightEarPlug,
    Headphones,
}


impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Plugged {
    Single,
    Both,
    None,
}


impl fmt::Display for Plugged {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Battery {
    Level(u8),
    None,
}

impl Battery {
    fn from_u8(value: u8) -> Battery {
        if value == 15 {
            Battery::None
        } else {
            Battery::Level(value)
        }
    }
}


impl fmt::Display for Battery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Battery::Level(value) => write!(f, "{:}%", value*10),
            Battery::None => write!(f, "None"),
        }
        
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Lid {
    Open(u8),
    Closed(u8),    
}

impl Lid {
    fn from_u8(value: u8) -> Lid {
        let lid_count = value & 0b0111; // last 3 bits
        let lid_open_event = value& 0b1000 == 0;
        match lid_open_event {
            true => Lid::Open(lid_count),
            false => Lid::Closed(lid_count)
        }
    }
}

impl fmt::Display for Lid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Model {
    AirPods1,
    AirPods2,
    AirPods3,
    AirPodsPro,
    AirPodsPro2,
    AirPodsMax,
    PowerbeatsPro,
    BeatsX,
    BeatsFlex,
    BeatsSolo3,
    BeatsStudio3,
    PowerBeats3,
    BeatsStudioBuds,
    BeatsSoloPro,
    Unknown(u16),
}

impl Model {
    fn from_bytes(bytes: [u8; 2]) -> Model {
        match u16::from_be_bytes(bytes) {
            0x0220 => Model::AirPods1,
            0x0f20 => Model::AirPods2,
            0x1320 => Model::AirPods3,
            0x0e20 => Model::AirPodsPro, 
            0x1420 => Model::AirPodsPro2, 
            0x0a20 => Model::AirPodsMax,
            0x0b20 => Model::PowerbeatsPro, // untested
            0x0520 => Model::BeatsX,
            0x1020 => Model::BeatsFlex,   
            0x1120 => Model::BeatsStudioBuds,          
            0x0620 => Model::BeatsSolo3,
            0x0920 => Model::BeatsStudio3,
            0x0320 => Model::PowerBeats3,
            0x0c20 => Model::BeatsSoloPro,
            0x2336 => Model::Unknown(0x2336),
            id => Model::Unknown(id),
        }
    }    

    fn is_single_device(&self) -> bool {
        match self {
            Model::AirPodsMax => true,
            Model::BeatsFlex => true,
            Model::BeatsSolo3 => true,
            Model::BeatsX => true,
            Model::BeatsStudio3 => true,
            Model::PowerbeatsPro => true,
            _ => false,
        }
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
    Red,
    Blue,
    Pink,
    Gray,
    Silver,
    Gold,
    RoseGold,
    SpaceGray,
    DarkBlue,
    LightBlue,
    Yellow,
    Unknown(u8),
}


impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Color {
    fn from_u8(value: u8) -> Color {
        match value {
            0x00 => Color::White,
            0x01 => Color::Black,
            0x02 => Color::Red,
            0x03 => Color::Blue,
            0x04 => Color::Pink,
            0x05 => Color::Gray,
            0x06 => Color::Silver,
            0x07 => Color::Gold,
            0x08 => Color::RoseGold,
            0x09 => Color::SpaceGray,
            0x10 => Color::Unknown(0x10), // AirPodsMax: Silver
            0x11 => Color::Unknown(0x11), // PowerbeatsPro
            0x0A => Color::DarkBlue,
            0x0B => Color::LightBlue,
            0x0C => Color::Yellow,
            id => Color::Unknown(id),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PairedMessage {
    pub model: Model,
    pub lid: Lid,
    pub color: Color,
    pub case_battery_level: Battery,
    pub left_battery_level: Battery,
    pub right_battery_level: Battery,
    pub case_charging: bool,
    pub right_charging: bool,
    pub left_charging: bool,
    pub plugged_in_ear: Plugged,
    pub plugged_in_case: Plugged,
    pub part: Part,
}

impl PairedMessage {

    fn from_bytes(bytes: &Vec<u8>) -> PairedMessage {
        // ensure correct protocol (0x7)
        let device_model = Model::from_bytes(bytes[3..5].try_into().unwrap());
        let device_color = Color::from_u8(bytes[9]);
        let lid = Lid::from_u8(bytes[8]);
        let left_battery_level = Battery::from_u8((bytes[6] >> 0x4) & 0x0f);
        let right_battery_level = Battery::from_u8(bytes[6] & 0x0f);
        let case_battery_level = Battery::from_u8(bytes[7] & 0x0f);
        let right_charging = (bytes[7] >> 0x4) & 0b0001 != 0;
        let left_charging = (bytes[7] >> 0x4) & 0b0010 != 0;
        let case_charging = (bytes[7] >> 0x4) & 0b0100 != 0;            
        let one_or_both_in_ear: bool = bytes[5] & (0x1 << 0x1) != 0;
        let both_in_case: bool = bytes[5] & (0x1 << 0x2) != 0;
        let both_in_ear: bool = bytes[5] & (0x1 << 0x3) != 0;
        let one_or_both_in_case: bool = bytes[5] & (0x1 << 0x4) != 0;
            
        let plugged_in_ear = match (both_in_ear, one_or_both_in_ear) {                
            (false, true) => Plugged::Single,
            (true, true) => Plugged::Both,
            _ => Plugged::None,
        };

        let plugged_in_case = match (both_in_case, one_or_both_in_case) {                
            (false, true) => Plugged::Single,
            (true, true) => Plugged::Both,
            _ => Plugged::None,
        };

        let part = match bytes[5] & (0x1 << 0x5) != 0 {
            _ if device_model.is_single_device() => Part::Headphones,
            false => Part::RightEarPlug,
            true => Part::LeftEarPlug,
        };
        let flipped = bytes[5] & (0x1 << 0x6) != 0;
            
        let mut msg = PairedMessage{
                model: device_model, 
                lid, 
                color: device_color, 
                left_battery_level,
                right_battery_level,
                case_battery_level,
                right_charging,
                left_charging,
                case_charging,
                plugged_in_case,
                plugged_in_ear,
                part,
        };
        if flipped {
            msg.left_battery_level = right_battery_level;
            msg.right_battery_level = left_battery_level;
            msg.left_charging = right_charging;
            msg.right_charging = left_charging;
        } 
        msg
    }

    pub fn cmp(&self, other: &PairedMessage) -> bool {
        if self.model != other.model || self.color != other.color || self.part != other.part {
            false
        } else {
            let mut characteristic_count = 0;
            if self.left_charging == other.left_charging {
                characteristic_count += 1;
            }
            if self.right_charging == other.right_charging {
                characteristic_count += 1;
            }
            if self.case_charging == other.case_charging {
                characteristic_count += 1;
            }
            if self.case_battery_level == other.case_battery_level {
                characteristic_count += 1;
            }
            if self.left_battery_level == other.left_battery_level {
                characteristic_count += 1;
            }
            if self.right_battery_level == other.right_battery_level {
                characteristic_count += 1;
            }
            if self.lid == other.lid {
                characteristic_count += 1;
            }
            if self.plugged_in_case == other.plugged_in_case {
                characteristic_count += 1;
            }
            if self.plugged_in_ear == other.plugged_in_ear {
                characteristic_count += 1;
            }
            characteristic_count >= 6 // 3 changes 
        }
    }    
}

pub struct PairingMessage {
    device_model: Model,
    device_color: Color,
    address: [u8; 6],
}

impl PairingMessage {
    fn from_bytes(bytes: &Vec<u8>) -> PairingMessage {
        let device_model = Model::from_bytes(bytes[3..5].try_into().unwrap());
        let address = bytes[5..11].try_into().unwrap();
        let device_color = Color::from_u8(bytes[16]);

        /*
        println!("unknown: {}", bytes[12]);
        println!("Right Battery: {}", bytes[13]);
        println!("Left Battery: {}", bytes[14]);
        println!("Case Battery: {}", bytes[15]);
        println!("mode: {}", device_model);
        println!("color {}", device_color);
        println!("address {:?}", address);
        */
        PairingMessage {
            device_color,
            address,
            device_model
        }
    }
}


pub enum ProximityEvent {
    Pairing(PairingMessage),
    Paired(PairedMessage),
}

impl ProximityEvent {
    pub fn from_manufacturer_data(data: HashMap<u16, Vec<u8>>) -> Option<ProximityEvent> {
        if let Some(manufacturer_data) = data.get(&AIRPODS_MANUFACTURER) {
            // ensure correct protocol (0x7)
            if manufacturer_data.len() > 3 && manufacturer_data[0] == PROXIMITIY_PROTOCOL {
                match manufacturer_data[2] {
                    PROXIMITIY_PAIRED_MODE if (
                        manufacturer_data.len() == usize::from(PROXIMITIY_PAIRED_PROTOCOL_LENGTH) + 2 && 
                        manufacturer_data[1] == PROXIMITIY_PAIRED_PROTOCOL_LENGTH 
                    )  => {
                            Some(ProximityEvent::Paired(PairedMessage::from_bytes(manufacturer_data)))
                    },
                    PROXIMITIY_PAIRING_MODE if (
                        manufacturer_data.len() == usize::from(PROXIMITIY_PAIRING_PROTOCOL_LENGTH) + 2 && 
                        manufacturer_data[1] == PROXIMITIY_PAIRING_PROTOCOL_LENGTH 
                    ) => {
                            Some(ProximityEvent::Pairing(PairingMessage::from_bytes(manufacturer_data)))
                    },
                    _ => None
                }
            } else {
                None
            }
        } else {
            None
        }
    }


}
