//! Embeds the branded icon and product information in the Windows executable.

#[cfg(windows)]
include!("../cc_engine/src/app_name.rs");

fn main() {
    println!("cargo:rerun-if-changed=../../assets/icons/app-icon.ico");
    println!("cargo:rerun-if-changed=../cc_engine/src/app_name.rs");
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("../../assets/icons/app-icon.ico")
            .set("FileDescription", APP_NAME)
            .set("ProductName", APP_NAME)
            .set("CompanyName", "Масштаб AI")
            .set(
                "LegalCopyright",
                "© Масштаб AI · mashtab-ai.ru; основано на ConstructCraft © Francis Rebouças, GPL-3.0",
            );
        if let Err(err) = res.compile() {
            // A missing resource compiler must not break the build; the
            // window icon is still set at runtime.
            println!("cargo:warning=could not embed the application icon: {err}");
        }
    }
}
