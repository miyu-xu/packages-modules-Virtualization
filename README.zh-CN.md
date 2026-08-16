# Android Virtualization 模块

[English](README.md) | 简体中文

本仓库提供 BSCP 的 Microdroid 控制面：`vm`、`virtmgr`、libvmclient、载荷配置、实例存储
和主机启动计划。Linux/KVM 是参考路径；macOS/HVF 与 Windows/WHPX 适配应保持相同的命令、
生命周期、错误和终止原因契约。

生产配置必须明确校验保护能力，不能在平台不支持时静默降级。构建请使用 BSCP 根仓库入口，
并运行对应平台的 Microdroid 回归脚本。完整 Android 兼容路径不替代本模块的发布门禁。
