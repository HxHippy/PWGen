import React, { useState, useEffect } from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import CodeBlock from '@theme/CodeBlock';

import styles from './index.module.css';

function detectOS() {
  const userAgent = window.navigator.userAgent;
  if (userAgent.indexOf('Win') !== -1) return 'windows';
  if (userAgent.indexOf('Mac') !== -1) return 'macos';
  if (userAgent.indexOf('Linux') !== -1) return 'linux';
  return 'linux'; // default
}

function getInstallCommand(os) {
  switch (os) {
    case 'windows':
      return {
        package: 'download',
        command: '# Download Windows installer\n# Visit: https://github.com/hxhippy/pwgen/releases/latest\n# Download: pwgen-windows-x64-installer.exe',
        alt: 'Scoop package coming soon: scoop install pwgen'
      };
    case 'macos':
      return {
        package: 'download',
        command: '# Download macOS installer\n# Visit: https://github.com/hxhippy/pwgen/releases/latest\n# Download: pwgen-1.2.0-macos-x64.pkg or .dmg',
        alt: 'Homebrew package coming soon: brew install pwgen'
      };
    case 'linux':
    default:
      return {
        package: 'snap',
        command: '# Install from Snap Store (Approved!)\nsudo snap install pwgen-rust',
        alt: 'Available now in the Snap Store! ✅'
      };
  }
}

function InstallCommand() {
  const [os, setOS] = useState('linux');
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    setOS(detectOS());
  }, []);

  const installInfo = getInstallCommand(os);

  const copyToClipboard = () => {
    navigator.clipboard.writeText(installInfo.command);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="install-section">
      <h3>🚀 Quick Install</h3>
      <div className="os-selector">
        <button 
          className={`os-button ${os === 'linux' ? 'active' : ''}`}
          onClick={() => setOS('linux')}>
          🐧 Linux
        </button>
        <button 
          className={`os-button ${os === 'macos' ? 'active' : ''}`}
          onClick={() => setOS('macos')}>
          🍎 macOS
        </button>
        <button 
          className={`os-button ${os === 'windows' ? 'active' : ''}`}
          onClick={() => setOS('windows')}>
          🪟 Windows
        </button>
      </div>
      <div className="install-command-container">
        <div className="install-command">
          <CodeBlock language="bash" title={`Install via ${installInfo.package}`}>
            {installInfo.command}
          </CodeBlock>
          <button className="copy-button" onClick={copyToClipboard}>
            {copied ? '✅ Copied!' : '📋 Copy'}
          </button>
        </div>
        <p className="install-alt">{installInfo.alt}</p>
      </div>
    </div>
  );
}

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <div className="hero-content">
          <div className="hero-logo">
            <img 
              src="/img/PWGenLogo-Wide.png" 
              alt="PwGen Logo" 
              className="hero-logo-image"
            />
          </div>
          <p className="hero__subtitle">{siteConfig.tagline}</p>
          
          <div className="download-buttons">
            <Link
              className="download-button download-button--primary"
              to="/download">
              🚀 Download Now
            </Link>
            <Link
              className="download-button"
              to="/docs/getting-started">
              📚 Get Started
            </Link>
            <Link
              className="download-button"
              href="https://github.com/hxhippy/pwgen">
              ⭐ GitHub
            </Link>
          </div>
          
          <InstallCommand />
          
          <div className="security-badges" style={{marginTop: '2rem'}}>
            <span className="security-badge">🔒 AES-256-GCM</span>
            <span className="security-badge">🦀 Memory Safe</span>
            <span className="security-badge">🚫 Zero Knowledge</span>
            <span className="security-badge">📱 Cross Platform</span>
            <span className="security-badge">⚡ 30-40% Smaller</span>
            <span className="security-badge">🔧 Flexible Builds</span>
          </div>
        </div>
      </div>
    </header>
  );
}

