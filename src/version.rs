use flexver_rs::FlexVer;

use crate::error::PackError;


#[derive(Debug, Clone)]
struct PackFormatVer {
    min_ver: String,
    max_ver: String,
    format_ver: i32,
    supports_filter: bool,
    supports_supported_formats: bool,
    supports_overlays: bool
}

impl PackFormatVer {
    pub fn new(min_ver: &str, max_ver: &str, format_ver: i32) -> Self {
        Self {
            min_ver: min_ver.to_string(),
            max_ver: max_ver.to_string(),
            format_ver,
            supports_filter: matches!(format_ver, 9..=i32::MAX),
            supports_supported_formats: matches!(format_ver, 18..=i32::MAX),
            supports_overlays: matches!(format_ver, 18..=i32::MAX)
        }
    }
}

// From https://minecraft.wiki/w/Pack_format
// Set manually, as there doesn't seem to be a way to get these from Piston Meta or Launcher Meta
fn get_pack_formats() -> Vec<PackFormatVer> {
    vec![
        PackFormatVer::new("1.6.1",  "1.8.9",  1),
        PackFormatVer::new("1.9",    "1.10.2", 2),
        PackFormatVer::new("1.11",   "1.12.2", 3),
        PackFormatVer::new("1.13",   "1.14.4", 4),
        PackFormatVer::new("1.15",   "1.16.1", 5),
        PackFormatVer::new("1.16.2", "1.16.5", 6),
        PackFormatVer::new("1.17",   "1.17.1", 7),
        PackFormatVer::new("1.18",   "1.18.2", 8),
        PackFormatVer::new("1.19",   "1.19.2", 9),
        PackFormatVer::new("1.19.3", "1.19.3", 12),
        PackFormatVer::new("1.19.4", "1.19.4", 13),
        PackFormatVer::new("1.20",   "1.20.1", 15),
        PackFormatVer::new("1.20.2", "1.20.2", 18)
    ]
}

pub fn get_format(mc_version: String) -> Result<i32, PackError> {
    let mc_flex_ver = FlexVer(mc_version.as_str());

    let versions = get_pack_formats();

    for ver in versions {
        if FlexVer(ver.min_ver.as_str()) <= mc_flex_ver && FlexVer(ver.max_ver.as_str()) >= mc_flex_ver {
            return Ok(ver.format_ver)
        }
    }

    Err(PackError::InvalidVersion)
}

