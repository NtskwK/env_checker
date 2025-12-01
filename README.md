# 运行时环境检测工具

一个用 Rust 编写的 Windows 系统运行时环境检测工具,可以检测系统上安装的 Visual C++ Runtime 和 .NET Runtime/SDK。

## 功能特性

### ✅ Visual C++ Runtime 检测

- 扫描 Windows 注册表检测已安装的 VC++ Runtime
- 支持检测 32 位 (x86) 和 64 位 (x64) 版本
- 识别各种 VC++ Runtime 变体:
  - Visual C++ Redistributable
  - Minimum Runtime
  - Additional Runtime
  - Debug Runtime
- 显示详细信息:版本号、发布者、安装位置
- 统计分析:按架构分类统计

### ✅ .NET Runtime / SDK 检测

- **通过 dotnet CLI 检测**:

  - 检测 dotnet 命令是否可用
  - 列出所有已安装的 .NET SDK
  - 列出所有已安装的 .NET Runtime,按类型分组:
    - 📘 .NET Runtime (Core)
    - 🌐 ASP.NET Core Runtime
    - 🖥️ Windows Desktop Runtime
  - 显示 SDK 基路径

- **通过注册表检测**:
  - 扫描注册表中的 .NET 相关安装
  - 显示版本、发布者、安装位置等详细信息

## 使用方法

### 编译并运行

```bash
cargo run
```

### 发布版本

```bash
cargo run --release
```

### 使用选项

运行程序后,会显示菜单:

```
=== 运行时环境检测工具 ===

请选择要检测的内容:
1. Visual C++ Runtime
2. .NET Runtime / SDK
3. 全部检测

输入选项 (1/2/3) 或直接按回车检测全部:
```

- 输入 `1`: 仅检测 Visual C++ Runtime
- 输入 `2`: 仅检测 .NET Runtime/SDK
- 输入 `3` 或直接回车: 检测全部

## 示例输出

### Visual C++ Runtime 检测示例

```
=== Visual C++ Runtime 检测 ===

✅ 检测到 27 个 Visual C++ Runtime 安装:

1. Microsoft Visual C++ 2015-2022 Redistributable (x64) - 14.40.33810
   版本: 14.40.33810.0
   发布者: Microsoft Corporation

2. Microsoft Visual C++ 2015-2022 Redistributable (x86) - 14.40.33810
   版本: 14.40.33810.0
   发布者: Microsoft Corporation

--- 统计信息 ---
32位版本 (x86): 3 个
64位版本 (x64): 4 个
```

### .NET Runtime 检测示例

```
=== .NET Runtime / SDK 检测 ===

📦 通过 dotnet CLI 检测:

✅ dotnet CLI 已安装 (默认版本: 9.0.101)

🔧 已安装的 .NET SDK (3 个):
   1. 8.0.404 [C:\Program Files\dotnet\sdk]
   2. 9.0.100 [C:\Program Files\dotnet\sdk]
   3. 9.0.101 [C:\Program Files\dotnet\sdk]

⚙️  已安装的 .NET Runtime (9 个):

   📘 .NET Runtime (Core):
      1. Microsoft.NETCore.App 8.0.11 [C:\Program Files\dotnet\shared\Microsoft.NETCore.App]
      2. Microsoft.NETCore.App 9.0.0 [C:\Program Files\dotnet\shared\Microsoft.NETCore.App]
      3. Microsoft.NETCore.App 9.0.1 [C:\Program Files\dotnet\shared\Microsoft.NETCore.App]

   🌐 ASP.NET Core Runtime:
      1. Microsoft.AspNetCore.App 8.0.11 [C:\Program Files\dotnet\shared\Microsoft.AspNetCore.App]
      2. Microsoft.AspNetCore.App 9.0.0 [C:\Program Files\dotnet\shared\Microsoft.AspNetCore.App]
      3. Microsoft.AspNetCore.App 9.0.1 [C:\Program Files\dotnet\shared\Microsoft.AspNetCore.App]

   🖥️  Windows Desktop Runtime:
      1. Microsoft.WindowsDesktop.App 8.0.11 [C:\Program Files\dotnet\shared\Microsoft.WindowsDesktop.App]
      2. Microsoft.WindowsDesktop.App 9.0.0 [C:\Program Files\dotnet\shared\Microsoft.WindowsDesktop.App]
      3. Microsoft.WindowsDesktop.App 9.0.1 [C:\Program Files\dotnet\shared\Microsoft.WindowsDesktop.App]

📁 SDK 基路径: C:\Program Files\dotnet\sdk\9.0.101\
```

## 技术细节

### 依赖项

- `winreg 0.52`: Windows 注册表访问
- Rust 标准库的 `std::process::Command`: 执行 dotnet CLI 命令

### 检测原理

#### Visual C++ Runtime

通过扫描以下注册表位置:

- `HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall`
- `HKEY_LOCAL_MACHINE\SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall`
- `HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall`

匹配包含 "Visual C++" 和 "Runtime/Redistributable" 关键字的条目。

#### .NET Runtime/SDK

1. **CLI 检测**: 执行 `dotnet --list-sdks` 和 `dotnet --list-runtimes` 命令
2. **注册表检测**: 扫描注册表中包含 ".NET" 和相关关键字的条目

## 系统要求

- Windows 操作系统
- Rust 工具链 (用于编译)

## 许可证

MIT License

## 作者

Created with Antigravity AI
