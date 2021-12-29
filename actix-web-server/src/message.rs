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
pub struct Message {
    pub probeId: String,
    pub eventId: String,
    messageType: String,
    // eventReceivedTime: std::time::SystemTime,
    // eventTransmissionTime: std::time::SystemTime,
    messageData: Vec<Measure>,
}
