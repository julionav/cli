/*
 * Rivet API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

/// CloudSvcMetrics : Metrics relating to a job service.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct CloudSvcMetrics {
    /// The job name.
    #[serde(rename = "job")]
    pub job: String,
    /// CPU metrics.
    #[serde(rename = "cpu")]
    pub cpu: Vec<f64>,
    /// Memory metrics.
    #[serde(rename = "memory")]
    pub memory: Vec<f64>,
    /// Peak memory metrics.
    #[serde(rename = "memory_max")]
    pub memory_max: Vec<f64>,
    /// Total allocated memory (MB).
    #[serde(rename = "allocated_memory", skip_serializing_if = "Option::is_none")]
    pub allocated_memory: Option<f64>,
}

impl CloudSvcMetrics {
    /// Metrics relating to a job service.
    pub fn new(job: String, cpu: Vec<f64>, memory: Vec<f64>, memory_max: Vec<f64>) -> CloudSvcMetrics {
        CloudSvcMetrics {
            job,
            cpu,
            memory,
            memory_max,
            allocated_memory: None,
        }
    }
}


