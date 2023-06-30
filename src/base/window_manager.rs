use crate::TkOption;
use std::fmt::{Display, Formatter, Result as fmtResult};

pub enum WMAttribute {
    // All platforms
    Alpha(TkOption<f64>),
    Fullscreen(TkOption<bool>),
    Topmost(TkOption<bool>),
    // Windows
    #[cfg(target_os = "windows")]
    Disabled(Option<bool>),
    // TODO: Find options for -toolwindow on Windows
    #[cfg(target_os = "windows")]
    ToolWindow(()),
    // TODO: Colour struct
    #[cfg(target_os = "windows")]
    TransparentColor(()),
    // Mac OSX
    // TODO: look into all these mac options
    #[cfg(target_os = "macos")]
    Modified(()),
    #[cfg(target_os = "macos")]
    Notify(()),
    #[cfg(target_os = "macos")]
    TitlePath(()),
    #[cfg(target_os = "macos")]
    Transparent(()),
    // X11 (Linux)
    #[cfg(target_os = "linux")]
    Type(TkOption<Vec<X11WMAttrType>>),
    #[cfg(target_os = "linux")]
    Zoomed(TkOption<bool>),
}

impl Display for WMAttribute {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        match self {
            Self::Alpha(_) => write!(f, "-alpha"),
            Self::Fullscreen(_) => write!(f, "-fullscreen"),
            Self::Topmost(_) => write!(f, "-topmost"),
            #[cfg(target_os = "windows")]
            Self::Disabled(_) => write!(f, "-disabled"),
            #[cfg(target_os = "windows")]
            Self::ToolWindow(_) => write!(f, "-toolwindow"),
            #[cfg(target_os = "windows")]
            Self::TransparentColor(_) => write!(f, "-transparentcolor"),
            #[cfg(target_os = "macos")]
            Self::Modified(_) => write!(f, "-modified"),
            #[cfg(target_os = "macos")]
            Self::Notify(_) => write!(f, "-notify"),
            #[cfg(target_os = "macos")]
            Self::TitlePath(_) => write!(f, "-titlepath"),
            #[cfg(target_os = "macos")]
            Self::Transparent(_) => write!(f, "-transparent"),
            #[cfg(target_os = "linux")]
            Self::Type(_) => write!(f, "-type"),
            #[cfg(target_os = "linux")]
            Self::Zoomed(_) => write!(f, "-zoomed"),
        }
    }
}

#[cfg(target_os = "linux")]
#[derive(PartialEq)]
pub enum X11WMAttrType {
    Desktop,
    Dock,
    Toolbar,
    Menu,
    Utility,
    Splash,
    Dialog,
    DropdownMenu,
    PopupMenu,
    Tooltip,
    Notification,
    Combo,
    DnD,
    Normal,
}

impl Display for X11WMAttrType {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        match self {
            Self::Desktop => write!(f, "desktop"),
            Self::Dock => write!(f, "dock"),
            Self::Toolbar => write!(f, "toolbar"),
            Self::Menu => write!(f, "menu"),
            Self::Utility => write!(f, "utility"),
            Self::Splash => write!(f, "splash"),
            Self::Dialog => write!(f, "dialog"),
            Self::DropdownMenu => write!(f, "dropdown_menu"),
            Self::PopupMenu => write!(f, "popup_menu"),
            Self::Tooltip => write!(f, "tooltip"),
            Self::Notification => write!(f, "notification"),
            Self::Combo => write!(f, "combo"),
            Self::DnD => write!(f, "dnd"),
            Self::Normal => write!(f, "normal"),
        }
    }
}

impl X11WMAttrType {
    pub fn from_str(string: &str) -> Self {
        match string {
            "desktop" => Self::Desktop,
            "dock" => Self::Dock,
            "toolbar" => Self::Toolbar,
            "menu" => Self::Menu,
            "utility" => Self::Utility,
            "splash" => Self::Splash,
            "dialog" => Self::Dialog,
            "dropdown_menu" => Self::DropdownMenu,
            "popup_menu" => Self::PopupMenu,
            "tooltip" => Self::Tooltip,
            "notification" => Self::Notification,
            "combo" => Self::Combo,
            "dnd" => Self::DnD,
            "normal" => Self::Normal,
            _ => panic!("Invalid X11WMAttrType name"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct WMAspectRatio {
    width: u32,
    height: u32,
}

impl PartialOrd for WMAspectRatio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.width * other.height).partial_cmp(&(self.height * other.width))
    }
}

impl Ord for WMAspectRatio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.width * other.height).cmp(&(self.height * other.width))
    }
}

impl WMAspectRatio {
    pub fn specified(width: u32, height: u32) -> WMAspectRatio {
        if width == 0 || height == 0 {
            panic!("Cannot have 0 in a specified aspect ratio. Maybe you needed WMAspectRatio::unspecified()?")
        }
        WMAspectRatio {
            width: width,
            height: height,
        }
    }

    pub fn unspecified() -> WMAspectRatio {
        WMAspectRatio {
            width: 0,
            height: 0,
        }
    }

    pub fn is_unspecified(&self) -> bool {
        self.width == 0 && self.height == 0
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

pub trait WindowManager {
    fn wm_aspect(
        &mut self,
        aspect_ratios: TkOption<&mut [WMAspectRatio; 2]>,
    ) -> TkOption<[WMAspectRatio; 2]>;
    fn wm_attribute(&mut self, attribute: WMAttribute) -> WMAttribute;
}
