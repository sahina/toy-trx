use rust_decimal::Decimal;

use toy_trx::model::Account;

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
