#![allow(clippy::uninlined_format_args)]
#![deny(unused_qualifications)]

use std::collections::BTreeMap;
use std::fmt::Display;
use std::io::Cursor;
use std::path::Path;
use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use openraft::Config;
use tokio::net::TcpListener;

use crate::app::App;
use crate::network::api;
use crate::network::management;
use crate::network::raft;
use crate::network::Network;
use crate::store::new_storage;
use crate::store::Request;
use crate::store::Response;

pub mod app;
pub mod log_store;
pub mod network;
pub mod store;

pub type NodeId = u64;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
pub struct Node {
    pub rpc_addr: String,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ rpc_addr: {} }}", self.rpc_addr)
    }
}

openraft::declare_raft_types!(
    pub TypeConfig:
        D = Request,
        R = Response,
        Node = Node,
        SnapshotData = Box<Cursor<Vec<u8>>>,
);

#[path = "./declare_types.rs"]
pub mod typ;

pub async fn start_example_raft_node<P>(node_id: NodeId, dir: P, listen: String,clusters: Vec<String>) -> Result<()>
where P: AsRef<Path> {
    // Create a configuration for the raft instance.
    let config = Config {
        heartbeat_interval: 500,
        election_timeout_min: 299,
        ..Default::default()
    };

    let config = Arc::new(config.validate().unwrap());

    let (log_store, state_machine_store) = new_storage(&dir).await;

    let kvs = state_machine_store.data.kvs.clone();

    // Create the network layer that will connect and communicate the raft instances and
    // will be used in conjunction with the store created above.
    let network = Network {};

    // Create a local raft instance.
    let raft = openraft::Raft::new(node_id, config.clone(), network, log_store, state_machine_store).await.unwrap();

    // Init member if not ready
    let mut members = BTreeMap::new();
    for (index, rpc_addr) in clusters.iter().enumerate() {
        let node = Node {
            rpc_addr: rpc_addr.clone(),
        };
        members.insert(index as u64, node);
    }
    if !raft.is_initialized().await? {
        raft.initialize(members).await?;
    }

    let app = Arc::new(App {
        id: node_id,
        rpc_addr: listen.clone(),
        raft,
        key_values: kvs,
        config,
    });

    let router = Router::new().merge(raft::rest()).merge(api::rest()).merge(management::rest()).with_state(app);

    let listener = TcpListener::bind(listen.clone()).await?;
    axum::serve(listener, router).await?;
    tracing::info!("App Server listening on: {}", listen);
    Ok(())
}
