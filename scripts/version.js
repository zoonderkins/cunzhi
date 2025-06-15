#!/usr/bin/env node

/**
 * 版本管理工具
 * 用于同步更新所有配置文件中的版本号
 */

import fs from 'node:fs'
import path from 'node:path'
import process from 'node:process'

// 需要更新版本号的文件配置
const VERSION_FILES = [
  {
    file: 'version.json',
    pattern: /"version"\s*:\s*"[^"]*"/,
    replacement: version => `"version": "${version}"`,
    updateDate: true,
  },
  {
    file: 'package.json',
    pattern: /"version"\s*:\s*"[^"]*"/,
    replacement: version => `"version": "${version}"`,
  },
  {
    file: 'Cargo.toml',
    pattern: /^version = "[^"]*"/m,
    replacement: version => `version = "${version}"`,
  },
  {
    file: 'tauri.conf.json',
    pattern: /"version"\s*:\s*"[^"]*"/,
    replacement: version => `"version": "${version}"`,
  },
]

// 注意：Rust 源文件中的版本号使用 env!("CARGO_PKG_VERSION") 宏
// 会自动从 Cargo.toml 读取，无需单独更新

/**
 * 读取当前版本号
 */
function getCurrentVersion() {
  try {
    const versionFile = path.join(process.cwd(), 'version.json')
    if (fs.existsSync(versionFile)) {
      const content = JSON.parse(fs.readFileSync(versionFile, 'utf8'))
      return content.version
    }

    const packageFile = path.join(process.cwd(), 'package.json')
    if (fs.existsSync(packageFile)) {
      const content = JSON.parse(fs.readFileSync(packageFile, 'utf8'))
      return content.version
    }

    return '0.0.0'
  }
  catch (error) {
    console.error('读取当前版本失败:', error.message)
    return '0.0.0'
  }
}

/**
 * 验证版本号格式
 */
function validateVersion(version) {
  const versionRegex = /^\d+\.\d+\.\d+$/
  return versionRegex.test(version)
}

/**
 * 更新单个文件的版本号
 */
function updateFileVersion(fileConfig, newVersion) {
  const filePath = path.join(process.cwd(), fileConfig.file)

  if (!fs.existsSync(filePath)) {
    console.log(`文件不存在，跳过: ${fileConfig.file}`)
    return false
  }

  try {
    const content = fs.readFileSync(filePath, 'utf8')
    let newContent = content.replace(fileConfig.pattern, fileConfig.replacement(newVersion))

    // 如果需要更新日期
    if (fileConfig.updateDate) {
      const currentDate = new Date().toISOString().split('T')[0] // YYYY-MM-DD格式
      newContent = newContent.replace(/"build_date"\s*:\s*"[^"]*"/, `"build_date": "${currentDate}"`)
    }

    if (content === newContent) {
      console.log(`未找到版本号模式，跳过: ${fileConfig.file}`)
      return false
    }

    fs.writeFileSync(filePath, newContent, 'utf8')
    console.log(`已更新: ${fileConfig.file}`)
    return true
  }
  catch (error) {
    console.error(`更新失败 ${fileConfig.file}:`, error.message)
    return false
  }
}

/**
 * 更新所有文件的版本号
 */
function updateAllVersions(newVersion) {
  console.log(`更新版本号到: ${newVersion}`)
  console.log('='.repeat(40))

  let successCount = 0

  for (const fileConfig of VERSION_FILES) {
    if (updateFileVersion(fileConfig, newVersion)) {
      successCount++
    }
  }

  console.log('='.repeat(40))
  console.log(`成功更新 ${successCount}/${VERSION_FILES.length} 个文件`)

  return successCount > 0
}

/**
 * 主函数
 */
function main() {
  const args = process.argv.slice(2)

  if (args.length === 0) {
    console.log('版本管理工具')
    console.log('')
    console.log('用法:')
    console.log('  node scripts/version.js <新版本号>')
    console.log('  node scripts/version.js --current')
    console.log('')
    console.log('示例:')
    console.log('  node scripts/version.js 1.2.3')
    console.log('  node scripts/version.js --current')
    process.exit(0)
  }

  if (args[0] === '--current') {
    const currentVersion = getCurrentVersion()
    console.log(`当前版本: ${currentVersion}`)
    process.exit(0)
  }

  const newVersion = args[0]

  if (!validateVersion(newVersion)) {
    console.error('版本号格式无效，请使用 x.y.z 格式')
    process.exit(1)
  }

  const currentVersion = getCurrentVersion()
  console.log(`当前版本: ${currentVersion}`)
  console.log(`新版本: ${newVersion}`)
  console.log('')

  if (updateAllVersions(newVersion)) {
    console.log('')
    console.log('版本号更新完成！')
  }
  else {
    console.error('版本号更新失败')
    process.exit(1)
  }
}

// 运行主函数
if (import.meta.url === `file://${process.argv[1]}`) {
  main()
}

export {
  getCurrentVersion,
  updateAllVersions,
  validateVersion,
}
