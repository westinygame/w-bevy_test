use bevy::prelude::{Res, EventReader};
use bevy::ecs::system::Resource;
use bevy::ecs::schedule::{IntoSystemDescriptor, SystemDescriptor};

pub fn assert_event_count<T>(cnt: u8) -> SystemDescriptor
where
    T: Resource,
{
    let system = move |mut reader: EventReader<T>| {
        let mut event_cnt = 0u8;
        for _ in reader.iter() {
            event_cnt += 1;
        }
        assert_eq!(
            event_cnt,
            cnt,
            "\nEvent count assertion failed.
               Event: {}
               Expected: {}, Actual: {}\n",
            std::any::type_name::<T>(),
            cnt,
            event_cnt
        );
    };
    system.into_descriptor()
}

pub fn assert_event<T>(expected: T) -> SystemDescriptor
where
    T: Resource + std::fmt::Debug + std::cmp::PartialEq,
{
    let system = move |mut reader: EventReader<T>| {
        let maybe_event = reader.iter().next();
        assert_eq!(
            maybe_event.is_some(),
            true,
            "Received no event {}\n",
            std::any::type_name::<T>()
        );
        assert_eq!(*maybe_event.unwrap(), expected);
    };
    system.into_descriptor()
}

pub fn assert_current_state<T>(expected: T) -> SystemDescriptor
where
    T: Resource + bevy::ecs::schedule::StateData,
{
    use bevy::ecs::prelude::State;
    let system = move |state: Res<State<T>>| {
        assert_eq!(state.current(), &expected);
    };
    system.into_descriptor()
}

pub fn assert_resource<T>(expected: T) -> SystemDescriptor
where
    T: Resource + std::fmt::Debug + std::cmp::PartialEq,
{
    let system = move |res: Res<T>| {
        assert_eq!(*res, expected);
    };
    system.into_descriptor()
}
