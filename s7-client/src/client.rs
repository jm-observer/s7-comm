use std::{
    net::{IpAddr, SocketAddr},
    time::Duration
};

use self::param::{ConnectMode, ConnectionType};
use crate::{
    build_copt_connect_request, build_s7_setup,
    error::*
};
use bytes::BytesMut;
use copt::{CoptDecoder, PduType, TpduSize};
use log::debug;
use s7_comm::{AckData, Frame, S7CommDecoder};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout
};
use tokio_util::codec::Decoder;
use tpkt::TpktDecoder;

mod param;
mod request_param;

pub struct S7Client {
    options: Options,
    connect: TcpStream
}

impl S7Client {
    pub async fn connect(
        options: Options
    ) -> Result<Self> {
        let mut req =
            tokio::net::TcpStream::connect(
                SocketAddr::new(
                    options.address,
                    options.port
                )
            )
            .await?;

        let mut buf = [0u8; 1000];
        {
            let frame =
                build_copt_connect_request()
                    .source_ref([0, 1])
                    .destination_ref([0, 0])
                    .class_and_others(
                        0, false, false
                    )
                    .pdu_size(TpduSize::L1024)
                    .src_tsap(
                        options
                            .conn_mode
                            .local_tsap()
                    )
                    .dst_tsap(
                        options
                            .conn_mode
                            .remote_tsap()
                    )
                    .build_to_request()
                    .unwrap();
            req.write_all(frame.as_ref())
                .await
                .unwrap();
            let mut bytes = BytesMut::new();
            let mut decoder = TpktDecoder(
                CoptDecoder(S7CommDecoder)
            );
            loop {
                let size = req
                    .read(&mut buf)
                    .await
                    .unwrap();
                if size == 0 {
                    bail!("size = 0");
                }
                bytes.extend_from_slice(
                    buf[0..size].as_ref()
                );
                if let Some(frame) = decoder
                    .decode(&mut bytes)
                    .unwrap()
                {
                    if let PduType::ConnectConfirm(
                    comm
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
            let frame = build_s7_setup()
                .max_amq_called(1)
                .max_amq_calling(1)
                .pdu_length(480)
                .pdu_ref(1024)
                .build()
                .unwrap();

            req.write_all(frame.as_ref())
                .await
                .unwrap();
            let mut bytes = BytesMut::new();
            let mut decoder = TpktDecoder(
                CoptDecoder(S7CommDecoder)
            );

            loop {
                let size = req
                    .read(&mut buf)
                    .await
                    .unwrap();
                bytes.extend_from_slice(
                    buf[0..size].as_ref()
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
                            ack_data
                        } = comm.payload()
                        {
                            debug!(
                                "{:?}",
                                header
                            );
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
        todo!()
    }

    async fn write(
        &mut self,
        framed: BytesMut
    ) -> Result<()>{
        timeout(
            self.options.write_timeout,
            self.connect.write_all(&framed)
        )
        .await
    }

    fn build_framed_copt_connect_request(
        options: &Options
    ) -> Result<BytesMut> {
        Ok(build_copt_connect_request()
            .source_ref([0, 1])
            .destination_ref([0, 0])
            .class_and_others(0, false, false)
            .pdu_size(TpduSize::L1024)
            .src_tsap(
                options.conn_mode.local_tsap()
            )
            .dst_tsap(
                options.conn_mode.remote_tsap()
            )
            .build_to_request()?)
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
