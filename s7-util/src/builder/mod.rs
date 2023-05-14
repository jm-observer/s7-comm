use crate::builder::copt_connect_request::CoptConnectRequestBuilder;
use crate::builder::copt_setup::CoptSetupBuilder;

mod copt_connect_request;
mod copt_setup;

pub struct Builder;

impl Builder {
    pub fn build_copt_connect_request() -> CoptConnectRequestBuilder {
        CoptConnectRequestBuilder::default()
    }

    pub fn build_copt_setup() -> CoptSetupBuilder {
        CoptSetupBuilder::default()
    }
}
