use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace::TraceLayer,
};
use tracing::Dispatch;
use tracing_subscriber::{fmt, layer::SubscriberExt, registry};

pub fn subscriber() -> Dispatch {
    let registry = registry().with(fmt::layer());
    registry.into()
}

pub fn trace_layer() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
}
