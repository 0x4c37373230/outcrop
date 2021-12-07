#![windows_subsystem = "windows"]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use {
    nwd::NwgUi,
    nwg::NativeUi,
    outcrop::{config, injection},
};

#[derive(Default, NwgUi)]
pub struct Outcrop {
    #[nwg_control(size: (500, 200), position: (300, 300), title: "OUTCROP", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [Outcrop::exit_program] )]
    window: nwg::Window,

    #[nwg_control(text: "Outcrop: A BDS dll mod loader and injector. Made by Luke7720 mainly because \
    Windows Defender tries to remove Xenos repeatedly and using Xenos was also confusing to some people. \n\
    ---------------------------------------------------------------------------------------------\
    ---------------------------
         ", size: (470, 70), position: (10, 10))]
    label: nwg::Label,

    #[nwg_control(text: "Select DLLs to inject", size: (235, 25), position: (10, 80))]
    label2: nwg::Label,

    #[nwg_control(text: "", size: (235, 25), position: (10, 110))]
    dll_path: nwg::TextInput,

    #[nwg_control(text: "See DLL list: ", size: (280, 25), position: (250, 80))]
    label3: nwg::Label,

    #[nwg_control(text: "Available mods", size: (235, 25), position: (250, 110))]
    #[nwg_events( OnButtonClick: [Outcrop::list] )]
    mod_list: nwg::Button,

    #[nwg_control(text: "Inject", size: (480, 30), position: (10, 150))]
    #[nwg_events( OnButtonClick: [Outcrop::inject] )]
    inject: nwg::Button,
}

impl Outcrop {
    fn inject(&self) {
        let dll = String::from(&self.dll_path.text());

        injection::mod_inject(&dll);
    }

    fn list(&self) {
        let dll_list = injection::dll_map();
        let mut dlls: String = String::from(" ");

        for (key, dll) in &dll_list {
            dlls.push_str(&format!("{}: {}\n", key, dll));
        }

        nwg::simple_message("Completed", &format!("{}", dlls));
    }

    fn exit_program(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    let cfg_exists = config::cfg_manager();

    if cfg_exists {
        if !config::read_cfg() {
            load_gui();
        } else {
            injection::inject_all();
        }
    } else {
        load_gui();
    }
}

fn load_gui() {
    nwg::init().expect("Failed to init Native Windows GUI");

    let _app = Outcrop::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
