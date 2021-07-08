use crate::{
    modules::{ModuleName, ModuleVersion},
    profile::Profile,
};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Report {
    V1(ReportV1),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ReportV1 {
    pub target: String,
    pub profile: Profile,
    pub hosts: Vec<Host>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Host {
    pub domain: String,
    pub resolves: bool,
    pub ips: Vec<IpAddr>,
    pub ports: Vec<Port>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Port {
    pub port: u16,
    pub protocol: Protocol,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone, Eq, PartialEq, Copy, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Tcp,
    Http,
    Https,
    // Ssh,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Finding {
    pub module: ModuleName,
    pub module_version: ModuleVersion,
    pub result: ModuleResult,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub enum ModuleResult {
    Url(String),
}