use indexmap::IndexMap;

#[cfg(test)]
mod tests {
    use super::*;

}


pub enum MetricType {
    Counter,
    Guage,
    Histogram,
    Summary,
    Untyped,
}

impl Default for MetricType {
    fn default() -> MetricType {
        MetricType::Untyped
    }
}

pub struct Metric {
    labels: IndexMap<String, String>,
    metric_type: MetricType,
    timestamp_ms: Option<i64>,
}