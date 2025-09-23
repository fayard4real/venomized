//! All IDs that will be used by the server and client
//! to exchange information are stored here.
//!
//! Also provide decode/encode methods for packets

use protocol::primitives::{
    byte::Byte, prefixed_array::PrefixedArray, uvarint::UVarInt, varint::VarInt, varlong::VarLong,
};

/// An alias for Entity Id in the game, to remove magic number
pub type Id = VarLong;

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, num_enum::TryFromPrimitive)]
pub enum LoginServerbound {
    /// Initiating dialogue with the server
    /// and providing basic information
    Login = 0,
}

pub enum ConfigureServerbound {
    /// The package needed for the server to calculate
    /// the number of chunks that need to be sent to the client.
    ///
    /// **TODO:** It is assumed that later I will be able
    /// to calculate the window size on the client and send
    /// this value to the server at the `Configure` and `Play`
    /// stages when the window size changes.
    ///
    /// *window - refers to the size of a regular terminal/stdout window.
    SetDrawDistance = 0,
}

pub enum PlayServerbound {
    /// Sent by the client in order to turn the snake
    TurnSnake = 0,
}

pub enum LoginClientbound {
    /// The package is required to switch
    /// the state to the `Configuration` stage.
    LoginSuccess = 0,
}

pub enum ConfigureClientbound {
    /// The package is required to switch
    /// the state to the `Play` stage.
    ConfigureAcknowledged = 0,
}

pub enum PlayClientbound {
    /// For the client *(player)*, all other players are Entities.
    /// They have their own ID, which the player's client can see,
    /// but the player does not know their own ID, nor does it
    /// know the IDs when sending TurnSnake packets.
    SynchonizeSnakePositionAndDirection = 0,

    /// The package is sent to the client if another player (entity) enters their loading zone.
    SpawnEntity = 1,

    /// Remove entities by provide prefiexed array of id's
    RemoveEntities = 2,

    /// Applies to all players except oneself
    UpdateEntityPositionAndDirection = 3,

    /// When another player (entity, snake) dies for any reason,
    /// all the apples they have eaten fall into the game world.
    /// This package is needed to deliver information about this
    /// to customers in a compact form.
    AppleSpawnButch = 4,

    /// The package needed for the server to calculate
    /// the number of chunks that need to be sent to the client.
    ///
    /// **TODO:** It is assumed that later I will be able
    /// to calculate the window size on the client and send
    /// this value to the server at the `Configure` and `Play`
    /// stages when the window size changes.
    ///
    /// *window - refers to the size of a regular terminal/stdout window.
    SetDrawDistance = 5,
}

// -- Type-safety aliases --
// (empty structs haven't weight, its just like marker in compile time)
pub struct Login;
pub struct SetDrawDistanceConfigure;
pub struct TurnSnake;

pub struct LoginSuccess;
pub struct ConfigureAcknowledged;
pub struct SynchonizePositionAndDirection;
pub struct SpawnEntity;
pub struct RemoveEntities;
pub struct UpdateEntityPositionAndDirection;
pub struct AppleSpawnButch;
pub struct SetDrawDistancePlay;
// -- Type-safety aliases end --

// -- Packet payloads --
pub struct LoginData {}
pub struct SetDrawDistanceConfigureData {}
pub struct TurnSnakeData {}

pub struct LoginSuccessData;
pub struct ConfigureAcknowledgedData;
pub struct SynchonizePositionAndDirectionData {
    pub x: UVarInt,
    pub y: UVarInt,
    pub direction: Byte,
}
pub struct SpawnEntityData {
    pub id: Id,
    pub x: UVarInt,
    pub y: UVarInt,
    pub direction: Byte,
}
pub struct RemoveEntitiesData {
    pub entities: PrefixedArray<Id>,
}
pub struct UpdateEntityPositionAndDirectionData {}
pub struct AppleSpawnButchData {}
pub struct SetDrawDistancePlayData {}
// -- Packet payloads end --

/*
struct Login {
    username: String,
}

struct LoginSuccess;

// talk position of player
struct SynchonizePositionAndDirection {
    x: u32, // 1-4 bytes
    y: u32, // 1-4 bytes
    direction: Direction // 1 bytes always
}

struct SpawnSnake {
    id: u64 // 4-8 bytes
    x:  u32 // 1-4 bytes
    y:  u32 // 1 - 4
    direction: Direction // 1
} // 7-17 bytes

struct RemoveSnake {
    array_of_ids: Array<id>
} // id = u64 = 4 bytes +-

struct UpdateApple {
    pos_x: u32 //(1-4 bytes)
    pos_y: u32 //(1-4 bytes)
    state: //Eaten or Spawn 0 or 1 (1 bytes)
} // 3-9 bytes

struct AppleSpawnButch {
    pos_x: u32,
    pos_y: u32,
    directions: Vec<u8> // 2 bits = 1 flag Direction
}
*/
