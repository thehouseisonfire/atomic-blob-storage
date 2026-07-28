#[cfg(test)]
mod tests {
    use atomic_blob_store::{
        AtomicBlobStoreOptions, BlobFormatIdentity, BlockingAtomicBlobStore, ENVELOPE_VERSION_V1,
        tokio::AtomicBlobStore,
    };

    fn options() -> AtomicBlobStoreOptions {
        AtomicBlobStoreOptions::new(
            BlobFormatIdentity::new(b"APPSTATE", ".state", ENVELOPE_VERSION_V1).unwrap(),
        )
    }

    #[cfg(any(unix, windows))]
    #[test]
    fn blocking_api_is_usable_by_a_downstream_crate() {
        let root = tempfile::tempdir().unwrap();
        let store =
            BlockingAtomicBlobStore::open(root.path(), "application-state", options()).unwrap();
        store.save(b"preferences", b"dark-mode".to_vec()).unwrap();
        assert_eq!(
            store.load(b"preferences").unwrap(),
            Some(b"dark-mode".to_vec())
        );
        store.close().unwrap();
    }

    #[cfg(any(unix, windows))]
    #[tokio::test]
    async fn tokio_api_is_usable_by_a_downstream_crate() {
        let root = tempfile::tempdir().unwrap();
        let store = AtomicBlobStore::open(root.path(), "application-state", options())
            .await
            .unwrap();
        store
            .save(b"preferences", b"dark-mode".to_vec())
            .await
            .unwrap();
        assert_eq!(
            store.load(b"preferences").await.unwrap(),
            Some(b"dark-mode".to_vec())
        );
        store.close().await.unwrap();
    }
}
