use serde::Deserialize;

static TOKEN_URL: &str = "http://169.254.169.254/latest/api/token";
static INSTANCE_IDENTITY_DOCUMENT_URL: &str =
    "http://169.254.169.254/latest/dynamic/instance-identity/document";

pub struct ImdsClient {
    client: reqwest::Client,
}

impl ImdsClient {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DynamicInstanceIdentityDocument {
    account_id: String,
    region: String,
    availability_zone: String,
    instance_id: String,
    instance_type: String,
    architecture: String,
}
