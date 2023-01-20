#![cfg(feature = "std")]

use std::string::String;
use std::{cell::RefCell, rc::Rc};

use arrayvec::ArrayVec;

use super::{NetworkPort, NetworkRuntime};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TestNetworkPacket {
    pub data: ArrayVec<u8, 255>,
    pub interface: String,
    pub time_critical: bool,
    pub index: usize,
}

#[derive(Debug, Default)]
pub struct TestRuntimeData {
    pub packet_buffer: ArrayVec<TestNetworkPacket, 255>,
}

#[derive(Debug, Clone, Default)]
pub struct TestRuntime {
    pub data: Rc<RefCell<TestRuntimeData>>,
}

#[derive(Debug, Default)]
pub struct TestRuntimePort {
    pub data: Rc<RefCell<TestRuntimeData>>,
    pub interface: String,
    pub time_critical: bool,
    pub send_index: usize,
}

#[derive(Debug)]
pub enum TestError {}

impl TestRuntime {
    pub fn get_sent(&self) -> Option<TestNetworkPacket> {
        self.data.borrow_mut().packet_buffer.pop()
    }
}

impl NetworkRuntime for TestRuntime {
    type InterfaceDescriptor = String;
    type PortType = TestRuntimePort;
    type Error = TestError;

    fn open(
        &mut self,
        interface: Self::InterfaceDescriptor,
        time_critical: bool,
    ) -> Result<Self::PortType, Self::Error> {
        Ok(TestRuntimePort {
            data: Rc::clone(&self.data),
            interface,
            time_critical,
            send_index: 0,
        })
    }

    fn recv(&mut self) -> Result<super::NetworkPacket, Self::Error> {
        todo!()
    }
}

impl NetworkPort for TestRuntimePort {
    fn send(&mut self, data: &[u8]) -> Option<usize> {
        let index = self.send_index;
        let mut data_array = ArrayVec::<u8, 255>::new();
        for item in data.iter() {
            data_array.push(*item);
        }

        self.send_index += 1;
        self.data
            .borrow_mut()
            .packet_buffer
            .push(TestNetworkPacket {
                data: data_array,
                interface: self.interface.clone(),
                time_critical: self.time_critical,
                index,
            });

        if self.time_critical {
            Some(index)
        } else {
            None
        }
    }
}
