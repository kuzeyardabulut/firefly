use super::*;

#[test]
fn with_different_process_sends_message_when_timer_expires() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(
                    milliseconds(),
                    strategy::term::heap_fragment_safe(arc_process.clone()),
                ),
                |(milliseconds, message)| {
                    let time = arc_process.integer(milliseconds);

                    let destination_arc_process = process::test(&arc_process);
                    let destination = destination_arc_process.pid_term();

                    let result =
                        erlang::start_timer_3(time, destination, message, arc_process.clone());

                    prop_assert!(
                        result.is_ok(),
                        "Timer reference not returned.  Got {:?}",
                        result
                    );

                    let timer_reference = result.unwrap();

                    prop_assert!(timer_reference.is_local_reference());

                    let timeout_message = timeout_message(timer_reference, message, &arc_process);

                    prop_assert!(!has_message(&destination_arc_process, timeout_message));

                    thread::sleep(Duration::from_millis(milliseconds + 1));

                    timer::timeout().unwrap();

                    prop_assert!(has_message(&destination_arc_process, timeout_message));

                    Ok(())
                },
            )
            .unwrap();
    });
}

#[test]
fn with_same_process_sends_message_when_timer_expires() {
    TestRunner::new(Config::with_source_file(file!()))
        .run(
            &(milliseconds(), strategy::process()).prop_flat_map(|(milliseconds, arc_process)| {
                (
                    Just(milliseconds),
                    Just(arc_process.clone()),
                    strategy::term::heap_fragment_safe(arc_process),
                )
            }),
            |(milliseconds, arc_process, message)| {
                let time = arc_process.integer(milliseconds);
                let destination = arc_process.pid_term();

                let result = erlang::start_timer_3(time, destination, message, arc_process.clone());

                prop_assert!(
                    result.is_ok(),
                    "Timer reference not returned.  Got {:?}",
                    result
                );

                let timer_reference = result.unwrap();

                prop_assert!(timer_reference.is_local_reference());

                let timeout_message = timeout_message(timer_reference, message, &arc_process);

                prop_assert!(!has_message(&arc_process, timeout_message));

                thread::sleep(Duration::from_millis(milliseconds + 1));
                timer::timeout().unwrap();

                prop_assert!(has_message(&arc_process, timeout_message));

                Ok(())
            },
        )
        .unwrap();
}

#[test]
fn without_process_sends_nothing_when_timer_expires() {
    with_process_arc(|arc_process| {
        TestRunner::new(Config::with_source_file(file!()))
            .run(
                &(milliseconds(), strategy::term(arc_process.clone())),
                |(milliseconds, message)| {
                    let time = arc_process.integer(milliseconds);
                    let destination = next_pid();

                    let result =
                        erlang::start_timer_3(time, destination, message, arc_process.clone());

                    prop_assert!(
                        result.is_ok(),
                        "Timer reference not returned.  Got {:?}",
                        result
                    );

                    let timer_reference = result.unwrap();

                    prop_assert!(timer_reference.is_local_reference());

                    let timeout_message = arc_process
                        .tuple_from_slice(&[atom_unchecked("timeout"), timer_reference, message])
                        .unwrap();

                    thread::sleep(Duration::from_millis(milliseconds + 1));
                    timer::timeout().unwrap();

                    // does not send to original process either
                    prop_assert!(!has_message(&arc_process, timeout_message));

                    Ok(())
                },
            )
            .unwrap();
    });
}
