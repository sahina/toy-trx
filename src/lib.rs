use anyhow::{bail, Result};
use csv::ReaderBuilder;

use crate::model::Transaction;

pub mod model;

/// Converts input row from csv file to `Transaction` struct
pub fn to_transaction(input_row: &str) -> Transaction {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(input_row.as_bytes());

    let record = rdr
        .records()
        .next()
        .unwrap()
        .unwrap();

    Transaction::from(&record)
}

pub fn try_to_transaction(input_row: &str) -> Result<Transaction> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(input_row.as_bytes());

    let record = rdr
        .records()
        .next();

    if let Some(row) = record {
        if let Ok(rec) = row {
            Ok(Transaction::from(&rec))
        } else {
            bail!("Invalid transaction")
        }
    } else {
        bail!("Input row can not be parsed")
    }
}