extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    // Generate OpenGL bindings for the given version and profile.
    Registry::new(Api::Gl, ({{gl-major}}, {{gl-minor}}), Profile::{{gl-profile}}, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}
