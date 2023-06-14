mod imds;

use opentelemetry_sdk::resource::ResourceDetector;
use opentelemetry_sdk::Resource;
use std::time::Duration;

pub struct AwsResource {}

impl AwsResource {}

impl ResourceDetector for AwsResource {
    fn detect(&self, timeout: Duration) -> Resource {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
