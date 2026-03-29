# 🚀 立即推送到 GitHub

## 方式 1: 自动化脚本（推荐）

### 运行推送脚本

```bash
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust
./push-to-github.sh
```

脚本会自动：
- ✅ 检查 Git 配置
- ✅ 引导创建 GitHub 仓库
- ✅ 添加远程仓库
- ✅ 推送代码

---

## 方式 2: 手动推送（分步执行）

### 步骤 1: 创建 GitHub 仓库

**立即访问**: https://github.com/new

填写以下信息：
- **Repository name**: `pure_live_rust`
- **Description**: `纯粹直播 Rust 版 - 高性能直播聚合播放器`
- **Public** (推荐) 或 **Private**
- ❌ **不要** 勾选 "Add a README file"
- ❌ **不要** 勾选 ".gitignore"
- ❌ **不要** 选择 License

点击 **"Create repository"**

### 步骤 2: 复制仓库地址

创建完成后，您会看到：
```
https://github.com/YOUR_USERNAME/pure_live_rust.git
```

### 步骤 3: 添加远程仓库

```bash
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust

# 替换 YOUR_USERNAME 为您的 GitHub 用户名
git remote add origin https://github.com/YOUR_USERNAME/pure_live_rust.git
```

### 步骤 4: 推送代码

```bash
git push -u origin main
```

**注意**: 
- 首次推送需要输入 GitHub 用户名和密码
- 密码位置输入 **Personal Access Token**
- 如果没有 token，请访问：https://github.com/settings/tokens

---

## 🔑 创建 Personal Access Token

### 步骤 1: 访问 Token 页面
https://github.com/settings/tokens

### 步骤 2: 生成新 token
1. 点击 "Generate new token (classic)"
2. 填写 Note: `pure_live_rust push`
3. 选择过期时间（建议 90 天）
4. **勾选权限**: `repo` (Full control of private repositories)
5. 点击 "Generate token"

### 步骤 3: 复制 token
- **立即复制**生成的 token（只显示一次！）
- 格式：`ghp_xxxxxxxxxxxx`

### 步骤 4: 使用 token 推送

```bash
git push -u origin main
# Username: YOUR_USERNAME
# Password: ghp_xxxxxxxxxxxx (粘贴 token)
```

---

## 📱 推送成功后

### 1. 访问仓库
```
https://github.com/YOUR_USERNAME/pure_live_rust
```

### 2. 启用 Actions
- 点击 **"Actions"** 标签
- 如果是第一次，点击 **"Enable workflows"**

### 3. 查看构建进度
- 工作流会自动开始运行
- 点击运行记录查看详细日志
- 等待 15-20 分钟

### 4. 下载 APK
- 构建完成后，点击最新的运行记录
- 在 **Artifacts** 部分下载 APK
- 解压后得到 `pure-live-rust.apk`

---

## ⚡ 快速命令（复制并修改）

```bash
# 进入项目目录
cd /Users/morgan/WorkSpace/03-项目/pure_live_rust

# 添加远程仓库（替换为您的 GitHub 用户名）
git remote add origin https://github.com/YOUR_USERNAME/pure_live_rust.git

# 推送代码
git push -u origin main
```

---

## 🐛 常见问题

### Q1: remote origin already exists

```bash
# 删除现有 remote
git remote remove origin

# 重新添加
git remote add origin https://github.com/YOUR_USERNAME/pure_live_rust.git
```

### Q2: Authentication failed

**解决**:
1. 创建 Personal Access Token（见上文）
2. 使用 token 作为密码
3. 或者更新凭据：
   ```bash
   git credential-osxkeychain erase
   # 然后重新推送
   ```

### Q3: failed to push some refs

```bash
# 先拉取远程更改
git pull --rebase origin main

# 再推送
git push -u origin main
```

### Q4: 403 Forbidden

**原因**: 仓库不存在或权限不足

**解决**:
1. 确认已在 GitHub 创建仓库
2. 检查仓库地址是否正确
3. 确认 token 有 repo 权限

---

## 📊 推送检查清单

- [ ] 已在 GitHub 创建仓库
- [ ] 仓库名称：`pure_live_rust`
- [ ] 已复制仓库地址
- [ ] 已添加 remote origin
- [ ] 已推送代码（git push -u origin main）
- [ ] 推送成功，看到仓库页面
- [ ] 已启用 GitHub Actions
- [ ] 构建已开始运行

---

## 🎯 下一步

推送成功后：

1. **访问仓库**: https://github.com/YOUR_USERNAME/pure_live_rust
2. **查看 Actions**: 点击 Actions 标签
3. **等待构建**: 约 15-20 分钟
4. **下载 APK**: 在 Artifacts 下载

---

## 📞 需要帮助？

如果遇到问题：

1. 查看错误信息
2. 检查网络连接
3. 确认 GitHub 账号正常
4. 查看 GitHub Status: https://www.githubstatus.com/

---

**准备好推送了吗？** 

选择方式 1（运行脚本）或方式 2（手动执行），立即开始！🚀
