use axum::{routing::get, Router};
use prometheus::{Encoder, GaugeVec, TextEncoder};
use std::time::Duration;
use sysinfo::{CpuRefreshKind, RefreshKind, System};

lazy_static::lazy_static! {
    static ref CPU_USAGE: GaugeVec = prometheus::register_gauge_vec!(
        "cpu_usage",
        "CPU Usage percentage",
        &["core", "brand"],
    ).unwrap();
}

#[tokio::main]
async fn main() {
    let metric_period = std::env::var("PERIOD")
        .unwrap_or("5".into())
        .parse::<u64>()
        .expect("Metric period should be a number");

    let app = Router::new().route("/metrics", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    let handle = axum::serve(listener, app);

    let handle2 = tokio::spawn(async move {
        let mut s = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        let mut metric = String::with_capacity(100);
        loop {
            s.refresh_cpu_all();
            metric.truncate(0);
            for cpu in s.cpus() {
                CPU_USAGE
                    .with_label_values(&[cpu.name(), cpu.brand()])
                    .set(cpu.cpu_usage() as f64);
            }
            tokio::time::sleep(Duration::from_secs(metric_period)).await;
        }
    });

    let _ = tokio::join!(handle, handle2);
}

async fn handler() -> String {
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    let encoder = TextEncoder::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}
