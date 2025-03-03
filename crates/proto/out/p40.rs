// This file is @generated by prost-build.
#[derive(proto_derive::CmdID)]
#[cmdid(40003)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs40003 {
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub kill_id_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(string, optional, tag = "10")]
    pub file_check: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, required, tag = "18")]
    pub auto_after: u32,
    #[prost(uint32, required, tag = "8")]
    pub bot_percentage: u32,
    #[prost(uint32, repeated, packed = "false", tag = "14")]
    pub commander_id_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "13")]
    pub data2: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub score: ::core::option::Option<u32>,
    #[prost(uint32, required, tag = "9")]
    pub extra_param: u32,
    #[prost(uint32, required, tag = "3")]
    pub key: u32,
    #[prost(uint32, required, tag = "16")]
    pub auto_before: u32,
    #[prost(uint32, optional, tag = "11")]
    pub boss_hp: ::core::option::Option<u32>,
    #[prost(uint32, required, tag = "2")]
    pub data: u32,
    #[prost(uint32, required, tag = "1")]
    pub system: u32,
    #[prost(message, repeated, tag = "12")]
    pub enemy_info: ::prost::alloc::vec::Vec<Enemyinfo>,
    #[prost(uint32, required, tag = "17")]
    pub auto_switch_time: u32,
    #[prost(message, repeated, tag = "15")]
    pub otherstatistics: ::prost::alloc::vec::Vec<Statisticsinfo>,
    #[prost(message, repeated, tag = "5")]
    pub statistics: ::prost::alloc::vec::Vec<Statisticsinfo>,
    #[prost(uint32, required, tag = "7")]
    pub total_time: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(40004)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc40004 {
    #[prost(uint32, required, tag = "4")]
    pub player_exp: u32,
    #[prost(uint32, required, tag = "6")]
    pub mvp: u32,
    #[prost(message, repeated, tag = "3")]
    pub extra_drop_info: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(message, repeated, tag = "2")]
    pub drop_info: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(message, repeated, tag = "5")]
    pub ship_exp_list: ::prost::alloc::vec::Vec<ShipExp>,
    #[prost(message, repeated, tag = "8")]
    pub hp_drop_info: ::prost::alloc::vec::Vec<Hpdropinfo>,
    #[prost(message, repeated, tag = "7")]
    pub commander_exp: ::prost::alloc::vec::Vec<CommanderExp>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Dropperformance {
    #[prost(uint32, required, tag = "1")]
    pub enemy_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub resource_num: u32,
    #[prost(uint32, required, tag = "3")]
    pub other_num: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ShipExp {
    #[prost(uint32, required, tag = "1")]
    pub ship_id: u32,
    #[prost(uint32, required, tag = "3")]
    pub intimacy: u32,
    #[prost(uint32, required, tag = "2")]
    pub exp: u32,
    #[prost(uint32, required, tag = "4")]
    pub energy: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuickReward {
    #[prost(message, repeated, tag = "1")]
    pub drop_list: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(40009)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc40009 {
    #[prost(uint32, required, tag = "2")]
    pub system: u32,
    #[prost(uint32, optional, tag = "3")]
    pub arg1: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "1")]
    pub re40004: ::prost::alloc::vec::Vec<Sc40004>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(40001)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs40001 {
    #[prost(uint32, required, tag = "1")]
    pub system: u32,
    #[prost(uint32, required, tag = "3")]
    pub data: u32,
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub data2: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub ship_id_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "5")]
    pub other_ship_id_list: ::prost::alloc::vec::Vec<Othershipid>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Othershipid {
    #[prost(uint32, required, tag = "1")]
    pub ship_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub user_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(40006)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc40006 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(40007)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs40007 {
    #[prost(uint32, required, tag = "1")]
    pub system: u32,
    #[prost(uint32, required, tag = "3")]
    pub cnt: u32,
    #[prost(uint32, required, tag = "2")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct CommanderExp {
    #[prost(uint32, required, tag = "2")]
    pub exp: u32,
    #[prost(uint32, required, tag = "1")]
    pub commander_id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Enemyinfo {
    #[prost(uint32, required, tag = "1")]
    pub enemy_id: u32,
    #[prost(uint32, required, tag = "3")]
    pub total_hp: u32,
    #[prost(uint32, required, tag = "2")]
    pub damage_taken: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Statisticsinfo {
    #[prost(uint32, required, tag = "6")]
    pub ship_gear_score: u32,
    #[prost(uint32, required, tag = "1")]
    pub ship_id: u32,
    #[prost(uint32, required, tag = "4")]
    pub hp_rest: u32,
    #[prost(uint32, required, tag = "2")]
    pub damage_cause: u32,
    #[prost(uint32, required, tag = "3")]
    pub damage_caused: u32,
    #[prost(uint32, required, tag = "5")]
    pub max_damage_once: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Hpdropinfo {
    #[prost(message, repeated, tag = "2")]
    pub drop_info: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(uint32, required, tag = "1")]
    pub hp: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(40005)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs40005 {
    #[prost(uint32, required, tag = "1")]
    pub system: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(40002)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc40002 {
    #[prost(message, repeated, tag = "3")]
    pub drop_performance: ::prost::alloc::vec::Vec<Dropperformance>,
    #[prost(uint32, optional, tag = "2")]
    pub key: ::core::option::Option<u32>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(40008)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc40008 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, repeated, tag = "2")]
    pub reward_list: ::prost::alloc::vec::Vec<QuickReward>,
}