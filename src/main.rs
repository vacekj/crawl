use ethers::prelude::*;
use ethers::providers::{Http, Provider};
use std::error::Error;
use std::time::{Duration, Instant};
use std::{fmt::Write};

use indicatif::{ProgressBar, ProgressState, ProgressStyle};

#[derive(Debug)]
struct Contract {
    address: Address,
    code: Bytes,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:9545";
    let provider = Provider::<Http>::try_from(url)?;

    let mut contracts: Vec<Contract> = vec![];

    let time = Instant::now();

    let latest_block = *&provider.get_block_number().await?.as_u64();
    println!("latest block {}", latest_block);

    let start_block = 2499000;
    let pb = ProgressBar::new(latest_block - start_block);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-"));


    for block_number in start_block..latest_block {
        let txs = &provider.get_block_with_txs(block_number).await?.unwrap();
        // println!("got block nr {} with txs {}", block_number, txs.transactions.len());
        for tx in &txs.transactions {
            if tx.to.is_none() {
                let receipt = &provider.get_transaction_receipt(tx.hash).await?.unwrap();
                contracts.push(Contract { address: receipt.contract_address.unwrap(), code: tx.input.clone() })
            }
        }

        pb.set_position(latest_block - block_number);
    }

    let contract_size_total: usize = contracts.iter().map(|b| b.code.len()).sum();

    println!("got {} contract creations with a total of {} bytes of code",
             contracts.len(), contract_size_total);


    dbg!(&contracts);

    pb.finish_with_message("indexed");
    let end = Instant::now();
    let elapsed_time = end - time;
    println!("Elapsed time: {:?}, contracts/s {}", elapsed_time, contracts.len() / elapsed_time.as_secs() as usize);

    Ok(())
}
