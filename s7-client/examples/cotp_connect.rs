use anyhow::{bail, Result};
use bytes::BytesMut;
use copt::{CoptDecoder, PduType, TpduSize};
use log::debug;
use s7_client::{
    ConnectMode, ConnectionType, Options,
    S7Client
};
use s7_comm::{AckData, Frame, S7CommDecoder};
use std::net::{IpAddr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::codec::Decoder;
use tpkt::TpktDecoder;

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
    let client =
        S7Client::connect(options).await?;

    Ok(())

    //     let frame = s7_client::build_s7_setup()
    //         .max_amq_called(1)
    //         .max_amq_calling(1)
    //         .pdu_length(480)
    //         .pdu_ref(1024)
    //         .build()
    //         .unwrap();

    //     req.write_all(frame.as_ref())
    //         .await
    //         .unwrap();
    //     let mut bytes = BytesMut::new();
    //     let mut decoder = TpktDecoder(
    //         CoptDecoder(S7CommDecoder)
    //     );

    //     loop {
    //         let size =
    //             req.read(&mut
    // buf).await.unwrap();         bytes.
    // extend_from_slice(
    // buf[0..size].as_ref()         );
    //         if let Some(frame) = decoder
    //             .decode(&mut bytes)
    //             .unwrap()
    //         {
    //             if let PduType::DtData(comm) =
    //                 frame.payload().pdu_type
    //             {
    //                 if let Frame::AckData {
    //                     header,
    //                     ack_data
    //                 } = comm.payload()
    //                 {
    //                     debug!("{:?}", header);
    //                     if let
    // AckData::SetupCommunication(data) =
    // ack_data {
    // debug!("{:?}", data);
    // break;                     }
    //                 }
    //             }
    //             bail!("todo");
    //         }
    //     }
    // }
    // {
    //     let frame = s7_client::build_s7_write()
    //         .pdu_ref(1024)
    //         .write_db_bytes(
    //             1,
    //             300,
    //             [0u8, 0, 0, 0x09].as_ref()
    //         )
    //         .build()
    //         .unwrap();
    //     // let frame =
    //     // init_s7_write().
    //     // to_bytes::<CoptEncoder<S7CommEncoder>>()?
    //     // ;
    //     req.write_all(frame.as_ref())
    //         .await
    //         .unwrap();
    //     let mut bytes = BytesMut::new();
    //     let mut decoder = TpktDecoder(
    //         CoptDecoder(S7CommDecoder)
    //     );
    //     loop {
    //         let size =
    //             req.read(&mut
    // buf).await.unwrap();         bytes.
    // extend_from_slice(
    // buf[0..size].as_ref()         );
    //         if let Some(frame) = decoder
    //             .decode(&mut bytes)
    //             .unwrap()
    //         {
    //             if let PduType::DtData(comm) =
    //                 frame.payload().pdu_type
    //             {
    //                 if let Frame::AckData {
    //                     header,
    //                     ack_data
    //                 } = comm.payload()
    //                 {
    //                     debug!("{:?}", header);
    //                     if let
    // AckData::WriteVar(
    // data                     ) = ack_data
    //                     {
    //                         debug!("{:?}",
    // data);                         break;
    //                     }
    //                 }
    //             }
    //             bail!("todo");
    //         }
    //     }
    // }
    // {
    //     let frame = s7_client::build_s7_read()
    //         .pdu_ref(1024)
    //         .read_db_bytes(1, 300, 4)
    //         .build()
    //         .unwrap();
    //     // let frame =
    //     // init_s7_read().
    //     // to_bytes::<CoptEncoder<S7CommEncoder>>()?
    //     // ;
    //     req.write_all(frame.as_ref())
    //         .await
    //         .unwrap();
    //     let mut bytes = BytesMut::new();
    //     let mut decoder = TpktDecoder(
    //         CoptDecoder(S7CommDecoder)
    //     );
    //     loop {
    //         let size =
    //             req.read(&mut
    // buf).await.unwrap();         bytes.
    // extend_from_slice(
    // buf[0..size].as_ref()         );
    //         if let Some(frame) = decoder
    //             .decode(&mut bytes)
    //             .unwrap()
    //         {
    //             if let PduType::DtData(comm) =
    //                 frame.payload().pdu_type
    //             {
    //                 if let Frame::AckData {
    //                     header,
    //                     ack_data
    //                 } = comm.payload()
    //                 {
    //                     debug!("{:?}", header);
    //                     if let
    // AckData::ReadVar(
    // data                     ) = ack_data
    //                     {
    //                         debug!("{:?}",
    // data);                         break;
    //                     }
    //                 }
    //             }
    //             bail!("todo");
    //         }
    //     }
    // }
}
