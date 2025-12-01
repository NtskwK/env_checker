use std::collections::HashMap;
use std::process::Command;

use crate::RuntimeInfo;
use winreg::RegKey;
use winreg::enums::*;

pub fn check_vcruntime() {
    println!("\n=== Visual C++ Runtime 检测 ===\n");

    let mut found_runtimes: HashMap<String, RuntimeInfo> = HashMap::new();

    // 检查不同的注册表位置
    let registry_paths = vec![
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        ),
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
        ),
        (
            HKEY_CURRENT_USER,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        ),
    ];

    for (hkey, path) in registry_paths {
        if let Ok(uninstall_key) = RegKey::predef(hkey).open_subkey(path) {
            for subkey_name in uninstall_key.enum_keys().filter_map(|x| x.ok()) {
                if let Ok(subkey) = uninstall_key.open_subkey(&subkey_name) {
                    // 读取显示名称
                    if let Ok(display_name) = subkey.get_value::<String, _>("DisplayName") {
                        // 检查是否是 Visual C++ Runtime
                        if is_vcruntime(&display_name) {
                            let version = subkey
                                .get_value::<String, _>("DisplayVersion")
                                .unwrap_or_else(|_| "未知版本".to_string());
                            let install_location =
                                subkey.get_value::<String, _>("InstallLocation").ok();
                            let publisher = subkey.get_value::<String, _>("Publisher").ok();

                            // 使用显示名称作为键,避免重复
                            found_runtimes.insert(
                                display_name.clone(),
                                RuntimeInfo {
                                    display_name,
                                    version,
                                    install_location,
                                    publisher,
                                },
                            );
                        }
                    }
                }
            }
        }
    }

    // 显示结果
    if found_runtimes.is_empty() {
        println!("❌ 未检测到任何 Visual C++ Runtime 安装");
        println!("\n建议:");
        println!("  - 访问 Microsoft 官网下载最新的 VC++ Redistributable");
        println!(
            "  - 下载地址: https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist"
        );
    } else {
        println!(
            "✅ 检测到 {} 个 Visual C++ Runtime 安装:\n",
            found_runtimes.len()
        );

        // 按名称排序显示
        let mut runtimes: Vec<_> = found_runtimes.values().collect();
        runtimes.sort_by(|a, b| a.display_name.cmp(&b.display_name));

        for (index, runtime) in runtimes.iter().enumerate() {
            println!("{}. {}", index + 1, runtime.display_name);
            println!("   版本: {}", runtime.version);
            if let Some(publisher) = &runtime.publisher {
                println!("   发布者: {}", publisher);
            }
            if let Some(location) = &runtime.install_location {
                println!("   安装位置: {}", location);
            }
            println!();
        }

        // 统计信息
        let x86_count = runtimes
            .iter()
            .filter(|r| r.display_name.contains("(x86)"))
            .count();
        let x64_count = runtimes
            .iter()
            .filter(|r| r.display_name.contains("(x64)"))
            .count();

        println!("--- 统计信息 ---");
        if x86_count > 0 {
            println!("32位版本 (x86): {} 个", x86_count);
        }
        if x64_count > 0 {
            println!("64位版本 (x64): {} 个", x64_count);
        }
    }
}

/// 判断是否是 Visual C++ Runtime
fn is_vcruntime(display_name: &str) -> bool {
    let name_lower = display_name.to_lowercase();

    // 匹配各种 VC++ Runtime 的名称模式
    (name_lower.contains("visual c++") || name_lower.contains("microsoft visual c++"))
        && (name_lower.contains("runtime")
            || name_lower.contains("redistributable")
            || name_lower.contains("redist")
            || name_lower.contains("minimum runtime")
            || name_lower.contains("additional runtime"))
}

