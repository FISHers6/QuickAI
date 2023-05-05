
# QuickAI - 快捷AI ⚡

[![GitHub license](https://img.shields.io/github/license/FISHers6/QuickAI?color=%238888FF)](https://github.com/FISHers6/QuickAI/blob/master/LICENSE) [![GitHub stars](https://img.shields.io/github/stars/FISHers6/QuickAI?color=yellow&style=social)](https://github.com/FISHers6/QuickAI/stargazers) [![GitHub issues](https://img.shields.io/github/issues/FISHers6/QuickAI?color=red)](https://github.com/FISHers6/QuickAI/issues) ![GitHub last commit](https://img.shields.io/github/last-commit/FISHers6/QuickAI?color=brightgreen)

- QuickAI是一款基于OpenAI开发的电脑版ChatGPT PC跨平台应用程序，它可以提供划词提问、快捷搜索、对话模式和快捷指令等多种功能。旨在为用户提供更便捷、更智能的PC桌面版AI使用体验。在使用时无需下载额外软件，适用于各种场景，如代码自动编辑、文件自动输入、提问、聊天软件自动回复等。
- 项目使用Rust Tauri、Typescript Vue3进行开发；支持Windows、MacOS和Linux三种主流操作系统，并在Web端提供部分功能的支持。通过Rust Tauri框架，QuickAI实现了跨平台设计，同时保证Web应用与PC客户端之间的互联性。

## 功能介绍 📝

- 自动输入：QuickAI提供自动输入功能，您可在代码编辑器、Word、PPT、聊天软件等场景中使用ChatGPT完成自动输入，无需额外下载任何软件。
- 划词提问：通过选中屏幕上要查询的文字, 并选择划词提示框，即能通过ChatGPT获取该问题的答案。
- 快捷搜索：QuickAI支持文件、图像（未来开放）、文本搜索，您可以设置快捷键来快速向ChatGPT查询。
- 对话模式：与ChatGPT进行同样的聊天体验，支持上下文对话并具有PDF、TEXT文件解析对话能力。
- 其他：支持主题更换、自定义代理、自定义Open API Key、提供Prompt快捷指令模板。

## 开发 🚀

1. 克隆本仓库：

```sh
git clone https://github.com/FISHers6/QuickAI.git
```

2. 安装依赖

```sh
cd QuickAI/
yarn install
cargo install tauri-cli // 如果已经安装tauri-cli, 请忽略此步骤
```

3. 运行应用程序

```sh
cargo tauri dev
```

## 贡献指南 🤝

如果您对该项目感兴趣并愿意为其做出贡献，欢迎提交pull request或者issue。

## 开源许可 📜

本项目基于[GPT许可证](https://github.com/FISHers6/QuickAI/blob/master/LICENSE)。
