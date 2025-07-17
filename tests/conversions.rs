use gnerkinf::create_stdout_writer;
use gnerkinf::givenf;
use gnerkinf::outline;
use gnerkinf::{given, given_data, WhenContext};

#[test]
fn test_bdd_type_conversion_sync() {
    given(
        "i have bike with 2 wheels",
        produce_bike,
        create_stdout_writer(),
    )
    .and("a car with 4 wheels", |bike| (bike, produce_car()))
    .when("i add transport wheels", |(bike, car)| Bike {
        wheels: bike.wheels + car.wheels,
    })
    .and("convert bike to car", |bike| Car {
        wheels: bike.wheels,
    })
    .then("i should have 6 wheels", |car| {
        assert_eq!(car.wheels, 6);
        car.wheels
    })
    .and("another 6 wheels", |wheels| {
        assert_eq!(wheels, 6);
    });
}

#[tokio::test]
async fn test_bdd_type_conversion_async_within_sync() {
    given(
        "i have bike with 2 wheels",
        produce_bike_async,
        create_stdout_writer(),
    )
    .and("a car with 4 wheels", |bike| async {
        (bike.await, produce_car())
    })
    .when("i add transport wheels", |f| async move {
        let (bike, car) = f.await;
        Bike {
            wheels: bike.wheels + car.wheels,
        }
    })
    .and("convert bike to car", |bike| async move {
        Car {
            wheels: bike.await.wheels,
        }
    })
    .then("i should have 6 wheels", |car| async move {
        let car = car.await;
        assert_eq!(car.wheels, 6);
        car.wheels
    })
    .and("another 6 wheels", |wheels| async move {
        assert_eq!(wheels.await, 6);
    });
}

#[tokio::test]
async fn test_bdd_type_conversion_async() {
    givenf(
        "i have bike with 2 wheels",
        produce_bike_async,
        create_stdout_writer(),
    )
    .await
    .andf("a car with 4 wheels", |bike| async {
        (bike, produce_car())
    })
    .await
    .whenf("i add transport wheels", |(bike, car)| async move {
        Bike {
            wheels: bike.wheels + car.wheels,
        }
    })
    .await
    .andf("convert bike to car", |bike| async move {
        Car {
            wheels: bike.wheels,
        }
    })
    .await
    .thenf("i should have 6 wheels", |car| async move {
        assert_eq!(car.wheels, 6);
        car.wheels
    })
    .await
    .andf("another 6 wheels", |wheels| async move {
        assert_eq!(wheels, 6);
    })
    .await;
}

#[tokio::test]
async fn test_bdd_type_conversion_one_await() {
    givenf(
        "i have bike with 2 wheels",
        produce_bike_async,
        create_stdout_writer(),
    )
    .andf("a car with 4 wheels", |bike| async {
        (bike, produce_car())
    })
    .whenf("i add transport wheels", |(bike, car)| async move {
        Bike {
            wheels: bike.wheels + car.wheels,
        }
    })
    .andf("convert bike to car", |bike| async move {
        Car {
            wheels: bike.wheels,
        }
    })
    .thenf("i should have 6 wheels", |car| async move {
        assert_eq!(car.wheels, 6);
        car.wheels
    })
    .and("another 6 wheels", |wheels| async move {
        assert_eq!(wheels, 6);
    })
    .await;
}

#[tokio::test]
async fn test_bdd_type_conversion_sync_async_chain() {
    givenf(
        "i have bike with 2 wheels",
        produce_bike_async,
        create_stdout_writer(),
    )
    .and("a car with 4 wheels", |bike| (bike, produce_car()))
    .when("i add transport wheels", |(bike, car)| Bike {
        wheels: bike.wheels + car.wheels,
    })
    .and("convert bike to car", |bike| Car {
        wheels: bike.wheels,
    })
    .thenf("i should have 6 wheels", |car| async move {
        assert_eq!(car.wheels, 6);
        car.wheels
    })
    .and("another 6 wheels", |wheels| {
        assert_eq!(wheels, 6);
    })
    .await;
}

