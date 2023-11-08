/*
 * FastAPI
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.1.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListMetricsResponse {
    #[serde(rename = "metrics")]
    pub metrics: Vec<crate::models::ListMetricsObject>,
    #[serde(rename = "count")]
    pub count: i32,
}

impl ListMetricsResponse {
    pub fn new(metrics: Vec<crate::models::ListMetricsObject>, count: i32) -> ListMetricsResponse {
        ListMetricsResponse {
            metrics,
            count,
        }
    }
}


