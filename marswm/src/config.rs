use std::cmp;
use libmars::common::*;
use serde::{Serialize, Deserialize};
use libmars::utils::configuration::*;

use crate::bindings::*;
use crate::layouts::LayoutType;
use crate::layouts::StackMode;
use crate::layouts::StackPosition;
use crate::rules::*;

const BUTTON_BINDINGS_FILE: &str = "buttonbindings";
const BUTTON_BINDINGS_EXT_FILE: &str = "buttonbindings_ext";
const CONFIG_DIR: &str = "luna";
const CONFIG_FILE: &str = "luna";
const KEY_BINDINGS_FILE: &str = "keybindings";
const KEY_BINDINGS_EXT_FILE: &str = "keybindings_ext";
const RULES_FILE: &str = "rules";
const AUTOSTART_SCRIPT: &str = "autostart";
const FILE_EXT: [&str; 2] = [".toml", ".yaml"];


#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(default)]
pub struct Configuration {
    /// number of workspaces for primary monitor
    pub primary_workspaces: u32,

    /// number of workspaces for secondary monitors
    pub secondary_workspaces: u32,

    /// script, executable or command to be executed on startup
    pub on_startup: Option<String>,

    /// where should windows be placed initially
    pub initial_placement: WindowPlacement,

    /// layout configuration
    pub layout: LayoutConfiguration,

    /// theming configuration
    pub theming: ThemingConfiguration,
}

#[derive(Serialize,Deserialize,PartialEq,Debug,Copy,Clone)]
#[serde(default)]
pub struct LayoutConfiguration {
    /// default layout for each workspace
    pub default: LayoutType,

    /// width of the gap between windows in a tiled layout
    pub gap_width: u32,

    /// ratio of main area vs stack area in a tiled layout
    pub main_ratio: f32,

    /// number of windows in the main area
    pub nmain: u32,

    /// position of the stack relative to the main windows (dynamic layout)
    pub stack_position: StackPosition,

    /// mode of laying out the windows in the stack area (dynamic layout)
    pub stack_mode: StackMode,
}

#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Clone)]
#[serde(default)]
pub struct ThemingConfiguration {
    /// color for active window frame
    pub active_color: u64,

    /// color for passive window frame
    pub inactive_color: u64,

    /// color of inner and outer border
    pub border_color: u64,

    /// use inverted version of active/inactive color for border
    pub invert_border_color: bool,

    /// width of the frame that client windows are reparented to
    pub frame_width: (u32, u32, u32, u32),

    /// width of the border around the inner window
    pub inner_border_width: u32,

    /// width of the border around the frame
    pub outer_border_width: u32,

    /// decoration dimensions for clients that want no decoration
    pub no_decoration: NoDecorThemingConfiguration,

    /// show title string at the top of the frame
    pub show_title: bool,

    /// vertical padding of title
    pub title_vpadding: u32,

    /// horizontal padding of title
    pub title_hpadding: u32,

    /// font to use for title
    pub font: String,
}

#[derive(Serialize,Deserialize,PartialEq,Eq,Debug,Clone)]
#[serde(default)]
pub struct NoDecorThemingConfiguration {
    /// width of the frame that client windows are reparented to
    pub frame_width: (u32, u32, u32, u32),

    /// width of the border around the inner window
    pub inner_border_width: u32,

    /// width of the border around the frame
    pub outer_border_width: u32,
}

#[derive(Serialize,Deserialize,Clone,Copy,Debug,PartialEq,Eq)]
#[serde(rename_all = "kebab-case")]
pub enum WindowPlacement {
    Centered,
    Pointer,
    Wherever,
}


