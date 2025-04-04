// This file is @generated by prost-build.
#[derive(proto_derive::CmdID)]
#[cmdid(24012)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc24012 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shipinchallenge {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub hp_rant: u32,
    #[prost(message, required, tag = "3")]
    pub ship_info: super::common::Shipinfo,
}
#[derive(proto_derive::CmdID)]
#[cmdid(24021)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc24021 {
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub pass_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, repeated, tag = "3")]
    pub awards: ::prost::alloc::vec::Vec<super::common::Kvdata>,
    #[prost(message, repeated, tag = "2")]
    pub times: ::prost::alloc::vec::Vec<super::common::Kvdata>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(24022)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs24022 {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub challengeids: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Userchallengeinfo {
    #[prost(message, repeated, tag = "3")]
    pub groupinc_list: ::prost::alloc::vec::Vec<Groupinfoinchallenge>,
    #[prost(uint32, required, tag = "2")]
    pub level: u32,
    #[prost(uint32, required, tag = "5")]
    pub issl: u32,
    #[prost(uint32, required, tag = "1")]
    pub current_score: u32,
    #[prost(uint32, required, tag = "6")]
    pub season_id: u32,
    #[prost(uint32, required, tag = "4")]
    pub mode: u32,
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub dungeon_id_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "8")]
    pub buff_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(24100)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc24100 {
    #[prost(uint32, required, tag = "1")]
    pub score: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(24004)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs24004 {
    #[prost(uint32, required, tag = "1")]
    pub activity_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(24020)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs24020 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(24002)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs24002 {
    #[prost(message, repeated, tag = "2")]
    pub group_list: ::prost::alloc::vec::Vec<Groupinfo>,
    #[prost(uint32, required, tag = "3")]
    pub mode: u32,
    #[prost(uint32, required, tag = "1")]
    pub activity_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(24011)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs24011 {
    #[prost(uint32, required, tag = "2")]
    pub mode: u32,
    #[prost(uint32, required, tag = "1")]
    pub activity_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(24023)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc24023 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, repeated, tag = "2")]
    pub drop_list: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(24010)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc24010 {
    #[prost(uint32, required, tag = "1")]
    pub score: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(24003)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc24003 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commanderinchallenge {
    #[prost(uint32, required, tag = "1")]
    pub pos: u32,
    #[prost(message, required, tag = "2")]
    pub commanderinfo: super::common::Commanderinfo,
}
#[derive(proto_derive::CmdID)]
#[cmdid(24005)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc24005 {
    #[prost(message, required, tag = "2")]
    pub current_challenge: Challengeinfo,
    #[prost(message, repeated, tag = "3")]
    pub user_challenge: ::prost::alloc::vec::Vec<Userchallengeinfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Groupinfoinchallenge {
    #[prost(message, repeated, tag = "3")]
    pub commanders: ::prost::alloc::vec::Vec<Commanderinchallenge>,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(message, repeated, tag = "2")]
    pub ships: ::prost::alloc::vec::Vec<Shipinchallenge>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Groupinfo {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub ship_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "3")]
    pub commanders: ::prost::alloc::vec::Vec<super::common::Commandersinfo>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Challengeinfo {
    #[prost(uint32, required, tag = "5")]
    pub season_id: u32,
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub dungeon_id_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "1")]
    pub season_max_score: u32,
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub buff_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "2")]
    pub activity_max_score: u32,
    #[prost(uint32, required, tag = "3")]
    pub season_max_level: u32,
    #[prost(uint32, required, tag = "4")]
    pub activity_max_level: u32,
}