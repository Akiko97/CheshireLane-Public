// This file is @generated by prost-build.
#[derive(proto_derive::CmdID)]
#[cmdid(33510)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc33510 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Mapinfo {
    #[prost(message, repeated, tag = "2")]
    pub cell_list: ::prost::alloc::vec::Vec<Chaptercellinfo>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub state_flag: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "4")]
    pub land_list: ::prost::alloc::vec::Vec<Landinfo>,
    #[prost(message, repeated, tag = "5")]
    pub pos_list: ::prost::alloc::vec::Vec<Worldposinfo>,
    #[prost(message, required, tag = "1")]
    pub id: Worldmapid,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33206)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc33206 {
    #[prost(message, optional, tag = "2")]
    pub task: ::core::option::Option<TaskInfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Worldposinfo {
    #[prost(message, repeated, tag = "2")]
    pub item_list: ::prost::alloc::vec::Vec<Worlditeminfo>,
    #[prost(message, required, tag = "1")]
    pub pos: Chaptercellpos,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33302)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33302 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, repeated, tag = "2")]
    pub drop_list: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BuffInfo {
    #[prost(uint32, required, tag = "5")]
    pub step: u32,
    #[prost(uint32, required, tag = "3")]
    pub timestamp: u32,
    #[prost(uint32, required, tag = "4")]
    pub round: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub stack: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33603)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33603 {
    #[prost(message, repeated, tag = "2")]
    pub drops: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AiAct {
    #[prost(uint32, required, tag = "7")]
    pub r#type: u32,
    #[prost(message, optional, tag = "3")]
    pub target_pos: ::core::option::Option<Chaptercellpos>,
    #[prost(message, repeated, tag = "4")]
    pub move_path: ::prost::alloc::vec::Vec<Chaptercellpos>,
    #[prost(message, repeated, tag = "6")]
    pub ship_update: ::prost::alloc::vec::Vec<Shipinchapter>,
    #[prost(message, required, tag = "1")]
    pub ai_pos: Chaptercellpos,
    #[prost(message, repeated, tag = "8")]
    pub pos_list: ::prost::alloc::vec::Vec<Worldposinfo>,
    #[prost(uint32, optional, tag = "2")]
    pub strategy_id: ::core::option::Option<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33102)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33102 {
    #[prost(message, optional, tag = "2")]
    pub world: ::core::option::Option<Worldinfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub new_flag_port_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub port_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, required, tag = "4")]
    pub count_info: Countinfo,
    #[prost(message, repeated, tag = "3")]
    pub chapter_award: ::prost::alloc::vec::Vec<Chapterawardinfo>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Groupinfoupdate {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(message, repeated, tag = "2")]
    pub buff_list: ::prost::alloc::vec::Vec<BuffInfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33301)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs33301 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub count: u32,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub arg: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33508)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc33508 {
    #[prost(uint32, required, tag = "1")]
    pub boss_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub hp: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33405)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs33405 {
    #[prost(message, repeated, tag = "1")]
    pub fleet_list: ::prost::alloc::vec::Vec<FleetChange>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33114)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33114 {
    #[prost(uint32, required, tag = "1")]
    pub is_world_open: u32,
    #[prost(uint32, required, tag = "4")]
    pub progress: u32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub ship_id_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub cmd_id_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Worldmapid {
    #[prost(uint32, required, tag = "2")]
    pub template_id: u32,
    #[prost(uint32, required, tag = "1")]
    pub random_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33413)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs33413 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33111)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc33111 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Worldinfo {
    #[prost(uint32, required, tag = "14")]
    pub action_power_fetch_count: u32,
    #[prost(message, repeated, tag = "3")]
    pub group_list: ::prost::alloc::vec::Vec<Groupinchapter>,
    #[prost(message, repeated, tag = "17")]
    pub cd_list: ::prost::alloc::vec::Vec<super::common::Idtimeinfo>,
    #[prost(uint32, required, tag = "11")]
    pub action_power: u32,
    #[prost(uint32, required, tag = "13")]
    pub last_recover_timestamp: u32,
    #[prost(uint32, optional, tag = "5")]
    pub task_finish_count: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "9")]
    pub goods_list: ::prost::alloc::vec::Vec<GoodsInfo>,
    #[prost(uint32, required, tag = "7")]
    pub submarine_state: u32,
    #[prost(message, repeated, tag = "18")]
    pub buff_list: ::prost::alloc::vec::Vec<BuffInfo>,
    #[prost(uint32, required, tag = "15")]
    pub last_change_group_timestamp: u32,
    #[prost(uint32, repeated, packed = "false", tag = "20")]
    pub sairen_chapter: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "8")]
    pub item_list: ::prost::alloc::vec::Vec<WorldItemInfo>,
    #[prost(uint32, optional, tag = "4")]
    pub round: ::core::option::Option<u32>,
    #[prost(uint32, required, tag = "1")]
    pub map_id: u32,
    #[prost(message, repeated, tag = "6")]
    pub task_list: ::prost::alloc::vec::Vec<TaskInfo>,
    #[prost(uint32, required, tag = "12")]
    pub action_power_extra: u32,
    #[prost(uint32, optional, tag = "2")]
    pub time: ::core::option::Option<u32>,
    #[prost(uint32, required, tag = "16")]
    pub enter_map_id: u32,
    #[prost(message, repeated, tag = "19")]
    pub chapter_list: ::prost::alloc::vec::Vec<Worldmapid>,
    #[prost(message, repeated, tag = "21")]
    pub month_boss: ::prost::alloc::vec::Vec<super::common::Kvdata>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct WorldtargetProcess {
    #[prost(uint32, required, tag = "1")]
    pub trigger_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub count: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33106)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs33106 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33203)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33203 {
    #[prost(message, repeated, tag = "1")]
    pub update_list: ::prost::alloc::vec::Vec<TaskInfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33113)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33113 {
    #[prost(uint32, required, tag = "3")]
    pub time: u32,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, repeated, tag = "2")]
    pub drop_list: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(uint32, repeated, packed = "false", tag = "4")]
    pub sairen_chapter: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct WorldItemInfo {
    #[prost(uint32, required, tag = "2")]
    pub count: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33103)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs33103 {
    #[prost(uint32, optional, tag = "3")]
    pub act_arg_1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub act_arg_2: ::core::option::Option<u32>,
    #[prost(uint32, required, tag = "1")]
    pub act: u32,
    #[prost(message, repeated, tag = "5")]
    pub pos_list: ::prost::alloc::vec::Vec<Chaptercellpos>,
    #[prost(uint32, required, tag = "2")]
    pub group_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33000)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs33000 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Landinfo {
    #[prost(uint32, required, tag = "2")]
    pub r#type: u32,
    #[prost(message, required, tag = "3")]
    pub dir: Chaptercellpos,
    #[prost(uint32, required, tag = "4")]
    pub distance: u32,
    #[prost(message, required, tag = "1")]
    pub pos: Chaptercellpos,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct WorldbossInfo {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "3")]
    pub lv: u32,
    #[prost(uint32, required, tag = "4")]
    pub hp: u32,
    #[prost(uint32, required, tag = "5")]
    pub owner: u32,
    #[prost(uint32, required, tag = "6")]
    pub last_time: u32,
    #[prost(uint32, required, tag = "2")]
    pub template_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33404)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33404 {
    #[prost(message, repeated, tag = "2")]
    pub drop_list: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FleetChange {
    #[prost(uint32, required, tag = "1")]
    pub group_id: u32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub ship_id: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TaskInfo {
    #[prost(uint32, required, tag = "2")]
    pub progress: u32,
    #[prost(uint32, required, tag = "3")]
    pub accept_time: u32,
    #[prost(uint32, required, tag = "5")]
    pub event_map_id: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "4")]
    pub submite_time: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Chaptercellinfo {
    #[prost(message, required, tag = "1")]
    pub pos: Chaptercellpos,
    #[prost(uint32, optional, tag = "2")]
    pub discovered: ::core::option::Option<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortInfo {
    #[prost(uint32, required, tag = "4")]
    pub next_refresh_time: u32,
    #[prost(message, repeated, tag = "3")]
    pub goods_list: ::prost::alloc::vec::Vec<GoodsInfo>,
    #[prost(uint32, required, tag = "1")]
    pub port_id: u32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub task_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33207)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs33207 {
    #[prost(uint32, required, tag = "1")]
    pub task_id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33108)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs33108 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33104)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33104 {
    #[prost(uint32, optional, tag = "13")]
    pub action_power_extra: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "14")]
    pub target_list: ::prost::alloc::vec::Vec<Worldtarget>,
    #[prost(message, repeated, tag = "3")]
    pub drop_list: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(message, repeated, tag = "5")]
    pub ship_update: ::prost::alloc::vec::Vec<Shipinchapter>,
    #[prost(uint32, required, tag = "10")]
    pub event_id: u32,
    #[prost(message, repeated, tag = "15")]
    pub cmd_collection_list: ::prost::alloc::vec::Vec<Groupcmdcollection>,
    #[prost(message, repeated, tag = "6")]
    pub ai_act_list: ::prost::alloc::vec::Vec<AiAct>,
    #[prost(message, repeated, tag = "11")]
    pub pos_list: ::prost::alloc::vec::Vec<Worldposinfo>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, optional, tag = "4")]
    pub enter_map_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub move_path: ::prost::alloc::vec::Vec<Chaptercellpos>,
    #[prost(message, repeated, tag = "8")]
    pub land_list: ::prost::alloc::vec::Vec<Landinfo>,
    #[prost(message, optional, tag = "7")]
    pub id: ::core::option::Option<Worldmapid>,
    #[prost(uint32, optional, tag = "12")]
    pub action_power: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "9")]
    pub group_update: ::prost::alloc::vec::Vec<Groupinfoupdate>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33509)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs33509 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33407)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs33407 {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub ship_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33410)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33410 {
    #[prost(message, repeated, tag = "3")]
    pub group_list: ::prost::alloc::vec::Vec<Groupinchapter>,
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33204)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33204 {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub delete_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MapVision {
    #[prost(uint32, required, tag = "1")]
    pub map_id: u32,
    #[prost(message, repeated, tag = "2")]
    pub vision_list: ::prost::alloc::vec::Vec<Chaptercellpos>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33105)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33105 {
    #[prost(message, repeated, tag = "1")]
    pub pos_list: ::prost::alloc::vec::Vec<Worldposinfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33101)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs33101 {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub enter_map_id: u32,
    #[prost(uint32, required, tag = "4")]
    pub camp: u32,
    #[prost(message, repeated, tag = "3")]
    pub elite_fleet_list: ::prost::alloc::vec::Vec<Elitefleetinfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33414)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33414 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub task_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "3")]
    pub next_refresh_time: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33602)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs33602 {
    #[prost(message, repeated, tag = "1")]
    pub list: ::prost::alloc::vec::Vec<WorldtargetFetch>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33110)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs33110 {
    #[prost(uint32, required, tag = "2")]
    pub data: u32,
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33601)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33601 {
    #[prost(message, repeated, tag = "1")]
    pub target_list: ::prost::alloc::vec::Vec<Worldtarget>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33202)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33202 {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub append_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cmdcollection {
    #[prost(uint32, required, tag = "1")]
    pub group_id: u32,
    #[prost(message, required, boxed, tag = "2")]
    pub cmd_collection: ::prost::alloc::boxed::Box<Cmdcollection>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33401)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs33401 {
    #[prost(uint32, required, tag = "1")]
    pub map_id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct GoodsInfo {
    #[prost(uint32, required, tag = "1")]
    pub goods_id: u32,
    #[prost(uint32, required, tag = "2")]
    pub count: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33208)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33208 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, repeated, tag = "2")]
    pub drops: ::prost::alloc::vec::Vec<super::common::Dropinfo>,
    #[prost(uint32, required, tag = "4")]
    pub intimacy: u32,
    #[prost(uint32, required, tag = "3")]
    pub exp: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33107)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33107 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, optional, tag = "2")]
    pub map: ::core::option::Option<Mapinfo>,
    #[prost(uint32, optional, tag = "3")]
    pub is_reset: ::core::option::Option<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33403)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs33403 {
    #[prost(uint32, required, tag = "2")]
    pub shop_type: u32,
    #[prost(uint32, required, tag = "1")]
    pub shop_id: u32,
    #[prost(uint32, required, tag = "3")]
    pub count: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33408)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc33408 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33409)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs33409 {
    #[prost(message, repeated, tag = "1")]
    pub elite_fleet_list: ::prost::alloc::vec::Vec<Elitefleetinfo>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Worldtarget {
    #[prost(message, repeated, tag = "2")]
    pub process_list: ::prost::alloc::vec::Vec<WorldtargetProcess>,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33402)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33402 {
    #[prost(message, optional, tag = "1")]
    pub port: ::core::option::Option<PortInfo>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Strategyinfo {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub count: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Worlditeminfo {
    #[prost(uint32, optional, tag = "4")]
    pub item_data: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub buff_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub boss_hp: ::core::option::Option<u32>,
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub effect_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub item_flag: ::core::option::Option<u32>,
    #[prost(uint32, required, tag = "1")]
    pub item_type: u32,
    #[prost(uint32, optional, tag = "2")]
    pub item_id: ::core::option::Option<u32>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Countinfo {
    #[prost(uint32, required, tag = "3")]
    pub task_progress: u32,
    #[prost(uint32, required, tag = "1")]
    pub step_count: u32,
    #[prost(uint32, required, tag = "2")]
    pub treasure_count: u32,
    #[prost(uint32, required, tag = "4")]
    pub activate_count: u32,
    #[prost(uint32, repeated, packed = "false", tag = "5")]
    pub collection_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33406)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc33406 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Chaptercellpos {
    #[prost(uint32, required, tag = "1")]
    pub row: u32,
    #[prost(uint32, required, tag = "2")]
    pub column: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33205)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs33205 {
    #[prost(uint32, required, tag = "1")]
    pub task_id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Shipinchapter {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub hp_rant: u32,
    #[prost(message, repeated, tag = "3")]
    pub buff_list: ::prost::alloc::vec::Vec<BuffInfo>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Groupcmdcollection {
    #[prost(uint32, required, tag = "1")]
    pub group_id: u32,
    #[prost(message, required, tag = "2")]
    pub cmd_collection: Cmdcollection,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33112)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Cs33112 {
    #[prost(uint32, required, tag = "1")]
    pub r#type: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Groupinchapter {
    #[prost(uint32, required, tag = "8")]
    pub bullet: u32,
    #[prost(uint32, required, tag = "4")]
    pub loss_flag: u32,
    #[prost(message, repeated, tag = "6")]
    pub ship_strategy_list: ::prost::alloc::vec::Vec<Strategyinfo>,
    #[prost(uint32, required, tag = "11")]
    pub damage_level: u32,
    #[prost(message, repeated, tag = "5")]
    pub box_strategy_list: ::prost::alloc::vec::Vec<Strategyinfo>,
    #[prost(uint32, repeated, packed = "false", tag = "10")]
    pub attach_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "13")]
    pub commander_list: ::prost::alloc::vec::Vec<super::common::Commandersinfo>,
    #[prost(message, optional, tag = "15")]
    pub cmd_collection: ::core::option::Option<Cmdcollection>,
    #[prost(message, required, tag = "9")]
    pub start_pos: Chaptercellpos,
    #[prost(message, required, tag = "3")]
    pub pos: Chaptercellpos,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "16")]
    pub bullet_max: u32,
    #[prost(message, repeated, tag = "2")]
    pub ship_list: ::prost::alloc::vec::Vec<Shipinchapter>,
    #[prost(uint32, repeated, packed = "false", tag = "7")]
    pub strategy_ids: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "12")]
    pub buff_list: ::prost::alloc::vec::Vec<BuffInfo>,
    #[prost(uint32, required, tag = "14")]
    pub kill_count: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldtargetFetch {
    #[prost(uint32, repeated, packed = "false", tag = "2")]
    pub star_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33109)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Sc33109 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WorldbossRank {
    #[prost(string, required, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(uint32, required, tag = "3")]
    pub damage: u32,
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Elitefleetinfo {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub ship_id_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "2")]
    pub commanders: ::prost::alloc::vec::Vec<super::common::Commandersinfo>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33415)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cs33415 {
    #[prost(uint32, repeated, packed = "false", tag = "1")]
    pub task_list: ::prost::alloc::vec::Vec<u32>,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33416)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33416 {
    #[prost(uint32, required, tag = "1")]
    pub result: u32,
    #[prost(message, repeated, tag = "2")]
    pub task_list: ::prost::alloc::vec::Vec<TaskInfo>,
}
#[derive(proto_derive::CmdID)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Chapterawardinfo {
    #[prost(uint32, required, tag = "1")]
    pub id: u32,
    #[prost(uint32, required, tag = "2")]
    pub award: u32,
    #[prost(uint32, required, tag = "3")]
    pub flag: u32,
}
#[derive(proto_derive::CmdID)]
#[cmdid(33001)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sc33001 {
    #[prost(message, repeated, tag = "6")]
    pub target_fetch_list: ::prost::alloc::vec::Vec<WorldtargetFetch>,
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub port_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, required, tag = "7")]
    pub count_info: Countinfo,
    #[prost(message, repeated, tag = "8")]
    pub out_shop_buy_list: ::prost::alloc::vec::Vec<GoodsInfo>,
    #[prost(uint32, repeated, packed = "false", tag = "9")]
    pub clean_chapter: ::prost::alloc::vec::Vec<u32>,
    #[prost(message, repeated, tag = "10")]
    pub chapter_award: ::prost::alloc::vec::Vec<Chapterawardinfo>,
    #[prost(message, repeated, tag = "5")]
    pub target_list: ::prost::alloc::vec::Vec<Worldtarget>,
    #[prost(message, repeated, tag = "11")]
    pub fleet_list: ::prost::alloc::vec::Vec<super::common::Fleetinfo>,
    #[prost(uint32, repeated, packed = "false", tag = "12")]
    pub new_flag_port_list: ::prost::alloc::vec::Vec<u32>,
    #[prost(uint32, required, tag = "4")]
    pub camp: u32,
    #[prost(uint32, required, tag = "2")]
    pub is_world_open: u32,
    #[prost(message, optional, tag = "1")]
    pub world: ::core::option::Option<Worldinfo>,
    #[prost(uint32, repeated, packed = "false", tag = "13")]
    pub global_flag_list: ::prost::alloc::vec::Vec<u32>,
}