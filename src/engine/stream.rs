use super::{BlobMetadata, Pending, Receiver, SaveStreamMessage, Sender};
pub struct SaveStreamEndpoint {
    pub(crate) chunks: Sender<SaveStreamMessage>,
    pub(crate) result: Pending<()>,
}

pub struct LoadStreamEndpoint {
    pub(crate) chunks: Receiver<Vec<u8>>,
    pub(crate) acknowledgement: Sender<()>,
    pub(crate) result: Pending<Option<BlobMetadata>>,
}
