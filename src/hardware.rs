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
