//Rust-Witnet is free software: you can redistribute it and/or modify
//it under the terms of the GNU General Public License as published by
//the Free Software Foundation, either version 3 of the License, or
//(at your option) any later version.
//
//Rust-Witnet is distributed in the hope that it will be useful,
//but WITHOUT ANY WARRANTY; without even the implied warranty of
//MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License
//along with Rust-Witnet. If not, see <http://www.gnu.org/licenses/>.
//
//This file is based on grin/src/server.rs from
// <https://github.com/mimblewimble/grin>,
// originally developed by The Grin Developers and distributed under the
// Apache License, Version 2.0. You may obtain a copy of the License at
// <http://www.apache.org/licenses/LICENSE-2.0>.

use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicBool;
use std::thread;
use std::time;

use chain;
use core::{global, genesis};
use adapters::*;
use p2p;
use pool;
use types::*;
use util::LOGGER;

/// Witnet server holding internal structures.
pub struct Server {
    /// Server config
    pub config: ServerConfig,
    /// Network server handler
    //TODO p2p: Arc<p2p::Server>,
    /// Data store access
    //TODO chain: Arc<chain::Chain>,
    /// In-memory objects pool
    //TODO tx_pool: Arc<RwLock<pool::TransactionPool<PoolToChainAdapter>>>,
    /// Synchronization status flag
    currently_syncing: Arc<AtomicBool>
}

/// Witnet server method implementations.
impl Server {
    /// Instantiates and starts a new Witnet server.
    pub fn start(config: ServerConfig) -> Result<Server, Error> {
        //let mut mining_config = config.mining_config.clone();
        let _serv = Server::new(config)?;
        // TODO mining start

        loop {
            thread::sleep(time::Duration::from_secs(10));
        }
    }

    /// Instantiates a new server associated with the provided future reactor.
    pub fn new(config: ServerConfig) -> Result<Server, Error> {
        // These are the adapters for passing objects between the memory pool, chain and network
        let pool_adapter = Arc::new(PoolToChainAdapter::new());
        let pool_net_adapter = Arc::new(PoolToNetAdapter::new());
        let tx_pool = Arc::new(RwLock::new(pool::TransactionPool::new(
            config.pool_config.clone(),
            pool_adapter.clone(),
            pool_net_adapter.clone(),
        )));

        let chain_adapter = Arc::new(ChainToPoolAndNetAdapter::new(tx_pool.clone()));

        // Load genesis block matching current chain type
        let genesis = match config.chain_type {
            global::ChainTypes::Testnet1 => genesis::genesis_dev(),
            _ => {
                //TODO: mine my own genesis block
                genesis::genesis_dev()
            }
        };

        info!(
            LOGGER,
            "Starting server, genesis block: {}",
            genesis.hash(),
        );

        let shared_chain = Arc::new(chain::Chain::init(
            config.db_root.clone(),
            chain_adapter.clone(),
            genesis.clone(),
        )?);

        pool_adapter.set_chain(shared_chain.clone());

        let currently_syncing = Arc::new(AtomicBool::new(true));

        let net_adapter = Arc::new(NetToChainAdapter::new(
            currently_syncing.clone(),
            shared_chain.clone(),
            tx_pool.clone(),
        ));

        let p2p_config = config.p2p_config.clone();
        let p2p_server = Arc::new(p2p::Server::new(
            config.db_root.clone(),
            config.capabilities,
            p2p_config,
            net_adapter.clone(),
            genesis.hash(),
        )?);

        warn!(LOGGER, "Witnet server started.");
        Ok(Server {
            config: config,
//            p2p: p2p_server,
//            chain: shared_chain,
//            tx_pool: tx_pool,
            currently_syncing: currently_syncing,
        })
    }
}