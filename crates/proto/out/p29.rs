// This file is @generated by prost-build.
#[derive(proto_derive::CmdID)]
#[cmdid(29091)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29091 {
    #[prost(uint32, required, tag = "2")]
    pub first_node: u32,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, required, tag = "3")]
    pub drop: Tbdrops,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29011)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29011 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29049)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29049 {
    #[prost(message, required, tag = "3")]
    pub res: Tbres,
    #[prost(message, required, tag = "2")]
    pub drop: Tbdrops,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbbenefit {
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub pendings: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "1")]
    pub actives: ::prost::alloc::vec::Vec<Tbbf>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29046)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29046 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29024)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc29024 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29003)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29003 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29031)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29031 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, required, tag = "2")]
    pub next_node: u32,
    #[prost(message, required, tag = "3")]
    pub drop: Tbdrops,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29014)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc29014 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, required, tag = "2")]
    pub first_node: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29022)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc29022 {
    #[prost(uint32, required, tag = "2")]
    pub talent: u32,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29090)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29090 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Tbdrop {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
    #[prost(int32, required, tag = "3")]
    pub number: i32,
    #[prost(uint32, required, tag = "2")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29006)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc29006 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Tbbf {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub round: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbdrops {
    #[prost(message, repeated, tag = "2")]
    pub benefit_drop: ::prost::alloc::vec::Vec<Tbdrop>,
    #[prost(message, repeated, tag = "1")]
    pub base_drop: ::prost::alloc::vec::Vec<Tbdrop>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29069)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29069 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, required, tag = "2")]
    pub first_node: u32,
    #[prost(message, required, tag = "3")]
    pub drop: Tbdrops,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29093)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29093 {
    #[prost(message, required, tag = "2")]
    pub tb: Tbinfo,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29015)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29015 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29005)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29005 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub ending_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29020)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29020 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub talents: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbfsm {
    #[prost(uint32, required, tag = "2")]
    pub current_node: u32,
    #[prost(uint32, required, tag = "1")]
    pub system_no: u32,
    #[prost(message, repeated, tag = "3")]
    pub cache: ::prost::alloc::vec::Vec<Tbfsmcache>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29016)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29016 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub chats: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbres {
    #[prost(message, repeated, tag = "1")]
    pub attrs: ::prost::alloc::vec::Vec<super::common::Kvdata>,
    #[prost(message, repeated, tag = "2")]
    pub resource: ::prost::alloc::vec::Vec<super::common::Kvdata>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29092)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29092 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbsite {
    #[prost(message, repeated, tag = "2")]
    pub work_counter: ::prost::alloc::vec::Vec<super::common::Kvdata>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub works: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub characters: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "4")]
    pub event_counter: ::prost::alloc::vec::Vec<super::common::Kvdata>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29008)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29008 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, required, tag = "2")]
    pub tb: Tbinfo,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29002)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29002 {
    #[prost(message, required, tag = "3")]
    pub permanent: Tbpermanent,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, required, tag = "2")]
    pub tb: Tbinfo,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29040)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs29040 {
    #[prost(message, repeated, tag = "2")]
    pub plans: ::prost::alloc::vec::Vec<super::common::Kvdata>,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29048)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29048 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbpermanent {
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub active_endings: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "1")]
    pub ng_plus_count: u32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub polaroids: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub endings: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbfsmcachechat {
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub chats: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "1")]
    pub finished: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29026)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29026 {
    #[prost(message, required, tag = "3")]
    pub drop: Tbdrops,
    #[prost(uint32, required, tag = "2")]
    pub first_node: u32,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29060)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29060 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbinfo {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(message, required, tag = "4")]
    pub res: Tbres,
    #[prost(message, required, tag = "5")]
    pub talent: Tbtalent,
    #[prost(message, repeated, tag = "8")]
    pub evaluations: ::prost::alloc::vec::Vec<super::common::Kvdata>,
    #[prost(string, required, tag = "9")]
    pub name: ::prost::alloc::string::String,
    #[prost(message, required, tag = "6")]
    pub plan: Tbplan,
    #[prost(uint32, required, tag = "10")]
    pub favor_lv: u32,
    #[prost(message, required, tag = "3")]
    pub round: Tbround,
    #[prost(message, required, tag = "2")]
    pub fsm: Tbfsm,
    #[prost(message, required, tag = "11")]
    pub benefit: Tbbenefit,
    #[prost(message, required, tag = "7")]
    pub site: Tbsite,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29033)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29033 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, required, tag = "2")]
    pub fsm: Tbfsm,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29004)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29004 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub endings: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29013)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29013 {
    #[prost(uint32, required, tag = "2")]
    pub rank: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29043)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29043 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, required, tag = "2")]
    pub first_node: u32,
    #[prost(message, required, tag = "3")]
    pub drop: Tbdrops,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29027)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29027 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29010)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc29010 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29018)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29018 {
    #[prost(uint32, required, tag = "2")]
    pub first_node: u32,
    #[prost(message, required, tag = "3")]
    pub drop: Tbdrops,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29021)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29021 {
    #[prost(uint32, required, tag = "2")]
    pub talent: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29042)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29042 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29045)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc29045 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29041)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29041 {
    #[prost(message, repeated, tag = "2")]
    pub plans: ::prost::alloc::vec::Vec<super::common::Kvdata>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, required, tag = "3")]
    pub drop: Tbdrops,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29067)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29067 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, required, tag = "2")]
    pub drop: Tbdrops,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29070)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29070 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub work_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29001)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29001 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29065)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29065 {
    #[prost(message, required, tag = "3")]
    pub drop: Tbdrops,
    #[prost(uint32, required, tag = "2")]
    pub first_node: u32,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29068)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29068 {
    #[prost(uint32, required, tag = "2")]
    pub character: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbtalent {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub talents: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29012)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc29012 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, required, tag = "2")]
    pub first_node: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbplan {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub plan_upgrade: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29023)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29023 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub talent: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbfsmcachesite {
    #[prost(message, required, tag = "4")]
    pub state: super::common::Kvdata,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub shops: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub events: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "3")]
    pub buys: ::prost::alloc::vec::Vec<super::common::Kvdata>,
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub character_this_round: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29066)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29066 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub shop: u32,
    #[prost(uint32, required, tag = "3")]
    pub num: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29007)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29007 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbfsmcacheplan {
    #[prost(message, repeated, tag = "2")]
    pub plans: ::prost::alloc::vec::Vec<super::common::Kvdata>,
    #[prost(uint32, required, tag = "1")]
    pub cur_index: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29017)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29017 {
    #[prost(uint32, required, tag = "2")]
    pub chat_id: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29044)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs29044 {
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub plan_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29047)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29047 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, required, tag = "3")]
    pub drop: Tbdrops,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29063)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29063 {
    #[prost(uint32, required, tag = "2")]
    pub first_node: u32,
    #[prost(message, required, tag = "3")]
    pub drop: Tbdrops,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29071)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc29071 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29030)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29030 {
    #[prost(uint32, optional, tag = "2")]
    pub branch: ::core::option::Option<u32>,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29028)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29028 {
    #[prost(message, required, tag = "2")]
    pub drop: Tbdrops,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29064)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29064 {
    #[prost(uint32, required, tag = "2")]
    pub event: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Tbround {
    #[prost(uint32, required, tag = "1")]
    pub round: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29019)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29019 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29025)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29025 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbfsmcache {
    #[prost(message, repeated, tag = "1")]
    pub cache_plan: ::prost::alloc::vec::Vec<Tbfsmcacheplan>,
    #[prost(message, repeated, tag = "5")]
    pub cache_end: ::prost::alloc::vec::Vec<Tbfsmcacheend>,
    #[prost(message, repeated, tag = "3")]
    pub cache_site: ::prost::alloc::vec::Vec<Tbfsmcachesite>,
    #[prost(message, repeated, tag = "4")]
    pub cache_chat: ::prost::alloc::vec::Vec<Tbfsmcachechat>,
    #[prost(message, repeated, tag = "2")]
    pub cache_talent: ::prost::alloc::vec::Vec<Tbfsmcachetalent>,
    #[prost(message, repeated, tag = "6")]
    pub cache_mind: ::prost::alloc::vec::Vec<Tbfsmcachemind>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29062)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29062 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub work_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29009)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs29009 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29061)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc29061 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, optional, tag = "2")]
    pub fsm_site: ::core::option::Option<Tbfsmcachesite>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub characters: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, required, tag = "4")]
    pub drop: Tbdrops,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbfsmcacheend {
    #[prost(uint32, required, tag = "2")]
    pub select: u32,
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub ends: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tbfsmcachetalent {
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub retalents: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub talents: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "1")]
    pub finished: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(29032)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs29032 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Tbfsmcachemind {}