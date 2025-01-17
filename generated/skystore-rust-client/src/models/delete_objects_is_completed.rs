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
pub struct DeleteObjectsIsCompleted {
    #[serde(rename = "ids")]
    pub ids: Vec<i32>,
    #[serde(rename = "multipart_upload_ids", skip_serializing_if = "Option::is_none")]
    pub multipart_upload_ids: Option<Vec<String>>,
    #[serde(rename = "op_type")]
    pub op_type: Vec<String>,
}

impl DeleteObjectsIsCompleted {
    pub fn new(ids: Vec<i32>, op_type: Vec<String>) -> DeleteObjectsIsCompleted {
        DeleteObjectsIsCompleted {
            ids,
            multipart_upload_ids: None,
            op_type,
        }
    }
}


