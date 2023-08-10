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

    let address: IpAddr =
        "192.168.199.3".parse()?;

    let options = Options::new(
        address,
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

    /*
    let area0 = s7_client::Area::DataBausteine(
        1,
        s7_client::DataSizeType::Byte {
            addr: 0,
            len: 1,
        },
    );
    let area1 = s7_client::Area::DataBausteine(
        1,
        s7_client::DataSizeType::Word {
            addr: 0,
            len: 2,
        },
    );
    let area2 = s7_client::Area::ProcessOutput(
        s7_client::DataSizeType::Byte {
            addr: 0,
            len: 1,
        },
    );
    */
    let area3 = s7_client::Area::ProcessInput(
        s7_client::DataSizeType::Bit {
            addr: 0,
            bit_addr: s7_client::BitAddr::Addr1,
        },
    );
    // let area4 = s7_client::Area::ProcessInput(
    //     s7_client::DataSizeType::Byte {
    //         addr: 0,
    //         len: 1,
    //     },
    // );
    let ack = client
        .read(vec![
            /*area0, area1, area2, */ area3,
        ])
        .await?;
    for data in ack {
        debug!("{:?}", data);
    }

    Ok(())
}
