<p align="center">
<img src="https://raw.githubusercontent.com/SirCesarium/sweet/main/editors/vscode/icon.png" width="128" alt="Sweet Icon">
</p>

<h1 align="center">🍬 Sweet for Visual Studio Code</h1>

<p align="center">
<strong>Real-time code health alerts for professional developers.</strong>
</p>

`Sweet` for VSCode brings the power of the **Sweet Index** directly into your editor. It alerts at the moment your code starts becoming "Bitter" and hard to maintain.

-----

## 🍬 Features

  - **File Size Guardian:** Get immediate warnings when a file exceeds your defined line limit. Keep your modules focused and atomic.

  - **Import Overload Detection:** Automatically identifies when a file has too many dependencies, signaling a potential violation of the Single Responsibility Principle.

  - **Deep Integration:** Fully compatible with your project's `.swtrc` configuration. It respects your hierarchical rules and custom thresholds.

  - **Lightweight:** Built in Rust-powered logic to ensure zero lag in your typing experience.

-----

## 🍭 How it works

The extension monitors your active editor and triggers **VSCode Warnings** (yellow squiggles) when:

1.  The **Line Count** exceeds your threshold (Default: 250 lines).
2.  The **Import Count** is too high for the specific language (e.g., \>15 imports in Rust).

> [!NOTE]  
> This extension focuses on **real-time structural health**. For deep metrics like **code duplication (Copy-Paste detection)** and global project analysis, please use the [Sweet CLI](https://github.com/SirCesarium/sweet).

-----

## 🍬 Global Metrics & Duplication

The VSCode extension is designed for *local* file health. To get the full picture of your project's technical debt, use `swt` in your terminal or CI:

```bash
# Detect duplicated code across the entire project
swt . --inspect
```

**Want to automate this?** Check out [Refinery-RS](https://github.com/SirCesarium/refinery-rs) to integrate these metrics directly into your GitHub Pull Requests.

-----

## ⚙️ Configuration

`Sweet` will automatically look for a `.swtrc` file in your workspace root. If none is found, it uses these sensible defaults:

```json
{
  "thresholds": {
    "global": { 
      "max_lines": 250
    },
    "overrides": {
      "rust": { "max_imports": 15 },
      "typescript": { "max_imports": 20 }
    }
  }
}
```

-----

## 🤝 Contributing

This extension is part of the [Sweet Ecosystem](https://github.com/SirCesarium/sweet). If you want to improve the VSCode integration or add support for more languages, feel free to open an issue or PR in the main repository.

**Happy coding\! Stay Sweet. 🍭**
