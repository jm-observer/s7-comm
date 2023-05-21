use anyhow::Result;
use log::debug;
use s7_client::{
    ConnectMode, ConnectionType, Options,
    S7Client
};
use std::net::IpAddr;

#[tokio::main]
async fn main() -> Result<()> {
    custom_utils::logger::logger_stdout_debug();

    let options = Options::new(
        IpAddr::from([10u8, 10, 12, 33]),
        102,
        ConnectMode::RackSlot {
            conn_type: ConnectionType::PG,
            rack:      0,
            slot:      1
        }
    );
    let mut client =
        S7Client::connect(options).await?;

    client.write_db_bit(1, 200, 1, true).await?;
    client
        .write_db_bytes(1, 100, [10, 20].as_ref())
        .await?;
    client
        .write_db_bytes(
            10,
            100,
            [10, 20].as_ref()
        )
        .await?;

    let area0 = s7_client::Area::DataBausteine(
        1,
        s7_client::DataSizeType::Byte {
            addr: 3000,
            len:  2
        }
    );
    let area1 = s7_client::Area::DataBausteine(
        2,
        s7_client::DataSizeType::Byte {
            addr: 3000,
            len:  2
        }
    );
    let area2 = s7_client::Area::DataBausteine(
        1,
        s7_client::DataSizeType::Bit {
            addr:     200,
            bit_addr: s7_client::BitAddr::Addr0
        }
    );
    let ack = client
        .read(vec![area0, area1, area2])
        .await?;
    for data in ack {
        debug!("{:?}", data);
    }

    Ok(())
}
