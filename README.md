# 🍬 swt (Sweet)

**Is your code sweet or bitter?**

`swt` is a blazing-fast, language-agnostic tool designed to monitor architectural health and code density. It identifies "God Files", deep nesting, and tangled dependencies, providing a clear metric of your project's maintainability.

## Core Metrics

`swt` evaluates your project based on the **Glucose Index**:

- **Coupling:** Excessive top-level imports and external dependencies.
- **Responsibility:** Multi-purpose files with high implementation density.
- **Cognitive Load:** Deep nesting and cyclomatic complexity patterns.

## Installation

Install the pre-compiled binary via Cargo:

```bash
cargo install swt
```

## Quick Start

Run a health check on your current directory:

```bash
swt check
```

Generate a summary of the most "bitter" files:

```bash
swt scan --top 10
```

## License

This project is licensed under the MIT License.
