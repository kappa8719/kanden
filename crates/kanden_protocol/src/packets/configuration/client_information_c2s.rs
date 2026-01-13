use crate::packets::play::client_information_c2s::{ChatMode, DisplayedSkinParts};
use crate::{Arm, Bounded, Decode, Encode, Packet, PacketState};

#[derive(Clone, Debug, Encode, Decode, Packet)]
#[packet(state = PacketState::Configuration)]
pub struct ClientInformationC2s<'a> {
    pub locale: Bounded<&'a str, 16>,
    pub view_distance: u8,
    pub chat_mode: ChatMode,
    pub chat_colors: bool,
    pub displayed_skin_parts: DisplayedSkinParts,
    pub main_arm: Arm,
    pub enable_text_filtering: bool,
    pub allow_server_listings: bool,
    pub particle_mode: ParticleMode,
}

#[derive(Clone, Debug, Encode, Decode)]
pub enum ParticleMode {
    All,
    Decreased,
    Minimal,
}
