use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
struct Measure {
    measureName: String,
    measureCode: String,
    measureUnit: String,
    measureValue: f64,
    measureValueDescription: String,
    measureType: String,
    componentReading: f64
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProbeRequest {
    probeId: String,
    eventId: String,
    messageType: String,
    eventTransmissionTime: u64,
    messageData: Vec<Measure>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProbeData {
    probe_id: String,
    event_id: String,
    message_type: String,
    event_received_time: u128,
    event_transmission_time: u64,
    message_data: Vec<Measure>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProbeResponse {
    pub probeId: String,
    pub eventId: String,
    messageType: String,
    eventReceivedTime: u128,
    eventTransmissionTime: u64,
    messageData: Vec<Measure>,
}

impl From<ProbeRequest> for ProbeData {
    fn from(payload: ProbeRequest) -> Self {
      ProbeData {
            probe_id: payload.probeId,
            event_id: payload.eventId,
            message_type: payload.messageType,
            event_transmission_time: payload.eventTransmissionTime,
            message_data: payload.messageData,
            event_received_time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis(),
        }
    }
}

impl From<ProbeData> for ProbeResponse {
    fn from(data: ProbeData) -> Self {
        ProbeResponse {
          probeId: data.probe_id,
          eventId: data.event_id,
          messageType: data.message_type,
          eventReceivedTime: data.event_received_time,
          eventTransmissionTime: data.event_transmission_time,
          messageData: data.message_data
        }
    }
}
