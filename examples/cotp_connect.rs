use anyhow::Result;
use copt::{Parameter, TpduSize};
use s7_comm::CoptFrame;
use std::net::{IpAddr, SocketAddr};

#[tokio::main]
async fn main() -> Result<()> {
    let req = tokio::net::TcpStream::connect(SocketAddr::new(IpAddr::from([127u8, 0, 0, 1]), 102))
        .await?;
    // let frame = tpkt::TpktFrame::new(
    //     CoptFrame::builder_of_connect()
    //         .source_ref([0, 1])
    //         .destination_ref([0, 0])
    //         .class_and_others(0, false, false)
    //         .push_parameter(Parameter::TpduSize(TpduSize::L1024))
    //         .push_parameter(Parameter::new_src_tsap([1, 0].to_vec()))
    //         .push_parameter(Parameter::new_dst_tsap([2, 1].to_vec()))
    //         .build_to_request(),
    // )
    // .to_bytes()?;

    Ok(())
}
