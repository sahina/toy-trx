use std::{env, fs};
use std::collections::HashMap;

use csv::Reader;

use crate::model::{Account, Transaction, TxType};

mod model;

fn main() {
    // read file
    let filename = filename_from_arg();
    let contents = fs::read_to_string(&filename)
        .expect("Could not read file!");

    // store account in a hashmap (database)
    let accounts: HashMap<u16, Account> = HashMap::new();

    // parse file content as csv
    let mut rdr = Reader::from_reader(contents.as_bytes());

    // iterate each line in file and update account records
    for result in rdr.records() {
        let record = result;

        match record {
            Ok(record) => {
                let trx = Transaction::from(&record);

                // match on trx type for computing account state
                match trx.tx_type {
                    TxType::DEPOSIT => {
                        // load or create account
                        // let account = accounts.contains_key(trx)
                        // increase  amount and availability by the amount
                    }
                    TxType::WITHDRAWAL => {
                        // find account
                        // decrease  amount and availability by the amount
                        // If a client does not have sufficient available funds the withdrawal should fail
                        // and the total amount of funds should not change
                    }
                    TxType::DISPUTE => {
                        // This means that the clients available funds should decrease by the amount disputed,
                        // their held funds should increase by the amount disputed,
                        // while their total funds should remain the same.
                        // A dispute references the transaction that is disputed by ID
                    }
                    TxType::RESOLVE => {
                        // Represents a resolution to a dispute, releasing the associated held funds.
                        // Funds that were previously disputed are no longer disputed.
                    }
                    TxType::CHARGEBACK => {
                        // A chargeback is the final state of a dispute and represents
                        // the client reversing a transaction.
                        // Funds that were held have now been withdrawn.
                        // This means that the clients held funds and total funds should decrease
                        // by the amount previously disputed. If a chargeback occurs
                        // the client’s account should be immediately frozen.
                    }
                    _ => {}
                }

                let item = Account::new(1);

                println!("{:?}", item);
            }
            Err(_err) => {
                // invalid record format, ignore it or stop the process?
                // lets ignore and log
                // todo: bring in log/trace crate to log the error
            }
        }
    }

    // todo: send result to stdout
}

fn filename_from_arg() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1);

    // fail if not filename is passed
    filename
        .expect("Run cli with `cargo run -- <file>.csv`")
        .to_owned()
}

#[cfg(test)]
mod app_test {
    use std::fs;

    use csv::Reader;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;

    use crate::model::{Account, Transaction, TxType};

    #[test]
    fn create_account_with_zero_defaults() {
        let account = Account::new(1);

        assert_eq!(1, account.client);
        assert_eq!(Decimal::default(), account.available);
        assert_eq!(Decimal::default(), account.held);
        assert_eq!(Decimal::default(), account.total);
        assert_eq!(false, account.locked);
    }

    #[test]
    fn default_account_with_defaults() {
        let account = Account::default();

        assert_eq!(0, account.client);
        assert_eq!(Decimal::default(), account.available);
        assert_eq!(Decimal::default(), account.held);
        assert_eq!(Decimal::default(), account.total);
        assert_eq!(false, account.locked);
    }

    #[test]
    fn account_from_string_record() {
        let contents = fs::read_to_string("transactions.csv").unwrap();
        let mut rdr = Reader::from_reader(contents.as_bytes());
        for result in rdr.records() {
            let record = result.unwrap();
            let transaction = Transaction::from(&record);

            assert_eq!(record.get(0).unwrap().parse::<TxType>().unwrap(), transaction.tx_type);
            assert_eq!(record.get(1).unwrap().parse::<u32>().unwrap(), transaction.tx);
            assert_eq!(record.get(2).unwrap().parse::<Decimal>().unwrap(), transaction.amount);
        }
    }

    #[test]
    fn deposit_account() {
        let mut account = Account::default();
        let tx = Transaction::new(1, TxType::DEPOSIT, dec!(100));
        account.deposit(tx).unwrap();

        assert_eq!(dec!(100), account.total);
        assert_eq!(dec!(100), account.available);
        assert_eq!(dec!(0), account.held);
    }

    #[test]
    fn withdraw_with_funds() {
        let tx_d = Transaction::new(1, TxType::DEPOSIT, dec!(100));
        let tx_w = Transaction::new(1, TxType::WITHDRAWAL, dec!(40));
        let mut account = Account::default();
        account.deposit(tx_d).unwrap();
        account.withdraw(tx_w).unwrap();

        assert_eq!(dec!(60), account.total);
        assert_eq!(dec!(60), account.available);
    }

    #[test]
    fn withdraw_without_funds() {
        let tx_w = Transaction::new(1, TxType::WITHDRAWAL, dec!(40));
        let mut account = Account::default();
        let result = account.withdraw(tx_w);

        assert_eq!(true, result.is_err());
    }

    #[test]
    fn dispute() {
        let mut account = Account::default();
        let tx_d = Transaction::new(1, TxType::DEPOSIT, dec!(10));
        account.deposit(tx_d);

        let result = account.dispute(1);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn no_duplicate_disputes() {
        let tx = Transaction::new(1, TxType::WITHDRAWAL, dec!(10));
        let mut account = Account::default();
        account.add_tx(tx);

        let result1 = account.dispute(1);
        let result2 = account.dispute(1);

        assert!(result1.is_ok());
        assert_eq!(result1.unwrap(), true);

        assert!(result2.is_ok());
        assert_eq!(result2.unwrap(), false);
    }

    #[test]
    fn resolve_account() {
        let mut account = Account::default();
        let tx_d = Transaction::new(1, TxType::DEPOSIT, dec!(10));

        // dispute a tx
        account.deposit(tx_d);
        account.dispute(1).unwrap();

        // resolve it
        let result = account.resolve(1);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}