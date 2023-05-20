mod s7_read;
mod s7_write;
use crate::builder::copt_connect_request::CoptConnectRequestBuilder;
use crate::builder::s7_setup::S7SetupBuilder;

use self::s7_read::S7ReadBuilder;
use self::s7_write::S7WriteBuilder;

mod copt_connect_request;
mod s7_setup;

pub fn build_copt_connect_request() -> CoptConnectRequestBuilder {
    CoptConnectRequestBuilder::default()
}

pub fn build_s7_setup() -> S7SetupBuilder {
    S7SetupBuilder::default()
}

pub fn build_s7_write() -> S7WriteBuilder {
    S7WriteBuilder::default()
}

pub fn build_s7_read() -> S7ReadBuilder {
    S7ReadBuilder::default()
}
