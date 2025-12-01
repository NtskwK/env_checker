use std::collections::HashMap;
use std::path::Path;
use std::process::Command;

use crate::RuntimeInfo;

pub fn check_vcruntime() {
    println!("\n=== Visual C++ Runtime 检测 ===\n");

    let mut found_runtimes: HashMap<String, RuntimeInfo> = HashMap::new();

    // 检查 System32 和 SysWOW64 目录中的 VC++ Runtime DLL
    let system_root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_string());

    let check_paths = vec![
        (format!("{}\\System32", system_root), "x64"),
        (format!("{}\\SysWOW64", system_root), "x86"),
    ];

    // VC++ Runtime DLL 版本映射
    let vcruntime_dlls = vec![
        // VC++ 2015-2022 (14.x)
        ("vcruntime140.dll", "2015-2022", "14.x"),
        ("vcruntime140_1.dll", "2015-2022", "14.x"),
        ("msvcp140.dll", "2015-2022", "14.x"),
        // VC++ 2013 (12.0)
        ("msvcr120.dll", "2013", "12.0"),
        ("msvcp120.dll", "2013", "12.0"),
        // VC++ 2012 (11.0)
        ("msvcr110.dll", "2012", "11.0"),
        ("msvcp110.dll", "2012", "11.0"),
        // VC++ 2010 (10.0)
        ("msvcr100.dll", "2010", "10.0"),
        ("msvcp100.dll", "2010", "10.0"),
        // VC++ 2008 (9.0)
        ("msvcr90.dll", "2008", "9.0"),
        ("msvcp90.dll", "2008", "9.0"),
        // VC++ 2005 (8.0)
        ("msvcr80.dll", "2005", "8.0"),
        ("msvcp80.dll", "2005", "8.0"),
    ];

    let mut detected_versions: HashMap<String, Vec<String>> = HashMap::new();

    for (dir_path, arch) in check_paths {
        for (dll_name, year, version) in &vcruntime_dlls {
            let dll_path = Path::new(&dir_path).join(dll_name);
            if dll_path.exists() {
                let key = format!("Visual C++ {} Redistributable ({})", year, arch);
                detected_versions
                    .entry(key)
                    .or_insert_with(Vec::new)
                    .push(version.to_string());
            }
        }
    }

    // 去重并创建 RuntimeInfo
    for (display_name, versions) in detected_versions {
        let version = versions
            .into_iter()
            .next()
            .unwrap_or_else(|| "未知".to_string());
        found_runtimes.insert(
            display_name.clone(),
            RuntimeInfo {
                display_name,
                version,
                install_location: Some(system_root.clone()),
                publisher: Some("Microsoft Corporation".to_string()),
            },
        );
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
                println!("   检测位置: {}", location);
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

pub fn check_dotnet() {
    println!("\n=== .NET Desktop Runtime 检测 ===\n");

    let mut found_any = false;

    // 检查 dotnet 命令是否可用
    match Command::new("dotnet").arg("--list-runtimes").output() {
        Ok(output) if output.status.success() => {
            let runtimes = String::from_utf8_lossy(&output.stdout);
            let runtime_lines: Vec<&str> = runtimes.lines().collect();

            // 只筛选 Windows Desktop Runtime
            let mut desktop_runtimes = Vec::new();

            for runtime in runtime_lines {
                let runtime = runtime.trim();
                if runtime.starts_with("Microsoft.WindowsDesktop.App") {
                    desktop_runtimes.push(runtime);
                }
            }

            if !desktop_runtimes.is_empty() {
                found_any = true;
                println!(
                    "🖥️  已安装的 Windows Desktop Runtime ({} 个):\n",
                    desktop_runtimes.len()
                );

                for (i, rt) in desktop_runtimes.iter().enumerate() {
                    println!("   {}. {}", i + 1, rt);
                }
                println!();
            }
        }
        _ => {
            println!("❌ dotnet CLI 未找到或未安装");
        }
    }

    if !found_any {
        println!("\n❌ 未检测到任何 Windows Desktop Runtime 安装");
        println!("\n建议:");
        println!("  - 访问 Microsoft 官网下载 .NET Desktop Runtime");
        println!("  - 下载地址: https://dotnet.microsoft.com/download");
    }
}
