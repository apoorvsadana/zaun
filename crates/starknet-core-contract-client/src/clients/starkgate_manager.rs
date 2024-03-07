use std::sync::Arc;

use crate::{interfaces::{
    StarkgateManager, ProxySupport
}, LocalWalletSignerMiddleware, StarknetContractClient};

use ethers::types::Address;

pub struct StarkgateManagerContractClient {
    manager: StarkgateManager<LocalWalletSignerMiddleware>,
    proxy_support: ProxySupport<LocalWalletSignerMiddleware>,
}

impl StarkgateManagerContractClient {
    pub fn new(address: Address, client: Arc<LocalWalletSignerMiddleware>) -> Self {
        Self {
            manager: StarkgateManager::new(address, client.clone()),
            proxy_support: ProxySupport::new(address, client.clone()),
        }
    }
}

impl AsRef<StarkgateManager<LocalWalletSignerMiddleware>> for StarkgateManagerContractClient {
    fn as_ref(&self) -> &StarkgateManager<LocalWalletSignerMiddleware> {
        &self.manager
    }
}

impl AsRef<ProxySupport<LocalWalletSignerMiddleware>> for StarkgateManagerContractClient {
    fn as_ref(&self) -> &ProxySupport<LocalWalletSignerMiddleware> {
        &self.proxy_support
    }
}

impl StarknetContractClient for StarkgateManagerContractClient {
    fn address(&self) -> Address {
        self.manager.address()
    }

    fn client(&self) -> Arc<LocalWalletSignerMiddleware> {
        self.manager.client()
    }
}