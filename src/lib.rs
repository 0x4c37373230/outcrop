extern crate native_windows_gui as nwg;

pub mod files {
    pub fn path_exists(path: &str) -> bool {
        std::fs::metadata(path).is_ok()
    }

    pub fn loader_files() -> bool {
        let dll_folder = String::from("./plugins");
        let bds_path = String::from("./bedrock_server.exe");

        if !path_exists(&dll_folder) || !path_exists(&bds_path) {
            return false;
        }
        true
    }
}

pub mod config {
    use {simple_config_parser::Config, std::fs::File, std::io::Write};

    pub fn cfg_manager() -> bool {
        if !crate::files::path_exists("./outcrop.cfg") {
            let mut config = File::create("outcrop.cfg").unwrap();

            match write!(config, "loadAllMods = 0") {
                Err(e) => {
                    nwg::simple_message("Error", &format!("{}", e));
                }
                _ => {}
            }
            return false;
        }
        true
    }

    pub fn read_cfg() -> bool {
        let cfg = Config::new().file("./outcrop.cfg").unwrap();
        let inject_all: i32 = cfg.get("loadAllMods").unwrap();

        inject_all != 0
    }
}

pub mod injection {
    use {
        crate::{bds, files},
        injrs::{inject_windows::*, process_windows::*},
        std::collections::HashMap,
        std::process::exit,
        std::thread,
        std::time,
    };

    pub fn inject_all() {
        let dll_paths = std::fs::read_dir("./plugins").unwrap();

        if !files::loader_files() {
            return;
        }

        bds::bds_thread();
        thread::sleep(time::Duration::from_millis(5000));

        let bds_process = Process::find_first_by_name("bedrock_server").unwrap();

        for path in dll_paths {
            let dll = path.unwrap().path().display().to_string();

            if dll.ends_with(".dll") {
                match bds_process.inject(&dll) {
                    Err(_e) => return,
                    _ => {}
                }
            }
        }
    }

    pub fn dll_map() -> HashMap<i32, String> {
        let dll_folder = String::from("./plugins");
        let dll_paths = std::fs::read_dir(&dll_folder).unwrap();

        if !files::path_exists(&dll_folder) || !files::path_exists("./bedrock_server.exe") {
            exit(-1);
        }

        let mut dll_number = 0;
        let mut dll_list: HashMap<i32, String> = HashMap::new();

        for path in dll_paths {
            let dll = path.unwrap().path().display().to_string();

            if dll.ends_with(".dll") {
                dll_number += 1;
                dll_list.insert(dll_number, dll);
            }
        }

        dll_list
    }

    pub fn mod_inject(dll_name: &str) {
        if !files::loader_files() {
            return;
        }

        bds::bds_thread();
        thread::sleep(time::Duration::from_millis(5000));

        let bds_process = Process::find_first_by_name("bedrock_server").unwrap();

        match bds_process.inject(&format!("./plugins/{}.dll", dll_name)) {
            Err(e) => {
                nwg::simple_message("Error", &format!("{}", e));
            }
            _ => {}
        }
    }
}

pub mod bds {
    use {std::thread, subprocess::Exec};

    pub fn bds_thread() {
        thread::spawn(|| match Exec::shell("bedrock_server.exe").join() {
            Err(e) => {
                nwg::simple_message("Error", &format!("{}", e));
            }
            _ => {}
        });
    }
}
