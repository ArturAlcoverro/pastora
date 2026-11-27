//! Embeds the default mods (defaults/mods/**), the program icons
//! (defaults/icons/*) and the frame's mods (defaults/frame/**) so Herdr can
//! install them.
//!
//! Worktrees may share one target dir, and with it this script and its output.
//! So nothing here names this checkout: paths are relative to the package and
//! the generated code includes files through the compiling crate's
//! `CARGO_MANIFEST_DIR`.

use std::path::Path;

fn walk(dir: &Path, base: &Path, root: &Path, out: &mut Vec<String>) {
    let mut entries: Vec<_> = std::fs::read_dir(dir).unwrap().flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk(&p, base, root, out);
        } else {
            let path = p.strip_prefix(base).unwrap().to_string_lossy().replace('\\', "/");
            println!("cargo:rerun-if-changed={path}");
            let rel = p.strip_prefix(root).unwrap().to_string_lossy().replace('\\', "/");
            out.push(format!("    ({rel:?}, include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{path}\"))),"));
        }
    }
}

fn embed(base: &Path, folder: &str, name: &str, file: &str) {
    println!("cargo:rerun-if-changed={folder}");
    let mut lines = Vec::new();
    let root = base.join(folder);
    walk(&root, base, &root, &mut lines);
    let code = format!("pub const {name}: &[(&str, &[u8])] = &[\n{}\n];\n", lines.join("\n"));
    std::fs::write(Path::new(&std::env::var("OUT_DIR").unwrap()).join(file), code).unwrap();
}

fn main() {
    let base = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    embed(&base, "defaults/mods", "DEFAULT_MODS", "default_mods.rs");
    embed(&base, "defaults/icons", "DEFAULT_ICONS", "default_icons.rs");
    embed(&base, "defaults/frame", "FRAME_MODS", "frame_mods.rs");
    // The name people see (window, menus, notices): Herdr, or the build's own.
    println!("cargo:rerun-if-env-changed=HERDR_APP_NAME");
    let brand = std::env::var("HERDR_APP_NAME").ok().filter(|n| !n.is_empty()).unwrap_or_else(|| "Herdr".into());
    println!("cargo:rustc-env=HERDR_BRAND={brand}");
    // Its icon in the setup window: a file in assets/icons/.
    println!("cargo:rerun-if-env-changed=HERDR_APP_ICON");
    let icon = std::env::var("HERDR_APP_ICON").ok().filter(|n| !n.is_empty()).unwrap_or_else(|| "herdr-ui-icon-clean.png".into());
    println!("cargo:rustc-env=HERDR_BRAND_ICON={icon}");

    // Windows takes the window, taskbar and Start menu icon from the program file.
    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        println!("cargo:rerun-if-changed=assets/icons/herdr-app.ico");
        winresource::WindowsResource::new().set_icon("assets/icons/herdr-app.ico").compile().expect("cannot embed the icon");
    }
}
