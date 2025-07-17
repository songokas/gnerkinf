// Given there are 3 ninjas
// And there are more than one ninja alive
// When 2 ninjas meet, they will fight
// Then one ninja dies (but not me)
// And there is one ninja less alive

use gnerkinf::create_stdout_writer;
use gnerkinf::given_data;
use gnerkinf::given_dataf;

#[test]
fn test_ninja() {
    let mut ninjas = [
        Ninja { alive: true },
        Ninja { alive: false },
        Ninja { alive: true },
    ];
    let initial_alive_ninja_count = ninjas.iter().filter(|ninja| ninja.alive).count();

    given_data(
        format!("there are {} ninjas", ninjas.len()),
        &mut ninjas,
        create_stdout_writer(),
    )
    .and("there are more than one ninja alive", |ninjas| {
        assert!(initial_alive_ninja_count > 1);
        let (first, second) = ninjas.split_at_mut(1);
        (&mut first[0], &mut second[1])
    })
    .when("2 ninjas meet, they will fight", |(me, second_ninja)| {
        me.fight(second_ninja);
        (me, second_ninja)
    })
    .then("one ninja dies (but not me)", |(me, second_ninja)| {
        assert!(me.alive);
        assert!(!second_ninja.alive);
    })
    .and("there is one ninja less alive", |()| {
        assert!(ninjas.iter().filter(|ninja| ninja.alive).count() < initial_alive_ninja_count)
    });
}

#[tokio::test]
async fn test_ninja_one_async() {
    let mut ninjas = [
        Ninja { alive: true },
        Ninja { alive: false },
        Ninja { alive: true },
    ];
    let initial_alive_ninja_count = ninjas.iter().filter(|ninja| ninja.alive).count();

    given_data(
        format!("there are {} ninjas", ninjas.len()),
        &mut ninjas,
        create_stdout_writer(),
    )
    .and("there are more than one ninja alive", |ninjas| {
        assert!(initial_alive_ninja_count > 1);
        let (first, second) = ninjas.split_at_mut(1);
        (&mut first[0], &mut second[1])
    })
    .whenf(
        "2 ninjas meet, they will fight",
        |(me, second_ninja)| async {
            me.fight_outside(second_ninja).await;
            (me, second_ninja)
        },
    )
    .await
    .then("one ninja dies (but not me)", |(me, second_ninja)| {
        assert!(me.alive);
        assert!(!second_ninja.alive);
    })
    .and("there is one ninja less alive", |()| {
        assert!(ninjas.iter().filter(|ninja| ninja.alive).count() < initial_alive_ninja_count)
    });
}

#[tokio::test]
async fn test_ninja_all_async() {
    let mut ninjas = [
        Ninja { alive: true },
        Ninja { alive: false },
        Ninja { alive: true },
    ];
    let initial_alive_ninja_count = ninjas.iter().filter(|ninja| ninja.alive).count();

    given_data(
        format!("there are {} ninjas", ninjas.len()),
        &mut ninjas,
        create_stdout_writer(),
    )
    .andf("there are more than one ninja alive", |ninjas| async {
        assert!(initial_alive_ninja_count > 1);
        let (first, second) = ninjas.split_at_mut(1);
        (&mut first[0], &mut second[1])
    })
    .await
    .whenf(
        "2 ninjas meet, they will fight",
        |(me, second_ninja)| async {
            me.fight_outside(second_ninja).await;
            (me, second_ninja)
        },
    )
    .await
    .thenf("one ninja dies (but not me)", |(me, second_ninja)| async {
        assert!(me.alive);
        assert!(!second_ninja.alive);
    })
    .await
    .andf("there is one ninja less alive", |()| async {
        assert!(ninjas.iter().filter(|ninja| ninja.alive).count() < initial_alive_ninja_count)
    })
    .await;
}

#[tokio::test]
async fn test_ninja_last_async() {
    let all_ninjas = [
        Ninja { alive: true },
        Ninja { alive: false },
        Ninja { alive: true },
    ];
    let initial_alive_ninja_count = all_ninjas.iter().filter(|ninja| ninja.alive).count();

    given_dataf(
        format!("there are {} ninjas", all_ninjas.len()),
        all_ninjas,
        create_stdout_writer(),
    )
    .and("there are more than one ninja alive", |ninjas| {
        assert!(ninjas.iter().filter(|ninja| ninja.alive).count() > 1);
        ninjas
    })
    .whenf("2 ninjas meet, they will fight", |mut ninjas| async {
        let (first, second) = ninjas.split_at_mut(1);
        first[0].fight_outside(&mut second[1]).await;
        ninjas
    })
    .then("one ninja dies (but not me)", |ninjas| {
        assert!(ninjas[0].alive);
        assert!(!ninjas[2].alive);
        ninjas
    })
    .and("there is one ninja less alive", move |ninjas| {
        assert!(ninjas.iter().filter(|ninja| ninja.alive).count() < initial_alive_ninja_count)
    })
    .await;
}

struct Ninja {
    alive: bool,
}

impl Ninja {
    fn fight(&mut self, ninja: &mut Ninja) {
        if !self.alive || !ninja.alive {
            panic!("Ninjas must be alive to fight");
        }
        self.alive = true;
        ninja.alive = false;
    }

    async fn fight_outside(&mut self, ninja: &mut Ninja) {
        if !self.alive || !ninja.alive {
            panic!("Ninjas must be alive to fight");
        }
        self.alive = true;
        ninja.alive = false;
    }
}
