/// address: identifier of an actor
/// messaging is best effort at most one message
/// message queue persisted outside the actor instance
/// respond with 0 or many messages
/// local state
/// side effects
/// 
/// examples of actor
/// supervisor : monitor and manage another actor
/// message forwarding : forward message to another actor. proxy
/// load balancer : load balance messages to other actors.
pub trait Actor<T> {
    fn handle_message(&self, message: Message);
}

pub struct Message {}



/* how it can be used?
repsond() // respond to the message sender.
spawn()   // spawn another actor.

enum PingMessage {
    PING,
    PONG,
}

struct PingActor {}

impl Actor for PingActor {
    type Message = PingMessage;

    fn handle_message(&self, message: Message) {
        match message {
            Message.PING => respond(Message.PONG),
            _ => println!("Unexpected message."),
        }
    }
}

fn main() {
    spawn(PingActor {}); // main loop started.
}

*/