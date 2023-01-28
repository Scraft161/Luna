use serde::{Serialize, Deserialize};
use x11::xlib;
use libmars::configuration::read_config_file;
use libmars::x11::draw::widget::*;

use crate::tray::*;


const CONFIG_DIR: &str = "marswm";
const CONFIG_FILE: &str = "marsbar.yaml";

const DEFAULT_FONT: &'static str = "serif";
const DEFAULT_LAYOUT_PADDING_HORZ: u32 = 4;
const DEFAULT_LAYOUT_PADDING_VERT: u32 = 4;
const DEFAULT_LAYOUT_SPACING: u32 = 4;
const DEFAULT_TEXT_PADDING_HORZ: u32 = 5;
const DEFAULT_TEXT_PADDING_VERT: u32 = 0;


pub trait CreateWidget<W: Widget> {
    fn create_widget(&self, display: *mut xlib::Display, parent: xlib::Window) -> Result<W, String>;
}


#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(default)]
pub struct BarStyle {
    pub background: u64,
    pub workspaces: ContainerWidgetStyle,
    pub title: TextWidgetStyle,
    pub status: ContainerWidgetStyle,
}

#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(default)]
pub struct TextWidgetStyle {
    pub foreground: u64,
    pub background: u64,
    pub padding_horz: u32,
    pub padding_vert: u32,
}

#[derive(Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(default)]
pub struct ContainerWidgetStyle {
    pub foreground: u64,
    pub inner_background: u64,
    pub outer_background: u64,
    pub padding_horz: u32,
    pub padding_vert: u32,
    pub text_padding_horz: u32,
    pub text_padding_vert: u32,
    pub spacing: u32,
}

#[derive(Default,Serialize,Deserialize,PartialEq,Debug,Clone)]
#[serde(default)]
pub struct Configuration {
    pub callback: Option<String>,
    pub style: BarStyle,
}


impl Default for BarStyle {
    fn default() -> Self {
        return BarStyle {
            background: 0x000000,
            workspaces: ContainerWidgetStyle::default(),
            title: TextWidgetStyle::default(),
            status: ContainerWidgetStyle::default(),
        }
    }
}

impl Default for ContainerWidgetStyle {
    fn default() -> Self {
        return ContainerWidgetStyle {
            foreground: 0x000000,
            inner_background: 0xffffff,
            outer_background: 0x000000,
            padding_horz: DEFAULT_LAYOUT_PADDING_HORZ,
            padding_vert: DEFAULT_LAYOUT_PADDING_VERT,
            text_padding_horz: DEFAULT_TEXT_PADDING_HORZ,
            text_padding_vert: DEFAULT_TEXT_PADDING_VERT,
            spacing: DEFAULT_LAYOUT_SPACING
        };
    }
}

impl Default for TextWidgetStyle {
    fn default() -> Self {
        return TextWidgetStyle {
            foreground: 0xffffff,
            background: 0x000000,
            padding_horz: DEFAULT_TEXT_PADDING_HORZ,
            padding_vert: DEFAULT_TEXT_PADDING_VERT
        };
    }
}

impl ContainerWidgetStyle {
    pub fn create_flow_layout_widget<W: Widget>(&self, display: *mut xlib::Display, parent: xlib::Window)
            -> Result<FlowLayoutWidget<W>, String> {
        return FlowLayoutWidget::new(display, parent, 0, 0, self.padding_horz, self.padding_vert, self.spacing,
                                     Vec::new(), self.outer_background);
    }

    pub fn create_text_widget(&self, display: *mut xlib::Display, parent: xlib::Window) -> Result<TextWidget, String> {
        return TextWidget::new(display, parent, 0, 0, self.text_padding_horz, self.text_padding_vert,
                               "".to_string(), DEFAULT_FONT, self.foreground, self.inner_background);
    }

    pub fn create_systray_widget(&self, display: *mut xlib::Display, parent: xlib::Window, parent_height: u32)
            -> Result<SystemTrayWidget, String> {
        return SystemTrayWidget::new(display, parent, 0, 0, parent_height - 2 * self.spacing, self.padding_horz,
                                     self.padding_horz, self.padding_vert, self.inner_background);
    }
}

impl TextWidgetStyle {
    pub fn create_text_widget(&self, display: *mut xlib::Display, parent: xlib::Window) -> Result<TextWidget, String> {
        return TextWidget::new(display, parent, 0, 0, self.padding_horz, self.padding_vert,
                               "".to_string(), DEFAULT_FONT, self.foreground, self.background);
    }
}


pub fn read_config() -> Configuration {
    let result = read_config_file(CONFIG_DIR, CONFIG_FILE);
    return match result {
        Ok(config) => config,
        Err(msg) => {
            println!("Unable to read configuration: {}", msg);
            Configuration::default()
        },
    };
}
