use crate::frame::{Frame, Head, Kind, StreamId};
use bytes::{BufMut, Bytes, BytesMut};

/// An HTTP/2 extension frame whose type is not understood by h2.
#[derive(Debug, Eq, PartialEq)]
pub struct Extension {
    frame_type: u8,
    flags: u8,
    stream_id: StreamId,
    payload: Bytes,
}

impl Extension {
    pub(crate) fn load(head: Head, payload: Bytes) -> Extension {
        let Kind::Unknown(frame_type) = head.kind() else {
            unreachable!("extension frame must have an unknown type");
        };

        Extension {
            frame_type,
            flags: head.flag(),
            stream_id: head.stream_id(),
            payload,
        }
    }

    /// Returns the extension frame type.
    pub fn frame_type(&self) -> u8 {
        self.frame_type
    }

    /// Returns the extension-defined flags.
    pub fn flags(&self) -> u8 {
        self.flags
    }

    /// Returns the stream associated with this frame.
    pub fn stream_id(&self) -> crate::StreamId {
        crate::StreamId::from_internal(self.stream_id)
    }

    /// Returns the extension frame payload.
    pub fn payload(&self) -> &Bytes {
        &self.payload
    }

    /// Consumes the frame and returns its payload.
    pub fn into_payload(self) -> Bytes {
        self.payload
    }

    pub(crate) fn encode(&self, dst: &mut BytesMut) {
        Head::new(Kind::Unknown(self.frame_type), self.flags, self.stream_id)
            .encode(self.payload.len(), dst);
        dst.put_slice(&self.payload);
    }

    pub(crate) fn internal_stream_id(&self) -> StreamId {
        self.stream_id
    }
}

impl<T> From<Extension> for Frame<T> {
    fn from(src: Extension) -> Frame<T> {
        Frame::Extension(src)
    }
}
