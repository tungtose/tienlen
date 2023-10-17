use naia_bevy_shared::{
    Channel, ChannelDirection, ChannelMode, Protocol, ProtocolPlugin, ReliableSettings,
    TickBufferSettings,
};

#[derive(Channel)]
pub struct PlayerCommandChannel;

#[derive(Channel)]
pub struct PlayerActionChannel;

#[derive(Channel)]
pub struct GameSystemChannel;

#[derive(Channel)]
pub struct EntityAssignmentChannel;

// Plugin
pub struct ChannelsPlugin;

impl ProtocolPlugin for ChannelsPlugin {
    fn build(&self, protocol: &mut Protocol) {
        protocol
            .add_channel::<GameSystemChannel>(
                ChannelDirection::ServerToClient,
                ChannelMode::UnorderedReliable(ReliableSettings::default()),
            )
            .add_channel::<PlayerActionChannel>(
                ChannelDirection::ClientToServer,
                ChannelMode::UnorderedReliable(ReliableSettings::default()),
            )
            .add_channel::<PlayerCommandChannel>(
                ChannelDirection::ClientToServer,
                ChannelMode::TickBuffered(TickBufferSettings::default()),
            )
            .add_channel::<EntityAssignmentChannel>(
                ChannelDirection::ServerToClient,
                ChannelMode::UnorderedReliable(ReliableSettings::default()),
            );
    }
}
