use std::fmt::format;

use message::show_message;
use winreg::enums::*;
use winreg::RegKey;

mod message;

fn main() {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let key =
        hklm.open_subkey_with_flags("System\\CurrentControlSet\\Control\\Print", KEY_WRITE);

    if let Err(_) = key {
        show_message("Error", "Failed to open key in regedit.");
        return;
    };

    let key = key.unwrap();

    let res = key.set_value("RpcAuthnLevelPrivacyEnabled", &0u32);

    if let Err(e) = res {
        let msg= format!("Failed to create value\n{}", e.to_string());
        show_message("Error", &msg);
    } else {
        show_message("Success", "Key was created");
    }
}
