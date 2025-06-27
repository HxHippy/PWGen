import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: '🔐 Advanced Password Management',
    Svg: null,
    description: (
      <>
        Generate secure passwords with customizable complexity, store them with AES-256-GCM 
        encryption, and organize with tags and smart search. Import from any browser or 
        password manager seamlessly.
      </>
    ),
  },
  {
    title: '🗝️ Comprehensive Secrets Management',
    Svg: null,
    description: (
      <>
        Store API keys, SSH keys, secure documents, environment variables, and database 
        connections. Built-in templates for AWS, GCP, GitHub, and more with expiration tracking.
      </>
    ),
  },
  {
    title: '🚀 Modern & Fast',
    Svg: null,
    description: (
      <>
        Built in Rust for maximum performance and security. Cross-platform desktop GUI with 
        powerful CLI for automation. Local-first architecture with optional cloud sync.
      </>
    ),
  },
  {
    title: '🛡️ Enterprise Security',
    Svg: null,
    description: (
      <>
        Military-grade encryption, zero-knowledge architecture, comprehensive audit logging, 
        and hardware security key support. Designed for both individuals and teams.
      </>
    ),
  },
  {
    title: '🎯 Developer Friendly',
    Svg: null,
    description: (
      <>
        Full CLI interface for automation, REST API for integrations, browser extensions 
        for auto-fill, and comprehensive documentation. Perfect for DevOps workflows.
      </>
    ),
  },
  {
    title: '🌐 Cross-Platform Excellence',
    Svg: null,
    description: (
      <>
        Native applications for Windows, macOS, and Linux. Consistent experience across 
        platforms with responsive design and platform-specific optimizations.
      </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        {Svg && (
          <div className={styles.featureSvg}>
            <Svg role="img" />
          </div>
        )}
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}