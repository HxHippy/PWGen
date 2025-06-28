import React from 'react';
import Layout from '@theme/Layout';

export default function Download() {
  return (
    <Layout title="Download PwGen" description="Download PwGen for your platform">
      <div className="container margin-vert--lg">
        <div className="row">
          <div className="col col--8 col--offset-2">
            <h1>📥 Download PwGen</h1>
            <p className="text--center margin-bottom--lg">
              Choose your platform and get started with the most secure password manager built in Rust.
            </p>

            {/* Latest Release Info */}
            <div className="card margin-bottom--lg">
              <div className="card__header">
                <h3>📦 Latest Release: v1.2.0</h3>
              </div>
              <div className="card__body">
                <p>
                  <strong>Release Date:</strong> June 2025<br/>
                  <strong>What's New:</strong> 30-40% smaller binaries, enhanced security, flexible builds, modern cryptography
                </p>
              </div>
            </div>

            {/* Platform Downloads */}
            <div className="row margin-bottom--lg">
              <div className="col col--4">
                <div className="card">
                  <div className="card__header text--center">
                    <h3>🐧 Linux</h3>
                  </div>
                  <div className="card__body text--center">
                    <p>Ubuntu, Debian, Fedora, Arch</p>
                    <div className="download-buttons">
                      <a href="/downloads/pwgen-linux-x64.tar.gz" className="download-button">
                        📦 Download .tar.gz
                      </a>
                      <a href="/downloads/pwgen-linux-x64.deb" className="download-button">
                        📦 Download .deb
                      </a>
                      <a href="/downloads/pwgen-linux-x64.rpm" className="download-button">
                        📦 Download .rpm
                      </a>
                    </div>
                  </div>
                </div>
              </div>

              <div className="col col--4">
                <div className="card">
                  <div className="card__header text--center">
                    <h3>🍎 macOS</h3>
                  </div>
                  <div className="card__body text--center">
                    <p>macOS 11.0 or later</p>
                    <div className="download-buttons">
                      <a href="/downloads/pwgen-macos-universal.dmg" className="download-button">
                        📦 Download .dmg
                      </a>
                      <a href="/downloads/pwgen-macos-universal.pkg" className="download-button">
                        📦 Download .pkg
                      </a>
                    </div>
                  </div>
                </div>
              </div>

              <div className="col col--4">
                <div className="card">
                  <div className="card__header text--center">
                    <h3>🪟 Windows</h3>
                  </div>
                  <div className="card__body text--center">
                    <p>Windows 10 or later</p>
                    <div className="download-buttons">
                      <a href="/downloads/pwgen-windows-x64.msi" className="download-button">
                        📦 Download .msi
                      </a>
                      <a href="/downloads/pwgen-windows-x64.exe" className="download-button">
                        📦 Download .exe
                      </a>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            {/* Alternative Installation Methods */}
            <div className="card margin-bottom--lg">
              <div className="card__header">
                <h3>🔧 Alternative Installation Methods</h3>
              </div>
              <div className="card__body">
                <h4>📦 Package Managers</h4>
                <div className="code-block-title">Cargo (Rust)</div>
                <pre><code>cargo install pwgen-cli pwgen-gui</code></pre>
                
                <div className="code-block-title">Homebrew (macOS/Linux)</div>
                <pre><code>brew tap hxhippy/pwgen<br/>brew install pwgen</code></pre>
                
                <div className="code-block-title">Scoop (Windows)</div>
                <pre><code>scoop bucket add your-bucket https://github.com/hxhippy/scoop-bucket<br/>scoop install pwgen</code></pre>

                <h4>🚀 Quick Install Scripts</h4>
                <div className="code-block-title">Linux/macOS</div>
                <pre><code>curl -sSL https://pwgenrust.dev/install.sh | bash</code></pre>
                
                <div className="code-block-title">Windows (PowerShell)</div>
                <pre><code>irm https://pwgenrust.dev/install.ps1 | iex</code></pre>
              </div>
            </div>

            {/* System Requirements */}
            <div className="card margin-bottom--lg">
              <div className="card__header">
                <h3>💻 System Requirements</h3>
              </div>
              <div className="card__body">
                <div className="row">
                  <div className="col col--4">
                    <h4>🐧 Linux</h4>
                    <ul>
                      <li>x86_64 architecture</li>
                      <li>glibc 2.17+ or musl</li>
                      <li>50 MB disk space</li>
                      <li>GTK 3.0+ (for GUI)</li>
                    </ul>
                  </div>
                  <div className="col col--4">
                    <h4>🍎 macOS</h4>
                    <ul>
                      <li>macOS 11.0 or later</li>
                      <li>Intel or Apple Silicon</li>
                      <li>50 MB disk space</li>
                      <li>Cocoa framework</li>
                    </ul>
                  </div>
                  <div className="col col--4">
                    <h4>🪟 Windows</h4>
                    <ul>
                      <li>Windows 10 or later</li>
                      <li>x86_64 architecture</li>
                      <li>50 MB disk space</li>
                      <li>Visual C++ Redistributable</li>
                    </ul>
                  </div>
                </div>
              </div>
            </div>

            {/* Verification */}
            <div className="card margin-bottom--lg">
              <div className="card__header">
                <h3>🔐 Download Verification</h3>
              </div>
              <div className="card__body">
                <p>All downloads are signed and include SHA256 checksums for verification:</p>
                <div className="code-block-title">Verify Download</div>
                <pre><code># Download checksums<br/>curl -O https://pwgenrust.dev/downloads/SHA256SUMS<br/>curl -O https://pwgenrust.dev/downloads/SHA256SUMS.sig<br/><br/># Verify signature<br/>gpg --verify SHA256SUMS.sig SHA256SUMS<br/><br/># Check file integrity<br/>sha256sum -c SHA256SUMS</code></pre>
                
                <p><strong>PGP Key:</strong> <code>0x1234567890ABCDEF</code></p>
                <p><a href="/downloads/pubkey.asc">Download Public Key</a></p>
              </div>
            </div>

            {/* Getting Started */}
            <div className="text--center">
              <h2>🚀 Ready to get started?</h2>
              <p>After installation, check out our getting started guide:</p>
              <a href="/docs/getting-started" className="download-button download-button--primary">
                📚 Getting Started Guide
              </a>
            </div>
          </div>
        </div>
      </div>
    </Layout>
  );
}