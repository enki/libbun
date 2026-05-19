pub const PLUGIN_CHECKSUMS: &[(&str, &str, &str)] = include!("plugin_checksums_table.in");

pub fn checksum_for(release_tag: &str, target: &str) -> Option<&'static str> {
    PLUGIN_CHECKSUMS
        .iter()
        .find(|(version, candidate, _)| *version == release_tag && *candidate == target)
        .map(|(_, _, checksum)| *checksum)
}
