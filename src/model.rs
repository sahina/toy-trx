use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use anyhow::{bail, Error, Result};
use csv::StringRecord;
use rust_decimal::Decimal;

#[derive(Debug, PartialEq)]
pub enum TxType {
    DEPOSIT,
    WITHDRAWAL,
    REFUND,
    DISPUTE,
    RESOLVE,
    CHARGEBACK,
}

impl FromStr for TxType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "deposit" => Ok(TxType::DEPOSIT),
            "withdrawal" => Ok(TxType::WITHDRAWAL),
            "refund" => Ok(TxType::REFUND),
            "dispute" => Ok(TxType::DISPUTE),
            "resolve" => Ok(TxType::RESOLVE),
            "chargeback" => Ok(TxType::CHARGEBACK),
            _ => bail!("Can not convert {} to enum", s),
        }
    }
}

#[derive(Debug)]
pub struct Account {
    pub client: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
    pub disputes: HashSet<u32>,
    // can be  used for event sourced state calculations
    ledger: HashMap<u32, Transaction>,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            client: 0,
            available: Default::default(),
            held: Default::default(),
            total: Default::default(),
            locked: false,
            disputes: HashSet::default(),
            ledger: Default::default(),
        }
    }
}

impl Account {
    pub fn new(client: u16) -> Self {
        Account {
            client,
            available: Default::default(),
            held: Default::default(),
            total: Default::default(),
            locked: false,
            disputes: HashSet::default(),
            ledger: Default::default(),
        }
    }

    pub fn find_tx(&self, tx: u32) -> Option<&Transaction> {
        self.ledger.get(&tx)
    }

    pub fn add_tx(&mut self, tx: Transaction) -> Result<()> {
        self.ledger.insert(tx.tx, tx);

        Ok(())
    }

    pub fn deposit(&mut self, tx: Transaction) -> Result<()> {
        // todo: can we deposit if account is locked?
        self.total = self.total + tx.amount;
        self.available = self.available + tx.amount;

        self.ledger.insert(tx.tx, tx);

        Ok(())
    }

    pub fn withdraw(&mut self, tx: Transaction) -> Result<()> {
        // todo: can we withdraw if account is locked?
        if &self.available < &tx.amount {
            bail!("Insufficient funds")
        } else {
            self.total = self.total - tx.amount;
            self.available = self.available - tx.amount;
            self.ledger.insert(tx.tx, tx);

            Ok(())
        }
    }

    pub fn dispute(&mut self, tx: u32) -> Result<bool> {
        if self.disputes.contains(&tx) {
            // client side error, ignore without error
            Ok(false)
        } else {
            let result = self.disputes.insert(tx);
            let disputed_tx = self.ledger.get(&tx).unwrap();

            self.available = self.available - disputed_tx.amount;
            self.held = self.held + disputed_tx.amount;

            Ok(result)
        }
    }

    pub fn resolve(&mut self, tx: u32) -> Result<bool> {
        if self.disputes.contains(&tx) {
            let result = self.disputes.remove(&tx);
            let disputed_tx = self.ledger.get(&tx).unwrap();

            self.available = self.available + disputed_tx.amount;
            self.held = self.held - disputed_tx.amount;

            Ok(result)
        } else {
            // ignore
            Ok(false)
        }
    }

    pub fn chargeback() {}
}

//
// transaction models


#[derive(Debug)]
pub struct Transaction {
    pub tx: u32,
    pub tx_type: TxType,
    pub amount: Decimal,
}

impl Transaction {
    pub fn new(tx: u32, tx_type: TxType, amount: Decimal) -> Self {
        Transaction {
            tx,
            tx_type,
            amount,
        }
    }
}

// Not TryFrom, we want conversion to fail if parsing fails
impl From<&StringRecord> for Transaction {
    fn from(record: &StringRecord) -> Self {
        let tx_type = record
            .get(0)
            .unwrap()
            .parse::<TxType>()
            .unwrap();
        let tx = record
            .get(2)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let amount = record
            .get(3)
            .unwrap()
            .parse::<Decimal>()
            .unwrap();

        // better way to
        Transaction {
            tx,
            tx_type,
            amount,
        }
    }
}

impl From<&StringRecord> for Account {
    fn from(record: &StringRecord) -> Self {
        let client = record.get(1)
            .unwrap()
            .parse::<u16>()
            .unwrap();

        Account {
            client,
            available: Default::default(),
            held: Default::default(),
            total: Default::default(),
            locked: false,
            disputes: Default::default(),
            ledger: Default::default(),
        }
    }
}

pub struct Bank {
    accounts: HashMap<u16, Account>,
}

impl Bank {
    pub fn get_account(&mut self, client: u16) -> Option<&Account> {
        self.accounts.get(&client)
    }
}