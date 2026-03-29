#!/bin/bash

# 纯粹直播 - 一键推送到 GitHub 脚本

echo "🚀 纯粹直播 Rust 版 - 推送到 GitHub"
echo "===================================="
echo ""

# 检查 Git 配置
echo "📋 检查 Git 配置..."
GIT_NAME=$(git config user.name)
GIT_EMAIL=$(git config user.email)

if [ -z "$GIT_NAME" ] || [ -z "$GIT_EMAIL" ]; then
    echo "❌ 请配置 Git 用户信息"
    read -p "请输入您的姓名： " NAME
    read -p "请输入您的邮箱： " EMAIL
    git config --global user.name "$NAME"
    git config --global user.email "$EMAIL"
else
    echo "✅ Git 用户：$GIT_NAME <$GIT_EMAIL>"
fi

echo ""
echo "📝 创建 GitHub 仓库步骤："
echo "========================"
echo ""
echo "1. 访问：https://github.com/new"
echo "2. 仓库名称：pure_live_rust"
echo "3. 描述：纯粹直播 Rust 版 - 高性能直播聚合播放器"
echo "4. 设为 Public 或 Private"
echo "5. 不要勾选 'Add a README file'"
echo "6. 点击 'Create repository'"
echo ""

read -p "按回车键继续..."

echo ""
echo "🔗 添加远程仓库地址"
echo "=================="
echo ""
echo "请选择仓库可见性："
echo "1. Public (公开，其他人可以看到)"
echo "2. Private (私有，只有您可以看到)"
echo ""

read -p "请选择 (1/2): " VISIBILITY

if [ "$VISIBILITY" = "1" ]; then
    VISIBILITY_TEXT="Public"
else
    VISIBILITY_TEXT="Private"
fi

echo ""
echo "您选择了：$VISIBILITY_TEXT"
echo ""
echo "⚠️  注意：GitHub 用户名需要在下一步输入"
echo ""

read -p "请输入您的 GitHub 用户名： " GITHUB_USERNAME

if [ -z "$GITHUB_USERNAME" ]; then
    echo "❌ GitHub 用户名不能为空"
    exit 1
fi

REMOTE_URL="https://github.com/${GITHUB_USERNAME}/pure_live_rust.git"

echo ""
echo "📤 准备推送到：$REMOTE_URL"
echo ""

# 检查是否已有 remote
EXISTING_REMOTE=$(git remote | grep origin)

if [ -n "$EXISTING_REMOTE" ]; then
    echo "⚠️  已存在 remote 'origin'，是否覆盖？"
    read -p "覆盖 remote (y/n): " OVERWRITE
    if [ "$OVERWRITE" = "y" ]; then
        git remote remove origin
        echo "✅ 已删除原有 remote"
    else
        echo "❌ 取消操作"
        exit 0
    fi
fi

# 添加 remote
echo "🔗 添加远程仓库..."
git remote add origin "$REMOTE_URL"

if [ $? -eq 0 ]; then
    echo "✅ 远程仓库添加成功"
else
    echo "❌ 添加远程仓库失败"
    exit 1
fi

echo ""
echo "📤 开始推送代码..."
echo ""

# 推送代码
git push -u origin main

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ 推送成功！"
    echo ""
    echo "🎉 下一步操作："
    echo "=============="
    echo ""
    echo "1. 访问您的仓库："
    echo "   $REMOTE_URL"
    echo ""
    echo "2. 点击 'Actions' 标签"
    echo "3. 如果是第一次，点击 'Enable workflows'"
    echo "4. 等待 Android APK 构建完成（约 15-20 分钟）"
    echo "5. 在 Actions -> Artifacts 下载 APK"
    echo ""
    echo "📱 构建完成后，您将得到："
    echo "   - pure-live-rust-android-arm64.apk (真机)"
    echo "   - pure-live-rust-android-x86_64.apk (模拟器)"
    echo ""
else
    echo ""
    echo "❌ 推送失败"
    echo ""
    echo "可能的原因："
    echo "1. 仓库尚未创建 - 请先在 GitHub 创建仓库"
    echo "2. 认证失败 - 需要使用 Personal Access Token"
    echo "3. 网络问题 - 请检查网络连接"
    echo ""
    echo "💡 解决方法："
    echo ""
    echo "1. 创建 Personal Access Token："
    echo "   访问 https://github.com/settings/tokens"
    echo "   创建新 token，勾选 'repo' 权限"
    echo ""
    echo "2. 使用 token 推送："
    echo "   git push -u origin main"
    echo "   用户名：您的 GitHub 用户名"
    echo "   密码：刚才创建的 token"
    echo ""
fi
