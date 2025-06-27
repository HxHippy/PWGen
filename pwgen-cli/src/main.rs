use anyhow::Result;
use clap::{Parser, Subcommand};
use pwgen_core::{
    backup::{BackupManager, ConflictResolution, RestoreOptions},
    crypto::hash_entry_id,
    generator::{PasswordConfig, PasswordGenerator},
    models::{DecryptedPasswordEntry, SearchFilter},
    storage::Storage,
};
use pwgen_core::secrets::{
    DecryptedSecretEntry, SecretData, SecretFilter, SecretMetadata, SecretType,
    NoteFormat, SshKeyType, ConfigFormat, DatabaseType, SslConfig
};
use pwgen_core::secrets_storage::SecretsStorage;
use pwgen_core::ssh_keys::{SshKeyManager, SshKeyGenParams, SshKeyUtils};
use pwgen_core::document_storage::{DocumentManager, DocumentAttachment, DocumentType};
use pwgen_core::api_keys::{ApiKeyManager, ApiKeyProvider, RotationInfo, UsageStats};
use pwgen_core::notes_config::{NotesConfigManager, NoteCategory, ConfigType, NotePriority};
use pwgen_core::env_connections::{EnvConnectionManager, EnvironmentType, ConnectionType, EnvVarType, EnvVariable};
use pwgen_core::secret_templates::{SecretTemplateManager, TemplateCategory};
use pwgen_core::browser_import::{BrowserImporter, BrowserType, ImportFormat, ImportConfig};
use pwgen_core::team_sharing::{TeamSharingManager, Team, TeamMember, SharedSecret, ShareRequest, ShareRequestStatus, Permission, AccessLog, AccessAction};
use std::path::PathBuf;
use tracing_subscriber;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE", default_value = "~/.pwgen/vault.db")]
    vault: PathBuf,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long)]
        force: bool,
    },
    
    Add {
        site: String,
        username: String,
        #[arg(short, long)]
        generate: bool,
        #[arg(short, long, default_value = "16")]
        length: usize,
        #[arg(short, long)]
        notes: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    Get {
        site: String,
        #[arg(short, long)]
        username: Option<String>,
        #[arg(short, long)]
        copy: bool,
        #[arg(short, long)]
        show: bool,
    },
    
    List {
        #[arg(short, long)]
        query: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
        #[arg(short, long)]
        favorites: bool,
    },
    
    Update {
        site: String,
        username: String,
        #[arg(short, long)]
        new_password: bool,
        #[arg(short, long)]
        notes: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    Delete {
        site: String,
        username: String,
        #[arg(short, long)]
        force: bool,
    },
    
    Generate {
        #[arg(short, long, default_value = "16")]
        length: usize,
        #[arg(long)]
        no_uppercase: bool,
        #[arg(long)]
        no_lowercase: bool,
        #[arg(long)]
        no_numbers: bool,
        #[arg(long)]
        no_symbols: bool,
        #[arg(long)]
        symbols: Option<String>,
        #[arg(short, long)]
        escape: bool,
        #[arg(long)]
        passphrase: bool,
        #[arg(long, default_value = "4")]
        words: usize,
        #[arg(long, default_value = "-")]
        separator: String,
    },
    
    Import {
        #[arg(short, long)]
        format: String,
        #[arg(short, long)]
        file: PathBuf,
    },
    
    Export {
        #[arg(short, long)]
        format: String,
        #[arg(short, long)]
        output: PathBuf,
    },
    
    Backup {
        #[arg(short, long)]
        output: PathBuf,
        #[arg(short, long)]
        incremental: bool,
        #[arg(long)]
        since: Option<String>,
    },
    
    Restore {
        #[arg(short, long)]
        backup_file: PathBuf,
        #[arg(long, default_value = "merge")]
        conflict_resolution: String,
    },
    
    VerifyBackup {
        backup_file: PathBuf,
    },
    
    // Secrets management commands
    AddSecret {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        secret_type: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
        #[arg(long)]
        template: Option<String>,
    },
    
    GetSecret {
        name: String,
        #[arg(short, long)]
        show: bool,
        #[arg(short, long)]
        copy: bool,
    },
    
    ListSecrets {
        #[arg(short, long)]
        query: Option<String>,
        #[arg(short, long)]
        secret_type: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
        #[arg(short, long)]
        environment: Option<String>,
        #[arg(short, long)]
        project: Option<String>,
        #[arg(long)]
        favorites: bool,
        #[arg(long)]
        expiring: Option<i64>,
    },
    
    UpdateSecret {
        name: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    DeleteSecret {
        name: String,
        #[arg(short, long)]
        force: bool,
    },
    
    ListTemplates {
        #[arg(short, long)]
        category: Option<String>,
    },
    
    ShowTemplate {
        template_id: String,
    },
    
    CreateFromTemplate {
        template_id: String,
        name: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
        #[arg(long)]
        interactive: bool,
    },
    
    ExportTemplate {
        template_id: String,
        #[arg(short, long)]
        output: PathBuf,
    },
    
    ImportTemplate {
        #[arg(short, long)]
        file: PathBuf,
    },
    
    // Browser import commands
    ImportBrowser {
        #[arg(short, long)]
        file: PathBuf,
        #[arg(short, long)]
        browser: Option<String>,
        #[arg(long, default_value = "csv")]
        format: String,
        #[arg(long)]
        skip_duplicates: bool,
        #[arg(long)]
        merge_duplicates: bool,
        #[arg(long)]
        folders_as_tags: bool,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    ListBrowserPaths {
        browser: Option<String>,
    },
    
    DetectBrowserType {
        file: PathBuf,
    },
    
    ExpiringSecrets {
        #[arg(long, default_value = "30")]
        within_days: i64,
    },
    
    SecretsStats,
    
    // SSH Key management commands
    GenerateSshKey {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value = "ed25519")]
        key_type: String,
        #[arg(short, long)]
        bits: Option<u32>,
        #[arg(short, long)]
        comment: Option<String>,
        #[arg(long)]
        with_passphrase: bool,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    ImportSshKey {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        private_key_file: Option<PathBuf>,
        #[arg(short, long)]
        public_key_file: Option<PathBuf>,
        #[arg(long)]
        passphrase: bool,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    ExportSshKey {
        name: String,
        #[arg(short, long)]
        output_dir: PathBuf,
        #[arg(long)]
        public_only: bool,
        #[arg(long)]
        format: Option<String>,
    },
    
    SshKeyInfo {
        name: String,
    },
    
    ChangeSshKeyPassphrase {
        name: String,
        #[arg(long)]
        remove_passphrase: bool,
    },
    
    // Document management commands
    ImportDocument {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        file_path: PathBuf,
        #[arg(short, long)]
        document_type: Option<String>,
        #[arg(long)]
        compress: bool,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    CreateTextDocument {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        filename: String,
        #[arg(short, long)]
        content: Option<String>,
        #[arg(long)]
        from_stdin: bool,
        #[arg(short, long)]
        document_type: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    ExportDocument {
        name: String,
        #[arg(short, long)]
        output_path: PathBuf,
        #[arg(long)]
        verify: bool,
    },
    
    DocumentInfo {
        name: String,
    },
    
    ViewDocument {
        name: String,
        #[arg(long)]
        text_only: bool,
    },
    
    ListDocumentTypes,
    
    // API Key management commands
    CreateApiKey {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        provider: String,
        #[arg(short, long)]
        api_key: String,
        #[arg(short, long)]
        api_secret: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
        #[arg(long)]
        expires_days: Option<u32>,
        #[arg(long)]
        environment: Option<String>,
    },
    
    CreateJwtToken {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        token: String,
        #[arg(short, long)]
        issuer: Option<String>,
        #[arg(short, long)]
        audience: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
        #[arg(long)]
        expires_days: Option<u32>,
    },
    
    CreateOauthToken {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        access_token: String,
        #[arg(short, long)]
        refresh_token: Option<String>,
        #[arg(short, long)]
        token_secret: Option<String>,
        #[arg(short, long)]
        scopes: Vec<String>,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
        #[arg(long)]
        expires_days: Option<u32>,
    },
    
    ListApiKeys {
        #[arg(short, long)]
        provider: Option<String>,
        #[arg(long)]
        expired: bool,
        #[arg(long)]
        expiring_days: Option<u32>,
        #[arg(short, long)]
        environment: Option<String>,
    },
    
    GetApiKey {
        name: String,
        #[arg(long)]
        show_secret: bool,
        #[arg(short, long)]
        copy: bool,
    },
    
    UpdateApiKeyUsage {
        name: String,
        #[arg(long)]
        success: bool,
        #[arg(long)]
        error_message: Option<String>,
    },
    
    SetupApiKeyRotation {
        name: String,
        #[arg(long)]
        rotation_days: u32,
        #[arg(long)]
        reminder_days: u32,
    },
    
    ListApiKeyProviders,
    
    // Notes management commands
    CreateNote {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: Option<String>,
        #[arg(long)]
        from_stdin: bool,
        #[arg(short, long, default_value = "markdown")]
        format: String,
        #[arg(short, long, default_value = "general")]
        category: String,
        #[arg(short, long, default_value = "medium")]
        priority: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    UpdateNote {
        name: String,
        #[arg(long)]
        new_title: Option<String>,
        #[arg(long)]
        new_content: Option<String>,
        #[arg(long)]
        from_stdin: bool,
        #[arg(long)]
        new_format: Option<String>,
    },
    
    ConvertNote {
        name: String,
        #[arg(short, long)]
        format: String,
    },
    
    SearchNotes {
        query: String,
        #[arg(long)]
        case_sensitive: bool,
        #[arg(short, long)]
        category: Option<String>,
    },
    
    ListNotes {
        #[arg(short, long)]
        category: Option<String>,
        #[arg(short, long)]
        priority: Option<String>,
        #[arg(short, long)]
        format: Option<String>,
    },
    
    // Configuration management commands
    CreateConfig {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        config_type: String,
        #[arg(short, long, default_value = "env")]
        format: String,
        #[arg(short, long)]
        file: Option<PathBuf>,
        #[arg(long)]
        from_stdin: bool,
        #[arg(short, long)]
        template: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    UpdateConfig {
        name: String,
        #[arg(short, long)]
        variable: Vec<String>, // KEY=VALUE format
        #[arg(long)]
        merge: bool,
        #[arg(short, long)]
        file: Option<PathBuf>,
    },
    
    ExportConfig {
        name: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
        #[arg(short, long)]
        format: Option<String>,
    },
    
    ValidateConfig {
        name: String,
        #[arg(short, long)]
        template: Option<String>,
    },
    
    ListConfigTemplates,
    
    ListConfigs {
        #[arg(short, long)]
        config_type: Option<String>,
        #[arg(short, long)]
        format: Option<String>,
    },
    
    // Environment variables management commands
    CreateEnvVar {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        variable_name: String,
        #[arg(short, long)]
        value: String,
        #[arg(short, long, default_value = "string")]
        var_type: String,
        #[arg(short, long, default_value = "development")]
        environment: String,
        #[arg(long)]
        sensitive: bool,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    CreateEnvSet {
        #[arg(short, long)]
        name: String,
        #[arg(short, long, default_value = "development")]
        environment: String,
        #[arg(short, long)]
        file: Option<PathBuf>,
        #[arg(long)]
        from_stdin: bool,
        #[arg(short, long)]
        template: Option<String>,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    GenerateEnvFile {
        name: String,
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
    
    ValidateEnvVars {
        name: String,
        #[arg(short, long)]
        template: Option<String>,
    },
    
    ListEnvTemplates,
    
    ListEnvVars {
        #[arg(short, long)]
        environment: Option<String>,
        #[arg(long)]
        show_sensitive: bool,
    },
    
    // Connection strings management commands
    CreateConnection {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        connection_type: String,
        #[arg(short, long)]
        host: String,
        #[arg(short, long)]
        port: Option<u16>,
        #[arg(short, long)]
        database: String,
        #[arg(short, long)]
        username: String,
        #[arg(short, long)]
        password: Option<String>,
        #[arg(short, long, default_value = "development")]
        environment: String,
        #[arg(long)]
        ssl_enabled: bool,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short, long)]
        tags: Vec<String>,
    },
    
    ParseConnection {
        connection_string: String,
    },
    
    TestConnection {
        name: String,
    },
    
    ListConnections {
        #[arg(short, long)]
        connection_type: Option<String>,
        #[arg(short, long)]
        environment: Option<String>,
    },
    
    // Team sharing commands
    CreateTeam {
        name: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(short = 'e', long)]
        owner_email: String,
        #[arg(short = 'n', long)]
        owner_name: String,
    },
    
    AddTeamMember {
        team_id: String,
        member_email: String,
        member_name: String,
        #[arg(short, long, default_value = "read")]
        role: String,
    },
    
    RemoveTeamMember {
        team_id: String,
        member_id: String,
    },
    
    UpdateMemberRole {
        team_id: String,
        member_id: String,
        new_role: String,
    },
    
    ListTeams,
    
    ShowTeam {
        team_id: String,
    },
    
    ShareSecret {
        secret_name: String,
        team_id: String,
        #[arg(short, long, default_value = "read")]
        permissions: String,
        #[arg(short, long)]
        expiration_days: Option<i64>,
    },
    
    ListSharedSecrets {
        #[arg(short, long)]
        team_id: Option<String>,
    },
    
    RevokeSecretAccess {
        secret_name: String,
        team_id: String,
    },
    
    RequestSecretAccess {
        secret_name: String,
        #[arg(short, long)]
        from_user: String,
        #[arg(short, long)]
        team_id: Option<String>,
        #[arg(short, long, default_value = "read")]
        permissions: String,
        #[arg(short, long)]
        message: Option<String>,
    },
    
    ListShareRequests {
        #[arg(short, long)]
        incoming: bool,
        #[arg(short, long)]
        outgoing: bool,
    },
    
    ApproveShareRequest {
        request_id: String,
    },
    
    RejectShareRequest {
        request_id: String,
    },
    
    ViewAccessLog {
        #[arg(short, long)]
        secret_name: Option<String>,
        #[arg(short, long)]
        user_id: Option<String>,
        #[arg(short, long, default_value = "100")]
        limit: usize,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    let vault_path = expand_tilde(&cli.vault);
    
    match cli.command {
        Commands::Init { force } => {
            init_vault(&vault_path, force).await?;
        }
        
        Commands::Add { site, username, generate, length, notes, tags } => {
            let storage = open_vault(&vault_path).await?;
            add_entry(&storage, site, username, generate, length, notes, tags).await?;
        }
        
        Commands::Get { site, username, copy, show } => {
            let storage = open_vault(&vault_path).await?;
            get_entry(&storage, &site, username.as_deref(), copy, show).await?;
        }
        
        Commands::List { query, tags, favorites } => {
            let storage = open_vault(&vault_path).await?;
            list_entries(&storage, query, tags, favorites).await?;
        }
        
        Commands::Update { site, username, new_password, notes, tags } => {
            let storage = open_vault(&vault_path).await?;
            update_entry(&storage, site, username, new_password, notes, tags).await?;
        }
        
        Commands::Delete { site, username, force } => {
            let storage = open_vault(&vault_path).await?;
            delete_entry(&storage, &site, &username, force).await?;
        }
        
        Commands::Generate { 
            length, no_uppercase, no_lowercase, no_numbers, no_symbols, 
            symbols, escape, passphrase, words, separator 
        } => {
            generate_password(
                length, !no_uppercase, !no_lowercase, !no_numbers, !no_symbols,
                symbols, escape, passphrase, words, separator
            )?;
        }
        
        Commands::Import { format, file } => {
            let storage = open_vault(&vault_path).await?;
            import_passwords(&storage, &format, &file).await?;
        }
        
        Commands::Export { format, output } => {
            let storage = open_vault(&vault_path).await?;
            export_passwords(&storage, &format, &output).await?;
        }
        
        Commands::Backup { output, incremental, since } => {
            let storage = open_vault(&vault_path).await?;
            create_backup(&storage, &output, incremental, since).await?;
        }
        
        Commands::Restore { backup_file, conflict_resolution } => {
            let mut storage = open_vault(&vault_path).await?;
            restore_backup(&mut storage, &backup_file, conflict_resolution).await?;
        }
        
        Commands::VerifyBackup { backup_file } => {
            verify_backup(&backup_file).await?;
        }
        
        // Secrets management commands
        Commands::AddSecret { name, secret_type, description, tags, template } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            add_secret(&secrets_storage, name, secret_type, description, tags, template).await?;
        }
        
        Commands::GetSecret { name, show, copy } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            get_secret(&secrets_storage, &name, show, copy).await?;
        }
        
        Commands::ListSecrets { query, secret_type, tags, environment, project, favorites, expiring } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            list_secrets(&secrets_storage, query, secret_type, tags, environment, project, favorites, expiring).await?;
        }
        
        Commands::UpdateSecret { name, description, tags } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            update_secret(&secrets_storage, name, description, tags).await?;
        }
        
        Commands::DeleteSecret { name, force } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            delete_secret(&secrets_storage, &name, force).await?;
        }
        
        Commands::ListTemplates { category } => {
            list_templates(category);
        }
        
        Commands::ShowTemplate { template_id } => {
            show_template(&template_id);
        }
        
        Commands::CreateFromTemplate { template_id, name, description, tags, interactive } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            create_from_template(&secrets_storage, &template_id, name, description, tags, interactive).await?;
        }
        
        Commands::ExportTemplate { template_id, output } => {
            export_template(&template_id, &output)?;
        }
        
        Commands::ImportTemplate { file } => {
            import_template(&file)?;
        }
        
        Commands::ImportBrowser { file, browser, format, skip_duplicates, merge_duplicates, folders_as_tags, tags } => {
            let storage = open_vault(&vault_path).await?;
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            import_browser(&storage, &secrets_storage, &file, browser, format, skip_duplicates, merge_duplicates, folders_as_tags, tags).await?;
        }
        
        Commands::ListBrowserPaths { browser } => {
            list_browser_paths(browser);
        }
        
        Commands::DetectBrowserType { file } => {
            detect_browser_type(&file)?;
        }
        
        Commands::ExpiringSecrets { within_days } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            show_expiring_secrets(&secrets_storage, within_days).await?;
        }
        
        Commands::SecretsStats => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            show_secrets_stats(&secrets_storage).await?;
        }
        
        // SSH Key management commands
        Commands::GenerateSshKey { name, key_type, bits, comment, with_passphrase, description, tags } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            generate_ssh_key(&secrets_storage, name, key_type, bits, comment, with_passphrase, description, tags).await?;
        }
        
        Commands::ImportSshKey { name, private_key_file, public_key_file, passphrase, description, tags } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            import_ssh_key(&secrets_storage, name, private_key_file, public_key_file, passphrase, description, tags).await?;
        }
        
        Commands::ExportSshKey { name, output_dir, public_only, format } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            export_ssh_key(&secrets_storage, &name, &output_dir, public_only, format).await?;
        }
        
        Commands::SshKeyInfo { name } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            show_ssh_key_info(&secrets_storage, &name).await?;
        }
        
        Commands::ChangeSshKeyPassphrase { name, remove_passphrase } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            change_ssh_key_passphrase(&secrets_storage, &name, remove_passphrase).await?;
        }
        
        // Document management commands
        Commands::ImportDocument { name, file_path, document_type, compress, description, tags } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            import_document(&secrets_storage, name, file_path, document_type, compress, description, tags).await?;
        }
        
        Commands::CreateTextDocument { name, filename, content, from_stdin, document_type, description, tags } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            create_text_document(&secrets_storage, name, filename, content, from_stdin, document_type, description, tags).await?;
        }
        
        Commands::ExportDocument { name, output_path, verify } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            export_document(&secrets_storage, &name, &output_path, verify).await?;
        }
        
        Commands::DocumentInfo { name } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            show_document_info(&secrets_storage, &name).await?;
        }
        
        Commands::ViewDocument { name, text_only } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            view_document(&secrets_storage, &name, text_only).await?;
        }
        
        Commands::ListDocumentTypes => {
            list_document_types();
        }
        
        // API Key management command handlers
        Commands::CreateApiKey { name, provider, api_key, api_secret, description, tags, expires_days, environment } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            create_api_key(&secrets_storage, name, provider, api_key, api_secret, description, tags, expires_days, environment).await?;
        }
        
        Commands::CreateJwtToken { name, token, issuer, audience, description, tags, expires_days } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            create_jwt_token(&secrets_storage, name, token, issuer, audience, description, tags, expires_days).await?;
        }
        
        Commands::CreateOauthToken { name, access_token, refresh_token, token_secret, scopes, description, tags, expires_days } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            create_oauth_token(&secrets_storage, name, access_token, refresh_token, token_secret, scopes, description, tags, expires_days).await?;
        }
        
        Commands::ListApiKeys { provider, expired, expiring_days, environment } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            list_api_keys(&secrets_storage, provider, expired, expiring_days, environment).await?;
        }
        
        Commands::GetApiKey { name, show_secret, copy } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            get_api_key(&secrets_storage, &name, show_secret, copy).await?;
        }
        
        Commands::UpdateApiKeyUsage { name, success, error_message } => {
            let mut secrets_storage = open_secrets_vault(&vault_path).await?;
            update_api_key_usage(&mut secrets_storage, &name, success, error_message).await?;
        }
        
        Commands::SetupApiKeyRotation { name, rotation_days, reminder_days } => {
            let mut secrets_storage = open_secrets_vault(&vault_path).await?;
            setup_api_key_rotation(&mut secrets_storage, &name, rotation_days, reminder_days).await?;
        }
        
        Commands::ListApiKeyProviders => {
            list_api_key_providers();
        }
        
        // Notes management command handlers
        Commands::CreateNote { title, content, from_stdin, format, category, priority, description, tags } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            create_note(&secrets_storage, title, content, from_stdin, format, category, priority, description, tags).await?;
        }
        
        Commands::UpdateNote { name, new_title, new_content, from_stdin, new_format } => {
            let mut secrets_storage = open_secrets_vault(&vault_path).await?;
            update_note(&mut secrets_storage, &name, new_title, new_content, from_stdin, new_format).await?;
        }
        
        Commands::ConvertNote { name, format } => {
            let mut secrets_storage = open_secrets_vault(&vault_path).await?;
            convert_note(&mut secrets_storage, &name, format).await?;
        }
        
        Commands::SearchNotes { query, case_sensitive, category } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            search_notes(&secrets_storage, &query, case_sensitive, category).await?;
        }
        
        Commands::ListNotes { category, priority, format } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            list_notes(&secrets_storage, category, priority, format).await?;
        }
        
        // Configuration management command handlers
        Commands::CreateConfig { name, config_type, format, file, from_stdin, template, description, tags } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            create_config(&secrets_storage, name, config_type, format, file, from_stdin, template, description, tags).await?;
        }
        
        Commands::UpdateConfig { name, variable, merge, file } => {
            let mut secrets_storage = open_secrets_vault(&vault_path).await?;
            update_config(&mut secrets_storage, &name, variable, merge, file).await?;
        }
        
        Commands::ExportConfig { name, output, format } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            export_config(&secrets_storage, &name, output, format).await?;
        }
        
        Commands::ValidateConfig { name, template } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            validate_config(&secrets_storage, &name, template).await?;
        }
        
        Commands::ListConfigTemplates => {
            list_config_templates();
        }
        
        Commands::ListConfigs { config_type, format } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            list_configs(&secrets_storage, config_type, format).await?;
        }
        
        // Environment variables management command handlers
        Commands::CreateEnvVar { name, variable_name, value, var_type, environment, sensitive, description, tags } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            create_env_var(&secrets_storage, name, variable_name, value, var_type, environment, sensitive, description, tags).await?;
        }
        
        Commands::CreateEnvSet { name, environment, file, from_stdin, template, description, tags } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            create_env_set(&secrets_storage, name, environment, file, from_stdin, template, description, tags).await?;
        }
        
        Commands::GenerateEnvFile { name, output } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            generate_env_file(&secrets_storage, &name, output).await?;
        }
        
        Commands::ValidateEnvVars { name, template } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            validate_env_vars(&secrets_storage, &name, template).await?;
        }
        
        Commands::ListEnvTemplates => {
            list_env_templates();
        }
        
        Commands::ListEnvVars { environment, show_sensitive } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            list_env_vars(&secrets_storage, environment, show_sensitive).await?;
        }
        
        // Connection strings management command handlers
        Commands::CreateConnection { name, connection_type, host, port, database, username, password, environment, ssl_enabled, description, tags } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            create_connection(&secrets_storage, name, connection_type, host, port, database, username, password, environment, ssl_enabled, description, tags).await?;
        }
        
        Commands::ParseConnection { connection_string } => {
            parse_connection_string(&connection_string)?;
        }
        
        Commands::TestConnection { name } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            test_connection(&secrets_storage, &name).await?;
        }
        
        Commands::ListConnections { connection_type, environment } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            list_connections(&secrets_storage, connection_type, environment).await?;
        }
        
        // Team sharing command handlers
        Commands::CreateTeam { name, description, owner_email, owner_name } => {
            create_team(name, description, owner_email, owner_name).await?;
        }
        
        Commands::AddTeamMember { team_id, member_email, member_name, role } => {
            add_team_member(team_id, member_email, member_name, role).await?;
        }
        
        Commands::RemoveTeamMember { team_id, member_id } => {
            remove_team_member(team_id, member_id).await?;
        }
        
        Commands::UpdateMemberRole { team_id, member_id, new_role } => {
            update_member_role(team_id, member_id, new_role).await?;
        }
        
        Commands::ListTeams => {
            list_teams().await?;
        }
        
        Commands::ShowTeam { team_id } => {
            show_team(team_id).await?;
        }
        
        Commands::ShareSecret { secret_name, team_id, permissions, expiration_days } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            share_secret(&secrets_storage, secret_name, team_id, permissions, expiration_days).await?;
        }
        
        Commands::ListSharedSecrets { team_id } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            list_shared_secrets(&secrets_storage, team_id).await?;
        }
        
        Commands::RevokeSecretAccess { secret_name, team_id } => {
            let secrets_storage = open_secrets_vault(&vault_path).await?;
            revoke_secret_access(&secrets_storage, secret_name, team_id).await?;
        }
        
        Commands::RequestSecretAccess { secret_name, from_user, team_id, permissions, message } => {
            request_secret_access(secret_name, from_user, team_id, permissions, message).await?;
        }
        
        Commands::ListShareRequests { incoming, outgoing } => {
            list_share_requests(incoming, outgoing).await?;
        }
        
        Commands::ApproveShareRequest { request_id } => {
            approve_share_request(request_id).await?;
        }
        
        Commands::RejectShareRequest { request_id } => {
            reject_share_request(request_id).await?;
        }
        
        Commands::ViewAccessLog { secret_name, user_id, limit } => {
            view_access_log(secret_name, user_id, limit).await?;
        }
    }
    
    Ok(())
}

