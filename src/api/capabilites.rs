pub const APIS: &[ApiInfo] = &[
    ApiInfo {
        key: 18,
        name: "ApiVersions",
        version_range: 0..=4,
    },
    ApiInfo {
        key: 75,
        name: "DescribeTopicPartitions",
        version_range: 0..=0,
    },
];

pub struct ApiInfo {
    pub key: u16,
    pub name: &'static str,
    pub version_range: std::ops::RangeInclusive<u16>,
}