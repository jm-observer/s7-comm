use crate::codec::S7Encoder;
use crate::error::*;
use bytes::BytesMut;
use copt::{ConnectComm, CoptFrame, Parameter, PduType, TpduSize};
use tokio_util::codec::Encoder;

#[derive(Default)]
pub struct CoptConnectRequestBuilder {
    source_ref: [u8; 2],
    destination_ref: [u8; 2],
    class: u8,
    extended_formats: bool,
    no_explicit_flow_control: bool,
    parameters: Vec<Parameter>,
}

impl CoptConnectRequestBuilder {
    pub fn source_ref(mut self, source_ref: [u8; 2]) -> Self {
        self.source_ref = source_ref;
        self
    }

    pub fn destination_ref(mut self, destination_ref: [u8; 2]) -> Self {
        self.destination_ref = destination_ref;
        self
    }

    pub fn class_and_others(
        mut self,
        class: u8,
        extended_formats: bool,
        no_explicit_flow_control: bool,
    ) -> Self {
        self.class = class;
        self.extended_formats = extended_formats;
        self.no_explicit_flow_control = no_explicit_flow_control;
        self
    }

    pub fn pdu_size(self, pdu_size: TpduSize) -> Self {
        self.push_parameter(Parameter::TpduSize(pdu_size))
    }

    pub fn src_tsap(self, src_tsap: [u8; 2]) -> Self {
        self.push_parameter(Parameter::new_src_tsap(src_tsap.to_vec()))
    }

    pub fn dst_tsap(self, dst_tsap: [u8; 2]) -> Self {
        self.push_parameter(Parameter::new_dst_tsap(dst_tsap.to_vec()))
    }

    pub fn push_parameter(mut self, parameter: Parameter) -> Self {
        self.parameters.push(parameter);
        self
    }

    pub fn build_to_request(self) -> Result<BytesMut> {
        let Self {
            destination_ref,
            source_ref,
            class,
            extended_formats,
            no_explicit_flow_control,
            parameters,
        } = self;
        let frame = tpkt::TpktFrame::new(CoptFrame {
            pdu_type: PduType::ConnectRequest(ConnectComm {
                destination_ref,
                source_ref,
                class,
                extended_formats,
                no_explicit_flow_control,
                parameters,
            }),
        });
        let mut dst = BytesMut::new();
        let mut encoder = S7Encoder::default();
        encoder.encode(frame, &mut dst)?;
        Ok(dst)
    }

    // pub fn build_to_confirm(self) -> CoptFrame<F> {
    //     let Self {
    //         destination_ref,
    //         source_ref,
    //         class,
    //         extended_formats,
    //         no_explicit_flow_control,
    //         parameters,
    //         ..
    //     } = self;
    //     CoptFrame {
    //         pdu_type: PduType::ConnectConfirm(ConnectComm {
    //             destination_ref,
    //             source_ref,
    //             class,
    //             extended_formats,
    //             no_explicit_flow_control,
    //             parameters,
    //         }),
    //     }
    // }
}
