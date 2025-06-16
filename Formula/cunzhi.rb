class Cunzhi < Formula
  desc "智能代码审查MCP工具集 - 包含寸止MCP服务器和等一下弹窗界面"
  homepage "https://github.com/imhuso/cunzhi"
  url "https://github.com/imhuso/cunzhi/archive/refs/tags/v0.2.0.tar.gz"
  sha256 "e38818a6f60c752a064f37b586b06d9fb06246b03b511a7b60f29dc205b7524c"
  license "MIT"
  version "0.2.0"

  depends_on "rust" => :build
  depends_on "node" => :build
  depends_on "pnpm" => :build

  def install
    # 构建前端资源
    system "pnpm", "install"
    system "pnpm", "build"
    
    # 构建Rust CLI工具
    system "cargo", "build", "--release"
    
    # 安装两个二进制文件
    bin.install "target/release/寸止"
    bin.install "target/release/等一下"
    
    # 安装文档
    doc.install "README.md"
    doc.install "INSTALL.md"
  end

  test do
    # 测试CLI工具是否正确安装
    assert_match "寸止", shell_output("#{bin}/寸止 --version")
    assert_match "寸止", shell_output("#{bin}/等一下 --version")
  end

  def caveats
    <<~EOS
      🎉 寸止 MCP 工具集安装完成！

      📋 使用方法：
        • MCP 服务器：寸止
        • 弹窗界面：等一下

      🔧 MCP 客户端配置：
        {
          "mcpServers": {
            "寸止": {
              "command": "寸止"
            }
          }
        }

      📚 更多信息：https://github.com/imhuso/cunzhi
    EOS
  end
end
