use super::*;

#[test]
fn without_number_addend_errors_badarith() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::integer::small(arc_process.clone()),
                    strategy::term::is_not_number(arc_process.clone()),
                ),
                |(augend, addend)| {
                    prop_assert_eq!(
                        erlang::add_2(augend, addend, &arc_process),
                        Err(badarith!().into())
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_small_integer_addend_without_underflow_or_overflow_returns_small_integer() {
    with(|augend, process| {
        let addend = process.integer(3);

        assert_eq!(
            erlang::add_2(augend, addend, &process),
            Ok(process.integer(5))
        );
    })
}

#[test]
fn with_small_integer_addend_with_underflow_returns_big_integer() {
    with_process(|process| {
        let augend = process.integer(-1_isize);
        let addend = process.integer(SmallInteger::MIN_VALUE);

        assert!(addend.is_smallint());

        let result = erlang::add_2(augend, addend, &process);

        assert!(result.is_ok());

        let sum = result.unwrap();

        assert!(sum.is_bigint());
    })
}

#[test]
fn with_small_integer_addend_with_overflow_returns_big_integer() {
    with(|augend, process| {
        let addend = process.integer(SmallInteger::MAX_VALUE);

        assert!(addend.is_smallint());

        let result = erlang::add_2(augend, addend, &process);

        assert!(result.is_ok());

        let sum = result.unwrap();

        assert!(sum.is_bigint());
    })
}

#[test]
fn with_big_integer_addend_returns_big_integer() {
    with(|augend, process| {
        let addend = process.integer(SmallInteger::MAX_VALUE + 1);

        assert!(addend.is_bigint());

        let result = erlang::add_2(augend, addend, &process);

        assert!(result.is_ok());

        let sum = result.unwrap();

        assert!(sum.is_bigint());
    })
}

#[test]
fn with_float_addend_without_underflow_or_overflow_returns_float() {
    with(|augend, process| {
        let addend = process.float(3.0).unwrap();

        assert_eq!(
            erlang::add_2(augend, addend, &process),
            Ok(process.float(5.0).unwrap())
        );
    })
}

#[test]
fn with_float_addend_with_underflow_returns_min_float() {
    with(|augend, process| {
        let addend = process.float(std::f64::MIN).unwrap();

        assert_eq!(
            erlang::add_2(augend, addend, &process),
            Ok(process.float(std::f64::MIN).unwrap())
        );
    })
}

#[test]
fn with_float_addend_with_overflow_returns_max_float() {
    with(|augend, process| {
        let addend = process.float(std::f64::MAX).unwrap();

        assert_eq!(
            erlang::add_2(augend, addend, &process),
            Ok(process.float(std::f64::MAX).unwrap())
        );
    })
}

fn with<F>(f: F)
where
    F: FnOnce(Term, &ProcessControlBlock) -> (),
{
    with_process(|process| {
        let augend = process.integer(2);

        f(augend, &process)
    })
}
