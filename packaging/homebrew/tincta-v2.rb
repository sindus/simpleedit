class TinctaV2 < Formula
  desc "A fast, stable, cross-platform text editor with syntax highlighting"
  homepage "https://sindus.github.io/tincta-V2"
  version "0.1.0"

  on_macos do
    if Hardware::CPU.arm?
      url "https://github.com/sindus/tincta-V2/releases/download/v#{version}/tincta-v#{version}-aarch64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_SHA256_AARCH64"
    else
      url "https://github.com/sindus/tincta-V2/releases/download/v#{version}/tincta-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "REPLACE_WITH_SHA256_X86_64"
    end
  end

  def install
    bin.install "tincta"
  end

  test do
    assert_match "tincta", shell_output("#{bin}/tincta --version 2>&1", 0).downcase
  end
end