function StatsSection() {
  return (
    <section className="stats">
      <div className="container">
        <div className="row">
          <div className="col col--3">
            <div className="stats__item">
              <div className="stats__number">256-bit</div>
              <div className="stats__label">AES-GCM Encryption</div>
            </div>
          </div>
          <div className="col col--3">
            <div className="stats__item">
              <div className="stats__number">0ms</div>
              <div className="stats__label">Network Latency</div>
            </div>
          </div>
          <div className="col col--3">
            <div className="stats__item">
              <div className="stats__number">3</div>
              <div className="stats__label">Platforms Supported</div>
            </div>
          </div>
          <div className="col col--3">
            <div className="stats__item">
              <div className="stats__number">40%</div>
              <div className="stats__label">Size Reduction (v1.2)</div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

function ScreenshotsSection() {
  return (
    <section className="screenshots-section">
      <div className="container">
        <div className="row">
          <div className="col col--12">
            <h2 style={{textAlign: 'center', marginBottom: '3rem'}}>
              📸 See PwGen in Action
            </h2>
          </div>
        </div>
        <div className="row">
          <div className="col col--6">
            <div className="screenshot-card">
              <img 
                src="/img/screenshot-main.png" 
                alt="PwGen Main Interface"
                className="screenshot-image"
              />
              <h3>🔐 Password Management</h3>
              <p>Clean, intuitive interface for managing all your passwords and secrets with powerful search and organization features.</p>
            </div>
          </div>
          <div className="col col--6">
            <div className="screenshot-card">
              <img 
                src="/img/screenshot-generator.png" 
                alt="PwGen Password Generator"
                className="screenshot-image"
              />
              <h3>⚡ Smart Generator</h3>
              <p>Advanced password generator with customizable rules, entropy visualization, and compliance templates.</p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

function CodeExamplesSection() {
  return (
    <section className="code-examples-section">
      <div className="container">
        <div className="row">
          <div className="col col--12">
            <h2 style={{textAlign: 'center', marginBottom: '3rem'}}>
              💻 Developer-Friendly CLI
            </h2>
          </div>
        </div>
        <div className="row">
          <div className="col col--6">
            <h3>🔑 Generate Passwords</h3>
            <CodeBlock language="bash" title="Command Line Usage">
{`# Generate a secure password
pwgen-cli generate --length 32 --include-symbols

# Generate with custom rules
pwgen-cli generate --min-length 16 --max-length 24 \\
  --require-uppercase --require-numbers \\
  --exclude-ambiguous

# Save to vault
pwgen-cli add --name "GitHub Token" \\
  --username "hxhippy" \\
  --url "github.com" \\
  --generate-password`}
            </CodeBlock>
          </div>
          <div className="col col--6">
            <h3>🗝️ Manage Secrets</h3>
            <CodeBlock language="bash" title="Secrets Management">
{`# Add API key with expiration
pwgen-cli secrets add --name "AWS_ACCESS_KEY" \\
  --value "AKIAIOSFODNN7EXAMPLE" \\
  --expires "2025-12-31" \\
  --tags "aws,production"

# Import from environment
pwgen-cli env import --prefix "PROD_" \\
  --vault "production-secrets"

# Export for deployment
pwgen-cli export --format json \\
  --tags "deployment" > secrets.json`}
            </CodeBlock>
          </div>
        </div>
        <div className="row" style={{marginTop: '2rem'}}>
          <div className="col col--12">
            <div className="code-showcase">
              <h3>🦀 Built with Rust - Here's the Core Encryption</h3>
              <CodeBlock language="rust" title="src/crypto.rs">
{`use aes_gcm::{Aes256Gcm, Key, Nonce};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use sha2::{Digest, Sha256};

pub struct CryptoEngine {
    cipher: Aes256Gcm,
    hasher: Argon2<'static>,
}

impl CryptoEngine {
    pub fn encrypt_vault(&self, data: &[u8], password: &str) -> Result<Vec<u8>> {
        // Derive key using Argon2 (v1.2 enhancement)
        let salt = self.generate_salt();
        let key = self.derive_key(password, &salt)?;
        
        // Encrypt with AES-256-GCM
        let nonce = Nonce::from_slice(&self.generate_nonce());
        let ciphertext = self.cipher.encrypt(nonce, data)?;
        
        // Return salt + nonce + ciphertext
        Ok([salt, nonce.as_slice(), &ciphertext].concat())
    }
    
    pub fn verify_integrity(&self, data: &[u8]) -> String {
        // SHA-256 fingerprint (v1.2: removed insecure MD5)
        format!("{:x}", Sha256::digest(data))
    }
}`}
              </CodeBlock>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

function SecuritySection() {
  return (
    <section className="security-section">
      <div className="container">
        <div className="row">
          <div className="col col--12">
            <h2 style={{textAlign: 'center', marginBottom: '3rem'}}>
              🛡️ Enterprise-Grade Security
            </h2>
          </div>
        </div>
        <div className="row">
          <div className="col col--4">
            <div className="text--center">
              <h3>🔐 Modern Cryptography</h3>
              <p>
                AES-256-GCM encryption with Argon2 key derivation and SHA-256 fingerprints (v1.2).
                Enhanced security with elimination of deprecated MD5 cryptography.
              </p>
            </div>
          </div>
          <div className="col col--4">
            <div className="text--center">
              <h3>🦀 Memory Safety</h3>
              <p>
                Built in Rust for guaranteed memory safety, preventing buffer overflows
                and other memory-related vulnerabilities common in other languages.
              </p>
            </div>
          </div>
          <div className="col col--4">
            <div className="text--center">
              <h3>🔒 Zero-Knowledge</h3>
              <p>
                Your master password never leaves your device. All encryption happens
                locally, ensuring complete privacy and security.
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

function PoweredBySection() {
  return (
    <section style={{padding: '3rem 0'}}>
      <div className="container">
        <div className="row">
          <div className="col col--12">
            <h2 style={{textAlign: 'center', marginBottom: '2rem'}}>
              Powered By Innovation
            </h2>
            <div style={{textAlign: 'center', fontSize: '1.1rem', color: '#6c757d'}}>
              <p>
                <strong><a href="https://travisasm.com" target="_blank">TRaViS</a></strong> - 
                AI-Powered EASM without asset caps | {' '}
                <strong><a href="https://kief.studio" target="_blank">Kief Studio</a></strong> - 
                AI Integration & Technology Consulting | {' '}
                <strong><a href="https://hxhippy.com" target="_blank">HxHippy</a></strong> - 
                <a href="https://x.com/HxHippy" target="_blank">@HxHippy</a>
              </p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.title} - ${siteConfig.tagline}`}
      description="Advanced password and secrets manager built in Rust with enterprise-grade security, modern UI, and cross-platform support.">
      <HomepageHeader />
      <main>
        <StatsSection />
        <ScreenshotsSection />
        <HomepageFeatures />
        <CodeExamplesSection />
        <SecuritySection />
        <PoweredBySection />
      </main>
    </Layout>
  );
}