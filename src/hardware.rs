use winreg::{RegKey, enums::HKEY_LOCAL_MACHINE};

pub fn check_cpu() -> String {
    use sysinfo::System;
    let mut sys = System::new_all();
    sys.refresh_cpu_all();

    if let Some(cpu) = sys.cpus().first() {
        return cpu.brand().to_string();
    } else {
        return "无法获取 CPU 信息".to_string();
    }
}

pub fn check_gpu() -> String {
    let registry_paths = vec![(
        HKEY_LOCAL_MACHINE,
        r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11ce-bfc1-08002be10318}",
    )];

    let mut found_gpus = Vec::new();

    for (hkey, path) in registry_paths {
        if let Ok(gpu_key) = RegKey::predef(hkey).open_subkey(path) {
            for subkey_name in gpu_key.enum_keys().filter_map(|x| x.ok()) {
                // 跳过非数字的子键
                if !subkey_name.chars().all(|c| c.is_numeric()) {
                    continue;
                }

                if let Ok(subkey) = gpu_key.open_subkey(&subkey_name) {
                    if let Ok(driver_desc) = subkey.get_value::<String, _>("DriverDesc") {
                        if !driver_desc.is_empty() && !found_gpus.contains(&driver_desc) {
                            found_gpus.push(driver_desc);
                        }
                    }
                }
            }
        }
    }

    if found_gpus.is_empty() {
        return "无法获取显卡信息".to_string();
    } else {
        let mut gpu_info = String::new();
        for (i, gpu) in found_gpus.iter().enumerate() {
            gpu_info.push_str(&format!("{}. {}\n", i + 1, gpu));
        }
        if found_gpus.len() > 1 {
            let summary = format!("共 {} 个显卡\n", found_gpus.len());
            gpu_info = summary + &gpu_info;
        }
        return gpu_info;
    }
}
