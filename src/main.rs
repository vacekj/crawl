use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use std::error::Error;
use std::time::Instant;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "";
    let url = format!("https://mainnet.infura.io/v3/{}", api_key);
    let provider = Provider::<Http>::try_from(url)?;

    let mut all_transactions = vec![];

    let mut contract_txs = vec![];

    let mut bytecodes = vec![];

    for block_number in 11031667..12031667 {
        let start = Instant::now();

        let txs = &provider.get_block_with_txs(block_number).await?.unwrap();
        // println!("got block nr {} with txs {}", block_number, txs.transactions.len());
        for tx in &txs.transactions {
            if tx.to.is_none() {
                println!("got contract creation tx");
                contract_txs.push(tx.clone());
                bytecodes.push(tx.input.clone());
            }
        }
        all_transactions.extend(txs.transactions.clone());
        let end = Instant::now();
        let elapsed_time = end - start;
        println!("Elapsed time: {:?}", elapsed_time);
    }
    let contract_size_total: usize = bytecodes.iter().map(|b| b.len()).sum();

    println!("got {} contract creations with a total of {} bytes of code",
             contract_txs.len(), contract_size_total);

    Ok(())
}