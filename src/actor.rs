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
pub trait Actor {
}

pub struct Message {}