impl Default for Configuration {
    fn default() -> Self {
        let autostart_path = match xdg::BaseDirectories::with_prefix(CONFIG_DIR) {
            Ok(conf_dir) => {
                if let Some(path) = conf_dir.find_config_file(AUTOSTART_SCRIPT) {
                    if let Some(utf_path) = path.to_str() {
                        Some(String::from(utf_path))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            Err(_) => None,
        };
        Configuration {
            primary_workspaces: 9,
            secondary_workspaces: 9,
            on_startup: autostart_path,
            initial_placement: WindowPlacement::default(),
            layout: LayoutConfiguration::default(),
            theming: ThemingConfiguration::default(),
        }
    }
}

impl Default for LayoutConfiguration {
    fn default() -> Self {
        LayoutConfiguration {
            default: LayoutType::Dynamic,
            gap_width: 5,
            main_ratio: 0.55,
            nmain: 1,
            stack_position: StackPosition::Right,
            stack_mode: StackMode::Split,
        }
    }
}

impl Default for ThemingConfiguration {
    fn default() -> Self {
        ThemingConfiguration {
            active_color: 0x30d6ff,
            inactive_color: 0x141414,
            border_color: 0x141414,
            invert_border_color: false,
            frame_width: (10, 1, 1, 1),
            inner_border_width: 0,
            outer_border_width: 0,
            no_decoration: NoDecorThemingConfiguration::default(),
            show_title: false,
            title_vpadding: 0,
            title_hpadding: 5,
            font: "sans".to_owned(),
        }
    }
}

impl Default for NoDecorThemingConfiguration {
    fn default() -> Self {
        NoDecorThemingConfiguration {
            frame_width: (0, 0, 0, 0),
            inner_border_width: 0,
            outer_border_width: 0,
        }
    }
}

impl Default for WindowPlacement {
    fn default() -> Self {
        Self::Centered
    }
}


impl WindowPlacement {
    pub fn calc(&self, client_dimensions: Dimensions, window_area: Dimensions, pointer: (i32, i32)) -> (i32, i32) {
        use WindowPlacement::*;
        match self {
            Pointer => {
                let (mut x, mut y) = pointer;
                x -= (client_dimensions.w() / 2) as i32;
                y -= (client_dimensions.h() / 2) as i32;
                x = cmp::max(x, window_area.x());
                y = cmp::max(y, window_area.y());
                x = cmp::min(x, window_area.x() + window_area.w() as i32 - client_dimensions.w() as i32);
                y = cmp::min(y, window_area.y() + window_area.h() as i32 - client_dimensions.h() as i32);
                (x, y)
            },
            Centered => {
                let x = window_area.center().0 - (client_dimensions.w() as i32 / 2);
                let y = window_area.center().1 - (client_dimensions.h() as i32 / 2);
                (x, y)
            },
            Wherever => client_dimensions.pos(),
        }
    }
}


pub fn read_button_bindings() -> Vec<ButtonBinding> {
    // read keybindings file
    let mut button_bindings = match read_config_file(CONFIG_DIR, BUTTON_BINDINGS_FILE) {
        Ok(config) => config,
        Err(msg) => {
            eprintln!("Unable to read button bindings: {}", msg);
            default_button_bindings()
        },
    };

    // read extended keybindings
    match read_config_file::<Vec<ButtonBinding>>(CONFIG_DIR, BUTTON_BINDINGS_EXT_FILE) {
        Ok(config) => button_bindings.extend(config),
        Err(msg) => {
            eprintln!("Unable to read extended button bindings: {}", msg);
        },
    }

    button_bindings
}

pub fn read_config() -> Configuration {
    return match read_config_file(CONFIG_DIR, CONFIG_FILE) {
        Ok(config) => config,
        Err(msg) => {
            eprintln!("Unable to read configuration: {}", msg);
            Configuration::default()
        },
    };
}

pub fn read_key_bindings(nworkspaces: u32) -> Vec<KeyBinding> {
    // read keybindings file
    let mut keybindings = match read_config_file(CONFIG_DIR, KEY_BINDINGS_FILE) {
        Ok(config) => config,
        Err(msg) => {
            eprintln!("Unable to read key bindings: {}", msg);
            default_key_bindings(nworkspaces)
        },
    };

    // read extended keybindings
    match read_config_file::<Vec<KeyBinding>>(CONFIG_DIR, KEY_BINDINGS_EXT_FILE) {
        Ok(config) => keybindings.extend(config),
        Err(msg) => {
            eprintln!("Unable to read extended key bindings: {}", msg);
        },
    }

    keybindings
}

pub fn read_rules() -> Vec<Rule> {
    let result = read_config_file(CONFIG_DIR, RULES_FILE);
    return match result {
        Ok(rules) => rules,
        Err(msg) => {
            eprintln!("Unable to read window rules: {}", msg);
            Vec::new()
        },
    };
}

