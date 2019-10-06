use std::collections::LinkedList;

pub trait Actor {
    fn handle_message(&self, message: Message);
}

#[derive(Debug)]
pub struct Message {
    data: String,
}

impl Message {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

struct PingActor {}

impl Actor for PingActor {
    fn handle_message(&self, message: Message) {
        println!("RECEIVED: {:?}", message);
    }
}

impl PingActor {
    pub fn new() -> Self {
        Self {}
    }
}

struct Wrapper<A: Actor> {
    actor: A,
    queue: LinkedList<Message>,
}

impl<A> Wrapper<A>
where
    A: Actor,
{
    pub fn queue_message(&mut self, message: Message) {
        self.queue.push_back(message);
    }

    pub fn poll(&mut self) {
        if let Some(m) = self.queue.pop_front() {
            self.actor.handle_message(m);
        }
    }
}

fn main() {
    let actor = PingActor::new();
    let mut wrapper = Wrapper {
        actor,
        queue: Default::default(),
    };

    wrapper.queue_message(Message::new("PING".to_string()));
    wrapper.poll()
}