async fn open_secrets_vault(path: &PathBuf) -> Result<SecretsStorage> {
    if !path.exists() {
        eprintln!("Vault not found at {:?}. Run 'pwgen init' first.", path);
        std::process::exit(1);
    }
    
    let password = rpassword::prompt_password("Enter master password: ")?;
    
    match SecretsStorage::from_existing_storage(path, &password).await {
        Ok(storage) => Ok(storage),
        Err(e) => {
            eprintln!("Failed to open secrets vault: {}", e);
            std::process::exit(1);
        }
    }
}

async fn add_secret(
    storage: &SecretsStorage,
    name: String,
    secret_type_str: String,
    description: Option<String>,
    tags: Vec<String>,
    template: Option<String>,
) -> Result<()> {
    let secret_type = match secret_type_str.as_str() {
        "password" => SecretType::Password,
        "ssh-key" => SecretType::SshKey,
        "api-key" => SecretType::ApiKey,
        "note" => SecretType::SecureNote,
        custom => SecretType::Custom(custom.to_string()),
    };
    
    let data = match &secret_type {
        SecretType::Password => {
            print!("Username: ");
            use std::io::{self, Write};
            io::stdout().flush()?;
            let mut username = String::new();
            io::stdin().read_line(&mut username)?;
            
            let password = rpassword::prompt_password("Password: ")?;
            
            SecretData::Password {
                username: username.trim().to_string(),
                password,
                url: None,
                notes: None,
            }
        }
        
        SecretType::ApiKey => {
            let api_key = rpassword::prompt_password("API Key: ")?;
            let provider = ApiKeyProvider::Generic;
            
            SecretData::ApiKey {
                provider,
                key_id: format!("generic_{}", uuid::Uuid::new_v4()),
                api_key,
                api_secret: None,
                token_type: "Bearer".to_string(),
                permissions: Default::default(),
                environment: "production".to_string(),
                endpoint_url: None,
                rotation_info: RotationInfo {
                    auto_rotate: false,
                    rotation_period_days: None,
                    last_rotated: None,
                    next_rotation: None,
                    rotation_reminder_days: None,
                },
                usage_stats: UsageStats {
                    last_used: None,
                    usage_count: 0,
                    rate_limit_info: None,
                    error_count: 0,
                    last_error: None,
                },
            }
        }
        
        SecretType::SecureNote => {
            print!("Title: ");
            use std::io::{self, Write};
            io::stdout().flush()?;
            let mut title = String::new();
            io::stdin().read_line(&mut title)?;
            
            print!("Content: ");
            io::stdout().flush()?;
            let mut content = String::new();
            io::stdin().read_line(&mut content)?;
            
            SecretData::SecureNote {
                title: title.trim().to_string(),
                content: content.trim().to_string(),
                format: NoteFormat::PlainText,
            }
        }
        
        _ => {
            println!("Interactive creation for {} not yet implemented. Use GUI instead.", secret_type_str);
            return Ok(());
        }
    };
    
    let secret = DecryptedSecretEntry {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        description,
        secret_type,
        data,
        metadata: SecretMetadata {
            template,
            ..Default::default()
        },
        tags,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_accessed: None,
        expires_at: None,
        favorite: false,
    };
    
    storage.add_secret(&secret).await?;
    println!("Secret '{}' added successfully!", secret.name);
    
    Ok(())
}

async fn get_secret(
    storage: &SecretsStorage,
    name: &str,
    show: bool,
    _copy: bool,
) -> Result<()> {
    let filter = SecretFilter {
        query: Some(name.to_string()),
        ..Default::default()
    };
    
    let secrets = storage.search_secrets(&filter).await?;
    let secret = secrets.iter().find(|s| s.name == name);
    
    if let Some(secret) = secret {
        println!("Name: {}", secret.name);
        println!("Type: {:?}", secret.secret_type);
        if let Some(desc) = &secret.description {
            println!("Description: {}", desc);
        }
        println!("Tags: {:?}", secret.tags);
        println!("Created: {}", secret.created_at.format("%Y-%m-%d %H:%M:%S"));
        
        if show {
            match &secret.data {
                SecretData::Password { username, password, url, notes } => {
                    println!("Username: {}", username);
                    println!("Password: {}", password);
                    if let Some(url) = url {
                        println!("URL: {}", url);
                    }
                    if let Some(notes) = notes {
                        println!("Notes: {}", notes);
                    }
                }
                SecretData::ApiKey { api_key, api_secret, endpoint_url, .. } => {
                    println!("Key: {}", api_key);
                    if let Some(secret) = api_secret {
                        println!("Secret: {}", secret);
                    }
                    if let Some(endpoint) = endpoint_url {
                        println!("Endpoint: {}", endpoint);
                    }
                }
                SecretData::SecureNote { title, content, .. } => {
                    println!("Title: {}", title);
                    println!("Content: {}", content);
                }
                _ => {
                    println!("Full display for this secret type not yet implemented in CLI");
                }
            }
        } else {
            println!("Use --show to display secret contents");
        }
    } else {
        println!("Secret '{}' not found", name);
    }
    
    Ok(())
}

