use super::*;

use proptest::strategy::Strategy;

#[test]
fn without_tuple_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::is_not_tuple(arc_process.clone()),
                    strategy::term::is_integer(arc_process.clone()),
                    strategy::term(arc_process.clone()),
                ),
                |(tuple, index, element)| {
                    prop_assert_eq!(
                        erlang::insert_element_3(index, tuple, element, &arc_process),
                        Err(badarg!().into())
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_tuple_without_integer_between_1_and_the_length_plus_1_inclusive_errors_badarg() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    strategy::term::tuple(arc_process.clone()),
                    strategy::term(arc_process.clone()),
                    strategy::term(arc_process.clone())
                )
                    .prop_filter("Index either needs to not be an integer or not be an integer in the index range 1..=(len + 1)", |(tuple, index, _element)| {
            let index_big_int_result: std::result::Result<BigInt, _> = (*index).try_into();

            match index_big_int_result {
                Ok(index_big_int) => {
                    let tuple_tuple: Boxed<Tuple> = (*tuple).try_into().unwrap();
                    let min_index: BigInt = 1.into();
                    let max_index: BigInt = (tuple_tuple.len() + 1).into();

                    !((min_index <= index_big_int) && (index_big_int <= max_index))
                }
                _ => true,
            }
        }),
                |(tuple, index, element)| {
                    prop_assert_eq!(
                        erlang::insert_element_3(index, tuple, element, &arc_process),
                        Err(badarg!().into())
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_tuple_with_integer_between_1_and_the_length_plus_1_inclusive_returns_tuple_with_element() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(1_usize..=4_usize)
                    .prop_flat_map(|len| {
                        (
                            proptest::collection::vec(
                                strategy::term(arc_process.clone()),
                                len..=len,
                            ),
                            0..=len,
                            strategy::term(arc_process.clone()),
                        )
                    })
                    .prop_map(|(element_vec, zero_based_index, element)| {
                        (
                            element_vec.clone(),
                            zero_based_index,
                            arc_process.tuple_from_slice(&element_vec).unwrap(),
                            arc_process.integer(zero_based_index + 1),
                            element,
                        )
                    }),
                |(mut element_vec, element_vec_index, tuple, index, element)| {
                    element_vec.insert(element_vec_index, element);

                    prop_assert_eq!(
                        erlang::insert_element_3(index, tuple, element, &arc_process),
                        Ok(arc_process.tuple_from_slice(&element_vec).unwrap())
                    );

                    Ok(())
                },
            )
            .unwrap();
    });
}
