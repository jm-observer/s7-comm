use anyhow::{bail, Result};
use bytes::BytesMut;
use copt::{
    CoptDecoder, CoptEncoder, Parameter, PduType,
    TpduSize,
};
use log::debug;
use s7_comm::{
    AckData, Frame, S7CommDecoder, S7CommEncoder,
};
use std::net::{IpAddr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_util::codec::Decoder;
use tpkt::{TpktDecoder, TpktFrame};

type CoptFrame = copt::CoptFrame<Frame>;

#[tokio::main]
async fn main() -> Result<()> {
    custom_utils::logger::logger_stdout_debug();
    let mut req = tokio::net::TcpStream::connect(
        SocketAddr::new(
            IpAddr::from([10u8, 10, 12, 33]),
            102,
        ),
    )
    .await?;

    let mut buf = [0u8; 1000];
    {
        let frame = init_copt_connect_request().to_bytes::<CoptEncoder<S7CommEncoder>>()?;
        req.write_all(frame.as_ref())
            .await
            .unwrap();
        let mut bytes = BytesMut::new();
        let mut decoder = TpktDecoder(
            CoptDecoder(S7CommDecoder),
        );
        loop {
            let size =
                req.read(&mut buf).await.unwrap();
            if size == 0 {
                bail!("size = 0");
            }
            bytes.extend_from_slice(
                buf[0..size].as_ref(),
            );
            if let Some(frame) = decoder
                .decode(&mut bytes)
                .unwrap()
            {
                if let PduType::ConnectConfirm(
                    comm,
                ) = frame.payload().pdu_type
                {
                    debug!("{:?}", comm);
                    break;
                }
            } else {
                debug!("{:?}", bytes);
            }
        }
    }
    {
        let frame = init_s7_setup().to_bytes::<CoptEncoder<S7CommEncoder>>()?;
        req.write_all(frame.as_ref())
            .await
            .unwrap();
        let mut bytes = BytesMut::new();
        let mut decoder = TpktDecoder(
            CoptDecoder(S7CommDecoder),
        );
        loop {
            let size =
                req.read(&mut buf).await.unwrap();
            bytes.extend_from_slice(
                buf[0..size].as_ref(),
            );
            if let Some(frame) = decoder
                .decode(&mut bytes)
                .unwrap()
            {
                if let PduType::DtData(comm) =
                    frame.payload().pdu_type
                {
                    if let Frame::AckData {
                        header,
                        ack_data,
                    } = comm.payload()
                    {
                        debug!("{:?}", header);
                        if let AckData::SetupCommunication(data) = ack_data {
                            debug!("{:?}", data);
                            break;
                        }
                    }
                }
                bail!("todo");
            }
        }
    }
    {
        let frame = init_s7_write().to_bytes::<CoptEncoder<S7CommEncoder>>()?;
        req.write_all(frame.as_ref())
            .await
            .unwrap();
        let mut bytes = BytesMut::new();
        let mut decoder = TpktDecoder(
            CoptDecoder(S7CommDecoder),
        );
        loop {
            let size =
                req.read(&mut buf).await.unwrap();
            bytes.extend_from_slice(
                buf[0..size].as_ref(),
            );
            if let Some(frame) = decoder
                .decode(&mut bytes)
                .unwrap()
            {
                if let PduType::DtData(comm) =
                    frame.payload().pdu_type
                {
                    if let Frame::AckData {
                        header,
                        ack_data,
                    } = comm.payload()
                    {
                        debug!("{:?}", header);
                        if let AckData::WriteVar(
                            data,
                        ) = ack_data
                        {
                            debug!("{:?}", data);
                            break;
                        }
                    }
                }
                bail!("todo");
            }
        }
    }
    {
        let frame = init_s7_read().to_bytes::<CoptEncoder<S7CommEncoder>>()?;
        req.write_all(frame.as_ref())
            .await
            .unwrap();
        let mut bytes = BytesMut::new();
        let mut decoder = TpktDecoder(
            CoptDecoder(S7CommDecoder),
        );
        loop {
            let size =
                req.read(&mut buf).await.unwrap();
            bytes.extend_from_slice(
                buf[0..size].as_ref(),
            );
            if let Some(frame) = decoder
                .decode(&mut bytes)
                .unwrap()
            {
                if let PduType::DtData(comm) =
                    frame.payload().pdu_type
                {
                    if let Frame::AckData {
                        header,
                        ack_data,
                    } = comm.payload()
                    {
                        debug!("{:?}", header);
                        if let AckData::ReadVar(
                            data,
                        ) = ack_data
                        {
                            debug!("{:?}", data);
                            break;
                        }
                    }
                }
                bail!("todo");
            }
        }
    }
    Ok(())
}

fn init_copt_connect_request(
) -> TpktFrame<CoptFrame> {
    TpktFrame::new(
        CoptFrame::builder_of_connect()
            .source_ref([0, 1])
            .destination_ref([0, 0])
            .class_and_others(0, false, false)
            .push_parameter(Parameter::TpduSize(
                TpduSize::L1024,
            ))
            .push_parameter(
                Parameter::new_src_tsap(
                    [1, 0].to_vec(),
                ),
            )
            .push_parameter(
                Parameter::new_dst_tsap(
                    [2, 1].to_vec(),
                ),
            )
            .build_to_request(),
    )
}

fn init_s7_setup() -> TpktFrame<CoptFrame> {
    TpktFrame::new(
        CoptFrame::builder_of_dt_data(
            s7_comm::Frame::job_setup(1024)
                .pdu_length(480)
                .max_amq_calling(1)
                .max_amq_called(1)
                .build(),
        )
        .build(0, true),
    )
}

fn init_s7_write() -> TpktFrame<CoptFrame> {
    TpktFrame::new(
        CoptFrame::builder_of_dt_data(
            s7_comm::Frame::job_write_var(1024)
                .write_bytes(
                    Some(1),
                    s7_comm::Area::DataBlocks,
                    300,
                    [0u8, 0, 0, 0x09].as_ref(),
                )
                .build(),
        )
        .build(0, true),
    )
}

fn init_s7_read() -> TpktFrame<CoptFrame> {
    TpktFrame::new(
        CoptFrame::builder_of_dt_data(
            s7_comm::Frame::job_read_var(1024)
                .read_bytes(
                    Some(1),
                    s7_comm::Area::DataBlocks,
                    300,
                    4,
                )
                .build(),
        )
        .build(0, true),
    )
}