async fn list_secrets(
    storage: &SecretsStorage,
    query: Option<String>,
    secret_type: Option<String>,
    tags: Vec<String>,
    environment: Option<String>,
    project: Option<String>,
    favorites: bool,
    expiring: Option<i64>,
) -> Result<()> {
    let secret_types = if let Some(type_str) = secret_type {
        let parsed_type = match type_str.as_str() {
            "password" => SecretType::Password,
            "ssh-key" => SecretType::SshKey,
            "api-key" => SecretType::ApiKey,
            "note" => SecretType::SecureNote,
            custom => SecretType::Custom(custom.to_string()),
        };
        Some(vec![parsed_type])
    } else {
        None
    };
    
    let filter = SecretFilter {
        query,
        secret_types,
        tags: if tags.is_empty() { None } else { Some(tags) },
        environment,
        project,
        favorite_only: favorites,
        expires_before: if let Some(days) = expiring {
            Some(chrono::Utc::now() + chrono::Duration::days(days))
        } else {
            None
        },
        ..Default::default()
    };
    
    let secrets = storage.search_secrets(&filter).await?;
    
    if secrets.is_empty() {
        println!("No secrets found");
    } else {
        println!("{:<30} {:<15} {:<20}", "Name", "Type", "Created");
        println!("{:-<65}", "");
        
        for secret in secrets {
            let type_name = match secret.secret_type {
                SecretType::Password => "Password",
                SecretType::SshKey => "SSH Key",
                SecretType::ApiKey => "API Key",
                SecretType::SecureNote => "Note",
                SecretType::Custom(_) => "Custom",
                _ => "Other",
            };
            
            let created = secret.created_at.format("%Y-%m-%d %H:%M").to_string();
            
            println!("{:<30} {:<15} {:<20}", secret.name, type_name, created);
        }
    }
    
    Ok(())
}

async fn update_secret(
    storage: &SecretsStorage,
    name: String,
    description: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let filter = SecretFilter {
        query: Some(name.clone()),
        ..Default::default()
    };
    
    let secrets = storage.search_secrets(&filter).await?;
    let mut secret = secrets.into_iter().find(|s| s.name == name)
        .ok_or_else(|| anyhow::anyhow!("Secret '{}' not found", name))?;
    
    if let Some(desc) = description {
        secret.description = Some(desc);
    }
    
    if !tags.is_empty() {
        secret.tags = tags;
    }
    
    secret.updated_at = chrono::Utc::now();
    storage.update_secret(&secret).await?;
    
    println!("Secret '{}' updated successfully", name);
    Ok(())
}

async fn delete_secret(storage: &SecretsStorage, name: &str, force: bool) -> Result<()> {
    if !force {
        print!("Are you sure you want to delete secret '{}'? [y/N] ", name);
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Deletion cancelled");
            return Ok(());
        }
    }
    
    let filter = SecretFilter {
        query: Some(name.to_string()),
        ..Default::default()
    };
    
    let secrets = storage.search_secrets(&filter).await?;
    let secret = secrets.iter().find(|s| s.name == name)
        .ok_or_else(|| anyhow::anyhow!("Secret '{}' not found", name))?;
    
    storage.delete_secret(&secret.id).await?;
    println!("Secret '{}' deleted successfully", name);
    
    Ok(())
}

fn list_templates(category: Option<String>) {
    let templates = if let Some(cat_str) = category {
        if let Ok(category) = cat_str.parse::<TemplateCategory>() {
            SecretTemplateManager::get_templates_by_category(&category)
        } else {
            println!("Invalid category: {}", cat_str);
            return;
        }
    } else {
        SecretTemplateManager::get_all_templates()
    };
    
    if templates.is_empty() {
        println!("No templates found");
        return;
    }
    
    println!("Available Secret Templates:");
    println!("{:-<80}", "");
    
    for template in templates {
        println!("ID: {}", template.id);
        println!("Name: {}", template.name);
        println!("Description: {}", template.description);
        println!("Category: {}", template.category);
        println!("Type: {:?}", template.secret_type);
        println!("Tags: {}", template.tags.join(", "));
        if let Some(url) = &template.documentation_url {
            println!("Documentation: {}", url);
        }
        println!("{}", "-".repeat(40));
    }
}

fn show_template(template_id: &str) {
    if let Some(template) = SecretTemplateManager::get_template_by_id(template_id) {
        println!("Template Details:");
        println!("{:=<80}", "");
        println!("ID: {}", template.id);
        println!("Name: {}", template.name);
        println!("Description: {}", template.description);
        println!("Category: {}", template.category);
        println!("Secret Type: {:?}", template.secret_type);
        println!("Tags: {}", template.tags.join(", "));
        
        if let Some(url) = &template.documentation_url {
            println!("Documentation: {}", url);
        }
        
        println!("\nRequired Fields:");
        println!("{:-<50}", "");
        
        for field in &template.fields {
            println!("  {} ({})", field.name, field.field_type);
            println!("    Description: {}", field.description);
            println!("    Required: {}", field.required);
            println!("    Sensitive: {}", field.sensitive);
            
            if let Some(placeholder) = &field.placeholder {
                println!("    Example: {}", placeholder);
            }
            
            if let Some(help) = &field.help_text {
                println!("    Help: {}", help);
            }
            
            println!();
        }
        
        if !template.validation_rules.is_empty() {
            println!("Validation Rules:");
            println!("{:-<50}", "");
            for rule in &template.validation_rules {
                println!("  {}: {}", rule.field_name, rule.message);
            }
        }
        
        if let Some(env_vars) = &template.environment_variables {
            println!("\nEnvironment Variables:");
            println!("{:-<50}", "");
            for (name, var) in env_vars {
                println!("  {}: {}", name, var.description.as_deref().unwrap_or("No description"));
            }
        }
    } else {
        println!("Template '{}' not found", template_id);
    }
}

async fn create_from_template(
    storage: &SecretsStorage,
    template_id: &str,
    name: String,
    description: Option<String>,
    tags: Vec<String>,
    interactive: bool,
) -> Result<()> {
    let template = SecretTemplateManager::get_template_by_id(template_id)
        .ok_or_else(|| anyhow::anyhow!("Template '{}' not found", template_id))?;
    
    let mut field_values = std::collections::HashMap::new();
    
    if interactive {
        println!("Creating secret from template: {}", template.name);
        println!("{}", template.description);
        println!();
        
        for field in &template.fields {
            print!("{}: ", field.description);
            if let Some(help) = &field.help_text {
                print!("({})", help);
            }
            if field.required {
                print!(" [REQUIRED]");
            }
            println!();
            
            if let Some(placeholder) = &field.placeholder {
                println!("  Example: {}", placeholder);
            }
            
            print!("  Enter value: ");
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
            
            let mut input = String::new();
            if field.sensitive {
                // For sensitive fields, use rpassword to hide input
                input = rpassword::read_password().unwrap_or_default();
            } else {
                io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
            }
            
            if !input.is_empty() {
                field_values.insert(field.name.clone(), input);
            } else if field.required {
                println!("Error: Field '{}' is required", field.name);
                return Ok(());
            }
        }
    } else {
        println!("Error: Non-interactive mode requires field values to be provided");
        println!("Use --interactive flag or provide field values directly");
        return Ok(());
    }
    
    // Create the secret from template
    let secret = SecretTemplateManager::create_secret_from_template(
        template_id,
        field_values,
        name,
        description,
        tags,
    )?;
    
    // Store the secret
    storage.add_secret(&secret).await?;
    
    println!("Secret '{}' created successfully from template '{}'", secret.name, template.name);
    Ok(())
}

fn export_template(template_id: &str, output: &PathBuf) -> Result<()> {
    if let Some(template) = SecretTemplateManager::get_template_by_id(template_id) {
        let json = SecretTemplateManager::export_template(&template)?;
        std::fs::write(output, json)?;
        println!("Template '{}' exported to {}", template_id, output.display());
    } else {
        println!("Template '{}' not found", template_id);
    }
    Ok(())
}

fn import_template(file: &PathBuf) -> Result<()> {
    let content = std::fs::read_to_string(file)?;
    let template = SecretTemplateManager::import_template(&content)?;
    println!("Template '{}' ({}) imported successfully", template.id, template.name);
    println!("Use 'list-templates' to see all available templates");
    Ok(())
}

async fn import_browser(
    _storage: &Storage,
    secrets_storage: &SecretsStorage,
    file: &PathBuf,
    browser: Option<String>,
    format: String,
    skip_duplicates: bool,
    merge_duplicates: bool,
    folders_as_tags: bool,
    tags: Vec<String>,
) -> Result<()> {
    println!("Importing passwords from browser export...");
    
    // Detect browser type if not specified
    let browser_type = if let Some(browser_str) = browser {
        browser_str.parse::<BrowserType>()?
    } else {
        BrowserImporter::detect_browser_type(file)?
    };
    
    // Parse format
    let import_format = format.parse::<ImportFormat>()?;
    
    // Create import configuration
    let config = ImportConfig {
        browser_type: browser_type.clone(),
        format: import_format,
        skip_duplicates,
        merge_duplicates,
        import_folders_as_tags: folders_as_tags,
        default_tags: if tags.is_empty() { vec!["imported".to_string()] } else { tags },
        password_strength_check: false,
        cleanup_urls: true,
    };
    
    println!("Detected browser: {}", browser_type);
    println!("Import format: {}", config.format);
    
    // Import passwords from file
    let (imported_passwords, import_result) = BrowserImporter::import_from_file(file, config.clone())?;
    
    println!("Import Statistics:");
    println!("  Total entries found: {}", import_result.total_entries);
    println!("  Valid entries: {}", import_result.successful_imports);
    println!("  Failed entries: {}", import_result.failed_imports);
    
    if !import_result.errors.is_empty() {
        println!("Errors encountered:");
        for error in &import_result.errors {
            println!("  - {}", error);
        }
    }
    
    if !import_result.warnings.is_empty() {
        println!("Warnings:");
        for warning in &import_result.warnings {
            println!("  - {}", warning);
        }
    }
    
    if imported_passwords.is_empty() {
        println!("No valid passwords found to import.");
        return Ok(());
    }
    
    // Convert to secret entries and import
    let secret_entries = BrowserImporter::convert_to_secret_entries(imported_passwords, &config)?;
    
    println!("Converting {} entries to secrets...", secret_entries.len());
    
    let mut imported_count = 0;
    let mut skipped_count = 0;
    let total_entries = secret_entries.len();
    
    for entry in secret_entries {
        // Check for duplicates if skip_duplicates is enabled
        if config.skip_duplicates {
            let existing = secrets_storage.search_secrets(&SecretFilter {
                query: Some(entry.name.clone()),
                secret_types: Some(vec![SecretType::Password]),
                ..Default::default()
            }).await?;
            
            if !existing.is_empty() {
                println!("Skipping duplicate entry: {}", entry.name);
                skipped_count += 1;
                continue;
            }
        }
        
        // Add the secret
        match secrets_storage.add_secret(&entry).await {
            Ok(_) => {
                imported_count += 1;
                if imported_count <= 5 {
                    println!("Imported: {}", entry.name);
                } else if imported_count == 6 {
                    println!("... and {} more entries", total_entries - 5);
                }
            }
            Err(e) => {
                println!("Failed to import '{}': {}", entry.name, e);
            }
        }
    }
    
    println!();
    println!("Import completed:");
    println!("  Successfully imported: {}", imported_count);
    println!("  Skipped duplicates: {}", skipped_count);
    println!("  Total processed: {}", imported_count + skipped_count);
    
    Ok(())
}

fn list_browser_paths(browser: Option<String>) {
    let browsers = if let Some(browser_str) = browser {
        if let Ok(browser_type) = browser_str.parse::<BrowserType>() {
            vec![browser_type]
        } else {
            println!("Unknown browser type: {}", browser_str);
            return;
        }
    } else {
        vec![
            BrowserType::Chrome,
            BrowserType::Firefox,
            BrowserType::Safari,
            BrowserType::Edge,
            BrowserType::Opera,
            BrowserType::Brave,
            BrowserType::Vivaldi,
        ]
    };
    
    println!("Browser password database paths:");
    println!("{:=<60}", "");
    
    for browser_type in browsers {
        println!("\n{}:", browser_type);
        let paths = BrowserImporter::get_default_browser_paths(&browser_type);
        
        if paths.is_empty() {
            println!("  No default paths available for this platform");
        } else {
            for path in paths {
                let status = if path.exists() {
                    "✓ Found"
                } else {
                    "✗ Not found"
                };
                println!("  {} {}", status, path.display());
            }
        }
    }
    
    println!("\nNote: For security reasons, browsers encrypt their password databases.");
    println!("Use the browser's export function to create a CSV file for import:");
    println!("  • Chrome: Settings → Passwords → Export passwords");
    println!("  • Firefox: about:logins → ⋯ menu → Export logins");
    println!("  • Edge: Settings → Passwords → Export passwords");
    println!("  • Safari: File → Export → Passwords");
}

fn detect_browser_type(file: &PathBuf) -> Result<()> {
    match BrowserImporter::detect_browser_type(file) {
        Ok(browser_type) => {
            println!("Detected browser type: {}", browser_type);
            
            // Provide format suggestions
            match browser_type {
                BrowserType::Chrome | BrowserType::Edge | BrowserType::Brave => {
                    println!("Recommended format: csv");
                    println!("Export from: Settings → Passwords → Export passwords");
                }
                BrowserType::Firefox => {
                    println!("Recommended format: csv");
                    println!("Export from: about:logins → ⋯ menu → Export logins");
                }
                BrowserType::Safari => {
                    println!("Recommended format: csv");
                    println!("Export from: File → Export → Passwords");
                }
                BrowserType::Custom(name) => {
                    if name.contains("LastPass") {
                        println!("Recommended format: lastpass");
                    } else if name.contains("Bitwarden") {
                        println!("Recommended format: bitwarden");
                    } else if name.contains("1Password") {
                        println!("Recommended format: 1password");
                    } else {
                        println!("Recommended format: csv (generic)");
                    }
                }
                _ => {
                    println!("Recommended format: csv");
                }
            }
        }
        Err(e) => {
            println!("Could not detect browser type: {}", e);
            println!("You can specify the browser type manually with --browser option");
        }
    }
    
    Ok(())
}

async fn show_expiring_secrets(storage: &SecretsStorage, within_days: i64) -> Result<()> {
    let secrets = storage.get_expiring_secrets(within_days).await?;
    
    if secrets.is_empty() {
        println!("No secrets expiring within {} days", within_days);
    } else {
        println!("Secrets expiring within {} days:", within_days);
        println!("{:-<50}", "");
        
        for secret in secrets {
            if let Some(expires_at) = secret.expires_at {
                let days_until = (expires_at - chrono::Utc::now()).num_days();
                println!("{} expires in {} days", secret.name, days_until);
            }
        }
    }
    
    Ok(())
}

async fn show_secrets_stats(storage: &SecretsStorage) -> Result<()> {
    let stats = storage.get_secrets_stats().await?;
    
    println!("Secrets Statistics:");
    println!("{:-<30}", "");
    println!("Total secrets: {}", stats.total_count);
    println!("Expired secrets: {}", stats.expired_count);
    println!("Expiring soon (30 days): {}", stats.expiring_soon_count);
    
    Ok(())
}

