use crate::core::parts::raw_capsule::RawCapsule;
use crate::core::parts::traits::Cmd;

#[derive(Debug, Clone, Default)]
pub struct RawChamber<T: Cmd> {
    pub upstream: Option<RawCapsule<T>>,
    pub downstream: Option<RawCapsule<T>>,
}

impl<T: Cmd> RawChamber<T> {
    pub fn new() -> Self {
        Self {
            upstream: None,
            downstream: None,
        }
    }

    pub fn update_upstream(&mut self, upstream: RawCapsule<T>) {
        self.upstream = Some(upstream);
    }

    pub fn update_downstream(&mut self, downstream: RawCapsule<T>) {
        self.downstream = Some(downstream);
    }
}
