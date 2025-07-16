#!/bin/bash

# 交互式发布脚本

set -e

# --- crates.io 发布 ---

echo "--- crates.io 发布 ---"

# 检查 Cargo.toml
echo "检查 Cargo.toml..."
if ! grep -q '^name = "geosot"' Cargo.toml || \
   ! grep -q '^version = ' Cargo.toml || \
   ! grep -q '^authors = ' Cargo.toml || \
   ! grep -q '^license = ' Cargo.toml || \
   ! grep -q '^description = ' Cargo.toml; then
    echo "错误：Cargo.toml 中的某些字段缺失或不正确。"
    echo "请确保 name, version, authors, license, 和 description 都已设置。"
    exit 1
fi

# 登录 crates.io
echo "请从 https://crates.io/settings/tokens 获取您的 API 令牌。"
read -p "请输入您的 crates.io API 令牌: " api_token
cargo login $api_token

# 发布到 crates.io
echo "准备发布到 crates.io..."
read -p "您确定要发布吗？(y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    cargo publish
    echo "成功发布到 crates.io！"
else
    echo "已取消发布到 crates.io。"
fi


# --- PyPI 发布 ---

echo -e "\n--- PyPI 发布 ---"

# 检查 pyproject.toml
echo "检查 pyproject.toml..."
if ! grep -q '^name = "geosot"' pyproject.toml || \
   ! grep -q '^version = ' pyproject.toml; then
    echo "错误：pyproject.toml 中的 name 或 version 缺失。"
    exit 1
fi

# 构建 wheel
echo "构建 wheel..."
maturin build --release

# 上传到 PyPI
echo "准备上传到 PyPI..."
echo "您需要一个 PyPI 帐户和 API 令牌。"
read -p "您确定要上传吗？(y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    pip install twine
    twine upload target/wheels/*
    echo "成功上传到 PyPI！"
else
    echo "已取消上传到 PyPI。"
fi

echo -e "\n发布完成！"
