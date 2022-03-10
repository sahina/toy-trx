use std::{env, fs, io};
use std::collections::HashMap;

use csv::ReaderBuilder;

use toy_trx::model::{Account, Transaction, TxType};

fn main() {
    // bank that holds accounts
    let mut bank: HashMap<u16, Account> = HashMap::new();

    // read file contents as string
    let contents = input_file_content();

    // create reader to iterate rows
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(contents.as_bytes());

    // parse rows as Transactions
    for result in rdr.records() {
        let record = result;

        // ignore parse error
        if let Ok(record) = record {
            // create transaction from csv row
            let trx = Transaction::from(&record);

            // create or find account from bank
            let mut account = bank
                .entry(trx.client)
                .or_insert(Account::new(trx.client));

            apply(&mut account, &trx);
        }
    }

    // write to console out
    print_out(&bank);
}

fn apply(account: &mut Account, trx: &Transaction) {
    match trx.tx_type {
        TxType::DEPOSIT => {
            account.deposit(trx).unwrap();
        }
        TxType::WITHDRAWAL => {
            // do not fail processing
            account.withdraw(trx).unwrap_or_default();
        }
        TxType::DISPUTE => {
            account.dispute(trx.tx).unwrap();
        }
        TxType::RESOLVE => {
            account.resolve(trx.tx).unwrap();
        }
        TxType::CHARGEBACK => {
            account.chargeback(trx.tx).unwrap();
        }
    }
}

fn input_file_content() -> String {
    let filename = filename_from_arg();
    let contents = fs::read_to_string(&filename)
        .expect("Could not read file!");

    contents
}

fn filename_from_arg() -> String {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1);

    // fail if not filename is passed
    filename
        .expect("Run cli with `cargo run -- <file>.csv`")
        .to_owned()
}

fn print_out(bank: &HashMap<u16, Account>) {
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(&["client", "available", "held", "total", "locked"]).unwrap();

    for (key, value) in bank {
        wtr.serialize((key, value.available, value.held, value.total, value.locked)).unwrap();
    }

    wtr.flush().unwrap();
}