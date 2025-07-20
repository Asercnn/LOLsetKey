use std::path::{Path, PathBuf};
use std::fs;
use sysinfo::{ ProcessesToUpdate, System};
use tauri::command;



#[command]
pub fn backup_configs(game_path: String) -> Result<(), String> {
    println!("备份游戏配置到当前目录: {}", game_path);
    let config_dir = Path::new(&game_path).join("Game").join("Config");
    let target_dir: PathBuf = std::env::current_dir().map_err(|e| e.to_string())?;
    for file in ["game.cfg", "PersistedSettings.json"] {
        let src = config_dir.join(file);
        let dest = target_dir.join(file);
        if !src.exists() {
            return Err(format!("游戏目录下缺少文件: {}", src.display()));
        }
        fs::copy(&src, &dest).map_err(|e| format!("备份文件失败 {}: {}", file, e))?;
    }
    Ok(())
}


#[command]
pub fn restore_configs(game_path: String) -> Result<(), String> {
    println!("恢复游戏配置到目录: {}", game_path);
    let config_dir = Path::new(&game_path).join("Game").join("Config");
    let source_dir = std::env::current_dir().map_err(|e| e.to_string())?;

    for file in ["game.cfg", "PersistedSettings.json"] {
        let src = source_dir.join(file);
        let dest = config_dir.join(file);
        if !src.exists() {
            return Err(format!("当前目录缺少文件: {}", src.display()));
        }
        fs::copy(&src, &dest).map_err(|e| format!("恢复文件失败 {}: {}", file, e))?;
    }
    Ok(())
}



/// 自动检测游戏路径（Windows示例，读取注册表或查找默认路径）
/// 这里只做简单示例，真实实现需要你补充
#[command]
pub fn find_lol_root() -> Option<String> {
    // 刷新进程信息
    let mut sys = System::new_all();
    sys.refresh_processes(
        ProcessesToUpdate::All,
        true
    ); 


    for (_pid, process ) in sys.processes() {
        let exe_path: Option<&Path>= process.exe();
        let exe_name: String = match exe_path?.file_name() {
            Some(name) => name.to_string_lossy().to_lowercase(),
            None => continue,
        };

        // 匹配进程名
        if exe_name.contains("leagueclient") || exe_name.contains("league of legends") {
            println!("发现英雄联盟进程路径: {}", exe_path?.display());
            // 往上两级目录通常是根目录
            let root_dir: &Path = match exe_path?.parent().and_then(|p| p.parent()) {
                Some(dir) => dir,
                None => continue,
            };
            return Some(root_dir.to_string_lossy().to_string());
        }
    }

    None
}