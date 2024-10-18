use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Serialize, Deserialize)]
struct Config {
    window: Window,
    location: Location,
    right_click: RightClick,
    zoom: f64,
    step: f64,
    image_path: String,
}

#[derive(Serialize, Deserialize)]
struct Window {
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize)]
struct Location {
    win_x: u32,
    win_y: u32,
}

#[derive(Serialize, Deserialize)]
struct RightClick {
    x: u32,
    y: u32,
}

fn main() {
    // 读取 JSON 文件
    let file_path = "config.json";
    let data = fs::read_to_string(file_path).expect("Unable to read file");
    let mut config: Config = serde_json::from_str(&data).expect("JSON was not well-formatted");

    // 定义不同的 right_click 参数
    let right_click_params = vec![
        RightClick { x: 21, y: 39 },
        RightClick { x: 50, y: 60 },
        RightClick { x: 100, y: 120 },
    ];

    for param in right_click_params {
        // 修改 right_click 参数
        config.right_click = param;

        // 将修改后的配置写回 JSON 文件
        let new_data = serde_json::to_string_pretty(&config).expect("Unable to serialize config");
        fs::write(file_path, new_data).expect("Unable to write file");

        // 调用 Python 脚本
        let output = Command::new("python")
            .arg("your_script.py")
            .output()
            .expect("Failed to execute command");

        // 打印 Python 脚本的输出
        println!("Output: {:?}", output);
    }
}
