use std::process::Command;

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

#[cfg(target_os = "windows")]
pub fn check_gpu() -> String {
    // 使用 WMIC 命令行工具获取 GPU 信息
    match Command::new("wmic")
        .args(&["path", "win32_VideoController", "get", "name"])
        .output()
    {
        Ok(output) if output.status.success() => {
            let gpu_output = String::from_utf8_lossy(&output.stdout);
            let mut found_gpus = Vec::new();

            for line in gpu_output.lines().skip(1) {
                // 跳过标题行
                let gpu_name = line.trim();
                if !gpu_name.is_empty() && gpu_name != "Name" {
                    found_gpus.push(gpu_name.to_string());
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
                    let summary = format!("共 {} 个GPU\n", found_gpus.len());
                    gpu_info = summary + &gpu_info;
                }
                return gpu_info;
            }
        }
        _ => {
            return "无法获取显卡信息 (WMIC 命令失败)".to_string();
        }
    }
}

#[cfg(target_os = "linux")]
pub fn check_gpu() -> String {
    match Command::new("lspci").output() {
        Ok(output) if output.status.success() => {
            let lspci_output = String::from_utf8_lossy(&output.stdout);
            let mut found_gpus = Vec::new();

            for line in lspci_output.lines() {
                if line.contains("VGA compatible controller") || line.contains("3D controller") {
                    let parts: Vec<&str> = line.split(": ").collect();
                    if parts.len() > 1 {
                        found_gpus.push(parts[1].to_string());
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
                return gpu_info;
            }
        }
        _ => {
            return "无法获取显卡信息 (lspci 命令失败)".to_string();
        }
    }
}

#[cfg(target_os = "macos")]
pub fn check_gpu() -> String {
    match Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()
    {
        Ok(output) if output.status.success() => {
            let profiler_output = String::from_utf8_lossy(&output.stdout);
            let mut found_gpus = Vec::new();
            let mut current_gpu = String::new();

            for line in profiler_output.lines() {
                let line = line.trim();
                if line.starts_with("Chipset Model:") {
                    current_gpu = line.replace("Chipset Model:", "").trim().to_string();
                    found_gpus.push(current_gpu.clone());
                }
            }

            if found_gpus.is_empty() {
                return "无法获取显卡信息".to_string();
            } else {
                let mut gpu_info = String::new();
                for (i, gpu) in found_gpus.iter().enumerate() {
                    gpu_info.push_str(&format!("{}. {}\n", i + 1, gpu));
                }
                return gpu_info;
            }
        }
        _ => {
            return "无法获取显卡信息 (system_profiler 命令失败)".to_string();
        }
    }
}
