use crate::{DirectionEnum, ProtocolError, ReportField, core::parts::traits::Cmd};
use dyn_clone::DynClone;

// 报文上/下行解析 处理之后的结果 第二小解析单位，比RawField大
#[derive(Debug, Clone)]
pub struct RawCapsule<T: Cmd> {
    pub bytes: Vec<u8>,
    pub hex: String,
    pub field_details: Vec<ReportField>,
    pub cmd: Option<T>,
    pub device_no: Option<String>,
    pub device_id: Option<String>,
    // 临时二进制存放处
    pub temp_bytes: Vec<u8>,
    pub direction: DirectionEnum,
    pub success: bool,
}

impl<T: Cmd + 'static> RawCapsule<T> {
    pub fn new_upstream(bytes: &[u8]) -> Self {
        let hex = hex::encode_upper(bytes);
        Self {
            bytes: bytes.to_vec(),
            hex,
            field_details: Vec::new(),
            cmd: None,
            device_no: None,
            device_id: None,
            temp_bytes: Vec::new(),
            direction: DirectionEnum::Upstream,
            success: true,
        }
    }

    pub fn new_downstream(cmd: T, device_no: &str, device_id: &str) -> Self {
        Self {
            bytes: Vec::new(),
            hex: String::new(),
            field_details: Vec::new(),
            cmd: Some(cmd),
            device_no: Some(device_no.into()),
            device_id: Some(device_id.into()),
            temp_bytes: Vec::new(),
            direction: DirectionEnum::Downstream,
            success: true,
        }
    }

    // 获取一个唯一值。它由device_id和device_no一起组成进行md5加密
    pub fn get_unique_id(&self) -> crate::defi::ProtocolResult<String> {
        let device_no = if let Some(dn) = self.device_no.as_ref() {
            dn.clone()
        } else {
            "0".into()
        };

        let device_id = if let Some(dn) = self.device_id.as_ref() {
            dn.clone()
        } else {
            "0".into()
        };

        if device_no == "0" && device_id == "0" {
            return Err(ProtocolError::CommonError(
                "RawCapsule requires at least 1 of device_no and device_id but found both none"
                    .into(),
            ));
        }
        crate::md5_digester::Md5Digester::digest_str_with_salt(&device_no, &device_id)
    }

    pub fn new_downstream_from_upstream(up_stream_capsule: &RawCapsule<T>) -> Self {
        let device_no = if up_stream_capsule.device_no.is_some() {
            up_stream_capsule.device_no.clone()
        } else {
            None
        };

        let device_id = if up_stream_capsule.device_id.is_some() {
            up_stream_capsule.device_id.clone()
        } else {
            None
        };
        Self {
            bytes: Vec::new(),
            hex: String::new(),
            field_details: Vec::new(),
            cmd: up_stream_capsule.get_cmd_clone(),
            device_no,
            device_id,
            temp_bytes: Vec::new(),
            direction: DirectionEnum::Downstream,
            success: true,
        }
    }

    pub fn fail(&mut self) {
        self.success = false;
    }

    pub fn is_success(&self) -> bool {
        self.success
    }

    pub fn get_bytes_ref(&self) -> &[u8] {
        &self.bytes
    }

    // 把二进制塞回去，同时自动生成hex,通常用于出口的capsule
    pub fn set_bytes_and_generate_hex(&mut self, bytes: &[u8]) -> crate::defi::ProtocolResult<()> {
        self.bytes = bytes.to_vec();
        self.hex = crate::utils::hex_util::bytes_to_hex(bytes)?;
        Ok(())
    }

    pub fn is_upstream(&self) -> bool {
        self.direction.is_upstream()
    }

    pub fn is_downstream(&self) -> bool {
        self.direction.is_downstream()
    }

    pub fn set_device_id(&mut self, device_id: &str) {
        self.device_id = Some(device_id.into());
    }

    pub fn set_device_no(&mut self, device_no: &str) {
        self.device_no = Some(device_no.into());
    }

    pub fn set_cmd(&mut self, cmd: T) {
        self.cmd = Some(cmd);
    }

    pub fn set_temp_bytes(&mut self, bytes: &[u8]) {
        self.temp_bytes = bytes.to_vec();
    }

    pub fn get_temp_bytes_clone(&self) -> Vec<u8> {
        self.temp_bytes.clone().to_vec()
    }

    pub fn get_cmd_ref(&self) -> Option<&T> {
        self.cmd.as_ref()
    }

    pub fn get_cmd_clone(&self) -> Option<T>
    where
        T: DynClone, // 约束 T 必须支持动态克隆
    {
        self.cmd.as_ref().map(|cmd| {
            // 利用 dyn_clone 克隆 trait 对象
            dyn_clone::clone(cmd)
        })
    }
    pub fn set_fields(&mut self, fields: Vec<ReportField>) {
        self.field_details = fields;
    }

    pub fn append_fields(&mut self, fields: Vec<ReportField>) {
        self.field_details.extend(fields);
    }

    pub fn prepend_fields(&mut self, fields: Vec<ReportField>) {
        let mut new_fields = fields;
        new_fields.append(&mut self.field_details);
        self.field_details = new_fields;
    }
}
