# SITCON@GDSC – Review Tool

本工具可簡化 SITCON@GDSC 評審流程，並提供一致的評審標準。

## 用法

### 本地端 (編譯 & 執行)

```bash
cargo run --release
```

### 瀏覽器 (serve)

```bash
cargo install trunk
trunk build --release
bun x serve dist
```

## 開發

### 本地端 (除錯執行)

```bash
cargo run
```

### 瀏覽器 (除錯 serve)

```bash
cargo install trunk
trunk serve
```

## 程式碼結構

本程式已經過大量抽象，可輕鬆支援 SITCON@GDSC 以外的審稿模式。

```plain
*
 `-- lib (通用演算法)
   `-- types (通用資料結構)
     `-- (本層主要是任何稿件都應該有的結構
          以及方便存取稿件的資料結構 (SortedHashMap))
     `-- rank (評分模式)
       `-- sitcon_gdsc (SITCON@GDSC 之細項評分及 Pan 的評分演算法)
     `-- deserialize (支援任何可以反序列化成 Manuscript 的 CSV 格式)
       `-- sitcon_gdsc (SITCON@GDSC CSV 原始檔對應欄位格式)
   `-- ui (通用前端)
     `-- (內含 main page 框架)
     `-- components
       `-- (各種 UI 元件)
       `-- rank: 內含依賴 types::rank 的 specialized implementation，所以要加入其他評分模式也需要更動這塊。
     `-- fonts
       `-- (字型，儲存在 Git LFS)
     `-- state (右上角程式狀態)
 `-- main (含 GUI 的主程式)
   `-- (Native GUI)
   `-- wasm (Web GUI)
```

## 特色

- 所有方法都有利用 Specialize (特化) 來製造 fallback。
  你的 Rank 等程式碼可以只實作一小部分，剩下的部分程式會
  幫你製造 Fallback。
- 所有和 SITCON@GDSC 這個特定活動相關的資料結構均已抽象至
  `sitcon_gdsc` 模組，你可以輕易抽走或加入屬於自己的模組，
  並且只需在 `main.rs` 和 `wasm.rs` 一處更新你的資料結構即可。
- 整體程式碼結構雖有些複雜，但基本易於擴充、容易掌握。
  不過需要先有 Rust 先備知識才能在開發上更隨心應手。
- 任何問題都可以 contact 作者來討論。

## License

MIT License
