actix::prelude::{Message, Recipient};
use uuid::Uuid;


mod canvas;


//This message handles sending the entire canvas status to the user. 
//Should only be sent sparingly
#[derive(Message)]
#[rtype(result = "()")]
pub struct CanvasStatus
{
    pub Vec<String>
}