# About

Testing framework using gherkin language, focused on usability and easy of use.

# Usage

```rust
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
```


Example with async

```rust
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
```

More examples in [tests](./tests)
