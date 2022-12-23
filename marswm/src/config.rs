use std::env;
use std::fs;
use std::path;
use serde::{Serialize, Deserialize};

use crate::layouts::LayoutType;

const CONFIG_DIR: &str = "marswm";
const CONFIG_FILE: &str = "marswm.toml";

#[derive(Serialize,Deserialize,PartialEq,Debug,Copy,Clone)]
#[serde(default)]
pub struct Configuration {
    pub workspaces: usize,
    pub default_layout: LayoutType,
    pub layout: LayoutConfiguration,
    pub theming: ThemingConfiguration,
}

#[derive(Serialize,Deserialize,PartialEq,Debug,Copy,Clone)]
#[serde(default)]
pub struct LayoutConfiguration {
    pub gap_width: u32,
    pub main_ratio: f32,
    pub nmain: u32,
}

#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Copy,Clone)]
#[serde(default)]
pub struct ThemingConfiguration {
    pub primary_color: u64,
    pub secondary_color: u64,
    pub background_color: u64,
    pub frame_width: u32,
    pub inner_border_width: u32,
    pub outer_border_width: u32,
}

impl Default for Configuration {
    fn default() -> Self {
        return Configuration {
            workspaces: 8,
            default_layout: LayoutType::Floating,
            layout: LayoutConfiguration::default(),
            theming: ThemingConfiguration::default(),
        }
    }
}

impl Default for LayoutConfiguration {
    fn default() -> Self {
        return LayoutConfiguration {
            gap_width: 4,
            main_ratio: 0.6,
            nmain: 1
        };
    }
}

impl Default for ThemingConfiguration {
    fn default() -> Self {
        return ThemingConfiguration {
            primary_color: 0xae0c0c,
            secondary_color: 0x1f464f,
            background_color: 0xceccc6,
            frame_width: 4,
            inner_border_width: 1,
            outer_border_width: 1
        };
    }
}


fn try_read(path: &path::Path) -> Result<Configuration, (bool, String)> {
    if !path.exists() {
        return Err((false, "".to_owned()));
    }

    let fs_result = fs::read(path);
    let raw = match fs_result {
        Ok(content) => content,
        Err(e) => return Err((true, e.to_string())),
    };

    match toml::from_slice(&raw) {
        Ok(config) => return Ok(config),
        Err(e) => return Err((true, e.to_string())),
    };
}

pub fn read_config() -> Configuration {
    // check configuration dir as specified in xdg base dir specification
    if let Ok(xdg_config) = env::var("XDG_CONFIG_HOME") {
        let path = path::Path::new(&xdg_config).join(CONFIG_DIR).join(CONFIG_FILE);
        match try_read(&path) {
            Ok(config) => return config,
            Err((exists, msg)) => if exists {
                println!("Error reading config: {}", msg);
                return Configuration::default();
            },
        }
    }

    // check ~/.config
    if let Ok(home) = env::var("HOME") {
        let path = path::Path::new(&home).join(".config").join(CONFIG_DIR).join(CONFIG_FILE);
        match try_read(&path) {
            Ok(config) => return config,
            Err((exists, msg)) => if exists {
                println!("Error reading config: {}", msg);
                return Configuration::default();
            },
        }
    }

    // check local working directory
    let path = path::Path::new(CONFIG_FILE);
    match try_read(&path) {
        Ok(config) => return config,
        Err((exists, msg)) => if exists {
            println!("Error reading config: {}", msg);
            return Configuration::default();
        },
    }

    println!("No configuration file found");
    return Configuration::default();
}