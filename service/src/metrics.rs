use lazy_static::lazy_static;
use prometheus as prom;

lazy_static! {
    pub static ref INPUT_QUEUE_LENGTH: prom::GaugeVec = prom::register_gauge_vec!(
        "kollector_input_message_queue",
        "Kollector internal message queue length",
        &["gateway"]
    )
    .unwrap();
    pub static ref OUTPUT_QUEUE_LENGTH: prom::GaugeVec = prom::register_gauge_vec!(
        "kollector_output_message_queue",
        "Kollector output internal message queue length",
        &["gateway"]
    )
    .unwrap();
}
