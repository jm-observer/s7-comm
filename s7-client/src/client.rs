use std::{
    net::{IpAddr, SocketAddr},
    time::Duration,
};

use crate::{
    build_copt_connect_request, build_s7_read,
    build_s7_setup, build_s7_write, error::*,
};
use bytes::BytesMut;
use copt::{
    CoptDecoder, CoptFrame, Parameter, PduType,
    TpduSize,
};
use log::debug;
use s7_comm::{
    AckData, DataItemVal, DataItemWriteResponse,
    Frame, S7CommDecoder,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    time::timeout,
};
use tokio_util::codec::Decoder;
use tpkt::{TpktDecoder, TpktFrame};

mod param;
mod request_param;

pub use param::*;
pub use request_param::*;

pub struct S7Client {
    options: Options,
    connect: TcpStream,
}

impl S7Client {
    pub async fn connect(
        options: Options,
    ) -> Result<Self> {
        let connect =
            tokio::net::TcpStream::connect(
                SocketAddr::new(
                    options.address,
                    options.port,
                ),
            )
            .await?;
        let mut client =
            Self { options, connect };
        client.copt_connect().await?;
        client.s7_setup().await?;
        Ok(client)
    }

    async fn copt_connect(
        &mut self,
    ) -> Result<()> {
        let frame =
            build_framed_copt_connect_request(
                &self.options,
            )?;
        self.write_frame(frame).await?;
        let frame =
            self.read_frame().await?.payload();
        if let PduType::ConnectConfirm(comm) =
            &frame.pdu_type
        {
            debug!("{:?}", comm);
            for item in &comm.parameters {
                if let Parameter::TpduSize(size) =
                    item
                {
                    self.options.tpdu_size =
                        size.clone();
                }
            }
        } else {
            return Err(Error::ConnectErr(
                format!(
                    "should recv connect \
                     confirm, but not {:?}",
                    frame
                ),
            ));
        }
        Ok(())
    }

    async fn s7_setup(&mut self) -> Result<()> {
        let frame =
            build_framed_s7_setup(&self.options)?;
        self.write_frame(frame).await?;
        let frame =
            self.read_frame().await?.payload();
        if let PduType::DtData(comm) =
            frame.pdu_type
        {
            if let Frame::AckData {
                ack_data,
                ..
            } = comm.payload()
            {
                if let AckData::SetupCommunication(data) = ack_data {
                        debug!("{:?}", data);
                        self.options.pdu_len = data.pdu_length();
                    }
            }
        } else {
            return Err(Error::ConnectErr(
                format!(
                    "should recv connect \
                     confirm, but not {:?}",
                    frame
                ),
            ));
        }
        Ok(())
    }

    pub async fn write_db_bytes(
        &mut self,
        db_number: u16,
        byte_addr: u16,
        data: &[u8],
    ) -> Result<Vec<DataItemWriteResponse>> {
        let frame = build_s7_write()
            .pdu_ref(
                self.options.tpdu_size.pdu_ref(),
            )
            .write_db_bytes(
                db_number, byte_addr, data,
            )
            .build()?;

        self.write(frame).await
    }

    pub async fn write_db_bit(
        &mut self,
        db_number: u16,
        byte_addr: u16,
        bit_addr: u8,
        data: bool,
    ) -> Result<Vec<DataItemWriteResponse>> {
        let frame = build_s7_write()
            .pdu_ref(
                self.options.tpdu_size.pdu_ref(),
            )
            .write_db_bit(
                db_number, byte_addr, bit_addr,
                data,
            )
            .build()?;
        self.write(frame).await
    }

    async fn write(
        &mut self,
        frame: BytesMut,
    ) -> Result<Vec<DataItemWriteResponse>> {
        self.write_frame(frame).await?;
        let frame =
            self.read_frame().await?.payload();
        if let PduType::DtData(comm) =
            frame.pdu_type
        {
            if let Frame::AckData {
                ack_data,
                ..
            } = comm.payload()
            {
                if let AckData::WriteVar(data) =
                    ack_data
                {
                    return Ok(data.data_item());
                }
            }
        }
        return Err(Error::Err(format!(
            "should recv read var"
        )));
    }

