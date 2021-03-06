use crate::packets::PacketBody;
use crate::SliceCursor;

/// Social handshake.
///
/// Direction: Not used.
#[derive(Debug)]
pub struct SocialHandshake {}

impl PacketBody for SocialHandshake {
    const TAG: u8 = 93;

    fn write_body(&self, _cursor: &mut SliceCursor) {}

    fn from_body(_cursor: &mut SliceCursor) -> Self {
        Self {}
    }
}
