// Given The account balance is $100
//       And the card is valid
//       And the machine contains enough money
//      When the Account Holder requests $20
//      Then the ATM should dispense $20
//       And the account balance should be $80
//       And the card should be returned

use bdd::{create_stdout_writer, given_data};

#[test]
fn test_withdrawal() {
    given_data(
        "The account balance is $100",
        Account { balance: 10000 },
        create_stdout_writer(),
    )
    .and("the card is valid", |account| {
        (account, Card { valid: true })
    })
    .and("the machine contains enough money", |(account, card)| {
        (account, card, Machine { money: 100000 })
    })
    .when(
        "the Account Holder requests $20",
        |(mut account, card, mut machine)| {
            let (money, card) = machine.request(&mut account, card, 2000);
            (money, account, card)
        },
    )
    .then("the ATM should dispense $20", |data| {
        assert_eq!(data.0, 2000);
        data
    })
    .and("the account balance should be $80", |data| {
        assert_eq!(data.1.balance, 8000);
        data.2
    })
    .and("the card should be returned", |card| card.unwrap());
}

struct Machine {
    money: u64,
}

impl Machine {
    fn request(&mut self, account: &mut Account, card: Card, money: u64) -> (u64, Option<Card>) {
        if !card.valid {
            return (0, None);
        }
        if account.balance >= money && self.money >= money {
            self.money -= money;
            account.balance -= money;
            return (money, card.into());
        }
        (0, card.into())
    }
}

struct Account {
    balance: u64,
}

struct Card {
    valid: bool,
}
