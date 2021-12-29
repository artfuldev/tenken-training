use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
enum MessageType {
    SpaceCartography,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
enum MeasureCode {
    SCSED,
    SCSEAA,
    SCSEPA,
    LER,
    PLSE,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
enum MeasureType {
    Positioning,
    Composition,
    Probe,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
struct Measure {
    measureCode: MeasureCode,
    measureType: MessageType, // This might also be derivable from measure code
    componentReading: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Payload {
    pub probeId: String,
    pub eventId: String,
    messageType: String,
    eventTransmissionTime: u64,
    messageData: Vec<Measure>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    pub probeId: String,
    pub eventId: String,
    messageType: String,
    eventReceivedTime: u128,
    eventTransmissionTime: u64,
    messageData: Vec<Measure>,
}

impl From<Payload> for Message {
    fn from(payload: Payload) -> Self {
        Message {
            probeId: payload.probeId,
            eventId: payload.eventId,
            messageType: payload.messageType,
            eventTransmissionTime: payload.eventTransmissionTime,
            messageData: payload.messageData,
            eventReceivedTime: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        }
    }
}
