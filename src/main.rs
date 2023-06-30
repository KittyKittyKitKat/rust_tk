#![allow(unused)]
use rust_tk::{
    Tk,
    TkOption::{self, *},
    WMAspectRatio, WMAttribute, WindowManager,
    X11WMAttrType::{self, *},
};

fn main() {
    let mut tk = Tk::new();

    // tk.wm_aspect(Set(&mut [
    //     WMAspectRatio::specified(4, 3),
    //     WMAspectRatio::specified(16, 9),
    // ]));

    tk.wm_aspect(Get);

    // tk.run_command(r#"label .l -text "Hello World""#).unwrap();
    // tk.run_command("pack .l").unwrap();
    // let ans = tk.run_command_with_response(r#".l cget -text"#).unwrap();
    // println!("{}", ans);

    // tk.run_command(r#"button .b -text "Hello World""#).unwrap();
    // tk.run_command("pack .b").unwrap();

    // let ans2 = tk.run_command_with_response(r#".b config"#).unwrap();
    // println!("{}", ans2);

    loop {}
}
