use crate::cdsl::isa::TargetIsa;
use crate::cdsl::settings::SettingGroupBuilder;

pub(crate) fn define() -> TargetIsa {
    let setting = SettingGroupBuilder::new("power64");
    TargetIsa::new("power64", setting.build())
}
