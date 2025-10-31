use crate::core::parts::raw_capsule::RawCapsule;
use crate::core::parts::traits::Cmd;

#[derive(Debug, Clone, Default)]
pub struct RawChamber<T: Cmd + Clone> {
    pub upstream: Option<RawCapsule<T>>,
    pub downstream: Option<RawCapsule<T>>,
    pub cmd_code: String,
    pub success: bool,
}

impl<T: Cmd + Clone> RawChamber<T> {
    pub fn new(in_capsule: &RawCapsule<T>, out_capsule: &RawCapsule<T>) -> Self {
        // 优先从 out_capsule 获取 cmd_code
        let cmd_code = out_capsule
            .cmd
            .as_ref()
            .map(|cmd| cmd.code())
            .or_else(|| in_capsule.cmd.as_ref().map(|cmd| cmd.code()))
            .unwrap_or_default();

        // 两个 capsule 的 success 都是 true 时，self.success 才为 true
        let success = in_capsule.success && out_capsule.success;

        Self {
            upstream: Some(in_capsule.clone()),
            downstream: Some(out_capsule.clone()),
            cmd_code,
            success,
        }
    }
}
