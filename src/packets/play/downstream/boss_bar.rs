use minecraft_net_proc::{Field, Packet, VarIntEnum};
use crate::fields::types::TextComponent;

VarIntEnum!(BossBarColor, {
    Pink, Blue, Red, Green, Yellow, Purple, White
});
VarIntEnum!(BossBarDevisions, {
    NoDivisions, Six, Ten, Twelve, Twenty,
});
Field!(BossBarAdd, {
    title: TextComponent,
    health: Float,
    color: BossBarColor,
    division: BossBarDevisions,
    flags: UByte,
});
Field!(BossBarUpdateStyle, {
    color: VarInt
    dividers: VarInt
});
VarIntEnum!(BossBarActions, {
    Add: BossBarAdd
    Remove
    UpdateHealth: Float
    UpdateTitle: TextComponent
    UpdateStyle: BossBarUpdateStyle
    UpdateFlags: UByte
});

Packet!(BossBar, 0x0A, {
    uuid: UUID
    action: VarInt
    action_data: BossBarActions
});