    pub async fn read(
        &mut self,
        area: &Area,
    ) -> Result<Vec<DataItemVal>> {
        let frame = build_framed_s7_read(
            &self.options,
            &[*area],
        )?;
        self.write_frame(frame).await?;
        let frame =
            self.read_frame().await?.payload();
        if let PduType::DtData(comm) =
            frame.pdu_type
        {
            if let Frame::AckData {
                ack_data,
                ..
            } = comm.payload()
            {
                if let AckData::ReadVar(data) =
                    ack_data
                {
                    return Ok(data.data_item());
                }
            }
        }
        return Err(Error::Err(format!(
            "should recv read var"
        )));
    }

    pub async fn read_vec(
        &mut self,
        areas: &[Area],
    ) -> Result<Vec<DataItemVal>> {
        let frame = build_framed_s7_read(
            &self.options,
            areas,
        )?;
        self.write_frame(frame).await?;
        let frame =
            self.read_frame().await?.payload();
        if let PduType::DtData(comm) =
            frame.pdu_type
        {
            if let Frame::AckData {
                ack_data,
                ..
            } = comm.payload()
            {
                if let AckData::ReadVar(data) =
                    ack_data
                {
                    return Ok(data.data_item());
                }
            }
        }
        return Err(Error::Err(format!(
            "should recv read var"
        )));
    }

    async fn write_frame(
        &mut self,
        framed: BytesMut,
    ) -> Result<()> {
        timeout(
            self.options.write_timeout,
            self.connect.write_all(&framed),
        )
        .await
        .map_err(|_| Error::WriteTimeout)??;
        Ok(())
    }

    async fn read_frame(
        &mut self,
    ) -> Result<TpktFrame<CoptFrame<Frame>>> {
        Ok(timeout(
            self.options.read_timeout,
            read_framed(&mut self.connect),
        )
        .await
        .map_err(|_| Error::WriteTimeout)??)
    }
}

#[derive(Debug, Clone)]
pub struct Options {
    pub read_timeout: Duration,
    pub write_timeout: Duration,
    address: IpAddr,
    port: u16,
    pub conn_mode: ConnectMode,
    pub tpdu_size: TpduSize,
    //PDULength variable to store pdu length
    // after connect
    pdu_len: u16,
}

impl Options {
    pub fn new(
        address: IpAddr,
        port: u16,
        conn_mode: ConnectMode,
    ) -> Options {
        Self {
            read_timeout: Duration::from_millis(
                500,
            ),
            write_timeout: Duration::from_millis(
                500,
            ),
            port,
            address,
            conn_mode,
            pdu_len: 480,
            tpdu_size: TpduSize::L2048,
        }
    }
}

async fn read_framed(
    req: &mut TcpStream,
) -> Result<TpktFrame<CoptFrame<Frame>>> {
    let mut buf = [0u8; 1000];
    let mut bytes = BytesMut::new();
    let mut decoder =
        TpktDecoder(CoptDecoder(S7CommDecoder));
    loop {
        let size = req.read(&mut buf).await?;
        bytes.extend_from_slice(
            buf[0..size].as_ref(),
        );
        if let Some(frame) =
            decoder.decode(&mut bytes)?
        {
            return Ok(frame);
        }
    }
}

fn build_framed_s7_read(
    options: &Options,
    areas: &[Area],
) -> Result<BytesMut> {
    let mut builder = build_s7_read()
        .pdu_ref(options.tpdu_size.pdu_ref());
    for area in areas {
        builder =
            builder.add_item((*area).into());
    }
    Ok(builder.build()?)
}

fn build_framed_copt_connect_request(
    options: &Options,
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

fn build_framed_s7_setup(
    options: &Options,
) -> Result<BytesMut> {
    Ok(build_s7_setup()
        .max_amq_called(1)
        .max_amq_calling(1)
        .pdu_length(options.pdu_len)
        .pdu_ref(options.tpdu_size.pdu_ref())
        .build()?)
}
