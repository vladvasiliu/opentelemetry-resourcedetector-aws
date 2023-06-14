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
    image_id: String,
}

#[cfg(test)]
mod tests {
    use crate::imds::client::DynamicInstanceIdentityDocument;

    static IMDS_INSTANCE_IDENTITY_DOCUMENT: &str = r#"{
    "devpayProductCodes" : null,
    "marketplaceProductCodes" : [ "1abc2defghijklm3nopqrs4tu" ],
    "availabilityZone" : "us-west-2b",
    "privateIp" : "10.158.112.84",
    "version" : "2017-09-30",
    "instanceId" : "i-1234567890abcdef0",
    "billingProducts" : null,
    "instanceType" : "t2.micro",
    "accountId" : "123456789012",
    "imageId" : "ami-5fb8c835",
    "pendingTime" : "2016-11-19T16:32:11Z",
    "architecture" : "x86_64",
    "kernelId" : null,
    "ramdiskId" : null,
    "region" : "us-west-2"}"#;

    #[test]
    fn can_deserialize_identity_document() {
        let result = serde_json::from_str::<DynamicInstanceIdentityDocument>(
            IMDS_INSTANCE_IDENTITY_DOCUMENT,
        );
        assert!(
            result.is_ok(),
            "Failed to deserialize instance identity document: {}",
            result.unwrap_err()
        );
    }
}
