extern crate core;

use std::fs;

use csv::{Reader, ReaderBuilder};
use rust_decimal_macros::dec;

use toy_trx::{to_transaction, try_to_transaction};
use toy_trx::model::{Transaction, TxType};

static INPUT: &str = r#"
deposit,1,1,1.0
deposit,1,2,2.0
withdrawal,1,3,1.0
dispute,1,2,
resolve,1,2,
chargeback,1,2,
"#;

#[test]
fn parse_csv_to_transactions() {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(INPUT.as_bytes());

    assert_eq!(6, rdr.records().count());

    for result in rdr.records() {
        let record = result;
        match record {
            Ok(record) => {
                let trx = Transaction::from(&record);

                match trx.tx_type {
                    TxType::DEPOSIT => {
                        assert_eq!(trx.tx_type, TxType::DEPOSIT);
                    }
                    TxType::WITHDRAWAL => {
                        assert_eq!(trx.tx_type, TxType::WITHDRAWAL);
                    }
                    TxType::DISPUTE => {
                        assert_eq!(trx.tx_type, TxType::DISPUTE);
                    }
                    TxType::RESOLVE => {
                        assert_eq!(trx.tx_type, TxType::RESOLVE);
                    }
                    TxType::CHARGEBACK => {
                        assert_eq!(trx.tx_type, TxType::CHARGEBACK);
                    }
                    _ => {
                        panic!("should have encountered known transaction")
                    }
                }
            }
            Err(_err) => {
                panic!("can not parse ")
            }
        }
    }
}

#[test]
fn parse_deposit_tx() {
    let deposit = "deposit,1,1,1.0";
    let trx = to_transaction(deposit);

    assert_eq!(trx.client, 1);
    assert_eq!(trx.tx, 1);
    assert_eq!(trx.tx_type, TxType::DEPOSIT);
    assert_eq!(trx.amount, Some(dec!(1.0)));
}

#[test]
fn parse_withdrawal_tx() {
    let withdrawal = "withdrawal,1,1,1.0";
    let trx = to_transaction(withdrawal);

    assert_eq!(trx.client, 1);
    assert_eq!(trx.tx, 1);
    assert_eq!(trx.tx_type, TxType::WITHDRAWAL);
    assert_eq!(trx.amount, Some(dec!(1.0)));
}

#[test]
fn parse_dispute_tx() {
    let dispute = "dispute,1,1,";
    let trx = to_transaction(dispute);

    assert_eq!(trx.client, 1);
    assert_eq!(trx.tx, 1);
    assert_eq!(trx.tx_type, TxType::DISPUTE);
    assert_eq!(trx.amount, None);
}

#[test]
fn parse_resolve_tx() {
    let resolve = "resolve,1,1,";
    let trx = to_transaction(resolve);

    assert_eq!(trx.client, 1);
    assert_eq!(trx.tx, 1);
    assert_eq!(trx.tx_type, TxType::RESOLVE);
    assert_eq!(trx.amount, None);
}

#[test]
fn parse_chargeback_tx() {
    let chargeback = "chargeback,1,1,";
    let trx = to_transaction(chargeback);

    assert_eq!(trx.client, 1);
    assert_eq!(trx.tx, 1);
    assert_eq!(trx.tx_type, TxType::CHARGEBACK);
    assert_eq!(trx.amount, None);
}

#[test]
fn try_parse_deposit_tx() {
    let deposit = "deposit,1,1,1.0";
    let trx = try_to_transaction(deposit);

    assert_eq!(trx.is_ok(), true);
}

#[test]
fn try_parse_withdrawal_tx() {
    let withdrawal = "withdrawal,1,1,1.0";
    let trx = try_to_transaction(withdrawal);

    assert_eq!(trx.is_ok(), true);
}

#[test]
fn try_parse_dispute_tx() {
    let dispute = "dispute,1,1,";
    let trx = try_to_transaction(dispute);

    assert_eq!(trx.is_ok(), true);
}

#[test]
fn try_parse_resolve_tx() {
    let resolve = "resolve,1,1,";
    let trx = try_to_transaction(resolve);

    assert_eq!(trx.is_ok(), true);
}

#[test]
fn try_parse_chargeback_tx() {
    let chargeback = "chargeback,1,1,";
    let trx = try_to_transaction(chargeback);

    assert_eq!(trx.is_ok(), true);
}

#[test]
fn parse_test_csv_file() {
    let contents = fs::read_to_string("./test-full.csv").unwrap();
    let mut rdr = Reader::from_reader(contents.as_bytes());

    assert_eq!(rdr.records().count(), 6);

    for result in rdr.records() {
        let record = result.unwrap();
        let transaction = Transaction::from(&record);

        assert!(transaction.tx > 0);
    }
}