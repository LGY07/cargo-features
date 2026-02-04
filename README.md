# cargo-features
缓解错误使用 features 的 crate 导致的头疼

## 安装

```bash
git clone https://github.com/LGY07/cargo-features.git && cd cargo-features/
cargo build --release
cp target/release/cargo-features ~/.cargo/bin/ # 检查一下路径对不对
```

## 使用

```bash
cargo features
```
```bash
Package cargo-features v0.1.0
        Dependency colored ^3.1
                Features:
        Dependency serde ^1.0
                Features:
                + derive
                - alloc
                - default
                - rc
                - serde_derive
                - std
                - unstable
        Dependency serde_json ^1.0
                Features:
```
