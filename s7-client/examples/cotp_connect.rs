use anyhow::Result;
use log::debug;
use s7_client::{
    ConnectMode, ConnectionType, Options,
    S7Client,
};
use std::net::IpAddr;

#[tokio::main]
async fn main() -> Result<()> {
    custom_utils::logger::logger_stdout_debug();

    let options = Options::new(
        IpAddr::from([192u8, 168, 1, 99]),
        102,
        ConnectMode::RackSlot {
            conn_type: ConnectionType::PG,
            rack: 0,
            slot: 1,
        },
    );
    let mut client =
        S7Client::connect(options).await?;

    for rs in client
        .write_db_bit(1, 200, 1, true)
        .await?
    {
        println!("{:?}", rs);
    }
    // for rs in client
    //     .write_db_bytes(1, 100, [10, 20].as_ref())
    //     .await?
    // {
    //     println!("{:?}", rs);
    // }

    let area0 = s7_client::Area::DataBausteine(
        1,
        s7_client::DataSizeType::Byte {
            addr: 100,
            len: 3,
        },
    );
    let area1 = s7_client::Area::DataBausteine(
        2,
        s7_client::DataSizeType::Byte {
            addr: 100,
            len: 2,
        },
    );
    let area2 = s7_client::Area::DataBausteine(
        1,
        s7_client::DataSizeType::Bit {
            addr: 200,
            bit_addr: s7_client::BitAddr::Addr0,
        },
    );
    let area3 = s7_client::Area::ProcessInput(
        s7_client::DataSizeType::Bit {
            addr: 0,
            bit_addr: s7_client::BitAddr::Addr0,
        },
    );
    // let area4 = s7_client::Area::ProcessInput(
    //     s7_client::DataSizeType::Byte {
    //         addr: 0,
    //         len: 1,
    //     },
    // );
    let ack = client
        .read(vec![area0, area1, area3, area2])
        .await?;
    for data in ack {
        debug!("{:?}", data);
    }

    Ok(())
}
