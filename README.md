# Focus Timer

一款用 Rust 编写的简洁 Windows 桌面番茄钟。浅色界面提供倒计时进度环、专注/休息切换、开始/暂停、重置、自定义时长和已完成次数。

到时间后，程序自动切换到下一阶段并暂停，点击 **Start focus** 可继续。设置和计数目前只在程序运行期间保留。

## Run

安装 [Rust 工具链](https://rustup.rs/) 后运行：

```powershell
cargo run --release
```

构建独立可执行文件：

```powershell
cargo build --release
```

可执行文件位于 `target\\release\\focus_timer.exe`。运行程序不需要 GitHub 账户。

## Test

```powershell
cargo test
```

运行上述命令可验证倒计时、阶段切换与重置逻辑。
