mod checkers;
mod hardware;

#[derive(Debug, Clone)]
struct RuntimeInfo {
    display_name: String,
    version: String,
    install_location: Option<String>,
    publisher: Option<String>,
}

fn main() {
    println!("=============== 运行时环境检测工具 ===============\n");
    println!("author: {}", env!("CARGO_PKG_AUTHORS"));
    println!("version: {}", env!("CARGO_PKG_VERSION"));

    println!("\n{}\n", "=".repeat(50));

    println!("Time: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
    println!("OS: {}", std::env::consts::OS);
    println!("Arch: {}", std::env::consts::ARCH);
    println!("CPU: {}", hardware::check_cpu());
    println!("GPU: {}", hardware::check_gpu());

    println!("\n{}", "=".repeat(50));

    checkers::check_vcruntime();
    println!("\n{}", "=".repeat(50));
    checkers::check_dotnet();
}
