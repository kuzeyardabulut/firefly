use super::*;

#[test]
fn with_lesser_small_integer_second_returns_first() {
    max(|_, process| process.integer(0), First)
}

#[test]
fn with_greater_small_integer_second_returns_second() {
    super::max(
        |process| process.integer(SmallInteger::MIN_VALUE - 1),
        |_, process| process.integer(SmallInteger::MIN_VALUE),
        Second,
    );
}

#[test]
fn with_lesser_big_integer_second_returns_first() {
    max(
        |_, process| process.integer(SmallInteger::MIN_VALUE - 1),
        First,
    )
}

#[test]
fn with_same_big_integer_second_returns_first() {
    max(|first, _| first, First)
}

#[test]
fn with_same_value_big_integer_second_returns_first() {
    max(
        |_, process| process.integer(SmallInteger::MAX_VALUE + 1),
        First,
    )
}

#[test]
fn with_greater_big_integer_second_returns_second() {
    max(
        |_, process| process.integer(SmallInteger::MAX_VALUE + 2),
        Second,
    )
}

#[test]
fn with_lesser_float_second_returns_first() {
    max(|_, process| process.float(1.0).unwrap(), First)
}

#[test]
fn with_greater_float_second_returns_second() {
    super::max(
        |process| process.integer(SmallInteger::MIN_VALUE - 1),
        |_, process| process.float(1.0).unwrap(),
        Second,
    );
}

#[test]
fn without_second_number_returns_second() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::integer::big(arc_process.clone()),
                    strategy::term::is_not_number(arc_process.clone()),
                ),
                |(first, second)| {
                    prop_assert_eq!(erlang::max_2(first, second), second);

                    Ok(())
                },
            )
            .unwrap();
    });
}

fn max<R>(second: R, which: FirstSecond)
where
    R: FnOnce(Term, &ProcessControlBlock) -> Term,
{
    super::max(
        |process| process.integer(SmallInteger::MAX_VALUE + 1),
        second,
        which,
    );
}
