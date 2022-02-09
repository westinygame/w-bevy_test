use bevy::app::prelude::CoreStage;
use bevy::ecs::schedule::{IntoSystemDescriptor};
use bevy::ecs::system::Resource;
use bevy::prelude::{App, EventWriter, ResMut};

pub mod assertion;

pub trait TestApp {
    fn send_events<T>(&mut self, events_to_send: Vec<Option<T>>) -> &mut Self
    where
        T: Resource;

    fn send_event<T>(&mut self, event: T) -> &mut Self
    where
        T: Resource;

    fn add_assert_system<P>(&mut self, system: impl IntoSystemDescriptor<P>) -> &mut Self;
}

impl TestApp for App {
    fn send_events<T>(&mut self, mut events_to_send: Vec<Option<T>>) -> &mut Self
    where
        T: Resource,
    {
        events_to_send.reverse();
        self.add_event::<T>()
            .insert_resource(EventsToSend(events_to_send))
            .add_system_to_stage(CoreStage::PreUpdate, send_events_system::<T>)
    }

    fn send_event<T>(&mut self, event: T) -> &mut Self
    where
        T: Resource,
    {
        self.send_events(vec![Some(event)])
    }

    fn add_assert_system<P>(&mut self, system: impl IntoSystemDescriptor<P>) -> &mut Self {
        self.add_system_to_stage(CoreStage::Last, system)
    }
}

struct EventsToSend<T>(Vec<Option<T>>)
where
    T: Resource;

fn send_events_system<T>(mut writer: EventWriter<T>, mut event_res: ResMut<EventsToSend<T>>)
where
    T: Resource,
{
    let maybe_event = event_res.0.pop().flatten();
    if let Some(event) = maybe_event {
        println!("Sending event");
        writer.send(event);
    } else { println!("Sending none"); }
}


#[cfg(test)]
mod test {
    use bevy::prelude::App;
    use super::*;

    #[derive(Debug, PartialEq)]
    struct TestEvent(u8);

    #[test]
    fn send_event_once() {
        App::new()
            .set_runner(|mut app: App| {
                app.update();
                assert_eq!(drain_events(&mut app), vec![TestEvent(100u8)]);

                app.update();
                assert_eq!(drain_events(&mut app).is_empty(), true);

                app.update();
                assert_eq!(drain_events(&mut app).is_empty(), true);
            })
            .send_event(TestEvent(100u8))
            .run();
    }

    fn drain_events(app: &mut App) -> Vec<TestEvent>
    {
        use bevy::app::Events;
        app.world.get_resource_mut::<Events<TestEvent>>()
            .expect("Resource Events<TestEvent>> not registered")
            .drain()
            .collect()
    }

    #[test]
    fn send_events_drained_once_ordered() {

        App::new()
            .set_runner(|mut app: App| {
                app.update();
                assert_eq!(drain_events(&mut app), vec![TestEvent(3u8)]);

                app.update();
                assert_eq!(drain_events(&mut app), vec![TestEvent(255u8)]);

                app.update();
                assert_eq!(drain_events(&mut app).is_empty(), true);

                app.update();
                assert_eq!(drain_events(&mut app), vec![TestEvent(0u8)]);

                app.update();
                assert_eq!(drain_events(&mut app).is_empty(), true);

                app.update();
                assert_eq!(drain_events(&mut app).is_empty(), true);
            })
            .send_events(
                vec![
                    Some(TestEvent(3u8)),   // 1st frame
                    Some(TestEvent(255u8)), // 2nd frame
                    None,                   // 3rd frame
                    Some(TestEvent(0u8)),   // 4th frame
                ]
            )
            .run();
    }
}
