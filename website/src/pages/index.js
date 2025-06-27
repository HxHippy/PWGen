import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';

import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <h1 className="hero__title">{siteConfig.title}</h1>
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
            href="https://github.com/your-username/pwgen">
            ⭐ GitHub
          </Link>
        </div>
        
        <div className="security-badges" style={{marginTop: '2rem'}}>
          <span className="security-badge">🔒 AES-256-GCM</span>
          <span className="security-badge">🦀 Memory Safe</span>
          <span className="security-badge">🚫 Zero Knowledge</span>
          <span className="security-badge">📱 Cross Platform</span>
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
              <div className="stats__number">100%</div>
              <div className="stats__label">Open Source</div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

function SecuritySection() {
  return (
    <section style={{padding: '4rem 0', backgroundColor: '#f8f9fa'}}>
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
              <h3>🔐 Military-Grade Encryption</h3>
              <p>
                AES-256-GCM encryption with PBKDF2 key derivation ensures your data
                is protected with the same standards used by governments and banks.
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
        <HomepageFeatures />
        <SecuritySection />
        <PoweredBySection />
      </main>
    </Layout>
  );
}