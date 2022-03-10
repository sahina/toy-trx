use rust_decimal_macros::dec;

use toy_trx::model::{Account, Transaction, TxType};

#[test]
fn deposit_account() {
    let mut account = Account::new(1);
    let tx = Transaction::new(1, 1, TxType::DEPOSIT, Some(dec!(100)));
    account.deposit(tx).unwrap();

    assert_eq!(dec!(100), account.total);
    assert_eq!(dec!(100), account.available);
    assert_eq!(dec!(0), account.held);
}


#[test]
fn withdraw_with_funds() {
    let tx_d = Transaction::new(1, 1, TxType::DEPOSIT, Some(dec!(100)));
    let tx_w = Transaction::new(1, 2, TxType::WITHDRAWAL, Some(dec!(40)));
    let mut account = Account::new(1);
    account.deposit(tx_d).unwrap();
    account.withdraw(tx_w).unwrap();

    assert_eq!(dec!(60), account.total);
    assert_eq!(dec!(60), account.available);
}

#[test]
fn withdraw_without_funds() {
    let tx_w = Transaction::new(1, 1, TxType::WITHDRAWAL, Some(dec!(40)));
    let mut account = Account::new(1);
    let result = account.withdraw(tx_w);

    assert_eq!(true, result.is_err());
}

#[test]
fn dispute() {
    let mut account = Account::new(1);
    let tx_d = Transaction::new(1, 1, TxType::DEPOSIT, Some(dec!(10)));
    account.deposit(tx_d).unwrap();

    let result = account.dispute(1);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
fn no_duplicate_disputes() {
    let tx = Transaction::new(1, 1, TxType::WITHDRAWAL, Some(dec!(10)));
    let mut account = Account::new(1);
    account.add_tx(tx).unwrap();

    let result1 = account.dispute(1);
    let result2 = account.dispute(1);

    assert!(result1.is_ok());
    assert_eq!(result1.unwrap(), true);

    assert!(result2.is_ok());
    assert_eq!(result2.unwrap(), false);
}

#[test]
fn resolve_account() {
    let mut account = Account::new(1);
    let tx_d = Transaction::new(1, 1, TxType::DEPOSIT, Some(dec!(10)));

    // dispute a tx
    account.deposit(tx_d).unwrap();
    account.dispute(1).unwrap();

    // resolve it
    let result = account.resolve(1);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}
