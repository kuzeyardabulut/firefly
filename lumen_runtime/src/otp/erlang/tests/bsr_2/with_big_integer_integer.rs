use super::*;

use num_traits::Num;

#[test]
fn with_negative_shifts_left_and_returns_big_integer() {
    with(|integer, process| {
        let shift = process.integer(-1);

        assert_eq!(
            erlang::bsr_2(integer, shift, &process),
            Ok(process.integer(
                <BigInt as Num>::from_str_radix(
                    "1011001110001111000011111000001111110000001111111000000011111111000000000",
                    2
                )
                .unwrap()
            ))
        );
    });
}

#[test]
fn with_positive_with_big_integer_underflow_without_small_integer_underflow_returns_small_integer()
{
    with(|integer, process| {
        let shift = process.integer(71);

        let result = erlang::bsr_2(integer, shift, &process);

        assert!(result.is_ok());

        let shifted = result.unwrap();

        assert!(shifted.is_smallint());
        assert_eq!(shifted, process.integer(0b1));
    })
}

#[test]
fn with_positive_with_underflow_returns_zero() {
    with(|integer, process| {
        let shift = process.integer(80);

        assert_eq!(
            erlang::bsr_2(integer, shift, &process),
            Ok(process.integer(0))
        );
    });
}

fn with<F>(f: F)
where
    F: FnOnce(Term, &ProcessControlBlock) -> (),
{
    with_process(|process| {
        let integer = process.integer(
            <BigInt as Num>::from_str_radix(
                "101100111000111100001111100000111111000000111111100000001111111100000000",
                2,
            )
            .unwrap(),
        );

        assert!(integer.is_bigint());

        f(integer, &process)
    })
}
