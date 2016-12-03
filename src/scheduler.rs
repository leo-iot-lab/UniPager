use std::sync::{Arc, RwLock, Mutex};
use std::collections::VecDeque;
use std::{thread, time};

use timeslots::TimeSlots;
use message::Message;
use transmitter::Transmitter;
use generator::Generator;

#[derive(Clone)]
pub struct Scheduler {
    queue: Arc<Mutex<VecDeque<Message>>>,
    state: Arc<RwLock<SchedulerState>>
}

struct SchedulerState {
    slots: TimeSlots
}

impl Scheduler {
    pub fn new() -> Scheduler {
        Scheduler {
            queue: Arc::new(Mutex::new(VecDeque::new())),
            state: Arc::new(RwLock::new(SchedulerState {
                slots: TimeSlots::new()
            })),
        }
    }

    pub fn run<T: Transmitter>(&self, mut transmitter: T) {
        loop {
            let mut queue = self.queue.lock().unwrap();
            let message = queue.pop_front();
            drop(queue);

            if let Some(message) = message {
                let generator = Generator::new(vec![message]);
                transmitter.send(generator);
            }

            thread::sleep(time::Duration::from_millis(500));
        }
    }

    pub fn set_time_slots(&self, slots: TimeSlots) {
        info!("Set time slots: {:?}", slots);
        self.state.write().unwrap().slots = slots;
    }

    pub fn enqueue(&self, msg: Message) {
        info!("Enqueue new message: {:?}", msg);
        self.queue.lock().unwrap().push_back(msg);
    }
}