use crate::core::parts::traits::Transport;
use crate::core::parts::transport_pair::TransportPair;

// informations with hex + bytes
#[derive(Debug, Clone, Default)]
pub struct TransportCarrier {
    pub device_no: Option<TransportPair>,
    pub device_no_padding: Option<TransportPair>,
    pub device_no_length: Option<TransportPair>,
    pub protocol_version: Option<TransportPair>,
    pub report_type: Option<TransportPair>,
    pub control_field: Option<TransportPair>,
    pub device_type: Option<TransportPair>,
    pub factory_code: Option<TransportPair>,
    pub upstream_count: Option<TransportPair>,
    pub downstream_count: Option<TransportPair>,
    pub cipher_slot: i8,
}

impl TransportCarrier {
    pub fn new_with_device_no(
        device_no: &str,
        device_no_bytes: &[u8],
        device_no_padding: &str,
        device_no_padding_bytes: &[u8],
    ) -> Self {
        Self {
            device_no: Some(TransportPair::new(device_no.into(), device_no_bytes.into())),
            device_no_padding: Some(TransportPair::new(
                device_no_padding.into(),
                device_no_padding_bytes.into(),
            )),
            device_no_length: None,
            control_field: None,
            report_type: None,
            protocol_version: None,
            device_type: None,
            factory_code: None,
            upstream_count: None,
            downstream_count: None,
            cipher_slot: -1,
        }
    }

    pub fn set_device_no_length(&mut self, hex: String, bytes: Vec<u8>) {
        let tp = TransportPair::new(hex, bytes);
        self._set_device_no_length(Some(tp));
    }

    fn _set_device_no_length(&mut self, device_no_length: Option<TransportPair>) {
        self.device_no_length = device_no_length;
    }

    pub fn set_report_type(&mut self, hex: String, bytes: Vec<u8>) {
        let tp = TransportPair::new(hex, bytes);
        self._set_report_type(Some(tp));
    }

    fn _set_report_type(&mut self, report_type: Option<TransportPair>) {
        self.report_type = report_type;
    }

    pub fn set_control_field(&mut self, hex: String, bytes: Vec<u8>) {
        let tp = TransportPair::new(hex, bytes);
        self._set_control_field(Some(tp));
    }

    fn _set_control_field(&mut self, control_field: Option<TransportPair>) {
        self.control_field = control_field;
    }

    pub fn set_device_no(&mut self, hex: String, bytes: Vec<u8>) {
        let tp = TransportPair::new(hex, bytes);
        self._set_device_no(Some(tp));
    }

    fn _set_device_no(&mut self, device_no: Option<TransportPair>) {
        self.device_no = device_no;
    }

    pub fn set_device_no_padding(&mut self, hex: String, bytes: Vec<u8>) {
        let tp = TransportPair::new(hex, bytes);
        self._set_device_no_padding(Some(tp));
    }

    fn _set_device_no_padding(&mut self, device_no_padding: Option<TransportPair>) {
        self.device_no_padding = device_no_padding;
    }

    pub fn set_protocol_version(&mut self, hex: String, bytes: Vec<u8>) {
        let tp = TransportPair::new(hex, bytes);
        self._set_protocol_version(Some(tp));
    }

    fn _set_protocol_version(&mut self, version: Option<TransportPair>) {
        self.protocol_version = version;
    }

    pub fn set_device_type(&mut self, hex: String, bytes: Vec<u8>) {
        let tp = TransportPair::new(hex, bytes);
        self._set_device_type(Some(tp));
    }

    fn _set_device_type(&mut self, device_type: Option<TransportPair>) {
        self.device_type = device_type;
    }

    pub fn set_factory_code(&mut self, hex: String, bytes: Vec<u8>) {
        let tp = TransportPair::new(hex, bytes);
        self._set_factory_code(Some(tp));
    }

    fn _set_factory_code(&mut self, factory_code: Option<TransportPair>) {
        self.factory_code = factory_code;
    }

    pub fn set_cipher_slot(&mut self, cipher_slot: i8) {
        self.cipher_slot = cipher_slot;
    }

    pub fn set_upstream_count(&mut self, hex: String, bytes: Vec<u8>) {
        let tp = TransportPair::new(hex, bytes);
        self._set_upstream_count(Some(tp));
    }

    fn _set_upstream_count(&mut self, count: Option<TransportPair>) {
        self.upstream_count = count;
    }

    pub fn set_downstream_count(&mut self, hex: String, bytes: Vec<u8>) {
        let tp = TransportPair::new(hex, bytes);
        self._set_downstream_count(Some(tp));
    }

    fn _set_downstream_count(&mut self, count: Option<TransportPair>) {
        self.downstream_count = count;
    }
}

impl Transport for TransportCarrier {
    fn device_no(&self) -> Option<TransportPair> {
        self.device_no.clone()
    }

    fn device_no_padding(&self) -> Option<TransportPair> {
        self.device_no_padding.clone()
    }

    fn device_no_length(&self) -> Option<TransportPair> {
        self.device_no_length.clone()
    }

    fn report_type(&self) -> Option<TransportPair> {
        self.report_type.clone()
    }

    fn control_field(&self) -> Option<TransportPair> {
        self.control_field.clone()
    }

    fn protocol_version(&self) -> Option<TransportPair> {
        self.protocol_version.clone()
    }

    fn device_type(&self) -> Option<TransportPair> {
        self.device_type.clone()
    }

    fn factory_code(&self) -> Option<TransportPair> {
        self.factory_code.clone()
    }

    fn upstream_count(&self) -> Option<TransportPair> {
        self.upstream_count.clone()
    }

    fn downstream_count(&self) -> Option<TransportPair> {
        self.downstream_count.clone()
    }

    fn cipher_slot(&self) -> i8 {
        self.cipher_slot
    }
}
