#![allow(unused)]

use crate::{window_manager::*, Interpreter, TkOption};
// use rust_tk_macros::WindowManager;

// #[derive(WindowManager)]
pub struct Tk {
    path_name: String,
    pub interpreter: Interpreter,
}

impl WindowManager for Tk {
    fn wm_aspect(
        &mut self,
        aspect_ratios: TkOption<&mut [WMAspectRatio; 2]>,
    ) -> TkOption<[WMAspectRatio; 2]> {
        let cmd = format!("wm aspect {} ", self.path_name);
        if let TkOption::Set(arr) = aspect_ratios {
            arr.sort();
            let min = arr[0];
            let max = arr[1];
            if min.is_unspecified() != max.is_unspecified() {
                panic!("Aspect ratios must either be both specified or unspecified");
            }
            if min.is_unspecified() {
                self.interpreter
                    .run_command(cmd + r#""" "" "" """#)
                    .expect("wm aspect should not have failed");
            } else {
                let aspect_str = format!(
                    "{} {} {} {}",
                    min.width(),
                    min.height(),
                    max.width(),
                    max.height(),
                );
                self.interpreter
                    .run_command(cmd + &aspect_str)
                    .expect("wm aspect should not have failed");
            }
            TkOption::Get
        } else {
            let res = self
                .interpreter
                .run_command_with_response(cmd)
                .expect("wm aspect should not have failed");
            let mut aspect_nums = res.split_whitespace().map(|n|{n.parse::<u32>().expect("wm aspect should always return valid numbers")});
            if let Some(min_width) = aspect_nums.next() {

            } else {

            }
            TkOption::Get
        }
    }

    fn wm_attribute(&mut self, attribute: WMAttribute) -> WMAttribute {
        let cmd = format!(
            "wm attributes {} {} ",
            self.path_name,
            attribute.to_string()
        );
        match attribute {
            WMAttribute::Alpha(opt) => {
                if let TkOption::Set(float_val) = opt {
                    self.interpreter
                        .run_command(cmd + &float_val.to_string())
                        .expect("wm attributes should not have failed");
                    WMAttribute::Alpha(TkOption::Get)
                } else {
                    let res = self
                        .interpreter
                        .run_command_with_response(cmd)
                        .expect("wm attributes should not have failed");
                    WMAttribute::Alpha(TkOption::Set(
                        res.parse::<f64>()
                            .expect("-alpha should always return a valid floating point"),
                    ))
                }
            }
            WMAttribute::Fullscreen(opt) => {
                if let TkOption::Set(bool_val) = opt {
                    self.interpreter
                        .run_command(cmd + &(bool_val as u8).to_string())
                        .expect("wm attributes should not have failed");
                    WMAttribute::Fullscreen(TkOption::Get)
                } else {
                    let res = self
                        .interpreter
                        .run_command_with_response(cmd)
                        .expect("wm attributes should not have failed");
                    WMAttribute::Fullscreen(TkOption::Set(
                        res.parse::<u8>()
                            .expect("-fullscreen should always return a valid bool-like int")
                            != 0,
                    ))
                }
            }
            WMAttribute::Topmost(opt) => {
                if let TkOption::Set(bool_val) = opt {
                    self.interpreter
                        .run_command(cmd + &(bool_val as u8).to_string())
                        .expect("wm attributes should not have failed");
                    WMAttribute::Topmost(TkOption::Get)
                } else {
                    let res = self
                        .interpreter
                        .run_command_with_response(cmd)
                        .expect("wm attributes should not have failed");
                    WMAttribute::Topmost(TkOption::Set(
                        res.parse::<u8>()
                            .expect("-topmost should always return a valid bool-like int")
                            != 0,
                    ))
                }
            }
            #[cfg(target_os = "windows")]
            WMAttribute::Disabled(opt) => {}
            #[cfg(target_os = "windows")]
            WMAttribute::ToolWindow(opt) => {}
            #[cfg(target_os = "windows")]
            WMAttribute::TransparentColor(opt) => {}
            #[cfg(target_os = "macos")]
            WMAttribute::Modified(opt) => {}
            #[cfg(target_os = "macos")]
            WMAttribute::Notify(opt) => {}
            #[cfg(target_os = "macos")]
            WMAttribute::TitlePath(opt) => {}
            #[cfg(target_os = "macos")]
            WMAttribute::Transparent(opt) => {}
            #[cfg(target_os = "linux")]
            WMAttribute::Type(opt) => {
                if let TkOption::Set(vec_var) = opt {
                    let types_as_string = vec_var
                        .iter()
                        .map(|attr| attr.to_string())
                        .collect::<Vec<String>>()
                        .join(" ");
                    println!("{}", cmd.clone() + "{" + &types_as_string + "}");
                    self.interpreter
                        .run_command(cmd + "{" + &types_as_string + "}")
                        .expect("wm attributes should not have failed");
                    WMAttribute::Type(TkOption::Get)
                } else {
                    let res = self
                        .interpreter
                        .run_command_with_response(cmd)
                        .expect("wm attributes should not have failed");
                    let types_as_vec: Vec<crate::X11WMAttrType> = res
                        .split_whitespace()
                        .map(|attr| crate::X11WMAttrType::from_str(attr))
                        .collect();
                    WMAttribute::Type(TkOption::Set(types_as_vec))
                }
            }
            #[cfg(target_os = "linux")]
            WMAttribute::Zoomed(opt) => {
                if let TkOption::Set(bool_val) = opt {
                    self.interpreter
                        .run_command(cmd + &(bool_val as u8).to_string())
                        .expect("wm attributes should not have failed");
                    WMAttribute::Zoomed(TkOption::Get)
                } else {
                    let res = self
                        .interpreter
                        .run_command_with_response(cmd)
                        .expect("wm attributes should not have failed");
                    WMAttribute::Zoomed(TkOption::Set(
                        res.parse::<u8>()
                            .expect("-zoomed should always return a valid bool-like int")
                            != 0,
                    ))
                }
            }
        }
    }
}

impl Tk {
    pub fn new() -> Tk {
        Tk {
            path_name: ".".to_string(),
            interpreter: Interpreter::start(),
        }
    }
}
