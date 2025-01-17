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
pub struct CreateBucketIsCompleted {
    #[serde(rename = "id")]
    pub id: i32,
    #[serde(rename = "creation_date")]
    pub creation_date: String,
}

impl CreateBucketIsCompleted {
    pub fn new(id: i32, creation_date: String) -> CreateBucketIsCompleted {
        CreateBucketIsCompleted {
            id,
            creation_date,
        }
    }
}


