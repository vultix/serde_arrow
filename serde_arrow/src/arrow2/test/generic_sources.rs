use crate::{
    internal::{
        event::Event,
        generic_sources::ListSource,
        source::{DynamicSource, EventSource, IntoEventSource},
    },
    test::utils::collect_events,
};

#[test]
fn list_source_no_content_nulls() {
    let events: Vec<Event<'static>> = vec![];
    let mut source = ListSource::new(
        DynamicSource::new(events.into_event_source()),
        vec![0, 0, 0, 0],
        vec![false, false, false],
    );

    assert_eq!(source.next().unwrap(), Some(Event::Null));
    assert_eq!(source.next().unwrap(), Some(Event::Null));
    assert_eq!(source.next().unwrap(), Some(Event::Null));
    assert_eq!(source.next().unwrap(), None);
}

#[test]
fn list_source_no_content_empty() {
    let events: Vec<Event<'static>> = vec![];
    let source = ListSource::new(
        DynamicSource::new(events.into_event_source()),
        vec![0, 0, 0, 0],
        vec![true, true, true],
    );
    let actual = collect_events(source).unwrap();
    let expected = vec![
        Event::StartSequence,
        Event::EndSequence,
        Event::StartSequence,
        Event::EndSequence,
        Event::StartSequence,
        Event::EndSequence,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn list_source_no_content_single_items() {
    let events: Vec<Event<'static>> = vec![Event::I8(13), Event::I8(21), Event::I8(42)];
    let source = ListSource::new(
        DynamicSource::new(events.into_event_source()),
        vec![0, 1, 2, 3],
        vec![true, true, true],
    );

    let actual = collect_events(source).unwrap();
    let expected = vec![
        Event::StartSequence,
        Event::I8(13),
        Event::EndSequence,
        Event::StartSequence,
        Event::I8(21),
        Event::EndSequence,
        Event::StartSequence,
        Event::I8(42),
        Event::EndSequence,
    ];

    assert_eq!(actual, expected);
}

#[test]
fn list_source_no_content_multiple_items() {
    let events: Vec<Event<'static>> = vec![
        Event::I8(0),
        Event::I8(1),
        Event::I8(2),
        Event::I8(3),
        Event::I8(4),
        Event::I8(5),
    ];
    let source = ListSource::new(
        DynamicSource::new(events.into_event_source()),
        vec![0, 3, 5, 6],
        vec![true, true, true],
    );

    let actual = collect_events(source).unwrap();
    let expected = vec![
        Event::StartSequence,
        Event::I8(0),
        Event::I8(1),
        Event::I8(2),
        Event::EndSequence,
        Event::StartSequence,
        Event::I8(3),
        Event::I8(4),
        Event::EndSequence,
        Event::StartSequence,
        Event::I8(5),
        Event::EndSequence,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn list_source_nested() {
    let events: Vec<Event<'static>> = vec![
        Event::StartSequence,
        Event::I8(0),
        Event::I8(1),
        Event::EndSequence,
        Event::StartSequence,
        Event::I8(2),
        Event::I8(3),
        Event::EndSequence,
        Event::StartSequence,
        Event::I8(4),
        Event::I8(5),
        Event::EndSequence,
    ];
    let source = ListSource::new(
        DynamicSource::new(events.into_event_source()),
        vec![0, 2, 3],
        vec![true, true],
    );

    let actual = collect_events(source).unwrap();
    let expected = vec![
        Event::StartSequence,
        Event::StartSequence,
        Event::I8(0),
        Event::I8(1),
        Event::EndSequence,
        Event::StartSequence,
        Event::I8(2),
        Event::I8(3),
        Event::EndSequence,
        Event::EndSequence,
        Event::StartSequence,
        Event::StartSequence,
        Event::I8(4),
        Event::I8(5),
        Event::EndSequence,
        Event::EndSequence,
    ];
    assert_eq!(actual, expected);
}

#[test]
fn list_source_structs() {
    let events: Vec<Event<'static>> = vec![
        Event::StartStruct,
        Event::Str("a"),
        Event::I8(0),
        Event::Str("b"),
        Event::I8(1),
        Event::EndStruct,
        Event::StartStruct,
        Event::Str("a"),
        Event::I8(2),
        Event::Str("b"),
        Event::I8(3),
        Event::EndStruct,
        Event::StartStruct,
        Event::Str("a"),
        Event::I8(4),
        Event::I8(5),
        Event::Str("b"),
        Event::EndStruct,
    ];
    let source = ListSource::new(
        DynamicSource::new(events.into_event_source()),
        vec![0, 2, 3],
        vec![true, true],
    );

    let actual = collect_events(source).unwrap();
    let expected = vec![
        Event::StartSequence,
        Event::StartStruct,
        Event::Str("a"),
        Event::I8(0),
        Event::Str("b"),
        Event::I8(1),
        Event::EndStruct,
        Event::StartStruct,
        Event::Str("a"),
        Event::I8(2),
        Event::Str("b"),
        Event::I8(3),
        Event::EndStruct,
        Event::EndSequence,
        Event::StartSequence,
        Event::StartStruct,
        Event::Str("a"),
        Event::I8(4),
        Event::I8(5),
        Event::Str("b"),
        Event::EndStruct,
        Event::EndSequence,
    ];
    assert_eq!(actual, expected);
}
