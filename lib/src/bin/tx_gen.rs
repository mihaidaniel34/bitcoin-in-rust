use std::process::exit;
use uuid::Uuid;
use lib::crypto::PrivateKey;
use lib::INITIAL_REWARD;
use lib::types::{Transaction, TransactionOutput};
use lib::utils::Saveable;

fn main() {
    let path = if let Some(path) = std::env::args().nth(1) {
        path
    } else  {
        eprintln!("Usage: tx_gen <tx_file>");
        exit(1);
    };

    let private_key = PrivateKey::new_key();
    let transaction = Transaction::new(
        vec![],
        vec![TransactionOutput {
            unique_id: Uuid::new_v4(),
            value: INITIAL_REWARD * 10u64.pow(8),
            pubkey: private_key.public_key(),
        }],
    );
    transaction.save_to_file(path).expect(
        "Failed to save transaction to file!",
    )

}