use std::env;

struct IOS {
    os: String,
    supported: bool
}

pub fn string_element_exists(element: &str, array: [&str; 15]) -> bool {
    for i in array {
        if i == element {
            return true;
        }
    }

    return false;
}

pub fn detect_os() -> IOS {
    let os = env::consts::OS;
    let os_name: &str;
    let supported: bool;

    let unix = "unix";

    match os {
        "android"=> {
            os_name = "Phone (Android)";
            supported = false;
        },

        "dragonfly"=>{
            os_name = unix;
            supported = true;
        },
        "freebsd" | "netbsd" | "openbsd" | "solaris" => {
            os_name = unix;
            supported = true;
        },

        "ios"=>{
            os_name = "Phone (iOS)";
            supported = false;
        },
        "macos"=>{
            os_name = os;
            supported = true;
        },

        "linux"=>{
            os_name = os;
            supported = true;
        },

        "windows"=>{
            os_name = os;
            supported = true;
        },

        _=>{
            os_name = os;
            supported = false;
        }
    }

    return IOS {
        supported,
        os: os_name.to_string()
    };
}

pub fn int_to_string(integer: i32) -> String {
    return integer.to_string();
}