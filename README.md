# RTD 📖

RTD (i.e. RustyToDo) 是一个用 Rust 编写的简单命令行TODO工具。

## 功能特性 ✴️

- 可以创建多个任务列表
- 友好的命令行界面
- 使用json文件存储

## 安装 ⚙️

确保你已经安装了[Rust](https://www.rust-lang.org/tools/install)。

```bash
$ git clone https://github.com/ikka0426/rtd.git
$ cd rtd
$ cargo build --release
```

## 使用 💻

### 选择任务列表

```bash
./rtd use <TODOLIST>
```

### 添加任务

```bash
./rtd add <EVENT>
```

### 修改任务完成情况

```bash
./rtd ch <ID> -<c|n>
```

使用以下命令来了解更多使用方法

```bash
./rtd help
```

## 贡献 👥

如果你想为 RTD 做贡献，欢迎提出问题（issues）或发起合并请求（pull requests）.

## 版权和许可证 📝

RTD 是基于 MIT 许可证发布的开源项目。详细信息请查阅 LICENSE 文件.

## 作者

<a href="https://github.com/ikka0426"><img alt="Static Badge" src="https://img.shields.io/badge/github-ikka0426-green"></a>
