// Given there are 1 coffees left in the machine
// And I have deposited 1 dollar
// When I press the coffee button
// Then I should be served a coffee

use core::num::NonZero;

use bdd::{create_stdout_writer, given_data, WhenContext};

#[test]
fn test_coffee_machine() {
    coffee_machine_scenario(1, NonZero::new(100).unwrap())
        .then("I should be served a coffee", |coffee| coffee.unwrap());
}

#[test]
fn test_coffee_machine_not_enough_deposit() {
    coffee_machine_scenario(1, NonZero::new(10).unwrap())
        .then("I should be not be served a coffee", |coffee| {
            assert!(coffee.is_none())
        });
}

#[test]
fn test_coffee_machine_not_enough_coffees() {
    coffee_machine_scenario(0, NonZero::new(100).unwrap())
        .then("I should be not be served a coffee", |coffee| {
            assert!(coffee.is_none())
        });
}

fn coffee_machine_scenario(coffees: u16, deposit: NonZero<u64>) -> WhenContext<Option<Coffee>> {
    given_data(
        format!("there are {coffees} coffees left in the machine"),
        create_matchine(coffees),
        create_stdout_writer(),
    )
    .and(
        format!("I have deposited {deposit} cents"),
        |mut machine| {
            machine.deposit(deposit);
            machine
        },
    )
    .when("I press the coffee button", |mut machine| {
        machine.make_coffee()
    })
}

struct Machine {
    coffees: Vec<Coffee>,
    deposit: u64,
}

impl Machine {
    fn deposit(&mut self, deposit: NonZero<u64>) {
        self.deposit = deposit.get();
    }

    fn make_coffee(&mut self) -> Option<Coffee> {
        if self.deposit >= 100 {
            self.deposit = 0;
            self.coffees.pop()
        } else {
            None
        }
    }
}

struct Coffee {}

fn create_matchine(coffees: u16) -> Machine {
    Machine {
        coffees: (0..coffees).map(|_| Coffee {}).collect(),
        deposit: 0,
    }
}
