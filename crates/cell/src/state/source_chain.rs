use super::{
    chain_cas::{ChainCasBuffer, HeaderCas},
    chain_sequence::ChainSequenceBuffer,
};
use core::ops::Deref;
use db::ReadManager;
use sx_state::{
    buffer::StoreBuffer,
    db::{self, DbManager},
    error::WorkspaceResult,
    Readable, Reader, RkvEnv, Writer,
};
use sx_types::{
    chain_header::ChainHeader,
    entry::Entry,
    prelude::Address,
    signature::{Provenance, Signature},
};

pub struct SourceChainBuffer<'env, R: Readable> {
    cas: ChainCasBuffer<'env, R>,
    sequence: ChainSequenceBuffer<'env, R>,
    rm: &'env ReadManager<'env>,
}

impl<'env, R: Readable> SourceChainBuffer<'env, R> {
    pub fn new(
        reader: &'env R,
        dbm: &'env DbManager,
        rm: &'env ReadManager,
    ) -> WorkspaceResult<Self> {
        Ok(Self {
            cas: ChainCasBuffer::primary(reader, dbm)?,
            sequence: ChainSequenceBuffer::new(reader, dbm)?,
            rm,
        })
    }

    pub fn chain_head(&self) -> Option<&Address> {
        self.sequence.chain_head()
    }

    pub fn get_entry(&self, k: &Address) -> WorkspaceResult<Option<Entry>> {
        self.cas.get_entry(k)
    }

    pub fn get_header(&self, k: &Address) -> WorkspaceResult<Option<ChainHeader>> {
        self.cas.get_header(k)
    }

    pub fn cas(&self) -> &ChainCasBuffer<R> {
        &self.cas
    }

    pub fn put_entry(&mut self, entry: Entry) -> () {
        let header = self.header_for_entry(&entry);
        self.cas.put((header, entry));
    }

    pub fn headers(&self) -> &HeaderCas<'env, R> {
        &self.cas.headers()
    }

    pub fn try_commit(&self, writer: &'env mut Writer) -> WorkspaceResult<()> {
        unimplemented!()
    }

    fn header_for_entry(&self, entry: &Entry) -> ChainHeader {
        unimplemented!()
        // let provenances = &[Provenance::new(
        //     self.snapshot.agent_id().unwrap().address(),
        //     Signature::fake(),
        // )];
        // let timestamp = chrono::Utc::now().timestamp().into();
        // let header = ChainHeader::new(
        //     entry.entry_type(),
        //     entry.address(),
        //     provenances,
        //     Some(self.new_head.clone()),
        //     None,
        //     None,
        //     timestamp,
        // );
        // Ok(header)
    }
}

impl<'env, R: Readable> StoreBuffer<'env> for SourceChainBuffer<'env, R> {
    fn finalize(self, writer: &'env mut Writer) -> WorkspaceResult<()> {
        self.cas.finalize(writer)?;
        self.sequence.finalize(writer)?;
        Ok(())
    }
}

#[cfg(test)]
pub mod tests {

    use super::{SourceChainBuffer, StoreBuffer};
    use sx_state::{
        db::{DbManager, ReadManager, WriteManager},
        env::create_lmdb_env,
        error::WorkspaceResult,
        test_utils::test_env,
    };
    use tempdir::TempDir;

    #[test]
    fn asdf() -> WorkspaceResult<()> {
        let arc = test_env();
        let env = arc.read().unwrap();
        let dbm = DbManager::new(&env)?;
        let rm = ReadManager::new(&env);
        rm.with_reader(|reader| {
            let source_chain = SourceChainBuffer::new(&reader, &dbm, &rm)?;
            Ok(())
        })?;
        Ok(())
    }
}