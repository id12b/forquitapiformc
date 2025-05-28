use std::fs;
use std::path::Path;

// Define a Mod structure
pub struct ModLoader {
    pub mods: Vec<String>,
}

impl ModLoader {
    // Initialize the mod loader
    pub fn new() -> Self {
        ModLoader { mods: Vec::new() }
    }

    // Load mods from a specified directory
    pub fn load_mods(&mut self, mod_path: &str) {
        if Path::new(mod_path).exists() {
            match fs::read_dir(mod_path) {
                Ok(entries) => {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            self.mods.push(entry.file_name().into_string().unwrap());
                        }
                    }
                    println!("Loaded mods: {:?}", self.mods);
                }
                Err(e) => println!("Error reading mod directory: {}", e),
            }
        } else {
            println!("Mod path does not exist.");
        }
    }

    // Display loaded mods
    pub fn list_mods(&self) {
        if self.mods.is_empty() {
            println!("No mods loaded.");
        } else {
            println!("Available mods:");
            for mod_name in &self.mods {
                println!("- {}", mod_name);
            }
        }
    }
}
#[no_mangle]
pub extern "C" fn load_mods(mod_path: *const i8) {
    let path = unsafe { std::ffi::CStr::from_ptr(mod_path).to_str().unwrap() };
    println!("Loading mods from: {}", path);
}
