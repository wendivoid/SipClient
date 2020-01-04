use super::Pipeline;

pub struct StreamingSession {
    pub session_id: String,
    pub pipeline: gstreamer::Pipeline,
    pub pipelines: Vec<Pipeline>
}
