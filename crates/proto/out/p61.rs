// This file is @generated by prost-build.
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShipidPosInfo {
    #[prost(uint32, required, tag = "3")]
    pub last_time: u32,
    #[prost(uint32, required, tag = "1")]
    pub pos: u32,
    #[prost(message, required, tag = "2")]
    pub ship: super::common::Shipinfo,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61014)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc61014 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61003)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs61003 {
    #[prost(message, repeated, tag = "1")]
    pub ship_ids: ::prost::alloc::vec::Vec<ShipidPos>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61005)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61005 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61016)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc61016 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61022)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc61022 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61026)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc61026 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61012)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc61012 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, repeated, tag = "3")]
    pub recommends: ::prost::alloc::vec::Vec<TeamCell>,
    #[prost(message, repeated, tag = "2")]
    pub ships: ::prost::alloc::vec::Vec<TeamChunk>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61032)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc61032 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ShipInEvent {
    #[prost(uint32, required, tag = "1")]
    pub user_id: u32,
    #[prost(uint32, required, tag = "4")]
    pub skin: u32,
    #[prost(uint32, required, tag = "3")]
    pub template_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub ship_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61007)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs61007 {
    #[prost(uint32, required, tag = "1")]
    pub event_tid: u32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub ship_ids: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61019)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs61019 {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub ids: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61024)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc61024 {
    #[prost(message, optional, tag = "2")]
    pub event_info: ::core::option::Option<EventBase>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, optional, tag = "3")]
    pub completed_info: ::core::option::Option<EventBaseCompleted>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61027)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61027 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61010)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc61010 {
    #[prost(message, repeated, tag = "2")]
    pub person_ships: ::prost::alloc::vec::Vec<ShipidPosInfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61001)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61001 {
    #[prost(uint32, required, tag = "1")]
    pub chapter_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61021)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc61021 {
    #[prost(uint32, required, tag = "1")]
    pub user_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61031)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61031 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61028)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc61028 {
    #[prost(message, required, tag = "2")]
    pub boss_event: EventBoss,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61034)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc61034 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct EventBoss {
    #[prost(uint32, required, tag = "3")]
    pub hp: u32,
    #[prost(uint32, required, tag = "1")]
    pub boss_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub damage: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TeamCell {
    #[prost(uint32, required, tag = "1")]
    pub user_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub ship_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61029)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61029 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61025)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs61025 {
    #[prost(message, repeated, tag = "1")]
    pub perf: ::prost::alloc::vec::Vec<EventPerformance>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct EventNode {
    #[prost(uint32, required, tag = "1")]
    pub position: u32,
    #[prost(uint32, required, tag = "3")]
    pub status: u32,
    #[prost(uint32, required, tag = "2")]
    pub node_id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Keyvalue {
    #[prost(uint32, required, tag = "1")]
    pub key: u32,
    #[prost(uint32, required, tag = "2")]
    pub value: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bosseventfleet {
    #[prost(uint32, required, tag = "1")]
    pub fleet_id: u32,
    #[prost(message, repeated, tag = "2")]
    pub ships: ::prost::alloc::vec::Vec<TeamCell>,
    #[prost(message, repeated, tag = "3")]
    pub commanders: ::prost::alloc::vec::Vec<super::common::Commandersinfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61013)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs61013 {
    #[prost(message, repeated, tag = "1")]
    pub fleet: ::prost::alloc::vec::Vec<Bosseventfleet>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61006)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc61006 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, optional, tag = "2")]
    pub operation: ::core::option::Option<CurrentOperation>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61035)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61035 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBase {
    #[prost(uint32, required, tag = "1")]
    pub event_id: u32,
    #[prost(uint32, required, tag = "3")]
    pub start_time: u32,
    #[prost(message, repeated, tag = "8")]
    pub eventnodes: ::prost::alloc::vec::Vec<EventNode>,
    #[prost(uint32, required, tag = "9")]
    pub efficiency: u32,
    #[prost(message, repeated, tag = "5")]
    pub shipinevent: ::prost::alloc::vec::Vec<ShipInEvent>,
    #[prost(uint32, required, tag = "2")]
    pub position: u32,
    #[prost(message, repeated, tag = "7")]
    pub attr_count_list: ::prost::alloc::vec::Vec<Keyvalue>,
    #[prost(uint32, required, tag = "4")]
    pub complete_time: u32,
    #[prost(message, repeated, tag = "6")]
    pub attr_acc_list: ::prost::alloc::vec::Vec<Keyvalue>,
    #[prost(message, repeated, tag = "10")]
    pub personship: ::prost::alloc::vec::Vec<PersonShipInPage>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PersonShipInPage {
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub ship_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "1")]
    pub page_id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ReportNode {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub status: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61011)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61011 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61038)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc61038 {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<RankInfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61018)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc61018 {
    #[prost(message, repeated, tag = "1")]
    pub reports: ::prost::alloc::vec::Vec<Report>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61020)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc61020 {
    #[prost(message, repeated, tag = "2")]
    pub drop_list: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TeamChunk {
    #[prost(uint32, required, tag = "1")]
    pub user_id: u32,
    #[prost(message, repeated, tag = "2")]
    pub ships: ::prost::alloc::vec::Vec<super::common::Shipinfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61002)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc61002 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61004)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc61004 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61033)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61033 {
    #[prost(uint32, required, tag = "1")]
    pub recommend_uid: u32,
    #[prost(uint32, required, tag = "2")]
    pub recommend_shipid: u32,
    #[prost(uint32, required, tag = "3")]
    pub cmd: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61030)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc61030 {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<RankInfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61009)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61009 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61036)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc61036 {
    #[prost(message, repeated, tag = "1")]
    pub recommends: ::prost::alloc::vec::Vec<TeamCell>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61017)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61017 {
    #[prost(uint32, required, tag = "1")]
    pub index: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61037)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61037 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrentOperation {
    #[prost(message, repeated, tag = "5")]
    pub perfs: ::prost::alloc::vec::Vec<EventPerformance>,
    #[prost(uint32, required, tag = "2")]
    pub start_time: u32,
    #[prost(uint32, required, tag = "8")]
    pub daily_count: u32,
    #[prost(message, repeated, tag = "9")]
    pub fleets: ::prost::alloc::vec::Vec<Bosseventfleet>,
    #[prost(message, repeated, tag = "3")]
    pub base_events: ::prost::alloc::vec::Vec<EventBase>,
    #[prost(message, optional, tag = "4")]
    pub boss_event: ::core::option::Option<EventBoss>,
    #[prost(uint32, required, tag = "10")]
    pub join_times: u32,
    #[prost(uint32, required, tag = "11")]
    pub is_participant: u32,
    #[prost(message, repeated, tag = "6")]
    pub formation_time: ::prost::alloc::vec::Vec<Keyvalue>,
    #[prost(uint32, required, tag = "1")]
    pub operation_id: u32,
    #[prost(message, repeated, tag = "7")]
    pub completed_events: ::prost::alloc::vec::Vec<EventBaseCompleted>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Report {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub event_id: u32,
    #[prost(uint32, required, tag = "6")]
    pub status: u32,
    #[prost(message, repeated, tag = "5")]
    pub nodes: ::prost::alloc::vec::Vec<ReportNode>,
    #[prost(uint32, required, tag = "4")]
    pub score: u32,
    #[prost(uint32, required, tag = "3")]
    pub event_type: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ShipidPos {
    #[prost(uint32, required, tag = "1")]
    pub pos: u32,
    #[prost(uint32, required, tag = "2")]
    pub ship_id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct RankInfo {
    #[prost(uint32, required, tag = "1")]
    pub user_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub damage: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61008)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc61008 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61023)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61023 {
    #[prost(uint32, required, tag = "1")]
    pub event_tid: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct EventPerformance {
    #[prost(uint32, required, tag = "2")]
    pub index: u32,
    #[prost(uint32, required, tag = "1")]
    pub event_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(61015)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs61015 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct EventBaseCompleted {
    #[prost(uint32, required, tag = "1")]
    pub event_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub position: u32,
}