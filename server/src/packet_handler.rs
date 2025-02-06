use crate::packet_error::PacketError;
use crate::state::State;
use protocol::prelude::{DecodePacket, PacketId};
use std::collections::HashMap;

pub struct PacketHandler<Output> {
    state: State,
    handlers: HashMap<u8, Box<dyn Fn(&[u8]) -> Result<Output, PacketError> + Send + Sync>>,
}

impl<Output> PacketHandler<Output> {
    // Create a new, empty PacketHandler.
    pub fn new(state: State) -> Self {
        Self {
            state,
            handlers: HashMap::new(),
        }
    }

    // Register a handler for the packet type T.
    // T must implement PacketId (which gives us a PACKET_ID constant)
    // and DecodePacket (to decode the payload).
    // The provided function takes a T and produces our Output.
    pub fn on<T>(mut self, func: impl Fn(T) -> Output + Send + Sync + 'static) -> Self
    where
        T: PacketId + DecodePacket + 'static,
    {
        // Using the associated PACKET_ID from T.
        let id = T::PACKET_ID;

        // Insert a boxed closure that will decode the payload into a T
        // and then apply the function.
        self.handlers.insert(
            id,
            Box::new(move |payload: &[u8]| {
                // Try to decode the packet.
                let packet = T::decode(payload)?;
                // Call our handler closure.
                Ok(func(packet))
            }),
        );
        self
    }

    // Given a packet_id and payload, try to look up a registered handler
    // and use it to process the payload.
    pub fn handle(&self, packet_id: u8, payload: &[u8]) -> Result<Output, PacketError> {
        if let Some(handler) = self.handlers.get(&packet_id) {
            handler(payload)
        } else {
            // Here we assume PacketError::new takes some state and the packet id.
            // You might need to adjust this to match your PacketError implementation.
            Err(PacketError::new(self.state.clone(), packet_id))
        }
    }
}
