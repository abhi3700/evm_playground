#![allow(unused_variables)]
// #![allow(unused_imports)]
#![allow(dead_code)]

// The `prelude` module provides a convenient way to import a number
// of common dependencies at once. This can be useful if you are working
// with multiple parts of the library and want to avoid having
// to import each dependency individually.
use ethers::core::rand::thread_rng;
use ethers::prelude::*;
use ethers::signers::LocalWallet;
mod uniswap_v2_pair;
use ethers::types::Bytes;
use ethers::utils::hex;

mod utils;
use utils::*;

mod simple;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let x = simple::main().await?;

    Ok(())
}
// NOTE: requires multi-thread feature on `tokio`
// we can customize as well in terms of no. of CPUs allotted for the work
// #[tokio::main]
// async fn main() -> eyre::Result<()> {
//     dotenv::from_path("./.env").expect("Failed in loading the file");
//     // let rpc_url = std::env::var("SEPOLIA_RPC_URL").expect("Please check if \'RPC URL\' empty");

//     // Subspace EVM RPC URL
//     // let rpc_url = "http://localhost:8545";
//     let rpc_url = "https://domain-3.evm.gemini-3f.subspace.network/ws".to_string();

//     // get the provider
//     let provider = Provider::<Http>::try_from(rpc_url).expect("Failed to connect w RPC");

//     // gas price
//     println!("gas price (in wei): {:?}", provider.get_gas_price().await?);

//     // gas limit
//     // So, just get a block with any number or hash. And then `Block` struct has a field `gas_limit` which can be called.
//     // NOTE: It is calculated on a per block basis. And it is same unless the chain is upgraded (like in Substrate) where
//     // we can change the block_gas_limit after a specific block.
//     let block_gas_limit = provider.get_block(1).await?.unwrap().gas_limit;
//     println!("Gas limit (per block): {}", block_gas_limit);

//     // get node info as struct
//     // let node_info = provider.node_info().await?;
//     // println!("Node info: {:?}", node_info); // for infura node, it's not available

//     // get block number
//     let current_block_number: U64 = provider.get_block_number().await?;
//     println!("current block number: {current_block_number}");

//     // get content of node's mempool
//     // ERROR: This method is not found in public node, may be it's there is local node.
//     // let tx_pool_content = provider.txpool_content().await?;
//     // println!("mempool content: {:?}", tx_pool_content);

//     // define address as `Address` H160
//     let addr = "0x0370D871f1D4B256E753120221F3Be87A40bd246".parse::<Address>()?;
//     // get current ETH balance of an account
//     let wei_bal = provider.get_balance(addr, None).await?;
//     // get the decimal value when divided by 10^18
//     let eth_bal = wei_bal.as_usize() as f64 / 1e18;
//     // let eth_bal = wei_bal.as_usize() as f64 / 1e18;
//     println!("balance: {} Wei", wei_bal);
//     println!("balance: {eth_bal:.18} ETH");
//     let eth_bal = ethers::utils::parse_units(wei_bal, "ether").unwrap();
//     println!("balance: {eth_bal:.18} ETH");

//     // === transfer some eth
//     let _ = transfer_eth(&provider).await?;

//     // get nonce of an account
//     let nonce = provider
//         .get_transaction_count(addr, Some(current_block_number.into()))
//         .await?;
//     println!("{} has a nonce: {}", addr, nonce);

//     // generate a single random keypair
//     let mut rng = thread_rng();
//     let wallet = LocalWallet::new(&mut rng);
//     println!("Successfully created new keypair.");
//     println!("Address:     {:?}", wallet.address());
//     println!("Private key: 0x{}", hex::encode(wallet.signer().to_bytes()));

//     // generate multiple random keypairs
//     for i in 0..5 {
//         let mut rng = thread_rng();
//         let wallet = LocalWallet::new(&mut rng);
//         println!("Address[{i}]:     {:?}", wallet.address());
//         println!(
//             "Private key[{i}]: 0x{}",
//             hex::encode(wallet.signer().to_bytes())
//         );
//     }

//     // recommended to not prefix w "0x"
//     let priv_key =
//         "d013bd87f33f1b38a895d549c6696e53d71cf44c29156452a48917923a0fa7d3".parse::<Bytes>()?;
//     // TODO: get address from private key
//     // let address = provider.import_raw_key(priv_key, "".to_string()).await?;
//     // println!("imported address: {}", address);

//     // TODO: Connect to a contract using Rust bindings generated by Foundry tool

//     // TODO: send a transaction

//     // TODO: call a function

//     // TODO: list the functions of a contract
//     let default_sender = provider.default_sender().unwrap_or_default();
//     println!("Default sender: {}", default_sender); // if None, then returns default

//     // TODO: check if a function exists in a contract

//     // In order to use this, just remove `#[tokio::main]` annotation over the `main` fn of `uniswap_v2_pair`.
//     // Just define the function as `async`
//     let mid_price = uniswap_v2_pair::main().await?;
//     println!("ETH/USDT price: {mid_price:.2}");

//     Ok(())
// }
