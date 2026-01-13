use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Activity {
    Core,
    Idle,
    Work,
    Play,
    Rest,
    Meet,
    Panic,
    Raid,
    PreRaid,
    Hide,
    Fight,
    Celebrate,
    AdmireItem,
    Avoid,
    Ride,
    PlayDead,
    LongJump,
    Ram,
    Tongue,
    Swim,
    LaySpawn,
    Sniff,
    Investigate,
    Roar,
    Emerge,
    Dig,
}
