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
pub struct DeleteObjectsResponse {
    #[serde(rename = "locators")]
    pub locators: ::std::collections::HashMap<String, Vec<crate::models::LocateObjectResponse>>,
    #[serde(rename = "delete_markers")]
    pub delete_markers: ::std::collections::HashMap<String, crate::models::DeleteMarker>,
    #[serde(rename = "op_type")]
    pub op_type: ::std::collections::HashMap<String, String>,
}

impl DeleteObjectsResponse {
    pub fn new(locators: ::std::collections::HashMap<String, Vec<crate::models::LocateObjectResponse>>, delete_markers: ::std::collections::HashMap<String, crate::models::DeleteMarker>, op_type: ::std::collections::HashMap<String, String>) -> DeleteObjectsResponse {
        DeleteObjectsResponse {
            locators,
            delete_markers,
            op_type,
        }
    }
}


