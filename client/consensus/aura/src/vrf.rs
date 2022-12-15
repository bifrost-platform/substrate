use sp_keystore::{SyncCryptoStore, SyncCryptoStorePtr, vrf::VRFSignature};
use sp_runtime::traits::Block;
use sc_network_gossip::GossipEngine;
use std::{sync::Arc, marker::PhantomData};

// Parameters for vrf worker
pub struct VRFWorkerParams<B: Block, N> {
	pub network: N,
	pub gossip_engine: GossipEngine<B>,
	pub keystore: SyncCryptoStorePtr,
	pub vrf: Vec<VRFSignature>
}

pub struct VRFNetworkParams<B: Block, N> {
	pub network: Arc<N>,
	pub protocol_name: String,
	pub _marker: PhantomData<B>
}

struct VRFWorker<B: Block, N> {
	pub network: N,
	pub gossip_engine: GossipEngine<B>,
	pub keystore: SyncCryptoStorePtr,
	// Store vrf signature
	pub vrf: Vec<VRFSignature>,
}

impl<B, N> VRFWorker<B, N>
where
	B: Block,
	N: Send + Sync + Clone
{
	pub fn new(vrf_worker_params: VRFWorkerParams<B, N>) -> Self {

		let VRFWorkerParams {
			network,
			gossip_engine,
			keystore,
			vrf,
		} = vrf_worker_params;

		Self {
			network,
			gossip_engine,
			keystore,
			vrf,
		}
	}

	pub async fn run(&mut self) {}

	pub async fn receive_message(&mut self) {}

	pub async fn gossip_message(&mut self) {}

	fn create_vrf(&mut self) {}
}

// Start the vrf worker. Future should be run by executor.
pub fn start_vrf_worker() {
	todo!()
}
