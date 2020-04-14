use std::env;
use std::fs;
use std::path::PathBuf;

const SPRITE_URL: &str = "https://unpkg.com/feather-icons@4.28.0/dist/feather-sprite.svg";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sprite_path = PathBuf::from(env::var("OUT_DIR")?).join("feather-sprite.svg");
    fs::write(
        &sprite_path,
        reqwest::blocking::get(SPRITE_URL)?.text()?,
    )?;
    println!(
        "cargo:rustc-env=FEATHERICONS_SPRITE={}",
        sprite_path.to_string_lossy()
    );

    Ok(())
}

// use std::env;
// use std::path::PathBuf;

// // TODO: rename to project root
// fn project_root_dir() -> PathBuf {
//     PathBuf::from(env!("CARGO_MANIFEST_DIR"))
// }

// fn out_dir() -> PathBuf {
//     PathBuf::from(env::var("OUT_DIR").unwrap())
// }

// fn frontend_path() -> PathBuf {
//     out_dir().join("frontend")
// }

// fn main() {
//     if cfg!(feature = "frontend") {
//         if cfg!(feature = "dev-frontend") {
//             if cfg!(not(debug_assertions)) {
//                 // TODO: eprintln + exit 1
//                 panic!("Building a release build with a dev-frontend does not make any sense");
//             }
//             // std::fs::create_dir_all(project_root_dir().join("dist")).unwrap();
//             println!(
//                 "cargo:rustc-env=FRONTEND_DIR={}",
//                 project_root_dir().join("frontend").to_string_lossy()
//             );
//         } else {
//             std::fs::create_dir_all(project_root_dir().join("dist")).unwrap();
//             std::process::Command::new("npm")
//                 .arg("install")
//                 .status()
//                 .expect("Could not run npm install");
//             std::process::Command::new("npm")
//                 .env("NODE_ENV", "production")
//                 .arg("ci")
//                 .status()
//                 .expect("Could not run npm ci");

//             println!(
//                 "cargo:rustc-env=FRONTEND_DIR={}",
//                 project_root_dir().join("dist").to_string_lossy()
//             );
//         }
//     }
// }
