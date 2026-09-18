//! 应用版本号：单一来源是 Cargo.toml 的 [package] version。
//! 更新流程 / 关于界面 / 版本比较统一从这里取，避免散落魔法数字。
//! 测试辅助：环境变量 ZZH_VERSION_OVERRIDE 可覆盖，用于验证更新流程。

use std::sync::OnceLock;

/// 当前版本号（与 installer.iss / Release tag 保持一致）。
pub fn app_version() -> &'static str {
    static VERSION: OnceLock<&'static str> = OnceLock::new();
    VERSION.get_or_init(|| {
        if let Ok(v) = std::env::var("ZZH_VERSION_OVERRIDE")
            && !v.trim().is_empty()
        {
            return Box::leak(v.into_boxed_str());
        }
        env!("CARGO_PKG_VERSION")
    })
}
