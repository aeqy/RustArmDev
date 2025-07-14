# RustArmDev
rust_arm_dev

# RustArmDev

RustArmDev 是一个基于 Rust 的 ARM Cortex-M（如 M4/M7）嵌入式开发模板项目，适合初学者快速上手 Rust 交叉编译与嵌入式开发。

## 项目特点
- 支持 `thumbv7em-none-eabihf` 目标（Cortex-M4/M7 等）
- 适合裸机/RTOS 嵌入式开发
- 结构清晰，易于扩展

## 环境要求
- Rust 工具链（推荐使用 [rustup](https://rustup.rs/) 安装）
- 交叉编译目标：`thumbv7em-none-eabihf`
- 推荐平台：macOS、Linux、Windows

## 快速开始

### 1. 安装 Rust 及目标支持
```sh
rustup target add thumbv7em-none-eabihf
```

### 2. 克隆本项目
```sh
git clone https://github.com/aeqy/RustArmDev.git
cd RustArmDev
```

### 3. 编译
```sh
cargo build
```

### 4. 运行/烧录/仿真
- 生成的二进制文件位于 `target/thumbv7em-none-eabihf/debug/` 目录下。
- 不能直接在 PC 上运行！
- 可通过 JLink/OpenOCD 烧录到开发板，或用 QEMU 仿真。

#### QEMU 仿真（可选）
```sh
# 需安装 qemu-system-arm
qemu-system-arm -M stm32-p103 -kernel target/thumbv7em-none-eabihf/debug/rust_arm_dev
```

#### 烧录到开发板（以 OpenOCD 为例）
```sh
openocd -f interface/jlink.cfg -f target/stm32f4x.cfg
# 另开终端
arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/rust_arm_dev
(gdb) target remote localhost:3333
(gdb) load
```

## 目录结构说明
- `src/`：主代码目录
- `memory.x`：链接脚本
- `.cargo/config.toml`：交叉编译配置
- `build.rs`：自定义构建脚本（如有）

## 贡献
欢迎提交 issue 和 PR，一起完善 Rust 嵌入式开发生态！

## 许可证
[Apache-2.0](LICENSE)
