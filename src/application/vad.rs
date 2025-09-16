use crate::{
    domain::ports::vad::{Vad, VadEvent},
    infrastructure::vad::local_vad::LocalVadAdapter,
};

#[derive(Debug, Clone)]
pub enum VadList {
    Local(LocalVadAdapter),
}

impl Vad for VadList {
    fn add_bytes(&mut self, bytes: &Vec<i16>) {
        match self {
            VadList::Local(adapter) => adapter.add_bytes(bytes),
        }
    }

    fn take_bytes(&mut self) -> Vec<i16> {
        match self {
            VadList::Local(adapter) => adapter.take_bytes(),
        }
    }

    fn process_frame(&mut self, bytes: &Vec<i16>) -> Option<VadEvent> {
        match self {
            VadList::Local(adapter) => adapter.process_frame(bytes),
        }
    }
}
