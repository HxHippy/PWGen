import React from 'react';
import Layout from '@theme/Layout';

export default function About() {
  return (
    <Layout
      title="About the Team"
      description="Meet the team behind PwGen-rust - security experts and innovators building the future of password management">
      <main>
        <div className="container margin-vert--lg">
          <div className="row">
            <div className="col col--12">
              <h1>About the Team</h1>
              <p className="hero__subtitle">
                Meet the security experts and innovators building the future of password management
              </p>
            </div>
          </div>

          {/* Brian Gagne / HxHippy */}
          <div className="row margin-vert--lg">
            <div className="col col--3">
              <div className="text--center">
                <img 
                  src="/img/hxhippy.jpeg" 
                  alt="Brian Gagne (HxHippy)"
                  style={{
                    borderRadius: '50%',
                    width: '200px',
                    height: '200px',
                    objectFit: 'cover',
                    border: '4px solid var(--ifm-color-primary)'
                  }}
                />
              </div>
            </div>
            <div className="col col--9">
              <h2>Brian Gagne (HxHippy)</h2>
              <h3 style={{color: 'var(--ifm-color-primary)', fontWeight: 'normal'}}>
                Visionary CTO & CIO | AI & Cybersecurity Strategist
              </h3>
              <p><strong>Author of "The Crossroads of AI Integration"</strong></p>
              <p>
                Brian is a visionary technology leader shaping secure AI adoption and resilient cyber defenses. 
                As CTO/COO at Kief Studio and CIO at TRaViS, he brings decades of experience in cybersecurity, 
                artificial intelligence, and enterprise technology solutions.
              </p>
              <p>
                His expertise spans memory-safe programming languages, cryptographic implementations, and 
                zero-trust security architectures. Brian's leadership in developing PwGen-rust reflects his 
                commitment to building tools that provide enterprise-grade security for everyone.
              </p>
              
              <div className="margin-top--md">
                <h4>Connect with Brian:</h4>
                <div style={{display: 'flex', gap: '1rem', flexWrap: 'wrap'}}>
                  <a href="https://www.linkedin.com/in/briansgagne/" target="_blank" rel="noopener noreferrer" 
                     className="button button--primary button--sm">
                    💼 LinkedIn
                  </a>
                  <a href="https://x.com/HxHippy" target="_blank" rel="noopener noreferrer" 
                     className="button button--secondary button--sm">
                    🐦 @HxHippy
                  </a>
                  <a href="https://hxhippy.com" target="_blank" rel="noopener noreferrer" 
                     className="button button--secondary button--sm">
                    🌐 Website
                  </a>
                  <a href="https://github.com/HxHippy" target="_blank" rel="noopener noreferrer" 
                     className="button button--secondary button--sm">
                    🐱 GitHub
                  </a>
                </div>
              </div>
            </div>
          </div>

          <hr className="margin-vert--xl" />

          {/* Organizations */}
          <div className="row">
            <div className="col col--12">
              <h2>Powered By</h2>
              <p>PwGen-rust is brought to you by leading organizations in AI and cybersecurity:</p>
            </div>
          </div>

          <div className="row margin-vert--lg">
            {/* Kief Studio */}
            <div className="col col--6">
              <div className="card">
                <div className="card__header text--center">
                  <picture>
                    <source media="(prefers-color-scheme: dark)" srcSet="/img/kief-studio-logo-light.png" />
                    <img 
                      src="/img/kief-studio-logo-dark.png" 
                      alt="Kief Studio"
                      style={{height: '60px', objectFit: 'contain'}}
                    />
                  </picture>
                </div>
                <div className="card__body">
                  <h3>Kief Studio</h3>
                  <p><strong>AI Integration & Technology Consulting</strong></p>
                  <p>
                    Kief Studio specializes in AI integration and technology consulting, helping organizations 
                    navigate the complex landscape of artificial intelligence adoption while maintaining 
                    security and compliance.
                  </p>
                  <div className="text--center margin-top--md">
                    <a href="https://kief.studio" target="_blank" rel="noopener noreferrer" 
                       className="button button--primary">
                      Visit Kief Studio
                    </a>
                  </div>
                </div>
              </div>
            </div>

            {/* TRaViS */}
            <div className="col col--6">
              <div className="card">
                <div className="card__header text--center">
                  <picture>
                    <source media="(prefers-color-scheme: dark)" srcSet="/img/travis-text-white.png" />
                    <img 
                      src="/img/travis-text-dark.png" 
                      alt="TRaViS"
                      style={{height: '60px', objectFit: 'contain'}}
                    />
                  </picture>
                </div>
                <div className="card__body">
                  <h3>TRaViS</h3>
                  <p><strong>Threat Reconnaissance And Vulnerability Identification System</strong></p>
                  <p>
                    TRaViS provides AI-powered External Attack Surface Management (EASM) without asset caps, 
                    offering comprehensive threat intelligence and vulnerability identification for modern 
                    cybersecurity needs.
                  </p>
                  <div className="text--center margin-top--md">
                    <a href="https://travisasm.com" target="_blank" rel="noopener noreferrer" 
                       className="button button--primary">
                      Visit TRaViS
                    </a>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <hr className="margin-vert--xl" />

          {/* Mission Statement */}
          <div className="row">
            <div className="col col--12">
              <h2>Our Mission</h2>
              <div className="hero hero--primary">
                <div className="container">
                  <div className="hero__body">
                    <h3 className="hero__title">
                      Building Security-First Solutions for Everyone
                    </h3>
                    <p className="hero__subtitle">
                      We believe that enterprise-grade security shouldn't be reserved for large corporations. 
                      PwGen-rust represents our commitment to democratizing cybersecurity tools, making 
                      military-grade encryption and memory-safe password management accessible to individuals, 
                      small businesses, and enterprises alike.
                    </p>
                    <p className="hero__subtitle">
                      Through the innovative use of Rust's memory safety guarantees and modern cryptographic 
                      standards, we're building the next generation of security tools that are both 
                      uncompromisingly secure and beautifully usable.
                    </p>
                  </div>
                </div>
              </div>
            </div>
          </div>

          {/* Contact */}
          <div className="row margin-vert--lg">
            <div className="col col--12">
              <h2>Get In Touch</h2>
              <p>
                Interested in collaborating, contributing, or learning more about our work? 
                We'd love to hear from you.
              </p>
              <div className="row">
                <div className="col col--4">
                  <h4>🔒 Security Issues</h4>
                  <p>Report vulnerabilities responsibly:</p>
                  <a href="mailto:security@pwgenrust.dev">security@pwgenrust.dev</a>
                </div>
                <div className="col col--4">
                  <h4>💬 General Inquiries</h4>
                  <p>Questions or feedback:</p>
                  <a href="https://github.com/HxHippy/PWGen/discussions" target="_blank" rel="noopener noreferrer">
                    GitHub Discussions
                  </a>
                </div>
                <div className="col col--4">
                  <h4>🤝 Business Partnerships</h4>
                  <p>Collaboration opportunities:</p>
                  <a href="https://kief.studio/contact" target="_blank" rel="noopener noreferrer">
                    Contact Kief Studio
                  </a>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </Layout>
  );
}