//! Les pages traduites (`content/en/…`, `content/es/…`) : leur liste, compilée dans le
//! serveur comme les pages françaises. Une page sans traduction s'affiche en français.

use std::fmt::Write;
use std::path::Path;

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut out = String::from(
        "/// Les pages traduites : (langue, parcours, page, Markdown).\npub static TRANSLATIONS: &[(&str, &str, &str, &str)] = &[\n",
    );
    for lang in ["en", "es"] {
        let dir = root.join("content").join(lang);
        println!("cargo:rerun-if-changed={}", dir.display());
        let mut files = Vec::new();
        for track in std::fs::read_dir(&dir).into_iter().flatten().flatten() {
            for page in std::fs::read_dir(track.path()).into_iter().flatten().flatten() {
                let path = page.path();
                if path.extension().is_some_and(|e| e == "md") {
                    files.push(path);
                }
            }
        }
        files.sort();
        for path in files {
            let track = path
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|n| n.to_str())
                .unwrap_or("");
            let page = path.file_stem().and_then(|n| n.to_str()).unwrap_or("");
            writeln!(
                out,
                "    ({lang:?}, {track:?}, {page:?}, include_str!({:?})),",
                path.display().to_string()
            )
            .unwrap();
        }
    }
    out.push_str("];\n");
    let dest = Path::new(&std::env::var("OUT_DIR").unwrap()).join("translations.rs");
    std::fs::write(dest, out).unwrap();
}