#[tokio::test]
async fn test_bdd_outline() {
    let data = vec![
        (Car { wheels: 2 }, Bike { wheels: 2 }),
        (Car { wheels: 4 }, Bike { wheels: 2 }),
    ];
    let expectations = [4, 6];
    outline(
        format!("{size} cars {size} bikes", size = data.len()),
        data,
        create_stdout_writer(),
    )
    .mapf(|data, index| async move {
        given_data(
            format!(
                "i have bike with {bike_wheels} wheels and a car with {car_wheels}",
                car_wheels = data.0.wheels,
                bike_wheels = data.1.wheels
            ),
            data,
            create_stdout_writer(),
        )
        .whenf("i add transport wheels", |(bike, car)| async move {
            Bike {
                wheels: bike.wheels + car.wheels,
            }
        })
        .await
        .and("convert bike to car", |bike| Car {
            wheels: bike.wheels,
        })
        .then(
            format!(
                "i should have {expected} wheels",
                expected = expectations[index]
            ),
            |car| {
                assert_eq!(car.wheels, expectations[index]);
                car.wheels
            },
        )
        .and(
            format!("another {expected} wheels", expected = expectations[index]),
            |wheels| {
                assert_eq!(wheels, expectations[index]);
            },
        );
    })
    .await
}

#[tokio::test]
async fn test_bdd_outline_as_loop() {
    let data = vec![
        (Car { wheels: 2 }, Bike { wheels: 2 }, 4),
        (Car { wheels: 4 }, Bike { wheels: 2 }, 6),
    ];
    for (car, bike, expected) in data {
        given_data("i have bike and a car", (car, bike), create_stdout_writer())
            .whenf("i add transport wheels", |(bike, car)| async move {
                Bike {
                    wheels: bike.wheels + car.wheels,
                }
            })
            .await
            .and("convert bike to car", |bike| Car {
                wheels: bike.wheels,
            })
            .then("i should have correct wheels", |car| {
                assert_eq!(car.wheels, expected);
                car.wheels
            })
            .and("another 6 wheels", |wheels| {
                assert_eq!(wheels, expected);
            });
    }
}

#[test]
fn test_bdd_functions() {
    let expected = 6;
    let spare = 2;
    given("i have bike", produce_bike, create_stdout_writer())
        .and(format!("spare {spare} wheels"), |bike| (bike, spare))
        .when("i add spare wheels", |(bike, spare)| bike.add_wheels(spare))
        .and("two more from the basement", |bike| bike.add_wheels(2))
        .then(
            format!("i should have {expected} wheels"),
            check_wheels(expected),
        );
}

#[test]
fn test_multiple_context1() {
    let expected = 8;
    given_some_context()
        .and("two more from the store", |bike| bike.add_wheels(2))
        .then(
            format!("i should have {expected} wheels"),
            check_wheels(expected),
        );
}

#[test]
fn test_multiple_context2() {
    let expected = 8;
    given_some_context()
        .and("two more from the garage", |bike| bike.add_wheels(2))
        .then(
            format!("i should have {expected} wheels"),
            check_wheels(expected),
        );
}

struct Bike {
    wheels: u8,
}

impl Bike {
    fn add_wheels(mut self, spare: u8) -> Self {
        self.wheels += spare;
        self
    }
}
struct Car {
    wheels: u8,
}

fn produce_bike() -> Bike {
    Bike { wheels: 2 }
}

async fn produce_bike_async() -> Bike {
    produce_bike()
}

fn produce_car() -> Car {
    Car { wheels: 4 }
}

fn given_some_context() -> WhenContext<Bike> {
    given("i have bike", produce_bike, create_stdout_writer())
        .and("spare two wheels", |bike| (bike, 2))
        .when("i add spare wheels", |(bike, spare)| bike.add_wheels(spare))
        .and("two more from the basement", |bike| bike.add_wheels(2))
}

fn check_wheels(wheels: u8) -> impl Fn(Bike) {
    move |bike| assert_eq!(bike.wheels, wheels)
}
