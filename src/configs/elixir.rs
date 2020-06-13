use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct ElixirConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub otp_version: SegmentConfig<'a>,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for ElixirConfig<'a> {
    fn new() -> Self {
        ElixirConfig {
            symbol: SegmentConfig::new("💧 "),
            version: SegmentConfig::default(),
            otp_version: SegmentConfig::default(),
            prefix: "via ",
            suffix: " ",
            style: Color::Purple.bold(),
            disabled: false,
        }
    }
}
