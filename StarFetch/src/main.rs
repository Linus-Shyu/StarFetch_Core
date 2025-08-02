// 声明模块而不是使用库方式
mod art;
mod hyperlink;
mod system;

fn main() {
    // 输出彩色LOGO
    println!("{}", art::colored_art());  // 修复函数名
    
    // 开发者信息（带超链接）
    print!("Developed by ");
    println!("{}", hyperlink::hyperlink(
        &hyperlink::styled_developer_name(),  // 修复函数名
        "https://github.com/Linus-Shyu"
    ));
    
    // 空行分隔
    println!();
    
    // 系统信息
    let _sys = system::init_system(); // 初始化系统信息
    system::print_hardware_info();
}
