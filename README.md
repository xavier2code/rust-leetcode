# Rust LeetCode Solutions

![Rust Tests](https://github.com/xavier2code/rust-leetcode/actions/workflows/rust.yml/badge.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)
![Contributions Welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg)
[![codecov](https://codecov.io/gh/xavier2code/rust-leetcode/graph/badge.svg?token=7X4GKYQ3ME)](https://codecov.io/gh/xavier2code/rust-leetcode)
## 项目简介
这是一个用 Rust 编程语言解决 [LeetCode](https://leetcode.com/) 算法题的项目。项目按照题目类型（如数组、字符串、树等）进行模块化管理，每个文件对应一个具体的题目解决方案。

## 项目目的
- **学习与实践 Rust 编程**：通过解决算法问题，深入学习 Rust 的语法、特性和最佳实践。
- **提升算法与数据结构能力**：通过 LeetCode 的题目，巩固算法与数据结构知识。
- **构建可复用的代码库**：将常见的算法问题和解决方案整理成模块化的代码库，方便复用和参考。

## 项目用户
- **编程学习者**：希望通过 Rust 学习算法与数据结构的开发者。
- **面试准备者**：正在准备技术面试，尤其是需要解决算法问题的求职者。
- **Rust 爱好者**：希望通过实际项目提升 Rust 编程能力的开发者。

## 项目结构
项目按照题目类型分为不同的模块，每个模块包含与该类型相关的题目解决方案。以下是项目的目录结构示例：

```plaintext
rust-leetcode
├── src
│   ├── main.rs                # 项目入口文件
│   ├── array                  # 数组相关题目模块
│   │   ├── [problem_3873_maximum_value_of_ordered_triplet.rs](https://github.com/xavier2code/rust-leetcode/blob/main/src/array/problem_3873_maximum_value_of_ordered_triplet.rs)
│   │   └── mod.rs
│   ├── string                 # 字符串相关题目模块
│   │   └── mod.rs
│   ├── tree                   # 树相关题目模块
│   │   └── mod.rs
│   └── mod.rs                 # 顶层模块声明
├── [Cargo.toml](https://github.com/xavier2code/rust-leetcode/blob/main/Cargo.toml)                  # Rust 项目配置文件
└── README.md                  # 项目说明文件
