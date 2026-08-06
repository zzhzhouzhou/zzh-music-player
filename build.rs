fn main() {
    slint_build::compile("src/main.slint").expect("Slint UI 编译失败");
    #[cfg(target_os = "windows")]
    {
            let mut res = winres::WindowsResource::new();
            res.set_icon("icons/appicon/zzhmp.ico");
            res.compile().unwrap();
    }
}
