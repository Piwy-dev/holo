use std::net::Ipv4Addr;

pub const PIM_VERSION: u8 = 2;
pub const PIM_HEADER_LEN: usize = 4;
pub const PIM_VERSION_SHIFT: u8 = 4;
pub const PIM_TYPE_MASK: u8 = 0x0f;
pub const ALL_PIM_ROUTERS_V4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 13);
