use anyhow::Result;
use clap::Parser;
use raft_kv::start_example_raft_node;
use tracing_subscriber::EnvFilter;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long)]
    pub cur_id: u64,

    #[clap(long, default_value="127.0.0.1:45678")]
    pub listen: String,

    #[clap(long)]
    pub data: String,

    #[clap(long)]
    pub rpc_addr: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Setup the logger
    tracing_subscriber::fmt()
    .with_target(true)
    .with_thread_ids(true)
    .with_level(true)
    .with_ansi(false)
    .with_env_filter(EnvFilter::from_default_env())
    .init();
    let args = Args::parse();
    start_example_raft_node(args.cur_id, args.data, args.listen, args.rpc_addr).await
}
