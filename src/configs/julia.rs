use crate::config::{ModuleConfig, RootModuleConfig, SegmentConfig};

use ansi_term::{Color, Style};
use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct JuliaConfig<'a> {
    pub symbol: SegmentConfig<'a>,
    pub version: SegmentConfig<'a>,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub style: Style,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for JuliaConfig<'a> {
    fn new() -> Self {
        JuliaConfig {
            symbol: SegmentConfig::new("ஃ "),
            version: SegmentConfig::default(),
            prefix: "via ",
            suffix: " ",
            style: Color::Purple.bold(),
            disabled: false,
        }
    }
}
