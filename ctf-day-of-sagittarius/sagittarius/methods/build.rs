use std::{collections::HashMap, path::Path};

use risc0_build::GuestOptions;

fn main() {
    let methods_path = Path::new(file!()).parent().unwrap().join("src/methods.rs");
    if !methods_path.exists() {
        let mut options = HashMap::new();
        options.insert("guest", GuestOptions {
            features: vec![],
            std: false,
        });
        risc0_build::embed_methods_with_options(options);
    }
}