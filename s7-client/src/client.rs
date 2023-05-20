use std::{
    net::{IpAddr, SocketAddr},
    time::Duration
};

use crate::{
    build_copt_connect_request, build_s7_setup,
    error::*
};
use bytes::BytesMut;
use copt::{
    CoptDecoder, CoptFrame, PduType, TpduSize
};
use log::debug;
use s7_comm::{AckData, Frame, S7CommDecoder};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout
};
use tokio_util::codec::Decoder;
use tpkt::{TpktDecoder, TpktFrame};

mod param;
mod request_param;

pub use param::*;
pub use request_param::*;

pub struct S7Client {
    options: Options,
    connect: TcpStream
}

impl S7Client {
    pub async fn connect(
        options: Options
    ) -> Result<Self> {
        let connect =
            tokio::net::TcpStream::connect(
                SocketAddr::new(
                    options.address,
                    options.port
                )
            )
            .await?;
        let mut client =
            Self { options, connect };
        {
            let frame =
                build_framed_copt_connect_request(
                    &client.options
                )?;
            client.write(frame).await?;
            let frame = client
                .read_frame()
                .await?
                .payload();
            if let PduType::ConnectConfirm(comm) =
                &frame.pdu_type
            {
                debug!("{:?}", comm);
            } else {
                return Err(Error::ConnectErr(
                    format!(
                        "should recv connect \
                         confirm, but not {:?}",
                        frame
                    )
                ));
            }
        }
        {
            let frame = build_framed_s7_setup(
                &client.options
            )?;
            client.write(frame).await?;
            let frame =
                client.read_frame().await?;
            if let PduType::DtData(comm) =
                frame.payload().pdu_type
            {
                if let Frame::AckData {
                    header,
                    ack_data
                } = comm.payload()
                {
                    debug!("{:?}", header);
                    if let AckData::SetupCommunication(data) = ack_data {
                            debug!("{:?}", data);
                        }
                }
            }
        }
        Ok(client)
    }

    async fn write(
        &mut self,
        framed: BytesMut
    ) -> Result<()> {
        timeout(
            self.options.write_timeout,
            self.connect.write_all(&framed)
        )
        .await
        .map_err(|_| Error::WriteTimeout)??;
        Ok(())
    }

    async fn read_frame(
        &mut self
    ) -> Result<TpktFrame<CoptFrame<Frame>>> {
        Ok(timeout(
            self.options.read_timeout,
            read_framed(&mut self.connect)
        )
        .await
        .map_err(|_| Error::WriteTimeout)??)
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    pub read_timeout:  Duration,
    pub write_timeout: Duration,
    address:           IpAddr,
    port:              u16,
    pub conn_mode:     ConnectMode,
    //PDULength variable to store pdu length
    // after connect
    pdu_size:          TpduSize
}

impl Options {
    pub fn new(
        address: IpAddr,
        port: u16,
        conn_mode: ConnectMode
    ) -> Options {
        Self {
            read_timeout: Duration::from_millis(
                500
            ),
            write_timeout: Duration::from_millis(
                500
            ),
            port,
            address,
            conn_mode,
            pdu_size: TpduSize::L512
        }
    }
}

async fn read_framed(
    req: &mut TcpStream
) -> Result<TpktFrame<CoptFrame<Frame>>> {
    let mut buf = [0u8; 1000];
    let mut bytes = BytesMut::new();
    let mut decoder =
        TpktDecoder(CoptDecoder(S7CommDecoder));
    loop {
        let size = req.read(&mut buf).await?;
        bytes.extend_from_slice(
            buf[0..size].as_ref()
        );
        if let Some(frame) =
            decoder.decode(&mut bytes)?
        {
            return Ok(frame);
        }
    }
}

fn build_framed_copt_connect_request(
    options: &Options
) -> Result<BytesMut> {
    Ok(build_copt_connect_request()
        .source_ref([0, 1])
        .destination_ref([0, 0])
        .class_and_others(0, false, false)
        .pdu_size(TpduSize::L1024)
        .src_tsap(options.conn_mode.local_tsap())
        .dst_tsap(options.conn_mode.remote_tsap())
        .build_to_request()?)
}

/// todo
fn build_framed_s7_setup(
    options: &Options
) -> Result<BytesMut> {
    Ok(build_s7_setup()
        .max_amq_called(1)
        .max_amq_calling(1)
        .pdu_length(480)
        .pdu_ref(1024)
        .build()?)
}