pub fn check_dotnet() {
    println!("\n=== .NET Runtime / SDK 检测 ===\n");

    let mut found_any = false;

    // 方法 1: 使用 dotnet CLI 检测
    println!("📦 通过 dotnet CLI 检测:\n");

    // 检查 dotnet 命令是否可用
    match Command::new("dotnet").arg("--version").output() {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
            println!("✅ dotnet CLI 已安装 (默认版本: {})\n", version);
            found_any = true;

            // 列出所有 SDK
            if let Ok(sdk_output) = Command::new("dotnet").arg("--list-sdks").output() {
                let sdks = String::from_utf8_lossy(&sdk_output.stdout);
                let sdk_lines: Vec<&str> = sdks.lines().collect();

                if !sdk_lines.is_empty() {
                    println!("🔧 已安装的 .NET SDK ({} 个):", sdk_lines.len());
                    for (i, sdk) in sdk_lines.iter().enumerate() {
                        println!("   {}. {}", i + 1, sdk.trim());
                    }
                    println!();
                }
            }

            // 列出所有 Runtime
            if let Ok(runtime_output) = Command::new("dotnet").arg("--list-runtimes").output() {
                let runtimes = String::from_utf8_lossy(&runtime_output.stdout);
                let runtime_lines: Vec<&str> = runtimes.lines().collect();

                if !runtime_lines.is_empty() {
                    println!("⚙️  已安装的 .NET Runtime ({} 个):", runtime_lines.len());

                    // 按类型分组
                    let mut core_runtimes = Vec::new();
                    let mut aspnet_runtimes = Vec::new();
                    let mut desktop_runtimes = Vec::new();

                    for runtime in runtime_lines {
                        let runtime = runtime.trim();
                        if runtime.starts_with("Microsoft.NETCore.App") {
                            core_runtimes.push(runtime);
                        } else if runtime.starts_with("Microsoft.AspNetCore.App") {
                            aspnet_runtimes.push(runtime);
                        } else if runtime.starts_with("Microsoft.WindowsDesktop.App") {
                            desktop_runtimes.push(runtime);
                        }
                    }

                    if !core_runtimes.is_empty() {
                        println!("\n   📘 .NET Runtime (Core):");
                        for (i, rt) in core_runtimes.iter().enumerate() {
                            println!("      {}. {}", i + 1, rt);
                        }
                    }

                    if !aspnet_runtimes.is_empty() {
                        println!("\n   🌐 ASP.NET Core Runtime:");
                        for (i, rt) in aspnet_runtimes.iter().enumerate() {
                            println!("      {}. {}", i + 1, rt);
                        }
                    }

                    if !desktop_runtimes.is_empty() {
                        println!("\n   🖥️  Windows Desktop Runtime:");
                        for (i, rt) in desktop_runtimes.iter().enumerate() {
                            println!("      {}. {}", i + 1, rt);
                        }
                    }
                    println!();
                }
            }

            // 显示 dotnet info
            if let Ok(info_output) = Command::new("dotnet").arg("--info").output() {
                let info = String::from_utf8_lossy(&info_output.stdout);

                // 提取安装路径
                for line in info.lines() {
                    if line.trim().starts_with("Base Path:") || line.contains("基路径:") {
                        println!(
                            "📁 SDK 基路径: {}",
                            line.split(':').nth(1).unwrap_or("").trim()
                        );
                    }
                }
            }
        }
        _ => {
            println!("❌ dotnet CLI 未找到或未安装");
        }
    }

    println!("\n{}", "-".repeat(60));

    // 方法 2: 检查注册表
    println!("\n📋 通过注册表检测:\n");

    let mut registry_found = false;
    let mut dotnet_installs: HashMap<String, RuntimeInfo> = HashMap::new();

    let registry_paths = vec![
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
        ),
        (
            HKEY_LOCAL_MACHINE,
            r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall",
        ),
    ];

    for (hkey, path) in registry_paths {
        if let Ok(uninstall_key) = RegKey::predef(hkey).open_subkey(path) {
            for subkey_name in uninstall_key.enum_keys().filter_map(|x| x.ok()) {
                if let Ok(subkey) = uninstall_key.open_subkey(&subkey_name) {
                    if let Ok(display_name) = subkey.get_value::<String, _>("DisplayName") {
                        if is_dotnet(&display_name) {
                            let version = subkey
                                .get_value::<String, _>("DisplayVersion")
                                .unwrap_or_else(|_| "未知版本".to_string());
                            let install_location =
                                subkey.get_value::<String, _>("InstallLocation").ok();
                            let publisher = subkey.get_value::<String, _>("Publisher").ok();

                            dotnet_installs.insert(
                                display_name.clone(),
                                RuntimeInfo {
                                    display_name,
                                    version,
                                    install_location,
                                    publisher,
                                },
                            );
                            registry_found = true;
                            found_any = true;
                        }
                    }
                }
            }
        }
    }

    if registry_found {
        let mut installs: Vec<_> = dotnet_installs.values().collect();
        installs.sort_by(|a, b| a.display_name.cmp(&b.display_name));

        println!("✅ 在注册表中找到 {} 个 .NET 相关安装:\n", installs.len());
        for (i, install) in installs.iter().enumerate() {
            println!("{}. {}", i + 1, install.display_name);
            println!("   版本: {}", install.version);
            if let Some(publisher) = &install.publisher {
                println!("   发布者: {}", publisher);
            }
            if let Some(location) = &install.install_location {
                println!("   安装位置: {}", location);
            }
            println!();
        }
    } else {
        println!("ℹ️  注册表中未找到 .NET 安装信息");
    }

    if !found_any {
        println!("\n❌ 未检测到任何 .NET 安装");
        println!("\n建议:");
        println!("  - 访问 Microsoft 官网下载 .NET SDK 或 Runtime");
        println!("  - 下载地址: https://dotnet.microsoft.com/download");
    }
}

/// 判断是否是 .NET 相关安装
fn is_dotnet(display_name: &str) -> bool {
    let name_lower = display_name.to_lowercase();

    (name_lower.contains(".net") || name_lower.contains("dotnet"))
        && (name_lower.contains("runtime")
            || name_lower.contains("sdk")
            || name_lower.contains("core")
            || name_lower.contains("framework")
            || name_lower.contains("desktop")
            || name_lower.contains("aspnetcore")
            || name_lower.contains("asp.net"))
}
