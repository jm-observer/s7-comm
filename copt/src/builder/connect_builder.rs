use crate::packet::{ConnectComm, CoptFrame, Parameter, PduType};
use std::fmt::Debug;

#[derive(Default)]
pub struct ConnectBuilder {
    destination_ref: [u8; 2],
    source_ref: [u8; 2],
    class: u8,
    extended_formats: bool,
    no_explicit_flow_control: bool,
    parameters: Vec<Parameter>,
}

impl ConnectBuilder {
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

    pub fn push_parameter(mut self, parameter: Parameter) -> Self {
        self.parameters.push(parameter);
        self
    }

    pub fn build_to_request<F: Debug + Eq + PartialEq>(self) -> CoptFrame<F> {
        let Self {
            destination_ref,
            source_ref,
            class,
            extended_formats,
            no_explicit_flow_control,
            parameters,
        } = self;
        CoptFrame {
            pdu_type: PduType::ConnectRequest(ConnectComm {
                destination_ref,
                source_ref,
                class,
                extended_formats,
                no_explicit_flow_control,
                parameters,
            }),
        }
    }

    pub fn build_to_confirm<F: Debug + Eq + PartialEq>(self) -> CoptFrame<F> {
        let Self {
            destination_ref,
            source_ref,
            class,
            extended_formats,
            no_explicit_flow_control,
            parameters,
        } = self;
        CoptFrame {
            pdu_type: PduType::ConnectConfirm(ConnectComm {
                destination_ref,
                source_ref,
                class,
                extended_formats,
                no_explicit_flow_control,
                parameters,
            }),
        }
    }
}
