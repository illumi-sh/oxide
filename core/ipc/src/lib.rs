use std::net::{Ipv4Addr, SocketAddrV4};

mod protocol;

pub mod packet;

pub use {protocol::*, request::*, response::*};

pub const OXIDE_IPC_LOCAL_ADDRESS: SocketAddrV4 =
    SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 42068);
