use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use anyhow::{bail, Error, Result};
use csv::StringRecord;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TxType {
    DEPOSIT,
    WITHDRAWAL,
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
            "dispute" => Ok(TxType::DISPUTE),
            "resolve" => Ok(TxType::RESOLVE),
            "chargeback" => Ok(TxType::CHARGEBACK),
            _ => bail!("Can not convert {} to enum", s),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Account {
    pub client: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
    pub disputes: HashSet<u32>,
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

    pub fn deposit(&mut self, tx: &Transaction) -> Result<()> {
        // todo: can we deposit if account is locked?
        self.total = self.total + tx.amount.unwrap();
        self.available = self.available + tx.amount.unwrap();

        self.ledger.insert(tx.tx, tx.clone());

        Ok(())
    }

    pub fn withdraw(&mut self, tx: &Transaction) -> Result<()> {
        // todo: can we withdraw if account is locked?
        if &self.available < &tx.amount.unwrap() {
            bail!("Insufficient funds")
        } else {
            self.total = self.total - tx.amount.unwrap();
            self.available = self.available - tx.amount.unwrap();
            self.ledger.insert(tx.tx, tx.clone());

            Ok(())
        }
    }

    pub fn dispute(&mut self, tx: u32) -> Result<bool> {
        if self.disputes.contains(&tx) {
            // tx already disputed
            // client side error, ignore without error
            Ok(false)
        } else {
            let result = self.disputes.insert(tx);
            let disputed_tx = self.ledger.get(&tx).unwrap();

            self.available = self.available - disputed_tx.amount.unwrap();
            self.held = self.held + disputed_tx.amount.unwrap();

            Ok(result)
        }
    }

    /// Like disputes, resolves do not specify an amount.
    /// Instead they refer to a transaction that was under dispute by ID.
    /// If the tx specified does not exist, or the tx isn’t under dispute,
    /// you can ignore the resolve and assume this is an error on our partner’s side.
    pub fn resolve(&mut self, tx: u32) -> Result<bool> {
        if self.disputes.contains(&tx) {
            let result = self.disputes.remove(&tx);
            let disputed_tx = self.ledger.get(&tx).unwrap();

            self.available = self.available + disputed_tx.amount.unwrap();
            self.held = self.held - disputed_tx.amount.unwrap();

            Ok(result)
        } else {
            // ignore
            Ok(false)
        }
    }

    pub fn chargeback(&mut self, _tx: u32) -> Result<bool> {
        // todo: impl
        Ok(true)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Transaction {
    pub client: u16,
    pub tx: u32,
    pub tx_type: TxType,
    pub amount: Option<Decimal>,
}

impl Transaction {
    pub fn new(client: u16, tx: u32, tx_type: TxType, amount: Option<Decimal>) -> Self {
        Transaction {
            client,
            tx,
            tx_type,
            amount,
        }
    }
}

impl From<&StringRecord> for Transaction {
    fn from(record: &StringRecord) -> Self {
        let tx_type = record
            .get(0)
            .unwrap()
            .parse::<TxType>()
            .unwrap();
        let client = record
            .get(1)
            .unwrap()
            .parse::<u16>()
            .unwrap();
        let tx = record
            .get(2)
            .unwrap()
            .parse::<u32>()
            .unwrap();

        let mut amount: Option<Decimal> = None;
        match record.get(3) {
            None => {}
            Some(parsed_amount) => {
                let currency = parsed_amount.parse::<Decimal>().unwrap_or_default();
                if currency != dec!(0) {
                    amount = Some(currency);
                }
            }
        }

        // better way to
        Transaction {
            client,
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

#[derive(Debug)]
pub struct Output {
    pub client: u16,
    pub available: Decimal,
    pub held: Decimal,
    pub total: Decimal,
    pub locked: bool,
}