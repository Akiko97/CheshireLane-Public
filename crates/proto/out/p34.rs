// This file is @generated by prost-build.
#[derive(proto_derive::CmdID)]
#[cmdid(34003)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34003 {
    #[prost(uint32, required, tag = "1")]
    pub group_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub target_pt: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34511)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34511 {
    #[prost(uint32, required, tag = "1")]
    pub boss_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34502)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc34502 {
    #[prost(uint32, required, tag = "9")]
    pub summon_free: u32,
    #[prost(uint32, required, tag = "10")]
    pub auto_fight_finish_time: u32,
    #[prost(uint32, required, tag = "7")]
    pub summon_pt_daily_acc: u32,
    #[prost(message, optional, tag = "3")]
    pub self_boss: ::core::option::Option<WorldbossInfo>,
    #[prost(uint32, required, tag = "14")]
    pub friend_support: u32,
    #[prost(uint32, required, tag = "1")]
    pub fight_count: u32,
    #[prost(uint32, required, tag = "8")]
    pub summon_pt_old_daily_acc: u32,
    #[prost(uint32, required, tag = "6")]
    pub summon_pt_old: u32,
    #[prost(uint32, required, tag = "13")]
    pub guild_support: u32,
    #[prost(message, repeated, tag = "4")]
    pub other_boss: ::prost::alloc::vec::Vec<WorldbossInfo>,
    #[prost(uint32, required, tag = "5")]
    pub summon_pt: u32,
    #[prost(uint32, required, tag = "15")]
    pub world_support: u32,
    #[prost(uint32, required, tag = "12")]
    pub auto_fight_max_damage: u32,
    #[prost(uint32, required, tag = "2")]
    pub fight_count_update_time: u32,
    #[prost(uint32, required, tag = "11")]
    pub default_boss_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34519)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34519 {
    #[prost(uint32, required, tag = "1")]
    pub boss_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub user_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34528)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc34528 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34514)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc34514 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34527)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34527 {
    #[prost(uint32, required, tag = "1")]
    pub boss_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34004)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc34004 {
    #[prost(message, repeated, tag = "2")]
    pub drop_list: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34520)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc34520 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, repeated, tag = "2")]
    pub ship_list: ::prost::alloc::vec::Vec<super::common::Shipinfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34509)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34509 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetaShipInfo {
    #[prost(uint32, required, tag = "2")]
    pub pt: u32,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub fetch_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "1")]
    pub group_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34508)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc34508 {
    #[prost(uint32, required, tag = "2")]
    pub hp: u32,
    #[prost(uint32, required, tag = "1")]
    pub boss_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34522)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc34522 {
    #[prost(message, optional, tag = "2")]
    pub boss: ::core::option::Option<WorldbossInfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34523)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34523 {
    #[prost(uint32, required, tag = "1")]
    pub boss_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34001)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs34001 {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub group_id: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34507)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc34507 {
    #[prost(message, required, tag = "1")]
    pub boss_info: WorldbossInfo,
    #[prost(message, required, tag = "2")]
    pub user_info: super::common::Usersimpleinfo,
    #[prost(uint32, required, tag = "3")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34512)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc34512 {
    #[prost(message, repeated, tag = "2")]
    pub drops: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34505)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34505 {
    #[prost(uint32, required, tag = "1")]
    pub boss_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34513)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34513 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34515)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34515 {
    #[prost(uint32, optional, tag = "2")]
    pub last_time: ::core::option::Option<u32>,
    #[prost(uint32, required, tag = "1")]
    pub boss_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34525)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34525 {
    #[prost(uint32, required, tag = "1")]
    pub boss_id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldbossRank {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "3")]
    pub damage: u32,
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34526)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc34526 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, required, tag = "3")]
    pub damage: u32,
    #[prost(uint32, required, tag = "2")]
    pub count: u32,
    #[prost(uint32, required, tag = "4")]
    pub oil: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34501)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34501 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34521)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs34521 {
    #[prost(uint32, required, tag = "1")]
    pub template_id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct WorldbossSimple {
    #[prost(uint32, required, tag = "3")]
    pub rank_count: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub hp: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34002)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc34002 {
    #[prost(message, repeated, tag = "1")]
    pub meta_ship_list: ::prost::alloc::vec::Vec<MetaShipInfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34517)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs34517 {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub boss_id: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct WorldbossInfo {
    #[prost(uint32, required, tag = "3")]
    pub lv: u32,
    #[prost(uint32, required, tag = "6")]
    pub last_time: u32,
    #[prost(uint32, required, tag = "8")]
    pub fight_count: u32,
    #[prost(uint32, required, tag = "5")]
    pub owner: u32,
    #[prost(uint32, required, tag = "4")]
    pub hp: u32,
    #[prost(uint32, required, tag = "2")]
    pub template_id: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "9")]
    pub rank_count: u32,
    #[prost(uint32, required, tag = "7")]
    pub kill_time: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34510)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc34510 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34516)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc34516 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34524)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc34524 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, required, tag = "2")]
    pub auto_fight_finish_time: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34503)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs34503 {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub user_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34518)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc34518 {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<WorldbossSimple>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34506)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc34506 {
    #[prost(message, repeated, tag = "1")]
    pub rank_list: ::prost::alloc::vec::Vec<WorldbossRank>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(34504)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc34504 {
    #[prost(message, repeated, tag = "1")]
    pub boss_list: ::prost::alloc::vec::Vec<WorldbossInfo>,
}