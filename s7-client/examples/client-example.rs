use std::net::IpAddr;

use anyhow::Result;
use log::{debug, info};

use s7_client::{
    ConnectMode, ConnectionType, Options,
    S7Client,
};

#[tokio::main]
async fn main() -> Result<()> {
    custom_utils::logger::logger_stdout_debug();

    debug!("start test");

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

    test_db_write(&mut client).await?;
    test_process_input(&mut client).await?;
    test_process_output(&mut client).await?;
    test_process_merker(&mut client).await?;

    info!("all test pass");

    Ok(())
}

async fn test_db_write(
    client: &mut S7Client,
) -> Result<()> {
    /*
     * write addr=0 bit6 = 1
     */
    let resp = client
        .write_bit(
            Some(1),
            s7_comm::Area::DataBlocks,
            0,
            6,
            true,
        )
        .await?;

    assert!(resp.return_code.is_ok());

    // check bit6 = 1
    let area = s7_client::Area::DataBausteine(
        1,
        s7_client::DataSizeType::Bit {
            addr: 0,
            bit_addr: s7_client::BitAddr::Addr6,
        },
    );

    let item = client.read(&area).await?;
    assert!(resp.return_code.is_ok());
    assert_eq!(item.data, &[1]);

    /*
     * write addr 100, bytes = [1,2,3,4]
     */
    let resp = client
        .write_bytes(
            Some(1),
            s7_comm::Area::DataBlocks,
            100,
            &[1, 2, 3, 4],
        )
        .await?;
    assert!(resp.return_code.is_ok());

    // db read
    let area = s7_client::Area::DataBausteine(
        1,
        s7_client::DataSizeType::Byte {
            addr: 100,
            len: 4,
        },
    );
    let resp = client.read(&area).await?;
    assert!(resp.return_code.is_ok());
    assert_eq!(resp.data, &[1, 2, 3, 4]);

    Ok(())
}

async fn test_process_input(
    client: &mut S7Client,
) -> Result<()> {
    /*
     * write addr=100 bit4 = 1
     */
    let rs = client
        .write_bit(
            None,
            s7_comm::Area::ProcessInput,
            100,
            4,
            true,
        )
        .await?;
    assert!(rs.return_code.is_ok());

    let area = s7_client::Area::ProcessInput(
        s7_client::DataSizeType::Bit {
            addr: 100,
            bit_addr: s7_client::BitAddr::Addr4,
        },
    );
    let resp = client.read(&area).await?;
    assert!(resp.return_code.is_ok());
    assert_eq!(resp.data, &[1]);

    /*
     * write addr=200 bytes = [1,2,3,4]
     */
    let rs = client
        .write_bytes(
            None,
            s7_comm::Area::ProcessInput,
            200,
            &[1, 2, 3, 4],
        )
        .await?;
    assert!(rs.return_code.is_ok());

    let area = s7_client::Area::ProcessInput(
        s7_client::DataSizeType::Byte {
            addr: 200,
            len: 4,
        },
    );
    let resp = client.read(&area).await?;
    assert!(resp.return_code.is_ok());
    assert_eq!(resp.data, &[1, 2, 3, 4]);

    Ok(())
}

async fn test_process_merker(
    client: &mut S7Client,
) -> Result<()> {
    /*
     * write addr=0 bit4 = 1
     */
    let rs = client
        .write_bit(
            None,
            s7_comm::Area::Merker,
            0,
            4,
            true,
        )
        .await?;
    assert!(rs.return_code.is_ok());

    let area = s7_client::Area::Merker(
        s7_client::DataSizeType::Bit {
            addr: 0,
            bit_addr: s7_client::BitAddr::Addr4,
        },
    );
    let resp = client.read(&area).await?;
    assert!(resp.return_code.is_ok());
    assert_eq!(resp.data, &[1]);

    /*
     * write addr=0 bytes = [1,2,3,4]
     */
    let rs = client
        .write_bytes(
            None,
            s7_comm::Area::Merker,
            0,
            &[1, 2, 3, 4],
        )
        .await?;
    assert!(rs.return_code.is_ok());

    let area = s7_client::Area::Merker(
        s7_client::DataSizeType::Byte {
            addr: 0,
            len: 4,
        },
    );

    let resp = client.read(&area).await?;
    assert!(resp.return_code.is_ok());
    assert_eq!(resp.data, &[1, 2, 3, 4]);

    Ok(())
}

async fn test_process_output(
    client: &mut S7Client,
) -> Result<()> {
    /*
    let rs = client
        .write_bit(
            None,
            s7_comm::Area::ProcessOutput,
            10,
            4,
            true,
        )
        .await?;
    assert!(rs.return_code.is_ok());
    */

    let area = s7_client::Area::ProcessOutput(
        s7_client::DataSizeType::Bit {
            addr: 0,
            bit_addr: s7_client::BitAddr::Addr4,
        },
    );
    let resp = client.read(&area).await?;
    assert!(resp.return_code.is_ok());
    assert_eq!(resp.data, &[0]);

    Ok(())
}