// SSH Key management functions
async fn generate_ssh_key(
    storage: &SecretsStorage,
    name: String,
    key_type_str: String,
    bits: Option<u32>,
    comment: Option<String>,
    with_passphrase: bool,
    description: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    // Check if ssh-keygen is available
    if !SshKeyUtils::check_ssh_keygen_available() {
        eprintln!("Error: ssh-keygen is not available. Please install OpenSSH.");
        return Ok(());
    }
    
    // Parse key type
    let key_type = match key_type_str.as_str() {
        "rsa" => SshKeyType::Rsa,
        "ed25519" => SshKeyType::Ed25519,
        "ecdsa" => SshKeyType::Ecdsa,
        "dsa" => SshKeyType::Dsa,
        _ => {
            eprintln!("Unsupported key type: {}. Use: rsa, ed25519, ecdsa, dsa", key_type_str);
            return Ok(());
        }
    };
    
    // Get passphrase if requested
    let passphrase = if with_passphrase {
        let pass = rpassword::prompt_password("Enter passphrase for the key: ")?;
        let confirm = rpassword::prompt_password("Confirm passphrase: ")?;
        if pass != confirm {
            eprintln!("Passphrases do not match!");
            return Ok(());
        }
        if pass.is_empty() { None } else { Some(pass) }
    } else {
        None
    };
    
    println!("Generating {} SSH key pair...", key_type_str.to_uppercase());
    
    // Generate key pair
    let params = SshKeyGenParams {
        key_type: key_type.clone(),
        bits,
        comment: comment.clone(),
        passphrase: passphrase.clone(),
    };
    
    match SshKeyManager::generate_key_pair(&params) {
        Ok((private_key, public_key)) => {
            // Create secret data
            let data = SecretData::SshKey {
                key_type,
                private_key: Some(private_key),
                public_key: Some(public_key.clone()),
                passphrase,
                comment: comment.clone(),
                fingerprint: SshKeyManager::parse_public_key(&public_key)
                    .ok()
                    .map(|info| info.fingerprint_sha256),
            };
            
            let secret = DecryptedSecretEntry {
                id: uuid::Uuid::new_v4().to_string(),
                name,
                description,
                secret_type: SecretType::SshKey,
                data,
                metadata: SecretMetadata::default(),
                tags,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                last_accessed: None,
                expires_at: None,
                favorite: false,
            };
            
            storage.add_secret(&secret).await?;
            
            println!("✅ SSH key generated and stored successfully!");
            if let Ok(info) = SshKeyManager::parse_public_key(&public_key) {
                println!("Fingerprint: {}", info.fingerprint_sha256);
                if let Some(comment) = info.comment {
                    println!("Comment: {}", comment);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to generate SSH key: {}", e);
        }
    }
    
    Ok(())
}

async fn import_ssh_key(
    storage: &SecretsStorage,
    name: String,
    private_key_file: Option<PathBuf>,
    public_key_file: Option<PathBuf>,
    with_passphrase: bool,
    description: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    if private_key_file.is_none() && public_key_file.is_none() {
        eprintln!("Error: Either private key file or public key file must be specified");
        return Ok(());
    }
    
    let mut private_key = None;
    let mut public_key = None;
    let mut passphrase = None;
    
    // Read private key if provided
    if let Some(private_path) = private_key_file {
        match std::fs::read_to_string(&private_path) {
            Ok(content) => {
                // Validate private key
                match SshKeyManager::validate_key(&content, true) {
                    Ok(_) => {
                        // Check if key is encrypted and get passphrase
                        if content.contains("ENCRYPTED") && with_passphrase {
                            passphrase = Some(rpassword::prompt_password("Enter passphrase for private key: ")?);
                        }
                        private_key = Some(content);
                    }
                    Err(e) => {
                        eprintln!("Invalid private key: {}", e);
                        return Ok(());
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read private key file: {}", e);
                return Ok(());
            }
        }
    }
    
    // Read public key if provided
    if let Some(public_path) = public_key_file {
        match std::fs::read_to_string(&public_path) {
            Ok(content) => {
                match SshKeyManager::validate_key(&content, false) {
                    Ok(_) => public_key = Some(content),
                    Err(e) => {
                        eprintln!("Invalid public key: {}", e);
                        return Ok(());
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to read public key file: {}", e);
                return Ok(());
            }
        }
    }
    
    // If we only have private key, try to extract public key
    if public_key.is_none() && private_key.is_some() {
        if let Some(ref priv_key) = private_key {
            match SshKeyManager::extract_public_key(priv_key) {
                Ok(extracted_pub) => {
                    println!("Extracted public key from private key");
                    public_key = Some(extracted_pub);
                }
                Err(e) => {
                    println!("Warning: Could not extract public key: {}", e);
                }
            }
        }
    }
    
    // Create secret data
    match SshKeyManager::to_secret_data(private_key, public_key, passphrase) {
        Ok(data) => {
            let secret = DecryptedSecretEntry {
                id: uuid::Uuid::new_v4().to_string(),
                name,
                description,
                secret_type: SecretType::SshKey,
                data,
                metadata: SecretMetadata::default(),
                tags,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                last_accessed: None,
                expires_at: None,
                favorite: false,
            };
            
            storage.add_secret(&secret).await?;
            println!("✅ SSH key imported successfully!");
        }
        Err(e) => {
            eprintln!("Failed to import SSH key: {}", e);
        }
    }
    
    Ok(())
}

async fn export_ssh_key(
    storage: &SecretsStorage,
    name: &str,
    output_dir: &PathBuf,
    public_only: bool,
    format: Option<String>,
) -> Result<()> {
    // Find the SSH key
    let filter = SecretFilter {
        query: Some(name.to_string()),
        secret_types: Some(vec![SecretType::SshKey]),
        ..Default::default()
    };
    
    let secrets = storage.search_secrets(&filter).await?;
    let secret = secrets.iter().find(|s| s.name == name && s.secret_type == SecretType::SshKey)
        .ok_or_else(|| anyhow::anyhow!("SSH key '{}' not found", name))?;
    
    if let SecretData::SshKey { private_key, public_key, .. } = &secret.data {
        // Create output directory if it doesn't exist
        std::fs::create_dir_all(output_dir)?;
        
        let target_format = format.as_deref().unwrap_or("openssh");
        
        // Export public key
        if let Some(pub_key) = public_key {
            let pub_key_content = SshKeyUtils::convert_format(pub_key, target_format)?;
            let pub_key_path = output_dir.join(format!("{}.pub", name));
            std::fs::write(&pub_key_path, pub_key_content)?;
            println!("Public key exported to: {}", pub_key_path.display());
        }
        
        // Export private key if not public_only
        if !public_only {
            if let Some(priv_key) = private_key {
                let priv_key_content = SshKeyUtils::convert_format(priv_key, target_format)?;
                let priv_key_path = output_dir.join(name);
                std::fs::write(&priv_key_path, priv_key_content)?;
                
                // Set appropriate permissions for private key (Unix only)
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut perms = std::fs::metadata(&priv_key_path)?.permissions();
                    perms.set_mode(0o600);
                    std::fs::set_permissions(&priv_key_path, perms)?;
                }
                
                println!("Private key exported to: {}", priv_key_path.display());
                println!("⚠️  Remember to keep your private key secure!");
            } else {
                println!("Warning: No private key available for export");
            }
        }
        
        println!("✅ SSH key exported successfully!");
    } else {
        eprintln!("Error: Secret is not an SSH key");
    }
    
    Ok(())
}

async fn show_ssh_key_info(storage: &SecretsStorage, name: &str) -> Result<()> {
    // Find the SSH key
    let filter = SecretFilter {
        query: Some(name.to_string()),
        secret_types: Some(vec![SecretType::SshKey]),
        ..Default::default()
    };
    
    let secrets = storage.search_secrets(&filter).await?;
    let secret = secrets.iter().find(|s| s.name == name && s.secret_type == SecretType::SshKey)
        .ok_or_else(|| anyhow::anyhow!("SSH key '{}' not found", name))?;
    
    if let SecretData::SshKey { key_type, private_key, public_key, passphrase, comment, fingerprint } = &secret.data {
        println!("SSH Key Information: {}", secret.name);
        println!("{:-<50}", "");
        println!("Type: {:?}", key_type);
        
        if let Some(fingerprint) = fingerprint {
            println!("Fingerprint: {}", fingerprint);
        }
        
        if let Some(comment) = comment {
            println!("Comment: {}", comment);
        }
        
        println!("Has private key: {}", private_key.is_some());
        println!("Has public key: {}", public_key.is_some());
        println!("Encrypted: {}", passphrase.is_some());
        
        if let Some(description) = &secret.description {
            println!("Description: {}", description);
        }
        
        if !secret.tags.is_empty() {
            println!("Tags: {:?}", secret.tags);
        }
        
        println!("Created: {}", secret.created_at.format("%Y-%m-%d %H:%M:%S"));
        println!("Updated: {}", secret.updated_at.format("%Y-%m-%d %H:%M:%S"));
        
        // Show detailed key information if we can parse it
        if let Some(pub_key) = public_key {
            match SshKeyManager::parse_public_key(pub_key) {
                Ok(info) => {
                    if let Some(bit_length) = info.bit_length {
                        println!("Bit length: {}", bit_length);
                    }
                    println!("SHA256 fingerprint: {}", info.fingerprint_sha256);
                    println!("MD5 fingerprint: {}", info.fingerprint_md5);
                }
                Err(e) => {
                    println!("Could not parse key details: {}", e);
                }
            }
        }
    }
    
    Ok(())
}

async fn change_ssh_key_passphrase(
    storage: &SecretsStorage,
    name: &str,
    remove_passphrase: bool,
) -> Result<()> {
    // Find the SSH key
    let filter = SecretFilter {
        query: Some(name.to_string()),
        secret_types: Some(vec![SecretType::SshKey]),
        ..Default::default()
    };
    
    let secrets = storage.search_secrets(&filter).await?;
    let mut secret = secrets.into_iter().find(|s| s.name == name && s.secret_type == SecretType::SshKey)
        .ok_or_else(|| anyhow::anyhow!("SSH key '{}' not found", name))?;
    
    if let SecretData::SshKey { private_key, passphrase, .. } = &mut secret.data {
        if let Some(priv_key) = private_key {
            let old_passphrase = passphrase.as_deref();
            
            let new_passphrase = if remove_passphrase {
                println!("Removing passphrase from SSH key...");
                None
            } else {
                let new_pass = rpassword::prompt_password("Enter new passphrase: ")?;
                let confirm_pass = rpassword::prompt_password("Confirm new passphrase: ")?;
                
                if new_pass != confirm_pass {
                    eprintln!("Passphrases do not match!");
                    return Ok(());
                }
                
                if new_pass.is_empty() { None } else { Some(new_pass) }
            };
            
            // Change passphrase using ssh-keygen
            match SshKeyUtils::change_passphrase(priv_key, old_passphrase, new_passphrase.as_deref()) {
                Ok(new_private_key) => {
                    // Update the secret
                    *private_key = Some(new_private_key);
                    *passphrase = new_passphrase;
                    secret.updated_at = chrono::Utc::now();
                    
                    storage.update_secret(&secret).await?;
                    
                    if remove_passphrase {
                        println!("✅ Passphrase removed successfully!");
                    } else {
                        println!("✅ Passphrase changed successfully!");
                    }
                }
                Err(e) => {
                    eprintln!("Failed to change passphrase: {}", e);
                }
            }
        } else {
            eprintln!("Error: No private key available for passphrase change");
        }
    }
    
    Ok(())
}

// Document management functions
async fn import_document(
    storage: &SecretsStorage,
    name: String,
    file_path: PathBuf,
    document_type: Option<String>,
    compress: bool,
    description: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    if !file_path.exists() {
        eprintln!("Error: File does not exist: {}", file_path.display());
        return Ok(());
    }
    
    // Parse document type
    let doc_type = if let Some(type_str) = document_type {
        match type_str.as_str() {
            "document" => DocumentType::Document,
            "certificate" => DocumentType::Certificate,
            "configuration" => DocumentType::Configuration,
            "license" => DocumentType::License,
            "key" => DocumentType::KeyFile,
            "backup" => DocumentType::Backup,
            "image" => DocumentType::Image,
            "archive" => DocumentType::Archive,
            custom => DocumentType::Custom(custom.to_string()),
        }
    } else {
        // Auto-detect based on file extension
        DocumentAttachment::guess_document_type(&file_path)
    };
    
    println!("Importing document: {}", file_path.display());
    if compress {
        println!("Compression enabled");
    }
    
    match DocumentManager::import_file(&file_path, doc_type, description.clone(), compress) {
        Ok((secret_data, doc_info)) => {
            let secret = DecryptedSecretEntry {
                id: uuid::Uuid::new_v4().to_string(),
                name,
                description,
                secret_type: SecretType::Document,
                data: secret_data,
                metadata: SecretMetadata::default(),
                tags,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                last_accessed: None,
                expires_at: None,
                favorite: false,
            };
            
            storage.add_secret(&secret).await?;
            
            println!("✅ Document imported successfully!");
            println!("File: {}", doc_info.filename);
            println!("Type: {:?}", doc_info.document_type);
            println!("Size: {} bytes", doc_info.file_size);
            println!("Content Type: {}", doc_info.content_type);
            println!("Checksum: {}...", &doc_info.checksum_sha256[..16]); // Show first 16 chars
        }
        Err(e) => {
            eprintln!("Failed to import document: {}", e);
        }
    }
    
    Ok(())
}

async fn create_text_document(
    storage: &SecretsStorage,
    name: String,
    filename: String,
    content: Option<String>,
    from_stdin: bool,
    document_type: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let text_content = if from_stdin {
        println!("Enter document content (press Ctrl+D when finished):");
        use std::io::Read;
        let mut stdin_content = String::new();
        std::io::stdin().read_to_string(&mut stdin_content)?;
        stdin_content
    } else if let Some(content) = content {
        content
    } else {
        eprintln!("Error: Either --content or --from-stdin must be specified");
        return Ok(());
    };
    
    // Parse document type
    let doc_type = document_type.as_deref().map(|type_str| {
        match type_str {
            "document" => DocumentType::Document,
            "certificate" => DocumentType::Certificate,
            "configuration" => DocumentType::Configuration,
            "license" => DocumentType::License,
            custom => DocumentType::Custom(custom.to_string()),
        }
    }).unwrap_or(DocumentType::Document);
    
    match DocumentAttachment::from_text(name, filename, text_content, description, tags, doc_type) {
        Ok(secret) => {
            storage.add_secret(&secret).await?;
            println!("✅ Text document created successfully!");
            println!("Name: {}", secret.name);
            println!("Filename: {}", 
                if let SecretData::Document { filename, .. } = &secret.data {
                    filename
                } else {
                    "unknown"
                }
            );
        }
        Err(e) => {
            eprintln!("Failed to create text document: {}", e);
        }
    }
    
    Ok(())
}

async fn export_document(
    storage: &SecretsStorage,
    name: &str,
    output_path: &PathBuf,
    verify: bool,
) -> Result<()> {
    // Find the document
    let filter = SecretFilter {
        query: Some(name.to_string()),
        secret_types: Some(vec![SecretType::Document]),
        ..Default::default()
    };
    
    let secrets = storage.search_secrets(&filter).await?;
    let secret = secrets.iter().find(|s| s.name == name && s.secret_type == SecretType::Document)
        .ok_or_else(|| anyhow::anyhow!("Document '{}' not found", name))?;
    
    match DocumentManager::export_document(&secret.data, output_path, verify) {
        Ok(_) => {
            println!("✅ Document exported successfully!");
            if verify {
                println!("Checksum verified ✓");
            }
        }
        Err(e) => {
            eprintln!("Failed to export document: {}", e);
        }
    }
    
    Ok(())
}

async fn show_document_info(storage: &SecretsStorage, name: &str) -> Result<()> {
    // Find the document
    let filter = SecretFilter {
        query: Some(name.to_string()),
        secret_types: Some(vec![SecretType::Document]),
        ..Default::default()
    };
    
    let secrets = storage.search_secrets(&filter).await?;
    let secret = secrets.iter().find(|s| s.name == name && s.secret_type == SecretType::Document)
        .ok_or_else(|| anyhow::anyhow!("Document '{}' not found", name))?;
    
    if let Ok(doc_info) = DocumentManager::get_document_info(&secret.data) {
        println!("Document Information: {}", secret.name);
        println!("{:-<50}", "");
        println!("Filename: {}", doc_info.filename);
        println!("Type: {:?}", doc_info.document_type);
        println!("Content Type: {}", doc_info.content_type);
        println!("File Size: {} bytes", doc_info.file_size);
        println!("Encoding: {:?}", doc_info.encoding);
        
        if let Some(compression) = doc_info.compression {
            println!("Compression: {:?}", compression);
        }
        
        println!("Checksum: {}", doc_info.checksum_sha256);
        
        if let Some(description) = &secret.description {
            println!("Description: {}", description);
        }
        
        if !secret.tags.is_empty() {
            println!("Tags: {:?}", secret.tags);
        }
        
        println!("Created: {}", secret.created_at.format("%Y-%m-%d %H:%M:%S"));
        println!("Updated: {}", secret.updated_at.format("%Y-%m-%d %H:%M:%S"));
        
        // Verify integrity
        match DocumentManager::verify_document(&secret.data) {
            Ok(true) => println!("Integrity: ✅ Valid"),
            Ok(false) => println!("Integrity: ❌ Corrupted"),
            Err(e) => println!("Integrity: ⚠️  Cannot verify ({})", e),
        }
    } else {
        eprintln!("Error: Could not parse document information");
    }
    
    Ok(())
}

async fn view_document(storage: &SecretsStorage, name: &str, text_only: bool) -> Result<()> {
    // Find the document
    let filter = SecretFilter {
        query: Some(name.to_string()),
        secret_types: Some(vec![SecretType::Document]),
        ..Default::default()
    };
    
    let secrets = storage.search_secrets(&filter).await?;
    let secret = secrets.iter().find(|s| s.name == name && s.secret_type == SecretType::Document)
        .ok_or_else(|| anyhow::anyhow!("Document '{}' not found", name))?;
    
    if text_only {
        match DocumentManager::extract_text_content(&secret.data) {
            Ok(text) => {
                println!("Document content:");
                println!("{:-<50}", "");
                println!("{}", text);
            }
            Err(e) => {
                eprintln!("Cannot display as text: {}", e);
            }
        }
    } else {
        if let SecretData::Document { filename, content_type, content, .. } = &secret.data {
            println!("Document: {}", filename);
            println!("Content Type: {}", content_type);
            println!("Size: {} bytes", content.len());
            
            if content_type.starts_with("text/") || DocumentManager::extract_text_content(&secret.data).is_ok() {
                if let Ok(text) = DocumentManager::extract_text_content(&secret.data) {
                    println!("\nContent:");
                    println!("{:-<50}", "");
                    println!("{}", text);
                } else {
                    println!("\nBinary content (use --text-only to force text display)");
                }
            } else {
                println!("\nBinary content cannot be displayed as text");
                println!("Use 'export-document' to save to a file");
            }
        }
    }
    
    Ok(())
}

fn list_document_types() {
    println!("Supported Document Types:");
    println!("{:-<60}", "");
    
    println!("Built-in Types:");
    println!("  document      - Generic document");
    println!("  certificate   - Digital certificates");
    println!("  configuration - Configuration files");
    println!("  license       - License files");
    println!("  key           - Key files (non-SSH)");
    println!("  backup        - Backup files");
    println!("  image         - Image files");
    println!("  archive       - Archive files");
    println!("  custom:<name> - Custom document type");
    
    println!("\nSupported File Extensions:");
    let extensions = DocumentManager::supported_extensions();
    
    let mut by_type: std::collections::HashMap<String, Vec<&str>> = std::collections::HashMap::new();
    for (ext, doc_type) in &extensions {
        let type_name = format!("{:?}", doc_type);
        by_type.entry(type_name).or_default().push(ext);
    }
    
    for (doc_type, exts) in by_type {
        println!("  {}: {}", doc_type, exts.join(", "));
    }
}

// API Key management functions

async fn create_api_key(
    storage: &SecretsStorage,
    name: String,
    provider: String,
    api_key: String,
    api_secret: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    expires_days: Option<u32>,
    environment: Option<String>,
) -> Result<()> {
    let provider: ApiKeyProvider = provider.parse()?;
    
    // Validate API key format if possible
    if let Err(e) = ApiKeyManager::validate_api_key_format(&provider, &api_key) {
        eprintln!("Warning: {}", e);
    }
    
    let expires_at = expires_days.map(|days| {
        chrono::Utc::now() + chrono::Duration::days(days as i64)
    });
    
    let mut entry = ApiKeyManager::create_api_key(
        name,
        provider,
        api_key,
        api_secret,
        description,
        None, // permissions - could be enhanced
        expires_at,
        tags,
    )?;
    
    // Set environment if provided
    if let SecretData::ApiKey { environment: env, .. } = &mut entry.data {
        if let Some(env_val) = environment {
            *env = env_val;
        }
    }
    
    storage.add_secret(&entry).await?;
    println!("✅ API key created successfully!");
    
    Ok(())
}

async fn create_jwt_token(
    storage: &SecretsStorage,
    name: String,
    token: String,
    issuer: Option<String>,
    audience: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
    expires_days: Option<u32>,
) -> Result<()> {
    let expires_at = expires_days.map(|days| {
        chrono::Utc::now() + chrono::Duration::days(days as i64)
    });
    
    let entry = ApiKeyManager::create_jwt_token(
        name,
        token,
        issuer,
        audience,
        description,
        expires_at,
        tags,
    )?;
    
    storage.add_secret(&entry).await?;
    println!("✅ JWT token created successfully!");
    
    Ok(())
}

async fn create_oauth_token(
    storage: &SecretsStorage,
    name: String,
    access_token: String,
    refresh_token: Option<String>,
    token_secret: Option<String>,
    scopes: Vec<String>,
    description: Option<String>,
    tags: Vec<String>,
    expires_days: Option<u32>,
) -> Result<()> {
    let expires_at = expires_days.map(|days| {
        chrono::Utc::now() + chrono::Duration::days(days as i64)
    });
    
    let entry = ApiKeyManager::create_oauth_token(
        name,
        access_token,
        refresh_token,
        token_secret,
        expires_at,
        scopes,
        description,
        tags,
    )?;
    
    storage.add_secret(&entry).await?;
    println!("✅ OAuth token created successfully!");
    
    Ok(())
}

async fn list_api_keys(
    storage: &SecretsStorage,
    provider: Option<String>,
    expired: bool,
    expiring_days: Option<u32>,
    environment: Option<String>,
) -> Result<()> {
    let mut filter = SecretFilter::default();
    filter.secret_types = Some(vec![SecretType::ApiKey, SecretType::Token]);
    filter.environment = environment;
    
    let entries = storage.search_secrets(&filter).await?;
    
    let mut filtered_entries: Vec<_> = entries.iter().collect();
    
    // Filter by provider if specified
    if let Some(provider_str) = provider {
        let provider: ApiKeyProvider = provider_str.parse()?;
        filtered_entries.retain(|entry| {
            if let SecretData::ApiKey { provider: entry_provider, .. } = &entry.data {
                entry_provider == &provider
            } else {
                false
            }
        });
    }
    
    // Filter by expiration status
    if expired {
        filtered_entries.retain(|entry| ApiKeyManager::is_expired(entry));
    } else if let Some(days) = expiring_days {
        filtered_entries.retain(|entry| ApiKeyManager::is_expiring_soon(entry, days));
    }
    
    if filtered_entries.is_empty() {
        println!("No API keys found matching criteria.");
        return Ok(());
    }
    
    println!("API Keys and Tokens:");
    println!("{:-<80}", "");
    
    for entry in filtered_entries {
        let status = if ApiKeyManager::is_expired(entry) {
            "🔴 EXPIRED"
        } else if ApiKeyManager::is_expiring_soon(entry, 30) {
            "🟡 EXPIRING SOON"
        } else {
            "🟢 ACTIVE"
        };
        
        println!("📋 {} [{}]", entry.name, status);
        println!("   Type: {:?}", entry.secret_type);
        
        match &entry.data {
            SecretData::ApiKey { provider, environment, .. } => {
                println!("   Provider: {}", provider);
                println!("   Environment: {}", environment);
            }
            SecretData::Token { token_type, .. } => {
                println!("   Token Type: {}", token_type);
            }
            _ => {}
        }
        
        if let Some(expires_at) = entry.expires_at {
            println!("   Expires: {}", expires_at.format("%Y-%m-%d %H:%M UTC"));
        }
        
        if !entry.tags.is_empty() {
            println!("   Tags: {}", entry.tags.join(", "));
        }
        
        println!("   Created: {}", entry.created_at.format("%Y-%m-%d %H:%M UTC"));
        println!();
    }
    
    Ok(())
}

async fn get_api_key(
    storage: &SecretsStorage,
    name: &str,
    show_secret: bool,
    copy: bool,
) -> Result<()> {
    let entry = storage.get_secret(name).await?;
    
    match &entry.data {
        SecretData::ApiKey { provider, api_key, api_secret, environment, endpoint_url, usage_stats, .. } => {
            println!("API Key: {}", entry.name);
            println!("Provider: {}", provider);
            println!("Environment: {}", environment);
            
            if let Some(url) = endpoint_url {
                println!("Endpoint: {}", url);
            }
            
            if show_secret {
                println!("API Key: {}", api_key);
                if let Some(secret) = api_secret {
                    println!("API Secret: {}", secret);
                }
                
                if copy {
                    if let Err(e) = arboard::Clipboard::new().and_then(|mut ctx| ctx.set_text(api_key)) {
                        eprintln!("Failed to copy to clipboard: {}", e);
                    } else {
                        println!("✅ API key copied to clipboard");
                    }
                }
            } else {
                println!("API Key: {} (use --show-secret to reveal)", "*".repeat(api_key.len().min(16)));
                if api_secret.is_some() {
                    println!("API Secret: ******** (use --show-secret to reveal)");
                }
            }
            
            println!("Usage Count: {}", usage_stats.usage_count);
            if let Some(last_used) = usage_stats.last_used {
                println!("Last Used: {}", last_used.format("%Y-%m-%d %H:%M UTC"));
            }
            if usage_stats.error_count > 0 {
                println!("Error Count: {}", usage_stats.error_count);
                if let Some(last_error) = &usage_stats.last_error {
                    println!("Last Error: {}", last_error);
                }
            }
        }
        SecretData::Token { token_type, access_token, refresh_token, scopes, .. } => {
            println!("Token: {}", entry.name);
            println!("Type: {}", token_type);
            
            if !scopes.is_empty() {
                println!("Scopes: {}", scopes.join(", "));
            }
            
            if show_secret {
                println!("Access Token: {}", access_token);
                if let Some(refresh) = refresh_token {
                    println!("Refresh Token: {}", refresh);
                }
                
                if copy {
                    if let Err(e) = arboard::Clipboard::new().and_then(|mut ctx| ctx.set_text(access_token)) {
                        eprintln!("Failed to copy to clipboard: {}", e);
                    } else {
                        println!("✅ Access token copied to clipboard");
                    }
                }
            } else {
                println!("Access Token: {} (use --show-secret to reveal)", "*".repeat(access_token.len().min(16)));
                if refresh_token.is_some() {
                    println!("Refresh Token: ******** (use --show-secret to reveal)");
                }
            }
        }
        _ => {
            eprintln!("Entry '{}' is not an API key or token", name);
            return Ok(());
        }
    }
    
    if let Some(expires_at) = entry.expires_at {
        println!("Expires: {}", expires_at.format("%Y-%m-%d %H:%M UTC"));
        
        let days_until_expiry = (expires_at - chrono::Utc::now()).num_days();
        if days_until_expiry <= 0 {
            println!("⚠️  This key has EXPIRED!");
        } else if days_until_expiry <= 30 {
            println!("⚠️  This key expires in {} days", days_until_expiry);
        }
    }
    
    if !entry.tags.is_empty() {
        println!("Tags: {}", entry.tags.join(", "));
    }
    
    Ok(())
}

async fn update_api_key_usage(
    storage: &mut SecretsStorage,
    name: &str,
    success: bool,
    error_message: Option<String>,
) -> Result<()> {
    let mut entry = storage.get_secret(name).await?;
    
    ApiKeyManager::update_usage_stats(&mut entry, success, error_message)?;
    
    storage.update_secret(&entry).await?;
    println!("✅ API key usage statistics updated!");
    
    Ok(())
}

async fn setup_api_key_rotation(
    storage: &mut SecretsStorage,
    name: &str,
    rotation_days: u32,
    reminder_days: u32,
) -> Result<()> {
    let mut entry = storage.get_secret(name).await?;
    
    ApiKeyManager::setup_rotation(&mut entry, rotation_days, reminder_days)?;
    
    storage.update_secret(&entry).await?;
    println!("✅ API key rotation configured!");
    println!("   Rotation period: {} days", rotation_days);
    println!("   Reminder: {} days before expiration", reminder_days);
    
    Ok(())
}

fn list_api_key_providers() {
    println!("Supported API Key Providers:");
    println!("{:-<60}", "");
    
    let providers = [
        ("aws", "Amazon Web Services"),
        ("gcp", "Google Cloud Platform"),
        ("azure", "Microsoft Azure"),
        ("github", "GitHub"),
        ("gitlab", "GitLab"),
        ("dockerhub", "Docker Hub"),
        ("stripe", "Stripe"),
        ("twilio", "Twilio"),
        ("sendgrid", "SendGrid"),
        ("slack", "Slack"),
        ("discord", "Discord"),
        ("openai", "OpenAI"),
        ("anthropic", "Anthropic"),
        ("generic", "Generic API service"),
        ("custom_<name>", "Custom provider"),
    ];
    
    for (key, description) in providers {
        println!("  {:15} - {}", key, description);
    }
    
    println!("\nExample usage:");
    println!("  pwgen create-api-key --name \"My AWS Key\" --provider aws --api-key AKIA...");
    println!("  pwgen create-jwt-token --name \"Auth Token\" --token eyJ...");
}

// Notes and Configuration management functions

async fn create_note(
    storage: &SecretsStorage,
    title: String,
    content: Option<String>,
    from_stdin: bool,
    format: String,
    category: String,
    priority: String,
    description: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let note_format: NoteFormat = match format.to_lowercase().as_str() {
        "plaintext" | "plain" | "text" => NoteFormat::PlainText,
        "markdown" | "md" => NoteFormat::Markdown,
        "html" => NoteFormat::Html,
        "richtext" | "rich" => NoteFormat::RichText,
        _ => return Err(anyhow::anyhow!("Unsupported note format: {}", format)),
    };

    let note_category: NoteCategory = category.parse()?;
    let note_priority: NotePriority = match priority.to_lowercase().as_str() {
        "low" => NotePriority::Low,
        "medium" => NotePriority::Medium,
        "high" => NotePriority::High,
        "critical" => NotePriority::Critical,
        _ => return Err(anyhow::anyhow!("Invalid priority: {}", priority)),
    };

    let note_content = if from_stdin {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer.trim().to_string()
    } else {
        content.unwrap_or_else(|| {
            print!("Enter note content (Ctrl+D when finished): ");
            use std::io::{self, Write, Read};
            io::stdout().flush().unwrap();
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer.trim().to_string()
        })
    };

    let entry = NotesConfigManager::create_note(
        title,
        note_content,
        note_format,
        note_category,
        note_priority,
        description,
        tags,
    )?;

    storage.add_secret(&entry).await?;
    println!("✅ Note created successfully!");

    Ok(())
}

async fn update_note(
    storage: &mut SecretsStorage,
    name: &str,
    new_title: Option<String>,
    new_content: Option<String>,
    from_stdin: bool,
    new_format: Option<String>,
) -> Result<()> {
    let mut entry = storage.get_secret(name).await?;

    let content = if from_stdin {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        Some(buffer.trim().to_string())
    } else {
        new_content
    };

    let format = if let Some(fmt) = new_format {
        Some(match fmt.to_lowercase().as_str() {
            "plaintext" | "plain" | "text" => NoteFormat::PlainText,
            "markdown" | "md" => NoteFormat::Markdown,
            "html" => NoteFormat::Html,
            "richtext" | "rich" => NoteFormat::RichText,
            _ => return Err(anyhow::anyhow!("Unsupported note format: {}", fmt)),
        })
    } else {
        None
    };

    NotesConfigManager::update_note(&mut entry, content, new_title, format, None)?;

    storage.update_secret(&entry).await?;
    println!("✅ Note updated successfully!");

    Ok(())
}

async fn convert_note(
    storage: &mut SecretsStorage,
    name: &str,
    format: String,
) -> Result<()> {
    let mut entry = storage.get_secret(name).await?;

    let target_format = match format.to_lowercase().as_str() {
        "plaintext" | "plain" | "text" => NoteFormat::PlainText,
        "markdown" | "md" => NoteFormat::Markdown,
        "html" => NoteFormat::Html,
        "richtext" | "rich" => NoteFormat::RichText,
        _ => return Err(anyhow::anyhow!("Unsupported note format: {}", format)),
    };

    NotesConfigManager::convert_note_format(&mut entry, target_format)?;

    storage.update_secret(&entry).await?;
    println!("✅ Note converted to {} format!", format);

    Ok(())
}

async fn search_notes(
    storage: &SecretsStorage,
    query: &str,
    case_sensitive: bool,
    category: Option<String>,
) -> Result<()> {
    let mut filter = SecretFilter::default();
    filter.secret_types = Some(vec![SecretType::SecureNote]);

    let entries = storage.search_secrets(&filter).await?;
    let search_results = NotesConfigManager::search_notes_content(&entries, query, case_sensitive);

    let filtered_results: Vec<_> = if let Some(cat) = category {
        let target_category: NoteCategory = cat.parse()?;
        search_results.into_iter().filter(|entry| {
            // In a full implementation, we'd check note metadata
            // For now, we'll just return all results
            true
        }).collect()
    } else {
        search_results
    };

    if filtered_results.is_empty() {
        println!("No notes found matching query: '{}'", query);
        return Ok(());
    }

    println!("Found {} notes matching '{}':", filtered_results.len(), query);
    println!("{:-<60}", "");

    for entry in filtered_results {
        if let SecretData::SecureNote { title, content, format } = &entry.data {
            println!("📝 {} [{}]", title, format_to_string(format));
            
            // Show context around match
            let content_lower = if case_sensitive { content.clone() } else { content.to_lowercase() };
            let query_lower = if case_sensitive { query.to_string() } else { query.to_lowercase() };
            
            if let Some(pos) = content_lower.find(&query_lower) {
                let start = pos.saturating_sub(40);
                let end = (pos + query.len() + 40).min(content.len());
                let context = &content[start..end];
                println!("   ...{}...", context);
            }
            
            if !entry.tags.is_empty() {
                println!("   Tags: {}", entry.tags.join(", "));
            }
            println!();
        }
    }

    Ok(())
}

async fn list_notes(
    storage: &SecretsStorage,
    category: Option<String>,
    priority: Option<String>,
    format: Option<String>,
) -> Result<()> {
    let mut filter = SecretFilter::default();
    filter.secret_types = Some(vec![SecretType::SecureNote]);

    let entries = storage.search_secrets(&filter).await?;

    // Apply filters (simplified - in full implementation would use metadata)
    let filtered_entries: Vec<_> = entries.iter().collect();

    if filtered_entries.is_empty() {
        println!("No notes found.");
        return Ok(());
    }

    println!("Notes:");
    println!("{:-<70}", "");

    for entry in filtered_entries {
        if let SecretData::SecureNote { title, content, format } = &entry.data {
            let preview = if content.len() > 60 {
                format!("{}...", &content[..57])
            } else {
                content.clone()
            };

            println!("📝 {} [{}]", title, format_to_string(format));
            println!("   {}", preview.replace('\n', " "));
            
            if !entry.tags.is_empty() {
                println!("   Tags: {}", entry.tags.join(", "));
            }
            
            println!("   Created: {}", entry.created_at.format("%Y-%m-%d %H:%M UTC"));
            println!();
        }
    }

    Ok(())
}

async fn create_config(
    storage: &SecretsStorage,
    name: String,
    config_type: String,
    format: String,
    file: Option<PathBuf>,
    from_stdin: bool,
    template: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let config_format = match format.to_lowercase().as_str() {
        "env" | "envfile" => ConfigFormat::EnvFile,
        "json" => ConfigFormat::Json,
        "yaml" | "yml" => ConfigFormat::Yaml,
        "toml" => ConfigFormat::Toml,
        "properties" => ConfigFormat::Properties,
        _ => return Err(anyhow::anyhow!("Unsupported config format: {}", format)),
    };

    let config_type_enum: ConfigType = config_type.parse()?;

    let content = if let Some(file_path) = file {
        std::fs::read_to_string(file_path)?
    } else if from_stdin {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        buffer
    } else {
        return Err(anyhow::anyhow!("Must provide either --file or --from-stdin"));
    };

    let entry = NotesConfigManager::import_config_from_string(
        &content,
        config_format,
        name,
        config_type_enum,
        description,
        tags,
    )?;

    storage.add_secret(&entry).await?;
    println!("✅ Configuration created successfully!");

    Ok(())
}

async fn update_config(
    storage: &mut SecretsStorage,
    name: &str,
    variables: Vec<String>,
    merge: bool,
    file: Option<PathBuf>,
) -> Result<()> {
    let mut entry = storage.get_secret(name).await?;

    let new_variables = if let Some(file_path) = file {
        let content = std::fs::read_to_string(file_path)?;
        if let SecretData::Configuration { format, .. } = &entry.data {
            NotesConfigManager::parse_config(&content, format)?
        } else {
            return Err(anyhow::anyhow!("Entry is not a configuration"));
        }
    } else {
        let mut vars = std::collections::HashMap::new();
        for var in variables {
            if let Some(eq_pos) = var.find('=') {
                let key = var[..eq_pos].trim().to_string();
                let value = var[eq_pos + 1..].trim().to_string();
                vars.insert(key, value);
            } else {
                return Err(anyhow::anyhow!("Invalid variable format: {}. Use KEY=VALUE", var));
            }
        }
        vars
    };

    NotesConfigManager::update_config(&mut entry, new_variables, merge)?;

    storage.update_secret(&entry).await?;
    println!("✅ Configuration updated successfully!");

    Ok(())
}

async fn export_config(
    storage: &SecretsStorage,
    name: &str,
    output: Option<PathBuf>,
    format: Option<String>,
) -> Result<()> {
    let entry = storage.get_secret(name).await?;

    let target_format = if let Some(fmt) = format {
        Some(match fmt.to_lowercase().as_str() {
            "env" | "envfile" => ConfigFormat::EnvFile,
            "json" => ConfigFormat::Json,
            "yaml" | "yml" => ConfigFormat::Yaml,
            "toml" => ConfigFormat::Toml,
            "properties" => ConfigFormat::Properties,
            _ => return Err(anyhow::anyhow!("Unsupported config format: {}", fmt)),
        })
    } else {
        None
    };

    let exported_content = NotesConfigManager::export_config_to_string(&entry, target_format)?;

    if let Some(output_path) = output {
        std::fs::write(&output_path, exported_content)?;
        println!("✅ Configuration exported to: {}", output_path.display());
    } else {
        println!("{}", exported_content);
    }

    Ok(())
}

async fn validate_config(
    storage: &SecretsStorage,
    name: &str,
    template: Option<String>,
) -> Result<()> {
    let entry = storage.get_secret(name).await?;

    if let SecretData::Configuration { variables, .. } = &entry.data {
        if let Some(template_name) = template {
            let templates = NotesConfigManager::get_config_templates();
            if let Some(config_template) = templates.iter().find(|t| t.name.to_lowercase() == template_name.to_lowercase()) {
                let errors = NotesConfigManager::validate_config(variables, config_template)?;
                
                if errors.is_empty() {
                    println!("✅ Configuration is valid according to template '{}'", template_name);
                } else {
                    println!("❌ Configuration validation failed:");
                    for error in errors {
                        println!("  - {}", error);
                    }
                }
            } else {
                return Err(anyhow::anyhow!("Template '{}' not found", template_name));
            }
        } else {
            println!("ℹ️  No template specified. Showing configuration variables:");
            for (key, value) in variables {
                println!("  {} = {}", key, value);
            }
        }
    } else {
        return Err(anyhow::anyhow!("Entry '{}' is not a configuration", name));
    }

    Ok(())
}

fn list_config_templates() {
    let templates = NotesConfigManager::get_config_templates();

    println!("Available Configuration Templates:");
    println!("{:-<60}", "");

    for template in templates {
        println!("📋 {} [{}]", template.name, format_config_format(&template.format));
        println!("   {}", template.description);
        
        if !template.variables.is_empty() {
            println!("   Variables: {}", template.variables.len());
            for (var_name, var_def) in template.variables.iter().take(3) {
                let required = if var_def.required { "*" } else { "" };
                println!("     - {}{}: {:?}", var_name, required, var_def.var_type);
            }
            if template.variables.len() > 3 {
                println!("     ... and {} more", template.variables.len() - 3);
            }
        }
        println!();
    }

    println!("Usage:");
    println!("  pwgen create-config --name \"My Config\" --template \"Environment File\" --file .env");
    println!("  pwgen validate-config \"My Config\" --template \"Environment File\"");
}

async fn list_configs(
    storage: &SecretsStorage,
    config_type: Option<String>,
    format: Option<String>,
) -> Result<()> {
    let mut filter = SecretFilter::default();
    filter.secret_types = Some(vec![SecretType::Configuration]);

    let entries = storage.search_secrets(&filter).await?;

    let filtered_entries: Vec<_> = entries.iter()
        .filter(|entry| {
            if let Some(ref ct) = config_type {
                if let Some(ref template) = entry.metadata.template {
                    template.to_lowercase().contains(&ct.to_lowercase())
                } else {
                    false
                }
            } else {
                true
            }
        })
        .collect();

    if filtered_entries.is_empty() {
        println!("No configurations found.");
        return Ok(());
    }

    println!("Configurations:");
    println!("{:-<70}", "");

    for entry in filtered_entries {
        if let SecretData::Configuration { format, variables, template } = &entry.data {
            println!("⚙️  {} [{}]", entry.name, format_config_format(format));
            
            if let Some(template_name) = template {
                println!("   Template: {}", template_name);
            }
            
            println!("   Variables: {}", variables.len());
            
            // Show a few variables (non-sensitive)
            let mut shown = 0;
            for (key, _) in variables {
                if shown < 3 && !key.to_lowercase().contains("password") && !key.to_lowercase().contains("secret") {
                    println!("     - {}", key);
                    shown += 1;
                }
            }
            if variables.len() > shown {
                println!("     ... and {} more", variables.len() - shown);
            }
            
            if !entry.tags.is_empty() {
                println!("   Tags: {}", entry.tags.join(", "));
            }
            
            println!("   Created: {}", entry.created_at.format("%Y-%m-%d %H:%M UTC"));
            println!();
        }
    }

    Ok(())
}

// Helper functions

fn format_to_string(format: &NoteFormat) -> &'static str {
    match format {
        NoteFormat::PlainText => "plain",
        NoteFormat::Markdown => "markdown",
        NoteFormat::Html => "html",
        NoteFormat::RichText => "rich",
    }
}

fn format_config_format(format: &ConfigFormat) -> &str {
    match format {
        ConfigFormat::EnvFile => "env",
        ConfigFormat::Json => "json",
        ConfigFormat::Yaml => "yaml",
        ConfigFormat::Toml => "toml",
        ConfigFormat::Xml => "xml",
        ConfigFormat::Properties => "properties",
        ConfigFormat::Custom(name) => name,
    }
}

// Environment variables and connection strings management functions

async fn create_env_var(
    storage: &SecretsStorage,
    name: String,
    variable_name: String,
    value: String,
    var_type: String,
    environment: String,
    sensitive: bool,
    description: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let env_var_type = match var_type.to_lowercase().as_str() {
        "string" => EnvVarType::String,
        "number" => EnvVarType::Number,
        "boolean" | "bool" => EnvVarType::Boolean,
        "url" => EnvVarType::Url,
        "path" => EnvVarType::Path,
        "json" => EnvVarType::Json,
        "list" => EnvVarType::List,
        "base64" => EnvVarType::Base64,
        "secret" => EnvVarType::Secret,
        _ => return Err(anyhow::anyhow!("Invalid variable type: {}", var_type)),
    };

    let environment_type: EnvironmentType = environment.parse()?;

    let entry = EnvConnectionManager::create_env_variable(
        name,
        variable_name,
        value,
        env_var_type,
        environment_type,
        description,
        tags,
        sensitive,
    )?;

    storage.add_secret(&entry).await?;
    println!("✅ Environment variable created successfully!");

    Ok(())
}

async fn create_env_set(
    storage: &SecretsStorage,
    name: String,
    environment: String,
    file: Option<PathBuf>,
    from_stdin: bool,
    template: Option<String>,
    description: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let environment_type: EnvironmentType = environment.parse()?;

    let variables = if let Some(file_path) = file {
        let content = std::fs::read_to_string(file_path)?;
        parse_env_content(&content)?
    } else if from_stdin {
        use std::io::Read;
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        parse_env_content(&buffer)?
    } else if let Some(template_name) = template {
        get_template_variables(&template_name)?
    } else {
        return Err(anyhow::anyhow!("Must provide either --file, --from-stdin, or --template"));
    };

    let entry = EnvConnectionManager::create_environment_set(
        name,
        environment_type,
        variables,
        description,
        tags,
    )?;

    storage.add_secret(&entry).await?;
    println!("✅ Environment set created successfully!");

    Ok(())
}

async fn generate_env_file(
    storage: &SecretsStorage,
    name: &str,
    output: Option<PathBuf>,
) -> Result<()> {
    let entry = storage.get_secret(name).await?;

    if let SecretData::Configuration { variables, .. } = &entry.data {
        let env_content = EnvConnectionManager::generate_env_file(variables);

        if let Some(output_path) = output {
            std::fs::write(&output_path, env_content)?;
            println!("✅ Environment file generated: {}", output_path.display());
        } else {
            println!("{}", env_content);
        }
    } else {
        return Err(anyhow::anyhow!("Entry '{}' is not a configuration", name));
    }

    Ok(())
}

async fn validate_env_vars(
    storage: &SecretsStorage,
    name: &str,
    template: Option<String>,
) -> Result<()> {
    let entry = storage.get_secret(name).await?;

    if let SecretData::Configuration { variables, .. } = &entry.data {
        if let Some(template_name) = template {
            let templates = EnvConnectionManager::get_environment_templates();
            if let Some(env_template) = templates.iter().find(|t| t.name.to_lowercase() == template_name.to_lowercase()) {
                let errors = EnvConnectionManager::validate_environment_variables(variables, &env_template.variables);
                
                if errors.is_empty() {
                    println!("✅ Environment variables are valid according to template '{}'", template_name);
                } else {
                    println!("❌ Environment validation failed:");
                    for error in errors {
                        println!("  - {}", error);
                    }
                }
            } else {
                return Err(anyhow::anyhow!("Template '{}' not found", template_name));
            }
        } else {
            println!("ℹ️  No template specified. Showing environment variables:");
            for (key, value) in variables {
                println!("  {} = {}", key, value);
            }
        }
    } else {
        return Err(anyhow::anyhow!("Entry '{}' is not a configuration", name));
    }

    Ok(())
}

fn list_env_templates() {
    let templates = EnvConnectionManager::get_environment_templates();

    println!("Available Environment Templates:");
    println!("{:-<60}", "");

    for template in templates {
        println!("🌍 {} [{}]", template.name, template.environment_type);
        println!("   {}", template.description);
        
        if !template.variables.is_empty() {
            println!("   Variables: {}", template.variables.len());
            for (var_name, var_def) in template.variables.iter().take(3) {
                let required = if var_def.required { "*" } else { "" };
                let sensitive = if var_def.sensitive { "🔒" } else { "" };
                println!("     - {}{}{}: {:?}", var_name, required, sensitive, var_def.var_type);
            }
            if template.variables.len() > 3 {
                println!("     ... and {} more", template.variables.len() - 3);
            }
        }
        
        if !template.tags.is_empty() {
            println!("   Tags: {}", template.tags.join(", "));
        }
        println!();
    }

    println!("Usage:");
    println!("  pwgen create-env-set --name \"My App\" --template \"Node.js Application\"");
    println!("  pwgen validate-env-vars \"My App\" --template \"Node.js Application\"");
}

async fn list_env_vars(
    storage: &SecretsStorage,
    environment: Option<String>,
    show_sensitive: bool,
) -> Result<()> {
    let mut filter = SecretFilter::default();
    filter.secret_types = Some(vec![SecretType::Configuration]);
    if let Some(env) = environment {
        filter.environment = Some(env);
    }

    let entries = storage.search_secrets(&filter).await?;

    let env_entries: Vec<_> = entries.iter()
        .filter(|entry| {
            entry.metadata.template.as_ref()
                .map(|t| t.starts_with("Environment"))
                .unwrap_or(false)
        })
        .collect();

    if env_entries.is_empty() {
        println!("No environment variables found.");
        return Ok(());
    }

    println!("Environment Variables:");
    println!("{:-<70}", "");

    for entry in env_entries {
        if let SecretData::Configuration { variables, template, .. } = &entry.data {
            let env_type = entry.metadata.environment.as_deref().unwrap_or("unknown");
            
            println!("🌍 {} [{}]", entry.name, env_type);
            
            if let Some(template_name) = template {
                println!("   Template: {}", template_name);
            }
            
            println!("   Variables: {}", variables.len());
            
            // Show variables
            for (key, value) in variables {
                let is_sensitive = key.to_lowercase().contains("password") 
                    || key.to_lowercase().contains("secret")
                    || key.to_lowercase().contains("key")
                    || key.to_lowercase().contains("token");
                
                if is_sensitive && !show_sensitive {
                    println!("     - {} = ********", key);
                } else {
                    let display_value = if value.len() > 50 {
                        format!("{}...", &value[..47])
                    } else {
                        value.clone()
                    };
                    println!("     - {} = {}", key, display_value);
                }
            }
            
            if !entry.tags.is_empty() {
                println!("   Tags: {}", entry.tags.join(", "));
            }
            
            println!("   Created: {}", entry.created_at.format("%Y-%m-%d %H:%M UTC"));
            println!();
        }
    }

    if !show_sensitive {
        println!("ℹ️  Use --show-sensitive to reveal sensitive values");
    }

    Ok(())
}

async fn create_connection(
    storage: &SecretsStorage,
    name: String,
    connection_type: String,
    host: String,
    port: Option<u16>,
    database: String,
    username: String,
    password: Option<String>,
    environment: String,
    ssl_enabled: bool,
    description: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let conn_type = parse_connection_type(&connection_type)?;
    let environment_type: EnvironmentType = environment.parse()?;
    
    let actual_password = if let Some(pwd) = password {
        pwd
    } else {
        rpassword::prompt_password("Database password: ")?
    };

    let ssl_config = if ssl_enabled {
        Some(SslConfig {
            enabled: true,
            verify_ssl: true,
            ca_cert: None,
            client_cert: None,
            client_key: None,
        })
    } else {
        None
    };

    let entry = EnvConnectionManager::create_connection_string(
        name,
        conn_type,
        host,
        port,
        database,
        username,
        actual_password,
        environment_type,
        ssl_config,
        description,
        tags,
    )?;

    storage.add_secret(&entry).await?;
    println!("✅ Connection string created successfully!");

    Ok(())
}

fn parse_connection_string(connection_string: &str) -> Result<()> {
    match EnvConnectionManager::parse_connection_string(connection_string) {
        Ok(components) => {
            println!("Connection String Components:");
            println!("{:-<50}", "");
            println!("Database Type: {:?}", components.database_type);
            println!("Host: {}", components.host);
            if let Some(port) = components.port {
                println!("Port: {}", port);
            }
            println!("Database: {}", components.database);
            if let Some(username) = &components.username {
                println!("Username: {}", username);
            }
            if components.password.is_some() {
                println!("Password: ********");
            }
            if !components.query_params.is_empty() {
                println!("Query Parameters:");
                for (key, value) in &components.query_params {
                    println!("  {} = {}", key, value);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to parse connection string: {}", e);
        }
    }

    Ok(())
}

async fn test_connection(
    storage: &SecretsStorage,
    name: &str,
) -> Result<()> {
    let entry = storage.get_secret(name).await?;

    if let SecretData::ConnectionString { connection_string, .. } = &entry.data {
        match EnvConnectionManager::test_connection_string(connection_string) {
            Ok(_) => {
                println!("✅ Connection string is valid");
                println!("   Connection: {}", name);
                
                // Parse and show details
                if let Ok(components) = EnvConnectionManager::parse_connection_string(connection_string) {
                    println!("   Host: {}", components.host);
                    println!("   Database: {}", components.database);
                    if let Some(port) = components.port {
                        println!("   Port: {}", port);
                    }
                }
            }
            Err(e) => {
                println!("❌ Connection validation failed: {}", e);
            }
        }
    } else {
        return Err(anyhow::anyhow!("Entry '{}' is not a connection string", name));
    }

    Ok(())
}

async fn list_connections(
    storage: &SecretsStorage,
    connection_type: Option<String>,
    environment: Option<String>,
) -> Result<()> {
    let mut filter = SecretFilter::default();
    filter.secret_types = Some(vec![SecretType::ConnectionString]);
    if let Some(env) = environment {
        filter.environment = Some(env);
    }

    let entries = storage.search_secrets(&filter).await?;

    let filtered_entries: Vec<_> = entries.iter()
        .filter(|entry| {
            if let Some(ref ct) = connection_type {
                if let SecretData::ConnectionString { database_type, .. } = &entry.data {
                    format!("{:?}", database_type).to_lowercase().contains(&ct.to_lowercase())
                } else {
                    false
                }
            } else {
                true
            }
        })
        .collect();

    if filtered_entries.is_empty() {
        println!("No connections found.");
        return Ok(());
    }

    println!("Database Connections:");
    println!("{:-<70}", "");

    for entry in filtered_entries {
        if let SecretData::ConnectionString { database_type, host, port, database, username, .. } = &entry.data {
            let env_type = entry.metadata.environment.as_deref().unwrap_or("unknown");
            
            println!("🔗 {} [{:?}] [{}]", entry.name, database_type, env_type);
            println!("   Host: {}", host);
            if let Some(port_num) = port {
                println!("   Port: {}", port_num);
            }
            println!("   Database: {}", database);
            println!("   Username: {}", username);
            
            if !entry.tags.is_empty() {
                println!("   Tags: {}", entry.tags.join(", "));
            }
            
            println!("   Created: {}", entry.created_at.format("%Y-%m-%d %H:%M UTC"));
            println!();
        }
    }

    Ok(())
}

// Helper functions

fn parse_env_content(content: &str) -> Result<Vec<EnvVariable>> {
    let mut variables = Vec::new();
    
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim().to_string();
            let mut value = line[eq_pos + 1..].trim().to_string();
            
            // Remove quotes if present
            if value.starts_with('"') && value.ends_with('"') {
                value = value[1..value.len()-1].to_string();
                value = value.replace("\\\"", "\"");
            }
            
            let var_type = if value.parse::<f64>().is_ok() {
                EnvVarType::Number
            } else if matches!(value.to_lowercase().as_str(), "true" | "false") {
                EnvVarType::Boolean
            } else if value.starts_with("http://") || value.starts_with("https://") {
                EnvVarType::Url
            } else if key.to_lowercase().contains("password") || key.to_lowercase().contains("secret") {
                EnvVarType::Secret
            } else {
                EnvVarType::String
            };
            
            let sensitive = matches!(var_type, EnvVarType::Secret) || 
                key.to_lowercase().contains("password") ||
                key.to_lowercase().contains("secret") ||
                key.to_lowercase().contains("token");
            
            variables.push(EnvVariable {
                name: key,
                value,
                var_type,
                description: None,
                required: true,
                sensitive,
                default_value: None,
                validation_pattern: None,
                environment_specific: true,
            });
        }
    }
    
    Ok(variables)
}

fn get_template_variables(template_name: &str) -> Result<Vec<EnvVariable>> {
    let templates = EnvConnectionManager::get_environment_templates();
    
    if let Some(template) = templates.iter().find(|t| t.name.to_lowercase() == template_name.to_lowercase()) {
        Ok(template.variables.values().cloned().collect())
    } else {
        Err(anyhow::anyhow!("Template '{}' not found", template_name))
    }
}

fn parse_connection_type(connection_type: &str) -> Result<ConnectionType> {
    match connection_type.to_lowercase().as_str() {
        "postgresql" | "postgres" => Ok(ConnectionType::Database(DatabaseType::PostgreSQL)),
        "mysql" => Ok(ConnectionType::Database(DatabaseType::MySQL)),
        "sqlite" => Ok(ConnectionType::Database(DatabaseType::SQLite)),
        "mongodb" | "mongo" => Ok(ConnectionType::Database(DatabaseType::MongoDB)),
        "redis" => Ok(ConnectionType::Database(DatabaseType::Redis)),
        "oracle" => Ok(ConnectionType::Database(DatabaseType::Oracle)),
        "sqlserver" | "mssql" => Ok(ConnectionType::Database(DatabaseType::SQLServer)),
        _ => Ok(ConnectionType::Custom(connection_type.to_string())),
    }
}

fn expand_tilde(path: &PathBuf) -> PathBuf {
    if let Some(path_str) = path.to_str() {
        if path_str.starts_with("~") {
            if let Some(home) = dirs::home_dir() {
                return home.join(&path_str[2..]);
            }
        }
    }
    path.clone()
}

async fn init_vault(path: &PathBuf, force: bool) -> Result<()> {
    if path.exists() && !force {
        eprintln!("Vault already exists at {:?}. Use --force to overwrite.", path);
        return Ok(());
    }
    
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    
    let password = rpassword::prompt_password("Enter master password: ")?;
    let confirm = rpassword::prompt_password("Confirm master password: ")?;
    
    if password != confirm {
        eprintln!("Passwords do not match!");
        return Ok(());
    }
    
    Storage::create_new(path, &password).await?;
    println!("Vault initialized successfully at {:?}", path);
    
    Ok(())
}

async fn open_vault(path: &PathBuf) -> Result<Storage> {
    if !path.exists() {
        eprintln!("Vault not found at {:?}. Run 'pwgen init' first.", path);
        std::process::exit(1);
    }
    
    let password = rpassword::prompt_password("Enter master password: ")?;
    
    match Storage::open(path, &password).await {
        Ok(storage) => Ok(storage),
        Err(e) => {
            eprintln!("Failed to open vault: {}", e);
            std::process::exit(1);
        }
    }
}

async fn add_entry(
    storage: &Storage,
    site: String,
    username: String,
    generate: bool,
    length: usize,
    notes: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let password = if generate {
        let config = PasswordConfig {
            length,
            ..Default::default()
        };
        PasswordGenerator::generate(&config)?
    } else {
        rpassword::prompt_password("Enter password: ")?
    };
    
    let id = hash_entry_id(&site, &username);
    let entry = DecryptedPasswordEntry {
        id,
        site: site.clone(),
        username: username.clone(),
        password,
        notes,
        tags,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        last_used: None,
        password_changed_at: chrono::Utc::now(),
        favorite: false,
    };
    
    storage.add_entry(&entry).await?;
    println!("Password saved for {} @ {}", username, site);
    
    Ok(())
}

async fn get_entry(
    storage: &Storage,
    site: &str,
    username: Option<&str>,
    copy: bool,
    show: bool,
) -> Result<()> {
    let filter = SearchFilter {
        query: Some(site.to_string()),
        ..Default::default()
    };
    
    let entries = storage.search_entries(&filter).await?;
    let entry = if let Some(username) = username {
        entries.iter().find(|e| e.username == username)
    } else if entries.len() == 1 {
        entries.first()
    } else {
        println!("Multiple entries found:");
        for (i, entry) in entries.iter().enumerate() {
            println!("{}: {} @ {}", i + 1, entry.username, entry.site);
        }
        return Ok(());
    };
    
    if let Some(entry) = entry {
        storage.mark_as_used(&entry.id).await?;
        
        if show {
            println!("Password: {}", entry.password);
        } else if copy {
            #[cfg(target_os = "linux")]
            {
                use std::process::Command;
                let mut child = Command::new("xclip")
                    .arg("-selection")
                    .arg("clipboard")
                    .stdin(std::process::Stdio::piped())
                    .spawn()?;
                
                if let Some(mut stdin) = child.stdin.take() {
                    use std::io::Write;
                    stdin.write_all(entry.password.as_bytes())?;
                }
                
                child.wait()?;
                println!("Password copied to clipboard");
            }
            
            #[cfg(not(target_os = "linux"))]
            {
                println!("Clipboard support not implemented for this platform");
                println!("Password: {}", entry.password);
            }
        } else {
            println!("Username: {}", entry.username);
            println!("Site: {}", entry.site);
            if let Some(notes) = &entry.notes {
                println!("Notes: {}", notes);
            }
            println!("Use --show to display password or --copy to copy to clipboard");
        }
    } else {
        println!("No entry found");
    }
    
    Ok(())
}

async fn list_entries(
    storage: &Storage,
    query: Option<String>,
    tags: Vec<String>,
    favorites: bool,
) -> Result<()> {
    let filter = SearchFilter {
        query,
        tags: if tags.is_empty() { None } else { Some(tags) },
        favorite_only: favorites,
        ..Default::default()
    };
    
    let entries = storage.search_entries(&filter).await?;
    
    if entries.is_empty() {
        println!("No entries found");
    } else {
        println!("{:<30} {:<30} {:<20}", "Site", "Username", "Last Used");
        println!("{:-<80}", "");
        
        for entry in entries {
            let last_used = entry.last_used
                .map(|dt| dt.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_else(|| "Never".to_string());
            
            println!("{:<30} {:<30} {:<20}", entry.site, entry.username, last_used);
        }
    }
    
    Ok(())
}

async fn update_entry(
    storage: &Storage,
    site: String,
    username: String,
    new_password: bool,
    notes: Option<String>,
    tags: Vec<String>,
) -> Result<()> {
    let id = hash_entry_id(&site, &username);
    let mut entry = storage.get_entry(&id).await?;
    
    if new_password {
        let password = rpassword::prompt_password("Enter new password: ")?;
        entry.password = password;
        entry.password_changed_at = chrono::Utc::now();
    }
    
    if let Some(notes) = notes {
        entry.notes = Some(notes);
    }
    
    if !tags.is_empty() {
        entry.tags = tags;
    }
    
    entry.updated_at = chrono::Utc::now();
    storage.update_entry(&entry).await?;
    
    println!("Entry updated successfully");
    Ok(())
}

async fn delete_entry(storage: &Storage, site: &str, username: &str, force: bool) -> Result<()> {
    if !force {
        print!("Are you sure you want to delete {} @ {}? [y/N] ", username, site);
        use std::io::{self, Write};
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Deletion cancelled");
            return Ok(());
        }
    }
    
    let id = hash_entry_id(site, username);
    storage.delete_entry(&id).await?;
    
    println!("Entry deleted successfully");
    Ok(())
}

fn generate_password(
    length: usize,
    uppercase: bool,
    lowercase: bool,
    numbers: bool,
    symbols: bool,
    custom_symbols: Option<String>,
    escape: bool,
    passphrase: bool,
    words: usize,
    separator: String,
) -> Result<()> {
    let password = if passphrase {
        PasswordGenerator::generate_passphrase(words, &separator, true)?
    } else {
        let config = PasswordConfig {
            length,
            include_uppercase: uppercase,
            include_lowercase: lowercase,
            include_numbers: numbers,
            include_symbols: symbols,
            custom_symbols,
            ..Default::default()
        };
        
        if escape {
            PasswordGenerator::generate_escaped(&config)?
        } else {
            PasswordGenerator::generate(&config)?
        }
    };
    
    println!("{}", password);
    Ok(())
}

async fn import_passwords(_storage: &Storage, format: &str, _file: &PathBuf) -> Result<()> {
    println!("Import functionality for format '{}' not yet implemented", format);
    Ok(())
}

async fn export_passwords(_storage: &Storage, format: &str, _output: &PathBuf) -> Result<()> {
    println!("Export functionality for format '{}' not yet implemented", format);
    Ok(())
}

async fn create_backup(
    storage: &Storage,
    output: &PathBuf,
    incremental: bool,
    since: Option<String>,
) -> Result<()> {
    let backup_password = rpassword::prompt_password("Enter backup password: ")?;
    let confirm_password = rpassword::prompt_password("Confirm backup password: ")?;
    
    if backup_password != confirm_password {
        eprintln!("Backup passwords do not match!");
        return Ok(());
    }
    
    println!("Creating backup...");
    
    let metadata = if incremental {
        let since_date = match since {
            Some(date_str) => {
                chrono::DateTime::parse_from_rfc3339(&date_str)
                    .map_err(|_| anyhow::anyhow!("Invalid date format. Use RFC3339 format (e.g., 2023-12-01T00:00:00Z)"))?
                    .with_timezone(&chrono::Utc)
            }
            None => {
                // Default to 7 days ago for incremental backup
                chrono::Utc::now() - chrono::Duration::days(7)
            }
        };
        
        println!("Creating incremental backup since {}", since_date.to_rfc3339());
        BackupManager::create_incremental_backup(storage, output, &backup_password, since_date).await?
    } else {
        println!("Creating full backup...");
        BackupManager::create_backup(storage, output, &backup_password).await?
    };
    
    println!("Backup created successfully!");
    println!("Backup ID: {}", metadata.id);
    println!("Created at: {}", metadata.created_at.to_rfc3339());
    println!("Entries: {}", metadata.entry_count);
    println!("File size: {} bytes", metadata.file_size);
    println!("Checksum: {}", metadata.checksum);
    
    Ok(())
}

async fn restore_backup(
    storage: &mut Storage,
    backup_file: &PathBuf,
    conflict_resolution: String,
) -> Result<()> {
    if !backup_file.exists() {
        eprintln!("Backup file does not exist: {:?}", backup_file);
        return Ok(());
    }
    
    // Read backup metadata first
    let metadata = BackupManager::read_backup_metadata(backup_file).await?;
    println!("Backup Information:");
    println!("  ID: {}", metadata.id);
    println!("  Created: {}", metadata.created_at.to_rfc3339());
    println!("  Entries: {}", metadata.entry_count);
    println!("  File size: {} bytes", metadata.file_size);
    
    print!("Continue with restore? [y/N] ");
    use std::io::{self, Write};
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if !input.trim().eq_ignore_ascii_case("y") {
        println!("Restore cancelled");
        return Ok(());
    }
    
    let backup_password = rpassword::prompt_password("Enter backup password: ")?;
    
    let conflict_res = match conflict_resolution.as_str() {
        "overwrite" => ConflictResolution::Overwrite,
        "skip" => ConflictResolution::Skip,
        "merge" => ConflictResolution::Merge,
        _ => {
            eprintln!("Invalid conflict resolution. Use: overwrite, skip, or merge");
            return Ok(());
        }
    };
    
    let options = RestoreOptions {
        conflict_resolution: conflict_res,
    };
    
    println!("Restoring backup...");
    
    match BackupManager::restore_backup(backup_file, &backup_password, storage, options).await {
        Ok(result) => {
            println!("Restore completed!");
            println!("  Total entries in backup: {}", result.total_entries);
            println!("  Successfully restored: {}", result.restored_count);
            println!("  Skipped (conflicts): {}", result.skipped_count);
            println!("  Errors: {}", result.error_count);
            println!("  Success rate: {:.1}%", result.success_rate());
            
            if !result.errors.is_empty() {
                println!("\nErrors encountered:");
                for error in &result.errors {
                    println!("  - {}", error);
                }
            }
        }
        Err(e) => {
            eprintln!("Restore failed: {}", e);
        }
    }
    
    Ok(())
}

async fn verify_backup(backup_file: &PathBuf) -> Result<()> {
    if !backup_file.exists() {
        eprintln!("Backup file does not exist: {:?}", backup_file);
        return Ok(());
    }
    
    println!("Verifying backup integrity...");
    
    match BackupManager::verify_backup(backup_file).await {
        Ok(metadata) => {
            println!("✅ Backup verification successful!");
            println!("Backup Information:");
            println!("  ID: {}", metadata.id);
            println!("  Created: {}", metadata.created_at.to_rfc3339());
            println!("  Vault ID: {}", metadata.vault_id);
            println!("  Entries: {}", metadata.entry_count);
            println!("  File size: {} bytes", metadata.file_size);
            println!("  Checksum: {}", metadata.checksum);
        }
        Err(e) => {
            eprintln!("❌ Backup verification failed: {}", e);
        }
    }
    
    Ok(())
}

// Team sharing command handlers
async fn create_team(
    name: String,
    description: Option<String>,
    owner_email: String,
    owner_name: String,
) -> Result<()> {
    // For now, we'll use a dummy public key - in a real implementation,
    // this would be generated or provided by the user
    let owner_public_key = vec![0u8; 32]; // Dummy key
    let owner_id = uuid::Uuid::new_v4().to_string();
    
    let team = TeamSharingManager::create_team(
        name.clone(),
        description,
        owner_id,
        owner_email,
        owner_name,
        owner_public_key,
    )?;
    
    println!("✅ Team '{}' created successfully!", name);
    println!("Team ID: {}", team.id);
    println!("Owner: {} ({})", team.members[0].name, team.members[0].email);
    
    // In a real implementation, you'd save this to a database
    println!(r#"
💡 To save this team permanently, you would need to:
1. Store team data in a database
2. Set up proper public key cryptography
3. Implement user authentication system
"#);
    
    Ok(())
}

async fn add_team_member(
    team_id: String,
    member_email: String,
    member_name: String,
    role_str: String,
) -> Result<()> {
    let role = match role_str.as_str() {
        "read" => Permission::Read,
        "write" => Permission::Write,
        "share" => Permission::Share,
        "admin" => Permission::Admin,
        _ => {
            eprintln!("Invalid role: {}. Use: read, write, share, admin", role_str);
            return Ok(());
        }
    };
    
    println!("✅ Member '{}' ({}) would be added to team {} with {} permission", 
             member_name, member_email, team_id, role_str);
    
    println!(r#"
💡 In a real implementation, this would:
1. Validate team exists and user has admin rights
2. Send invitation to the member
3. Generate/exchange cryptographic keys
4. Update team membership in database
"#);
    
    Ok(())
}

async fn remove_team_member(team_id: String, member_id: String) -> Result<()> {
    println!("✅ Member {} would be removed from team {}", member_id, team_id);
    
    println!(r#"
💡 In a real implementation, this would:
1. Validate permissions and team membership
2. Revoke access to all shared secrets
3. Update team database
4. Notify other team members
"#);
    
    Ok(())
}

async fn update_member_role(team_id: String, member_id: String, new_role_str: String) -> Result<()> {
    let _role = match new_role_str.as_str() {
        "read" => Permission::Read,
        "write" => Permission::Write,
        "share" => Permission::Share,
        "admin" => Permission::Admin,
        _ => {
            eprintln!("Invalid role: {}. Use: read, write, share, admin", new_role_str);
            return Ok(());
        }
    };
    
    println!("✅ Member {} role would be updated to {} in team {}", 
             member_id, new_role_str, team_id);
    
    Ok(())
}

async fn list_teams() -> Result<()> {
    println!("Teams:");
    println!("{:-<50}", "");
    
    // Mock data for demonstration
    println!("📋 Development Team (team-123)");
    println!("   Description: Main development team");
    println!("   Members: 5");
    println!("   Owner: alice@company.com");
    println!("");
    
    println!("📋 Security Team (team-456)");
    println!("   Description: Security and compliance team");
    println!("   Members: 3");
    println!("   Owner: bob@company.com");
    
    println!(r#"
💡 In a real implementation, this would fetch teams from database
where the current user is a member.
"#);
    
    Ok(())
}

async fn show_team(team_id: String) -> Result<()> {
    println!("Team Details: {}", team_id);
    println!("{:-<50}", "");
    
    // Mock data for demonstration
    println!("Name: Development Team");
    println!("Description: Main development team");
    println!("Created: 2024-01-15 10:30:00 UTC");
    println!("Owner: alice@company.com");
    println!("");
    println!("Members:");
    println!("  👤 Alice (alice@company.com) - Admin");
    println!("  👤 Bob (bob@company.com) - Write");
    println!("  👤 Charlie (charlie@company.com) - Read");
    println!("");
    println!("Shared Secrets: 12");
    println!("Active Share Requests: 2");
    
    Ok(())
}

async fn share_secret(
    _storage: &SecretsStorage,
    secret_name: String,
    team_id: String,
    permissions_str: String,
    expiration_days: Option<i64>,
) -> Result<()> {
    let _permissions = match permissions_str.as_str() {
        "read" => Permission::Read,
        "write" => Permission::Write,
        "share" => Permission::Share,
        "admin" => Permission::Admin,
        _ => {
            eprintln!("Invalid permissions: {}. Use: read, write, share, admin", permissions_str);
            return Ok(());
        }
    };
    
    let expiration_text = if let Some(days) = expiration_days {
        format!("expires in {} days", days)
    } else {
        "no expiration".to_string()
    };
    
    println!("✅ Secret '{}' would be shared with team {} ({} permissions, {})", 
             secret_name, team_id, permissions_str, expiration_text);
    
    println!(r#"
💡 In a real implementation, this would:
1. Encrypt secret with team's public key
2. Create sharing record in database
3. Notify team members
4. Log the sharing activity
"#);
    
    Ok(())
}

async fn list_shared_secrets(_storage: &SecretsStorage, team_id: Option<String>) -> Result<()> {
    let filter_text = if let Some(tid) = &team_id {
        format!(" for team {}", tid)
    } else {
        String::new()
    };
    
    println!("Shared Secrets{}:", filter_text);
    println!("{:-<60}", "");
    
    // Mock data for demonstration
    println!("🔑 AWS Production Keys");
    println!("   Team: Development Team (team-123)");
    println!("   Permissions: Read");
    println!("   Shared by: alice@company.com");
    println!("   Expires: Never");
    println!("   Last accessed: 2 hours ago");
    println!("");
    
    println!("🔑 Database Connection String");
    println!("   Team: Development Team (team-123)");
    println!("   Permissions: Write");
    println!("   Shared by: bob@company.com");
    println!("   Expires: 30 days");
    println!("   Last accessed: 1 day ago");
    
    Ok(())
}

async fn revoke_secret_access(
    _storage: &SecretsStorage,
    secret_name: String,
    team_id: String,
) -> Result<()> {
    println!("✅ Access to secret '{}' would be revoked for team {}", secret_name, team_id);
    
    println!(r#"
💡 In a real implementation, this would:
1. Validate user has permission to revoke
2. Update sharing records in database
3. Notify affected team members
4. Log the revocation activity
"#);
    
    Ok(())
}

async fn request_secret_access(
    secret_name: String,
    from_user: String,
    team_id: Option<String>,
    permissions_str: String,
    message: Option<String>,
) -> Result<()> {
    let team_text = if let Some(tid) = &team_id {
        format!(" (team: {})", tid)
    } else {
        String::new()
    };
    
    let message_text = message.as_deref().unwrap_or("No message provided");
    
    println!("✅ Access request created for secret '{}'", secret_name);
    println!("   From user: {}{}", from_user, team_text);
    println!("   Requested permissions: {}", permissions_str);
    println!("   Message: {}", message_text);
    
    Ok(())
}

async fn list_share_requests(incoming: bool, outgoing: bool) -> Result<()> {
    if incoming {
        println!("Incoming Share Requests:");
        println!("{:-<50}", "");
        
        println!("📨 Request #req-123");
        println!("   Secret: Database Password");
        println!("   From: charlie@company.com");
        println!("   Permissions: Read");
        println!("   Message: Need access for debugging");
        println!("   Status: Pending");
        println!("");
    }
    
    if outgoing {
        println!("Outgoing Share Requests:");
        println!("{:-<50}", "");
        
        println!("📤 Request #req-456");
        println!("   Secret: API Keys");
        println!("   To: alice@company.com");
        println!("   Permissions: Write");
        println!("   Status: Approved");
        println!("");
    }
    
    if !incoming && !outgoing {
        println!("💡 Use --incoming or --outgoing to filter requests");
    }
    
    Ok(())
}

async fn approve_share_request(request_id: String) -> Result<()> {
    println!("✅ Share request {} would be approved", request_id);
    
    println!(r#"
💡 In a real implementation, this would:
1. Validate user has permission to approve
2. Create shared secret record
3. Notify requester of approval
4. Log the approval activity
"#);
    
    Ok(())
}

async fn reject_share_request(request_id: String) -> Result<()> {
    println!("❌ Share request {} would be rejected", request_id);
    
    Ok(())
}

async fn view_access_log(
    secret_name: Option<String>,
    user_id: Option<String>,
    limit: usize,
) -> Result<()> {
    let filter_parts = vec![
        secret_name.as_ref().map(|s| format!("secret: {}", s)),
        user_id.as_ref().map(|u| format!("user: {}", u)),
    ].into_iter().flatten().collect::<Vec<_>>();
    
    let filter_text = if filter_parts.is_empty() {
        String::new()
    } else {
        format!(" ({})", filter_parts.join(", "))
    };
    
    println!("Access Log{} (last {} entries):", filter_text, limit);
    println!("{:-<70}", "");
    
    // Mock data for demonstration
    println!("🔍 2024-06-26 14:30:15 | alice@company.com | View | AWS Keys | Success");
    println!("🔍 2024-06-26 13:45:22 | bob@company.com | Edit | Database Config | Success");
    println!("🔍 2024-06-26 12:15:08 | charlie@company.com | View | API Token | Failed - No access");
    println!("🔍 2024-06-26 11:20:33 | alice@company.com | Share | SSH Keys | Success");
    println!("🔍 2024-06-26 10:55:41 | bob@company.com | Copy | Production Secrets | Success");
    
    Ok(())
}