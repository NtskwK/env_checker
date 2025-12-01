# 运行时环境检测工具

一个用 Rust 编写的 Windows 系统运行时环境检测工具,可以检测系统上安装的 Visual C++ Runtime 和 .NET Desktop Runtime。

## 功能特性

### ✅ Visual C++ Runtime 检测

- 通过检测系统目录中的 DLL 文件来识别已安装的 VC++ Runtime
- 支持检测 32 位 (x86) 和 64 位 (x64) 版本
- 支持的 VC++ Runtime 版本:
  - Visual C++ 2005 (8.0)
  - Visual C++ 2008 (9.0)
  - Visual C++ 2010 (10.0)
  - Visual C++ 2012 (11.0)
  - Visual C++ 2013 (12.0)
  - Visual C++ 2015-2022 (14.x)
- 显示详细信息:版本号、发布者、检测位置
- 统计分析:按架构分类统计

### ✅ .NET Desktop Runtime 检测

- **通过 dotnet CLI 检测**:
  - 检测 dotnet 命令是否可用
  - 列出所有已安装的 Windows Desktop Runtime
  - 显示版本号和安装路径

> **注意**: 本工具专注于检测 Windows Desktop Runtime，不检测 .NET SDK、.NET Core Runtime 或 ASP.NET Core Runtime。

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
2. .NET Desktop Runtime
3. 全部检测

输入选项 (1/2/3) 或直接按回车检测全部:
```

- 输入 `1`: 仅检测 Visual C++ Runtime
- 输入 `2`: 仅检测 .NET Desktop Runtime
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

### .NET Desktop Runtime 检测示例

```
=== .NET Desktop Runtime 检测 ===

�️  已安装的 Windows Desktop Runtime (6 个):

   1. Microsoft.WindowsDesktop.App 3.1.32 [C:\Program Files\dotnet\shared\Microsoft.WindowsDesktop.App]
   2. Microsoft.WindowsDesktop.App 6.0.36 [C:\Program Files\dotnet\shared\Microsoft.WindowsDesktop.App]
   3. Microsoft.WindowsDesktop.App 8.0.16 [C:\Program Files\dotnet\shared\Microsoft.WindowsDesktop.App]
   4. Microsoft.WindowsDesktop.App 8.0.18 [C:\Program Files\dotnet\shared\Microsoft.WindowsDesktop.App]
   5. Microsoft.WindowsDesktop.App 9.0.7 [C:\Program Files\dotnet\shared\Microsoft.WindowsDesktop.App]
   6. Microsoft.WindowsDesktop.App 10.0.0 [C:\Program Files\dotnet\shared\Microsoft.WindowsDesktop.App]
```

## 技术细节

### 依赖项

- `winreg 0.52`: Windows 注册表访问
- Rust 标准库的 `std::process::Command`: 执行 dotnet CLI 命令

### 检测原理

#### Visual C++ Runtime

通过检测 Windows 系统目录中的 VC++ Runtime DLL 文件:

- `%SystemRoot%\System32` (64 位 DLL)
- `%SystemRoot%\SysWOW64` (32 位 DLL)

根据检测到的 DLL 文件名（如 `vcruntime140.dll`、`msvcr120.dll` 等）来识别对应的 VC++ Runtime 版本。

#### .NET Desktop Runtime

执行 `dotnet --list-runtimes` 命令，筛选出 `Microsoft.WindowsDesktop.App` 类型的 Runtime。

## 系统要求

- Windows 操作系统
- Rust 工具链 (用于编译)

## 许可证

MIT License

## 作者

Created with Antigravity AI
