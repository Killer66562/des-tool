# DES Tool

## Structure

```
.
├── assets
│   └── styles
│       ├── input.css
│       └── output.css
├── Cargo.lock
├── Cargo.toml
├── .github
│   └── workflows
│       └── github-pages.yml
├── .gitignore
├── index.html
├── package.json
├── package-lock.json
└── src
    ├── components
    │   ├── encryption_form.rs
    │   ├── hex_input.rs
    │   ├── mod.rs
    │   ├── result_form.rs
    │   ├── round_outputs_table.rs
    │   └── subkeys_table.rs
    ├── des //演算法核心
    │   ├── encrypt_decrypt.rs //加密解密函數
    │   ├── key.rs //金鑰處理
    │   ├── mod.rs //模組入口
    │   ├── permute.rs //排列處理
    │   ├── round.rs //輪次處理
    │   └── states.rs //狀態定義
    ├── layouts
    │   ├── default.rs
    │   └── mod.rs
    ├── lib.rs
    ├── main.rs
    └── pages
        ├── home_page.rs
        └── mod.rs
```

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/Killer66562/des-tool.git
   ```
2. Navigate to the project directory:
   ```
   cd des-tool
   ```
3. Install dependencies:
   ```
   cargo install trunk
   rustup target add wasm32-unknown-unknown
   ```
4. Run the development server:
   ```
   trunk serve
   ```

## Links

- [安裝Rust](https://rust-lang.org/zh-TW/tools/install/)
- [GitHub Repository](https://github.com/Killer66562/des-tool)
- [Live Demo](https://killer66562.github.io/des-tool)
