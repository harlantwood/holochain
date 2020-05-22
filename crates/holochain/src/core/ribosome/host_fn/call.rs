use crate::core::ribosome::error::RibosomeResult;
use crate::core::ribosome::wasm_ribosome::WasmRibosome;
use crate::core::ribosome::HostContext;
use holochain_zome_types::CallInput;
use holochain_zome_types::CallOutput;
use std::sync::Arc;

pub async fn call(
    _ribosome: Arc<WasmRibosome>,
    _host_context: Arc<HostContext>,
    _input: CallInput,
) -> RibosomeResult<CallOutput> {
    unimplemented!();
}