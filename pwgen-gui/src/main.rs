#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(feature = "clipboard")]
use arboard::Clipboard;
use chrono::Utc;
use eframe::egui;
use pwgen_core::{
    generator::{PasswordConfig, PasswordGenerator},
    models::{DecryptedPasswordEntry, SearchFilter, SortField, SortOrder},
    storage::Storage,
    secrets::{DecryptedSecretEntry, SecretType, SecretData, SecretFilter},
    secrets_storage::SecretsStorage,
};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
// System tray functionality disabled - will be re-enabled once dependencies are resolved

struct PwGenApp {
    // State
    storage: Arc<Mutex<Option<Storage>>>,
    secrets_storage: Arc<Mutex<Option<SecretsStorage>>>,
    entries: Vec<DecryptedPasswordEntry>,
    filtered_entries: Vec<DecryptedPasswordEntry>,
    secrets: Vec<DecryptedSecretEntry>,
    filtered_secrets: Vec<DecryptedSecretEntry>,
    
    // UI State
    screen: Screen,
    current_tab: MainTab,
    master_password: String,
    master_password_confirm: String,
    error_message: String,
    success_message: String,
    
    // Entry form
    show_add_dialog: bool,
    edit_entry: Option<DecryptedPasswordEntry>,
    entry_site: String,
    entry_username: String,
    entry_password: String,
    entry_notes: String,
    entry_tags: String,
    show_password: bool,
    
    // Search and pagination
    search_query: String,
    search_field: SearchField,
    filter_favorites: bool,
    filter_tags: String,
    show_advanced_search: bool,
    current_page: usize,
    entries_per_page: usize,
    total_pages: usize,
    
    // Responsive layout
    window_width: f32,
    is_compact_mode: bool,
    
    // Generator
    show_generator: bool,
    gen_length: u8,
    gen_uppercase: bool,
    gen_lowercase: bool,
    gen_numbers: bool,
    gen_symbols: bool,
    gen_exclude_ambiguous: bool,
    generated_password: String,
    
    // Dialog states
    show_settings: bool,
    show_about: bool,
    show_import: bool,
    show_backup: bool,
    show_statistics: bool,
    
    // Secrets management
    show_secrets_view: bool,
    show_add_secret_dialog: bool,
    selected_secret_type: SecretType,
    current_secret_tab: SecretType,
    
    // Selected entry
    selected_entry_id: Option<String>,
    
    // Inline tag editing
    editing_tags_for_entry: Option<String>,
    temp_tags: String,
    
    // Secret creation form
    secret_name: String,
    secret_description: String,
    secret_tags: String,
    
    // API Key fields
    api_provider: String,
    api_key_id: String,
    api_key: String,
    api_secret: String,
    api_environment: String,
    api_endpoint: String,
    
    // SSH Key fields
    ssh_key_type: String,
    ssh_private_key: String,
    ssh_public_key: String,
    ssh_passphrase: String,
    ssh_comment: String,
    
    // Document fields
    document_filename: String,
    document_content: Vec<u8>,
    
    // Configuration fields
    config_variables: String,
    
    // Secure note fields
    note_title: String,
    note_content: String,
    
    // Database connection fields
    db_type: String,
    db_host: String,
    db_port: String,
    db_name: String,
    db_username: String,
    db_password: String,
    
    // Runtime
    runtime: Arc<tokio::runtime::Runtime>,
    
    // Images
    logo_wide: Option<egui::TextureHandle>,
    logo_square: Option<egui::TextureHandle>,
    
    // Settings
    minimize_to_tray: bool,
    auto_lock_minutes: u32,
    show_system_tray: bool,
    
    // Tray - disabled for now
    // _tray_icon: Option<TrayIcon>,
}

#[derive(PartialEq, Default)]
enum Screen {
    #[default]
    Login,
    Main,
}

#[derive(PartialEq, Default, Clone, Copy)]
enum SearchField {
    #[default]
    All,
    Site,
    Username,
    Notes,
    Tags,
}

#[derive(PartialEq, Default, Clone, Copy)]
enum MainTab {
    #[default]
    Passwords,
    Secrets,
    Generator,
    Tools,
    Settings,
}

impl PwGenApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Set up custom fonts
        setup_custom_fonts(&cc.egui_ctx);
        
        // Load logos
        let logo_wide = load_logo_wide(&cc.egui_ctx);
        let logo_square = load_logo_square(&cc.egui_ctx);
        
        // Create tokio runtime
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
        );
        
        // Create system tray
        let tray_icon = create_tray_icon();
        
        let mut app = Self {
            runtime,
            storage: Arc::new(Mutex::new(None)),
            secrets_storage: Arc::new(Mutex::new(None)),
            gen_length: 16,
            gen_uppercase: true,
            gen_lowercase: true,
            gen_numbers: true,
            gen_symbols: true,
            gen_exclude_ambiguous: true,
            entries: Vec::new(),
            filtered_entries: Vec::new(),
            secrets: Vec::new(),
            filtered_secrets: Vec::new(),
            screen: Screen::Login,
            current_tab: MainTab::Passwords,
            master_password: String::new(),
            master_password_confirm: String::new(),
            error_message: String::new(),
            success_message: String::new(),
            show_add_dialog: false,
            edit_entry: None,
            entry_site: String::new(),
            entry_username: String::new(),
            entry_password: String::new(),
            entry_notes: String::new(),
            entry_tags: String::new(),
            show_password: false,
            search_query: String::new(),
            search_field: SearchField::All,
            filter_favorites: false,
            filter_tags: String::new(),
            show_advanced_search: false,
            current_page: 0,
            entries_per_page: 50,
            total_pages: 0,
            window_width: 1200.0,
            is_compact_mode: false,
            show_generator: false,
            generated_password: String::new(),
            show_settings: false,
            show_about: false,
            show_import: false,
            show_backup: false,
            show_statistics: false,
            show_secrets_view: false,
            show_add_secret_dialog: false,
            selected_secret_type: SecretType::Password,
            current_secret_tab: SecretType::Password,
            selected_entry_id: None,
            editing_tags_for_entry: None,
            temp_tags: String::new(),
            secret_name: String::new(),
            secret_description: String::new(),
            secret_tags: String::new(),
            api_provider: String::new(),
            api_key_id: String::new(),
            api_key: String::new(),
            api_secret: String::new(),
            api_environment: "production".to_string(),
            api_endpoint: String::new(),
            ssh_key_type: "RSA".to_string(),
            ssh_private_key: String::new(),
            ssh_public_key: String::new(),
            ssh_passphrase: String::new(),
            ssh_comment: String::new(),
            document_filename: String::new(),
            document_content: Vec::new(),
            config_variables: String::new(),
            note_title: String::new(),
            note_content: String::new(),
            db_type: "PostgreSQL".to_string(),
            db_host: String::new(),
            db_port: "5432".to_string(),
            db_name: String::new(),
            db_username: String::new(),
            db_password: String::new(),
            logo_wide,
            logo_square,
            minimize_to_tray: true,
            auto_lock_minutes: 10,
            show_system_tray: true,
            // _tray_icon: tray_icon,
        };
        
        // Check if vault exists
        if let Some(vault_path) = app.get_vault_path() {
            if vault_path.exists() {
                app.screen = Screen::Login;
            }
        }
        
        app
    }
    
    fn get_vault_path(&self) -> Option<PathBuf> {
        dirs::data_dir()
            .map(|d| d.join("pwgen").join("vault.db"))
    }
    
    fn init_vault(&mut self) {
        if self.master_password.len() < 12 {
            self.error_message = "Master password must be at least 12 characters".to_string();
            return;
        }
        
        if self.master_password != self.master_password_confirm {
            self.error_message = "Passwords do not match".to_string();
            return;
        }
        
        let vault_path = match self.get_vault_path() {
            Some(p) => p,
            None => {
                self.error_message = "Could not determine vault path".to_string();
                return;
            }
        };
        
        // Create parent directory if needed
        if let Some(parent) = vault_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        
        let password = self.master_password.clone();
        let storage_mutex = self.storage.clone();
        let secrets_storage_mutex = self.secrets_storage.clone();
        let runtime = self.runtime.clone();
        let vault_path_clone = vault_path.clone();
        
        self.error_message.clear();
        
        // Run async operation
        runtime.block_on(async {
            match Storage::create_new(&vault_path, &password).await {
                Ok(storage) => {
                    *storage_mutex.lock().unwrap() = Some(storage);
                    
                    // Initialize secrets storage
                    match SecretsStorage::create_new(&vault_path_clone, &password).await {
                        Ok(secrets_storage) => {
                            *secrets_storage_mutex.lock().unwrap() = Some(secrets_storage);
                        }
                        Err(e) => {
                            return Err(format!("Failed to initialize secrets storage: {}", e));
                        }
                    }
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
            Ok(())
        }).unwrap_or_else(|e| {
            self.error_message = format!("Failed to create vault: {}", e);
        });
        
        if self.error_message.is_empty() {
            self.screen = Screen::Main;
            self.master_password.clear();
            self.master_password_confirm.clear();
            self.load_entries();
            self.load_secrets();
        }
    }
    
    fn unlock_vault(&mut self) {
        let vault_path = match self.get_vault_path() {
            Some(p) => p,
            None => {
                self.error_message = "Could not determine vault path".to_string();
                return;
            }
        };
        
        let password = self.master_password.clone();
        let storage_mutex = self.storage.clone();
        let secrets_storage_mutex = self.secrets_storage.clone();
        let runtime = self.runtime.clone();
        let vault_path_clone = vault_path.clone();
        
        self.error_message.clear();
        
        runtime.block_on(async {
            match Storage::open(&vault_path, &password).await {
                Ok(storage) => {
                    *storage_mutex.lock().unwrap() = Some(storage);
                    
                    // Initialize secrets storage
                    match SecretsStorage::from_existing_storage(&vault_path_clone, &password).await {
                        Ok(secrets_storage) => {
                            *secrets_storage_mutex.lock().unwrap() = Some(secrets_storage);
                        }
                        Err(e) => {
                            return Err(format!("Failed to initialize secrets storage: {}", e));
                        }
                    }
                }
                Err(e) => {
                    return Err(e.to_string());
                }
            }
            Ok(())
        }).unwrap_or_else(|e| {
            self.error_message = format!("Error unlocking vault: {}", e);
        });
        
        if self.error_message.is_empty() {
            self.screen = Screen::Main;
            self.master_password.clear();
            self.load_entries();
            self.load_secrets();
            self.success_message = "Vault unlocked successfully".to_string();
        }
    }
    
    fn lock_vault(&mut self) {
        *self.storage.lock().unwrap() = None;
        *self.secrets_storage.lock().unwrap() = None;
        self.entries.clear();
        self.filtered_entries.clear();
        self.secrets.clear();
        self.filtered_secrets.clear();
        self.selected_entry_id = None;
        self.screen = Screen::Login;
        self.success_message = "Vault locked".to_string();
    }
    
    fn load_entries(&mut self) {
        let storage_mutex = self.storage.clone();
        let runtime = self.runtime.clone();
        
        self.entries = runtime.block_on(async {
            let storage_guard = storage_mutex.lock().unwrap();
            if let Some(storage) = storage_guard.as_ref() {
                let filter = SearchFilter {
                    query: None,
                    tags: None,
                    favorite_only: false,
                    sort_by: SortField::Site,
                    sort_order: SortOrder::Ascending,
                };
                storage.search_entries(&filter).await.unwrap_or_default()
            } else {
                vec![]
            }
        });
        
        self.current_page = 0; // Reset to first page when loading entries
        self.filter_entries();
    }
    
    fn load_secrets(&mut self) {
        let secrets_storage_mutex = self.secrets_storage.clone();
        let runtime = self.runtime.clone();
        
        self.secrets = runtime.block_on(async {
            let secrets_storage_guard = secrets_storage_mutex.lock().unwrap();
            if let Some(secrets_storage) = secrets_storage_guard.as_ref() {
                let filter = SecretFilter::default();
                secrets_storage.search_secrets(&filter).await.unwrap_or_default()
            } else {
                vec![]
            }
        });
        
        self.filter_secrets();
    }
    
    fn filter_secrets(&mut self) {
        // Simple filtering for now - can be enhanced later
        self.filtered_secrets = self.secrets.clone();
    }
    
    fn filter_entries(&mut self) {
        // First apply search filter
        let mut filtered = if self.search_query.is_empty() {
            self.entries.clone()
        } else {
            let query = self.search_query.to_lowercase();
            self.entries.iter()
                .filter(|e| {
                    match self.search_field {
                        SearchField::All => {
                            e.site.to_lowercase().contains(&query) ||
                            e.username.to_lowercase().contains(&query) ||
                            e.notes.as_ref().map(|n| n.to_lowercase().contains(&query)).unwrap_or(false) ||
                            e.tags.iter().any(|t| t.to_lowercase().contains(&query))
                        }
                        SearchField::Site => e.site.to_lowercase().contains(&query),
                        SearchField::Username => e.username.to_lowercase().contains(&query),
                        SearchField::Notes => e.notes.as_ref().map(|n| n.to_lowercase().contains(&query)).unwrap_or(false),
                        SearchField::Tags => e.tags.iter().any(|t| t.to_lowercase().contains(&query)),
                    }
                })
                .cloned()
                .collect::<Vec<_>>()
        };
        
        // Apply favorites filter
        if self.filter_favorites {
            filtered.retain(|e| e.favorite);
        }
        
        // Apply tag filter
        if !self.filter_tags.is_empty() {
            let tag_filters: Vec<String> = self.filter_tags
                .split(',')
                .map(|s| s.trim().to_lowercase())
                .filter(|s| !s.is_empty())
                .collect();
            
            if !tag_filters.is_empty() {
                filtered.retain(|e| {
                        tag_filters.iter().any(|filter_tag| {
                            e.tags.iter().any(|entry_tag| entry_tag.to_lowercase().contains(filter_tag))
                        })
                    });
            }
        }
        
        // Calculate pagination
        self.total_pages = filtered.len().div_ceil(self.entries_per_page);
        if self.total_pages == 0 {
            self.total_pages = 1;
        }
        
        // Ensure current page is valid
        if self.current_page >= self.total_pages {
            self.current_page = if self.total_pages > 0 { self.total_pages - 1 } else { 0 };
        }
        
        // Apply pagination
        let start_idx = self.current_page * self.entries_per_page;
        let end_idx = std::cmp::min(start_idx + self.entries_per_page, filtered.len());
        
        self.filtered_entries = if start_idx < filtered.len() {
            filtered[start_idx..end_idx].to_vec()
        } else {
            Vec::new()
        };
    }
    
    fn save_entry(&mut self) {
        if self.entry_site.is_empty() || self.entry_username.is_empty() || self.entry_password.is_empty() {
            self.error_message = "Please fill in all required fields".to_string();
            return;
        }
        
        let storage_mutex = self.storage.clone();
        let runtime = self.runtime.clone();
        
        let site = self.entry_site.clone();
        let username = self.entry_username.clone();
        let password = self.entry_password.clone();
        let notes = if self.entry_notes.is_empty() { None } else { Some(self.entry_notes.clone()) };
        let tags: Vec<String> = self.entry_tags.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        self.error_message.clear();
        
        runtime.block_on(async {
            let storage_guard = storage_mutex.lock().unwrap();
            if let Some(storage) = storage_guard.as_ref() {
                if let Some(existing) = &self.edit_entry {
                    // Update existing entry
                    let mut updated = existing.clone();
                    updated.site = site;
                    updated.username = username;
                    updated.password = password;
                    updated.notes = notes;
                    updated.tags = tags;
                    updated.updated_at = Utc::now();
                    
                    storage.update_entry(&updated).await
                } else {
                    // Add new entry
                    let new_entry = DecryptedPasswordEntry {
                        id: uuid::Uuid::new_v4().to_string(),
                        site: site.clone(),
                        username: username.clone(),
                        password: password.clone(),
                        notes: notes.clone(),
                        tags: tags.clone(),
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                        last_used: None,
                        password_changed_at: Utc::now(),
                        favorite: false,
                    };
                    storage.add_entry(&new_entry).await
                }
            } else {
                Err(pwgen_core::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Storage not initialized")))
            }
        }).unwrap_or_else(|e| {
            self.error_message = format!("Failed to save entry: {}", e);
        });
        
        if self.error_message.is_empty() {
            self.success_message = if self.edit_entry.is_some() { "Entry updated successfully" } else { "Entry added successfully" }.to_string();
            self.show_add_dialog = false;
            self.clear_entry_form();
            self.load_entries();
        }
    }
    
    fn delete_entry(&mut self, id: &str) {
        let storage_mutex = self.storage.clone();
        let runtime = self.runtime.clone();
        let id_string = id.to_string();
        
        runtime.block_on(async {
            let storage_guard = storage_mutex.lock().unwrap();
            if let Some(storage) = storage_guard.as_ref() {
                storage.delete_entry(&id_string).await
            } else {
                Err(pwgen_core::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Storage not initialized")))
            }
        }).unwrap_or_else(|e| {
            self.error_message = format!("Failed to delete entry: {}", e);
        });
        
        if self.error_message.is_empty() {
            self.success_message = "Entry deleted successfully".to_string();
            self.selected_entry_id = None;
            self.load_entries();
        }
    }
    
    fn clear_entry_form(&mut self) {
        self.entry_site.clear();
        self.entry_username.clear();
        self.entry_password.clear();
        self.entry_notes.clear();
        self.entry_tags.clear();
        self.show_password = false;
        self.edit_entry = None;
    }
    
    fn generate_password(&mut self) {
        let config = PasswordConfig {
            length: self.gen_length as usize,
            include_uppercase: self.gen_uppercase,
            include_lowercase: self.gen_lowercase,
            include_numbers: self.gen_numbers,
            include_symbols: self.gen_symbols,
            exclude_ambiguous: self.gen_exclude_ambiguous,
            custom_symbols: None,
            min_uppercase: if self.gen_uppercase { 1 } else { 0 },
            min_lowercase: if self.gen_lowercase { 1 } else { 0 },
            min_numbers: if self.gen_numbers { 1 } else { 0 },
            min_symbols: if self.gen_symbols { 1 } else { 0 },
        };
        
        match PasswordGenerator::generate(&config) {
            Ok(password) => {
                self.generated_password = password;
            }
            Err(e) => {
                self.error_message = format!("Failed to generate password: {}", e);
            }
        }
    }
    
    #[cfg(feature = "clipboard")]
    fn copy_to_clipboard(&self, text: &str) {
        if let Ok(mut clipboard) = Clipboard::new() {
            if clipboard.set_text(text).is_ok() {
                // Don't set success message here as it's called frequently
            }
        }
    }
    
    #[cfg(not(feature = "clipboard"))]
    fn copy_to_clipboard(&self, _text: &str) {
        // Clipboard functionality disabled
    }
    
    fn quick_copy_entry(&mut self, entry: &DecryptedPasswordEntry, field: &str) {
        match field {
            "username" => {
                self.copy_to_clipboard(&entry.username);
                self.success_message = format!("Username copied for {}", entry.site);
            }
            "password" => {
                self.copy_to_clipboard(&entry.password);
                self.success_message = format!("Password copied for {}", entry.site);
            }
            _ => {}
        }
    }
    
    fn copy_secret_data(&mut self, secret: &DecryptedSecretEntry) {
        use pwgen_core::secrets::SecretData;
        
        match &secret.data {
            SecretData::ApiKey { api_key, .. } => {
                self.copy_to_clipboard(api_key);
                self.success_message = format!("API key copied for {}", secret.name);
            }
            SecretData::Token { access_token, .. } => {
                self.copy_to_clipboard(access_token);
                self.success_message = format!("Access token copied for {}", secret.name);
            }
            SecretData::SshKey { private_key: Some(private_key), .. } => {
                self.copy_to_clipboard(private_key);
                self.success_message = format!("SSH private key copied for {}", secret.name);
            }
            SecretData::SshKey { public_key: Some(public_key), .. } => {
                self.copy_to_clipboard(public_key);
                self.success_message = format!("SSH public key copied for {}", secret.name);
            }
            SecretData::SecureNote { content, .. } => {
                self.copy_to_clipboard(content);
                self.success_message = format!("Note content copied for {}", secret.name);
            }
            SecretData::ConnectionString { connection_string, .. } => {
                self.copy_to_clipboard(connection_string);
                self.success_message = format!("Connection string copied for {}", secret.name);
            }
            SecretData::Configuration { variables, .. } => {
                // Copy the first environment variable value, or a formatted string of all variables
                if let Some((key, value)) = variables.iter().next() {
                    self.copy_to_clipboard(value);
                    self.success_message = format!("Variable '{}' copied for {}", key, secret.name);
                } else {
                    self.success_message = format!("No variables to copy for {}", secret.name);
                }
            }
            _ => {
                self.success_message = "Copy not supported for this secret type".to_string();
            }
        }
    }
    
    fn delete_secret(&mut self, secret_id: &str) {
        let secrets_storage_mutex = self.secrets_storage.clone();
        let runtime = self.runtime.clone();
        let id_string = secret_id.to_string();
        
        let result = runtime.block_on(async {
            let secrets_storage_guard = secrets_storage_mutex.lock().unwrap();
            if let Some(secrets_storage) = secrets_storage_guard.as_ref() {
                secrets_storage.delete_secret(&id_string).await
            } else {
                Err(pwgen_core::Error::Other("Secrets storage not available".to_string()))
            }
        });
        
        match result {
            Ok(_) => {
                self.success_message = "Secret deleted successfully".to_string();
                self.load_secrets(); // Reload secrets list
            }
            Err(e) => {
                self.error_message = format!("Failed to delete secret: {}", e);
            }
        }
    }
}

impl eframe::App for PwGenApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Update window dimensions for responsive layout
        let screen_rect = ctx.screen_rect();
        self.window_width = screen_rect.width();
        self.is_compact_mode = self.window_width < 1000.0;
        
        // TODO: Handle tray menu events when system tray is re-enabled
        
        match self.screen {
            Screen::Login => self.show_login_screen(ctx),
            Screen::Main => self.show_main_screen(ctx, frame),
        }
    }
}

impl PwGenApp {
    fn show_login_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                
                // Display wide logo prominently
                if let Some(logo) = &self.logo_wide {
                    ui.add(
                        egui::Image::from_texture(logo)
                            .fit_to_exact_size(egui::vec2(200.0, 67.0))
                            .rounding(egui::Rounding::same(8.0))
                    );
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("Password Manager")
                            .size(16.0)
                            .color(ui.visuals().weak_text_color())
                    );
                } else {
                    // Fallback if logo doesn't load
                    ui.heading(
                        egui::RichText::new("🦀 PwGen")
                            .size(32.0)
                            .color(ui.visuals().strong_text_color())
                    );
                    ui.label(
                        egui::RichText::new("Password Manager")
                            .size(16.0)
                            .color(ui.visuals().weak_text_color())
                    );
                }
                
                ui.add_space(40.0);
                
                let vault_exists = self.get_vault_path()
                    .map(|p| p.exists())
                    .unwrap_or(false);
                
                ui.group(|ui| {
                    ui.set_min_width(350.0);
                    ui.vertical_centered(|ui| {
                        if vault_exists {
                            ui.heading("Welcome Back");
                            ui.label("Enter your master password to unlock the vault");
                        } else {
                            ui.heading("Create Your Vault");
                            ui.label("Set a strong master password to protect your data");
                        }
                        
                        ui.add_space(20.0);
                        
                        ui.horizontal(|ui| {
                            ui.label("Master Password:");
                            let response = ui.add(
                                egui::TextEdit::singleline(&mut self.master_password)
                                    .password(true)
                                    .desired_width(200.0)
                                    .hint_text("Enter password...")
                                    .font(egui::TextStyle::Monospace)
                            );
                            
                            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                if vault_exists {
                                    self.unlock_vault();
                                } else if !self.master_password_confirm.is_empty() {
                                    self.init_vault();
                                }
                            }
                        });
                        
                        if !vault_exists {
                            ui.horizontal(|ui| {
                                ui.label("Confirm Password:");
                                let response = ui.add(
                                    egui::TextEdit::singleline(&mut self.master_password_confirm)
                                        .password(true)
                                        .desired_width(200.0)
                                        .hint_text("Confirm password...")
                                        .font(egui::TextStyle::Monospace)
                                );
                                
                                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    self.init_vault();
                                }
                            });
                        }
                        
                        ui.add_space(20.0);
                        
                        if vault_exists {
                            if ui.button("🔓 Unlock Vault").clicked() {
                                self.unlock_vault();
                            }
                        } else {
                            if ui.button("🔐 Create Vault").clicked() {
                                self.init_vault();
                            }
                            
                            ui.add_space(10.0);
                            ui.label("⚠ Remember: Your master password cannot be recovered!");
                        }
                    });
                });
                
                if !self.error_message.is_empty() {
                    ui.add_space(10.0);
                    ui.colored_label(egui::Color32::from_rgb(255, 100, 100), &self.error_message);
                }
                
                if !self.success_message.is_empty() {
                    ui.add_space(10.0);
                    ui.colored_label(egui::Color32::from_rgb(100, 255, 100), &self.success_message);
                }
            });
        });
    }
    
    fn show_main_screen(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("➕ New Entry").clicked() {
                        self.show_add_dialog = true;
                        self.clear_entry_form();
                        ui.close_menu();
                    }
                    if ui.button("🔒 Lock Vault").clicked() {
                        self.lock_vault();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("⚙ Settings").clicked() {
                        self.show_settings = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("❌ Exit").clicked() {
                        self.minimize_to_tray = false; // Force exit
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("Edit", |ui| {
                    if ui.button("🔍 Search").clicked() {
                        // Focus search field
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("📋 Copy Username").clicked() {
                        if let Some(id) = &self.selected_entry_id {
                            if let Some(entry) = self.entries.iter().find(|e| e.id == *id).cloned() {
                                self.quick_copy_entry(&entry, "username");
                            }
                        }
                        ui.close_menu();
                    }
                    if ui.button("🔑 Copy Password").clicked() {
                        if let Some(id) = &self.selected_entry_id {
                            if let Some(entry) = self.entries.iter().find(|e| e.id == *id).cloned() {
                                self.quick_copy_entry(&entry, "password");
                            }
                        }
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Tools", |ui| {
                    if ui.button("🔐 Secrets Manager").clicked() {
                        self.show_secrets_view = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("🎲 Password Generator").clicked() {
                        self.show_generator = true;
                        self.generate_password();
                        ui.close_menu();
                    }
                    if ui.button("📊 Vault Statistics").clicked() {
                        self.show_statistics = true;
                        ui.close_menu();
                    }
                    if ui.button("📥 Import from Browser").clicked() {
                        self.show_import = true;
                        ui.close_menu();
                    }
                    if ui.button("💾 Backup Vault").clicked() {
                        self.show_backup = true;
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("📚 Documentation").clicked() {
                        if let Err(e) = open::that("https://github.com/hxhippy/pwgen/blob/main/README.md") {
                            eprintln!("Failed to open documentation: {}", e);
                        }
                        ui.close_menu();
                    }
                    if ui.button("🌐 Kief Studio").clicked() {
                        if let Err(e) = open::that("https://kief.studio") {
                            eprintln!("Failed to open Kief Studio: {}", e);
                        }
                        ui.close_menu();
                    }
                    if ui.button("🔍 TRaViS ASM").clicked() {
                        if let Err(e) = open::that("https://travisasm.com") {
                            eprintln!("Failed to open TRaViS: {}", e);
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("ℹ About PwGen").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });
        
        // Header with logo and search - responsive layout
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(5.0);
            
            if self.is_compact_mode {
                // Compact mode: stack elements vertically
                ui.vertical(|ui| {
                    // Top row: Logo and main action buttons
                    ui.horizontal(|ui| {
                        // Display logo if available
                        if let Some(logo) = &self.logo_wide {
                            ui.add(
                                egui::Image::from_texture(logo)
                                    .fit_to_exact_size(egui::vec2(100.0, 33.0))
                                    .rounding(egui::Rounding::same(4.0))
                            );
                        } else {
                            ui.heading(
                                egui::RichText::new("🦀 PwGen")
                                    .size(20.0)
                                    .color(ui.visuals().strong_text_color())
                            );
                        }
                        
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("🔒 Lock").clicked() {
                                self.lock_vault();
                            }
                            if ui.button("🎲 Generate").clicked() {
                                self.show_generator = true;
                                self.generate_password();
                            }
                            if ui.button("➕ Add Entry").clicked() {
                                self.show_add_dialog = true;
                                self.clear_entry_form();
                            }
                        });
                    });
                    
                    ui.add_space(3.0);
                    
                    // Second row: Search and advanced search toggle
                    ui.horizontal(|ui| {
                        ui.label("🔍");
                        let search_response = ui.add(
                            egui::TextEdit::singleline(&mut self.search_query)
                                .desired_width(ui.available_width() - 60.0)
                                .hint_text("Search passwords...")
                        );
                        if search_response.changed() {
                            self.filter_entries();
                        }
                        
                        if ui.small_button("🔧").on_hover_text("Advanced Search").clicked() {
                            self.show_advanced_search = !self.show_advanced_search;
                        }
                    });
                    
                    // Third row: Pagination controls (if needed)
                    if self.total_pages > 1 {
                        ui.add_space(2.0);
                        ui.horizontal(|ui| {
                            ui.label("📄");
                            
                            if ui.add_enabled(self.current_page > 0, egui::Button::new("◀")).clicked() {
                                self.current_page -= 1;
                                self.filter_entries();
                            }
                            
                            ui.label(format!("{}/{}", self.current_page + 1, self.total_pages));
                            
                            if ui.add_enabled(self.current_page < self.total_pages - 1, egui::Button::new("▶")).clicked() {
                                self.current_page += 1;
                                self.filter_entries();
                            }
                            
                            ui.separator();
                            ui.label("Per page:");
                            egui::ComboBox::from_id_source("entries_per_page")
                                .selected_text(format!("{}", self.entries_per_page))
                                .width(80.0)
                                .show_ui(ui, |ui| {
                                    for &size in &[25, 50, 100, 200] {
                                        if ui.selectable_value(&mut self.entries_per_page, size, format!("{}", size)).clicked() {
                                            self.current_page = 0;
                                            self.filter_entries();
                                        }
                                    }
                                });
                        });
                    }
                });
            } else {
                // Wide mode: horizontal layout
                ui.horizontal(|ui| {
                    // Display logo if available
                    if let Some(logo) = &self.logo_wide {
                        ui.add(
                            egui::Image::from_texture(logo)
                                .fit_to_exact_size(egui::vec2(120.0, 40.0))
                                .rounding(egui::Rounding::same(4.0))
                        );
                    } else {
                        ui.heading(
                            egui::RichText::new("🦀 PwGen")
                                .size(24.0)
                                .color(ui.visuals().strong_text_color())
                        );
                    }
                    
                    ui.separator();
                    
                    // Search
                    ui.add_space(5.0);
                    ui.label("🔍");
                    let search_width = if self.window_width > 1400.0 { 400.0 } else { 300.0 };
                    let search_response = ui.add(
                        egui::TextEdit::singleline(&mut self.search_query)
                            .desired_width(search_width)
                            .hint_text("Search passwords...")
                    );
                    if search_response.changed() {
                        self.filter_entries();
                    }
                    
                    // Advanced search toggle
                    if ui.small_button("🔧").on_hover_text("Advanced Search").clicked() {
                        self.show_advanced_search = !self.show_advanced_search;
                    }
                    
                    // Pagination controls
                    if self.total_pages > 1 {
                        ui.separator();
                        ui.label("📄");
                        
                        if ui.add_enabled(self.current_page > 0, egui::Button::new("◀")).clicked() {
                            self.current_page -= 1;
                            self.filter_entries();
                        }
                        
                        ui.label(format!("{}/{}", self.current_page + 1, self.total_pages));
                        
                        if ui.add_enabled(self.current_page < self.total_pages - 1, egui::Button::new("▶")).clicked() {
                            self.current_page += 1;
                            self.filter_entries();
                        }
                        
                        ui.separator();
                        ui.label("Per page:");
                        egui::ComboBox::from_id_source("entries_per_page")
                            .selected_text(format!("{}", self.entries_per_page))
                            .width(80.0)
                            .show_ui(ui, |ui| {
                                for &size in &[25, 50, 100, 200] {
                                    if ui.selectable_value(&mut self.entries_per_page, size, format!("{}", size)).clicked() {
                                        self.current_page = 0;
                                        self.filter_entries();
                                    }
                                }
                            });
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("🔒 Lock").clicked() {
                            self.lock_vault();
                        }
                        
                        if ui.button("🎲 Generate").clicked() {
                            self.show_generator = true;
                            self.generate_password();
                        }
                        
                        if ui.button("➕ Add Entry").clicked() {
                            self.show_add_dialog = true;
                            self.clear_entry_form();
                        }
                    });
                });
            }
            
            ui.add_space(5.0);
        });
        
        // Advanced search panel
        if self.show_advanced_search {
            egui::TopBottomPanel::top("advanced_search").show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("🔍 Advanced Search & Filters");
                    ui.separator();
                    
                    if self.is_compact_mode {
                        // Compact mode: stack filter controls vertically
                        ui.horizontal(|ui| {
                            ui.label("Search in:");
                            egui::ComboBox::from_id_source("search_field")
                                .selected_text(match self.search_field {
                                    SearchField::All => "All Fields",
                                    SearchField::Site => "Website",
                                    SearchField::Username => "Username",
                                    SearchField::Notes => "Notes",
                                    SearchField::Tags => "Tags",
                                })
                                .width(140.0)
                                .show_ui(ui, |ui| {
                                    let mut changed = false;
                                    changed |= ui.selectable_value(&mut self.search_field, SearchField::All, "All Fields").changed();
                                    changed |= ui.selectable_value(&mut self.search_field, SearchField::Site, "Website").changed();
                                    changed |= ui.selectable_value(&mut self.search_field, SearchField::Username, "Username").changed();
                                    changed |= ui.selectable_value(&mut self.search_field, SearchField::Notes, "Notes").changed();
                                    changed |= ui.selectable_value(&mut self.search_field, SearchField::Tags, "Tags").changed();
                                    if changed {
                                        self.filter_entries();
                                    }
                                });
                            
                            if ui.checkbox(&mut self.filter_favorites, "⭐ Favorites").changed() {
                                self.filter_entries();
                            }
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("Clear All").clicked() {
                                    self.search_query.clear();
                                    self.filter_tags.clear();
                                    self.filter_favorites = false;
                                    self.search_field = SearchField::All;
                                    self.filter_entries();
                                }
                            });
                        });
                        
                        ui.add_space(2.0);
                        
                        ui.horizontal(|ui| {
                            ui.label("Tags:");
                            let tag_response = ui.add(
                                egui::TextEdit::singleline(&mut self.filter_tags)
                                    .desired_width(ui.available_width() - 50.0)
                                    .hint_text("Filter by tags (comma-separated)")
                            );
                            if tag_response.changed() {
                                self.filter_entries();
                            }
                        });
                    } else {
                        // Wide mode: horizontal layout
                        ui.horizontal(|ui| {
                            // Search field selector
                            ui.label("Search in:");
                            egui::ComboBox::from_id_source("search_field")
                                .selected_text(match self.search_field {
                                    SearchField::All => "All Fields",
                                    SearchField::Site => "Website",
                                    SearchField::Username => "Username",
                                    SearchField::Notes => "Notes",
                                    SearchField::Tags => "Tags",
                                })
                                .width(140.0)
                                .show_ui(ui, |ui| {
                                    let mut changed = false;
                                    changed |= ui.selectable_value(&mut self.search_field, SearchField::All, "All Fields").changed();
                                    changed |= ui.selectable_value(&mut self.search_field, SearchField::Site, "Website").changed();
                                    changed |= ui.selectable_value(&mut self.search_field, SearchField::Username, "Username").changed();
                                    changed |= ui.selectable_value(&mut self.search_field, SearchField::Notes, "Notes").changed();
                                    changed |= ui.selectable_value(&mut self.search_field, SearchField::Tags, "Tags").changed();
                                    if changed {
                                        self.filter_entries();
                                    }
                                });
                            
                            ui.separator();
                            
                            // Favorites filter
                            if ui.checkbox(&mut self.filter_favorites, "⭐ Favorites only").changed() {
                                self.filter_entries();
                            }
                            
                            ui.separator();
                            
                            // Tag filter
                            ui.label("Tags:");
                            let tag_width = if self.window_width > 1400.0 { 200.0 } else { 150.0 };
                            let tag_response = ui.add(
                                egui::TextEdit::singleline(&mut self.filter_tags)
                                    .desired_width(tag_width)
                                    .hint_text("Filter by tags (comma-separated)")
                            );
                            if tag_response.changed() {
                                self.filter_entries();
                            }
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("Clear All").clicked() {
                                    self.search_query.clear();
                                    self.filter_tags.clear();
                                    self.filter_favorites = false;
                                    self.search_field = SearchField::All;
                                    self.filter_entries();
                                }
                            });
                        });
                    }
                });
            });
        }
        
        // Status bar
        if !self.error_message.is_empty() || !self.success_message.is_empty() {
            egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if !self.error_message.is_empty() {
                        ui.colored_label(egui::Color32::from_rgb(255, 100, 100), &self.error_message);
                        if ui.small_button("✖").clicked() {
                            self.error_message.clear();
                        }
                    }
                    if !self.success_message.is_empty() {
                        ui.colored_label(egui::Color32::from_rgb(100, 255, 100), &self.success_message);
                        if ui.small_button("✖").clicked() {
                            self.success_message.clear();
                        }
                    }
                });
            });
        }
        
        // Footer with stats
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("📊 {} total entries", self.entries.len()));
                ui.separator();
                
                // Calculate total filtered entries before pagination
                let total_filtered = if self.search_query.is_empty() {
                    self.entries.len()
                } else {
                    let query = self.search_query.to_lowercase();
                    self.entries.iter()
                        .filter(|e| {
                            e.site.to_lowercase().contains(&query) ||
                            e.username.to_lowercase().contains(&query) ||
                            e.notes.as_ref().map(|n| n.to_lowercase().contains(&query)).unwrap_or(false) ||
                            e.tags.iter().any(|t| t.to_lowercase().contains(&query))
                        })
                        .count()
                };
                
                if self.total_pages > 1 {
                    let start_entry = self.current_page * self.entries_per_page + 1;
                    let end_entry = std::cmp::min((self.current_page + 1) * self.entries_per_page, total_filtered);
                    ui.label(format!("📄 Showing {}-{} of {} entries", start_entry, end_entry, total_filtered));
                } else {
                    ui.label(format!("🔍 {} shown", self.filtered_entries.len()));
                }
                
                // Show active filters
                let mut active_filters = Vec::new();
                if !self.search_query.is_empty() {
                    let field_name = match self.search_field {
                        SearchField::All => "all",
                        SearchField::Site => "site",
                        SearchField::Username => "username", 
                        SearchField::Notes => "notes",
                        SearchField::Tags => "tags",
                    };
                    active_filters.push(format!("🔍 {} '{}'", field_name, self.search_query));
                }
                if self.filter_favorites {
                    active_filters.push("⭐ favorites".to_string());
                }
                if !self.filter_tags.is_empty() {
                    active_filters.push(format!("🏷 tags '{}'", self.filter_tags));
                }
                
                if !active_filters.is_empty() && !self.is_compact_mode {
                    ui.separator();
                    ui.label("Filters:");
                    for filter in active_filters {
                        ui.small(&filter);
                    }
                } else if !active_filters.is_empty() && self.is_compact_mode {
                    // In compact mode, only show filter count
                    ui.separator();
                    ui.small(format!("🎯 {} filters active", active_filters.len()));
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Show security status with color-coded shield
                    let vault_secure = self.storage.lock().unwrap().is_some();
                    if vault_secure {
                        ui.colored_label(
                            egui::Color32::from_rgb(50, 200, 50), 
                            "🛡 Vault Secure"
                        );
                    } else {
                        ui.colored_label(
                            egui::Color32::from_rgb(200, 50, 50), 
                            "🛡 Vault Locked"
                        );
                    }
                });
            });
        });
        
        // Tab navigation
        egui::TopBottomPanel::top("tabs").show(ctx, |ui| {
            ui.add_space(5.0);
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 8.0;
                
                // Tab buttons with icons and styling
                let mut tab_button = |ui: &mut egui::Ui, tab: MainTab, icon: &str, text: &str| {
                    let is_selected = self.current_tab == tab;
                    let text_color = if is_selected {
                        ui.visuals().strong_text_color()
                    } else {
                        ui.visuals().text_color()
                    };
                    
                    let button_style = if is_selected {
                        egui::Button::new(format!("{} {}", icon, text))
                            .fill(ui.visuals().selection.bg_fill)
                            .stroke(egui::Stroke::new(1.0, ui.visuals().selection.stroke.color))
                    } else {
                        egui::Button::new(format!("{} {}", icon, text))
                            .fill(egui::Color32::TRANSPARENT)
                    };
                    
                    if ui.add(button_style).clicked() {
                        self.current_tab = tab;
                    }
                };
                
                tab_button(ui, MainTab::Passwords, "🔑", "Passwords");
                tab_button(ui, MainTab::Secrets, "🔐", "Secrets");
                tab_button(ui, MainTab::Generator, "🎲", "Generator");
                tab_button(ui, MainTab::Tools, "🛠", "Tools");
                tab_button(ui, MainTab::Settings, "⚙", "Settings");
                
                // Add entry count badges
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    match self.current_tab {
                        MainTab::Passwords => {
                            ui.label(
                                egui::RichText::new(format!("({} entries)", self.entries.len()))
                                    .size(11.0)
                                    .color(ui.visuals().weak_text_color())
                            );
                        }
                        MainTab::Secrets => {
                            ui.label(
                                egui::RichText::new(format!("({} secrets)", self.secrets.len()))
                                    .size(11.0)
                                    .color(ui.visuals().weak_text_color())
                            );
                        }
                        _ => {}
                    }
                });
            });
            ui.add_space(5.0);
        });
        
        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_tab {
                MainTab::Passwords => self.show_passwords_tab(ui),
                MainTab::Secrets => self.show_secrets_tab(ui),
                MainTab::Generator => self.show_generator_tab(ui),
                MainTab::Tools => self.show_tools_tab(ui),
                MainTab::Settings => self.show_settings_tab(ui),
            }
        });
        
        // Show dialogs
        self.show_entry_dialog(ctx);
        self.show_generator_dialog(ctx);
        self.show_settings_dialog(ctx);
        self.show_about_dialog(ctx);
        self.show_import_dialog(ctx);
        self.show_backup_dialog(ctx);
        self.show_statistics_dialog(ctx);
        self.show_tag_edit_dialog(ctx);
        self.show_secrets_view(ctx);
        self.show_add_secret_dialog(ctx);
    }
    
    fn show_passwords_tab(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            
            let entries_to_display = self.filtered_entries.clone();
            for entry in entries_to_display {
                let is_selected = self.selected_entry_id.as_ref() == Some(&entry.id);
                
                ui.push_id(&entry.id, |ui| {
                    let response = ui.add(
                        egui::SelectableLabel::new(is_selected, "")
                    );
                    
                    if response.clicked() {
                        self.selected_entry_id = Some(entry.id.clone());
                    }
                    
                    // Clone values for context menu
                    let entry_id = entry.id.clone();
                    let entry_username = entry.username.clone();
                    let entry_password = entry.password.clone();
                    let entry_for_edit = entry.clone();
                    
                    response.context_menu(|ui| {
                        ui.set_min_width(150.0);
                        if ui.button("📋 Copy Username").clicked() {
                            self.copy_to_clipboard(&entry_username);
                            self.success_message = "Username copied!".to_string();
                            ui.close_menu();
                        }
                        if ui.button("🔑 Copy Password").clicked() {
                            self.copy_to_clipboard(&entry_password);
                            self.success_message = "Password copied!".to_string();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("✏ Edit").clicked() {
                            self.edit_entry = Some(entry_for_edit.clone());
                            self.entry_site = entry_for_edit.site.clone();
                            self.entry_username = entry_for_edit.username.clone();
                            self.entry_password = entry_for_edit.password.clone();
                            self.entry_notes = entry_for_edit.notes.clone().unwrap_or_default();
                            self.entry_tags = entry_for_edit.tags.join(", ");
                            self.show_add_dialog = true;
                            ui.close_menu();
                        }
                        if ui.button("🗑 Delete").clicked() {
                            self.delete_entry(&entry_id);
                            ui.close_menu();
                        }
                    });
                    
                    // Entry row layout - responsive
                    if self.is_compact_mode {
                        // Compact mode: stack info vertically with buttons on right
                        ui.horizontal(|ui| {
                            ui.group(|ui| {
                                ui.set_min_width(ui.available_width() - 80.0);
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.strong(&entry.site);
                                        if entry.favorite {
                                            ui.label("⭐");
                                        }
                                    });
                                    ui.small(&entry.username);
                                    if !entry.tags.is_empty() && entry.tags.len() <= 2 {
                                        ui.horizontal(|ui| {
                                            for tag in entry.tags.iter().take(2) {
                                                ui.small(format!("🏷{}", tag));
                                            }
                                            if entry.tags.len() > 2 {
                                                ui.small("...");
                                            }
                                            if ui.small_button("✏").on_hover_text("Edit tags").clicked() {
                                                self.editing_tags_for_entry = Some(entry.id.clone());
                                                self.temp_tags = entry.tags.join(", ");
                                            }
                                        });
                                    } else if entry.tags.is_empty() {
                                        ui.horizontal(|ui| {
                                            ui.small("🏷No tags");
                                            if ui.small_button("✏").on_hover_text("Add tags").clicked() {
                                                self.editing_tags_for_entry = Some(entry.id.clone());
                                                self.temp_tags = String::new();
                                            }
                                        });
                                    }
                                });
                            });
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("🔑").on_hover_text("Copy password").clicked() {
                                    self.copy_to_clipboard(&entry.password);
                                    self.success_message = "Password copied!".to_string();
                                }
                                if ui.small_button("👤").on_hover_text("Copy username").clicked() {
                                    self.copy_to_clipboard(&entry.username);
                                    self.success_message = "Username copied!".to_string();
                                }
                            });
                        });
                    } else {
                        // Wide mode: full horizontal layout
                        ui.horizontal(|ui| {
                            ui.group(|ui| {
                                ui.set_min_width(ui.available_width() - 120.0);
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        ui.strong(&entry.site);
                                        if entry.favorite {
                                            ui.label("⭐");
                                        }
                                    });
                                    ui.label(&entry.username);
                                    if !entry.tags.is_empty() {
                                        ui.horizontal(|ui| {
                                            for tag in &entry.tags {
                                                ui.small(format!("🏷{}", tag));
                                            }
                                            if ui.small_button("✏").on_hover_text("Edit tags").clicked() {
                                                self.editing_tags_for_entry = Some(entry.id.clone());
                                                self.temp_tags = entry.tags.join(", ");
                                            }
                                        });
                                    } else {
                                        ui.horizontal(|ui| {
                                            ui.small("🏷No tags");
                                            if ui.small_button("✏").on_hover_text("Add tags").clicked() {
                                                self.editing_tags_for_entry = Some(entry.id.clone());
                                                self.temp_tags = String::new();
                                            }
                                        });
                                    }
                                });
                            });
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("🔑").on_hover_text("Copy password").clicked() {
                                    self.copy_to_clipboard(&entry.password);
                                    self.success_message = "Password copied!".to_string();
                                }
                                if ui.small_button("👤").on_hover_text("Copy username").clicked() {
                                    self.copy_to_clipboard(&entry.username);
                                    self.success_message = "Username copied!".to_string();
                                }
                            });
                        });
                    }
                });
            }
        });
    }
    
    fn show_secrets_tab(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("🔐 Secrets Manager");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("➕ Add Secret").clicked() {
                    self.show_add_secret_dialog = true;
                    self.selected_secret_type = SecretType::ApiKey;
                }
            });
        });
        
        ui.separator();
        
        // Secrets filter tabs
        ui.horizontal(|ui| {
            let mut secret_tab_button = |ui: &mut egui::Ui, secret_type: SecretType, icon: &str, text: &str| {
                let is_selected = self.current_secret_tab == secret_type;
                let button_style = if is_selected {
                    egui::Button::new(format!("{} {}", icon, text))
                        .fill(ui.visuals().selection.bg_fill)
                } else {
                    egui::Button::new(format!("{} {}", icon, text))
                        .fill(egui::Color32::TRANSPARENT)
                };
                
                if ui.add(button_style).clicked() {
                    self.current_secret_tab = secret_type;
                }
            };
            
            secret_tab_button(ui, SecretType::ApiKey, "🔑", "API Keys");
            secret_tab_button(ui, SecretType::SshKey, "🔐", "SSH Keys");
            secret_tab_button(ui, SecretType::Document, "📄", "Documents");
            secret_tab_button(ui, SecretType::Configuration, "⚙", "Config");
            secret_tab_button(ui, SecretType::SecureNote, "📝", "Notes");
            secret_tab_button(ui, SecretType::ConnectionString, "🗄", "Database");
        });
        
        ui.separator();
        
        // Secrets list
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 4.0;
            
            let filtered_secrets: Vec<_> = self.filtered_secrets.iter()
                .filter(|secret| secret.secret_type == self.current_secret_tab)
                .cloned()
                .collect();
            
            if filtered_secrets.is_empty() {
                ui.vertical_centered(|ui| {
                    ui.add_space(50.0);
                    ui.label(format!("📭 No {} found", match self.current_secret_tab {
                        SecretType::ApiKey => "API keys",
                        SecretType::SshKey => "SSH keys", 
                        SecretType::Document => "documents",
                        SecretType::Configuration => "configurations",
                        SecretType::SecureNote => "secure notes",
                        SecretType::ConnectionString => "database connections",
                        _ => "secrets",
                    }));
                    ui.small("Click 'Add Secret' to create your first secret");
                });
            } else {
                let mut action = None;
                
                for secret in &filtered_secrets {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.vertical(|ui| {
                                ui.strong(&secret.name);
                                if let Some(desc) = &secret.description {
                                    ui.small(desc);
                                }
                                if !secret.tags.is_empty() {
                                    ui.horizontal(|ui| {
                                        for tag in secret.tags.iter().take(3) {
                                            ui.small(format!("🏷{}", tag));
                                        }
                                        if secret.tags.len() > 3 {
                                            ui.small("...");
                                        }
                                    });
                                }
                            });
                            
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.small_button("📋").on_hover_text("Copy").clicked() {
                                    action = Some(("copy", secret.clone()));
                                }
                                if ui.small_button("✏").on_hover_text("Edit").clicked() {
                                    action = Some(("edit", secret.clone()));
                                }
                                if ui.small_button("🗑").on_hover_text("Delete").clicked() {
                                    action = Some(("delete", secret.clone()));
                                }
                            });
                        });
                    });
                    ui.add_space(2.0);
                }
                
                // Handle actions outside the loop to avoid borrowing issues
                if let Some((action_type, secret)) = action {
                    match action_type {
                        "copy" => self.copy_secret_data(&secret),
                        "edit" => self.success_message = "Edit not yet implemented".to_string(),
                        "delete" => self.delete_secret(&secret.id),
                        _ => {}
                    }
                }
            }
        });
    }
    
    fn show_generator_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("🎲 Password Generator");
        ui.separator();
        
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(250.0);
                
                ui.group(|ui| {
                    ui.label("🔧 Settings");
                    
                    egui::Grid::new("generator_settings").num_columns(2).show(ui, |ui| {
                        ui.label("Length:");
                        ui.add(egui::Slider::new(&mut self.gen_length, 4..=128));
                        ui.end_row();
                        
                        ui.label("Uppercase:");
                        ui.checkbox(&mut self.gen_uppercase, "A-Z");
                        ui.end_row();
                        
                        ui.label("Lowercase:");
                        ui.checkbox(&mut self.gen_lowercase, "a-z");
                        ui.end_row();
                        
                        ui.label("Numbers:");
                        ui.checkbox(&mut self.gen_numbers, "0-9");
                        ui.end_row();
                        
                        ui.label("Symbols:");
                        ui.checkbox(&mut self.gen_symbols, "!@#$");
                        ui.end_row();
                        
                        ui.label("Exclude ambiguous:");
                        ui.checkbox(&mut self.gen_exclude_ambiguous, "0O1lI");
                        ui.end_row();
                    });
                    
                    ui.separator();
                    
                    if ui.button("🎲 Generate New Password").clicked() {
                        self.generate_password();
                    }
                });
            });
            
            ui.separator();
            
            ui.vertical(|ui| {
                ui.label("🔑 Generated Password:");
                ui.add_space(5.0);
                
                if !self.generated_password.is_empty() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.generated_password)
                                    .desired_width(300.0)
                                    .font(egui::TextStyle::Monospace)
                            );
                            if ui.button("📋 Copy").clicked() {
                                self.copy_to_clipboard(&self.generated_password);
                                self.success_message = "Password copied!".to_string();
                            }
                        });
                    });
                    
                    ui.add_space(10.0);
                    
                    // Password strength indicator
                    let strength = self.calculate_password_strength(&self.generated_password);
                    ui.label(format!("💪 Strength: {}", strength));
                } else {
                    ui.label("Generate a password to see it here");
                }
            });
        });
    }
    
    fn show_tools_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("🛠 Tools & Utilities");
        ui.separator();
        ui.add_space(10.0);
        
        // Main Tools Grid - 2x3 layout for better space usage
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("main_tools_grid")
                .num_columns(2)
                .spacing([30.0, 20.0])
                .min_col_width(350.0)
                .show(ui, |ui| {
                    
                    // Data Management Card
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(350.0, 200.0));
                        ui.vertical(|ui| {
                            ui.heading("📦 Data Management");
                            ui.separator();
                            ui.add_space(10.0);
                            
                            // Import section
                            ui.horizontal(|ui| {
                                ui.label("📥");
                                ui.vertical(|ui| {
                                    if ui.button("Import from Browser").clicked() {
                                        self.show_import = true;
                                    }
                                    ui.small("Chrome, Firefox, Safari, Edge...");
                                });
                            });
                            
                            ui.add_space(8.0);
                            
                            // Export section
                            ui.horizontal(|ui| {
                                ui.label("📤");
                                ui.vertical(|ui| {
                                    if ui.button("Create Backup").clicked() {
                                        self.show_backup = true;
                                    }
                                    ui.small("Encrypted vault backup");
                                });
                            });
                            
                            ui.add_space(8.0);
                            
                            // Sync section
                            ui.horizontal(|ui| {
                                ui.label("🔄");
                                ui.vertical(|ui| {
                                    ui.add_enabled(false, egui::Button::new("Cloud Sync"));
                                    ui.small("Coming soon - sync across devices");
                                });
                            });
                        });
                    });
                    
                    // Analysis & Security Card
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(350.0, 200.0));
                        ui.vertical(|ui| {
                            ui.heading("🔍 Analysis & Security");
                            ui.separator();
                            ui.add_space(10.0);
                            
                            // Statistics section
                            ui.horizontal(|ui| {
                                ui.label("📊");
                                ui.vertical(|ui| {
                                    if ui.button("Vault Statistics").clicked() {
                                        self.show_statistics = true;
                                    }
                                    ui.small(format!("{} passwords • {} secrets", self.entries.len(), self.secrets.len()));
                                });
                            });
                            
                            ui.add_space(8.0);
                            
                            // Security audit section
                            ui.horizontal(|ui| {
                                ui.label("🛡");
                                ui.vertical(|ui| {
                                    ui.add_enabled(false, egui::Button::new("Security Audit"));
                                    ui.small("Password strength & breach check");
                                });
                            });
                            
                            ui.add_space(8.0);
                            
                            // Duplicate detection
                            ui.horizontal(|ui| {
                                ui.label("🔍");
                                ui.vertical(|ui| {
                                    ui.add_enabled(false, egui::Button::new("Find Duplicates"));
                                    ui.small("Identify duplicate passwords");
                                });
                            });
                        });
                    });
                    
                    ui.end_row();
                    
                    // Vault Management Card
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(350.0, 200.0));
                        ui.vertical(|ui| {
                            ui.heading("🔐 Vault Management");
                            ui.separator();
                            ui.add_space(10.0);
                            
                            // Reload data
                            ui.horizontal(|ui| {
                                ui.label("🔄");
                                ui.vertical(|ui| {
                                    if ui.button("Reload Data").clicked() {
                                        self.load_entries();
                                        self.success_message = "Data reloaded successfully".to_string();
                                    }
                                    ui.small("Refresh entries and secrets");
                                });
                            });
                            
                            ui.add_space(8.0);
                            
                            // Lock vault
                            ui.horizontal(|ui| {
                                ui.label("🔒");
                                ui.vertical(|ui| {
                                    if ui.button("Lock Vault").clicked() {
                                        self.lock_vault();
                                    }
                                    ui.small("Secure and close vault");
                                });
                            });
                            
                            ui.add_space(8.0);
                            
                            // Database maintenance
                            ui.horizontal(|ui| {
                                ui.label("🔧");
                                ui.vertical(|ui| {
                                    ui.add_enabled(false, egui::Button::new("Database Repair"));
                                    ui.small("Optimize and repair database");
                                });
                            });
                        });
                    });
                    
                    // Utilities & Cleanup Card
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(350.0, 200.0));
                        ui.vertical(|ui| {
                            ui.heading("🧹 Utilities & Cleanup");
                            ui.separator();
                            ui.add_space(10.0);
                            
                            // Clean duplicates
                            ui.horizontal(|ui| {
                                ui.label("🗑");
                                ui.vertical(|ui| {
                                    ui.add_enabled(false, egui::Button::new("Remove Duplicates"));
                                    ui.small("Clean up duplicate entries");
                                });
                            });
                            
                            ui.add_space(8.0);
                            
                            // Fix tags
                            ui.horizontal(|ui| {
                                ui.label("🏷");
                                ui.vertical(|ui| {
                                    ui.add_enabled(false, egui::Button::new("Organize Tags"));
                                    ui.small("Clean and organize tags");
                                });
                            });
                            
                            ui.add_space(8.0);
                            
                            // Export reports
                            ui.horizontal(|ui| {
                                ui.label("📋");
                                ui.vertical(|ui| {
                                    ui.add_enabled(false, egui::Button::new("Export Report"));
                                    ui.small("Generate security report");
                                });
                            });
                        });
                    });
                    
                    ui.end_row();
                    
                    // System Information Card
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(350.0, 200.0));
                        ui.vertical(|ui| {
                            ui.heading("ℹ System Information");
                            ui.separator();
                            ui.add_space(10.0);
                            
                            egui::Grid::new("system_info_grid").num_columns(2).spacing([20.0, 5.0]).show(ui, |ui| {
                                ui.label("Version:");
                                ui.label("PwGen v0.1.0");
                                ui.end_row();
                                
                                ui.label("Engine:");
                                ui.label("🦀 Rust + egui");
                                ui.end_row();
                                
                                ui.label("Encryption:");
                                ui.label("🔒 AES-256-GCM");
                                ui.end_row();
                                
                                ui.label("Storage:");
                                ui.label("💾 SQLite");
                                ui.end_row();
                                
                                ui.label("Layout:");
                                ui.label(if self.is_compact_mode { "📱 Compact" } else { "🖥 Wide" });
                                ui.end_row();
                                
                                ui.label("Window:");
                                ui.label(format!("{:.0}px wide", self.window_width));
                                ui.end_row();
                            });
                        });
                    });
                    
                    // Resources & Support Card
                    ui.group(|ui| {
                        ui.set_min_size(egui::vec2(350.0, 200.0));
                        ui.vertical(|ui| {
                            ui.heading("🌐 Resources & Support");
                            ui.separator();
                            ui.add_space(10.0);
                            
                            // Documentation
                            ui.horizontal(|ui| {
                                ui.label("📚");
                                ui.vertical(|ui| {
                                    if ui.button("Documentation").clicked() {
                                        let _ = open::that("https://github.com/hxhippy/pwgen/blob/main/README.md");
                                    }
                                    ui.small("User guide and API docs");
                                });
                            });
                            
                            ui.add_space(8.0);
                            
                            // About
                            ui.horizontal(|ui| {
                                ui.label("ℹ");
                                ui.vertical(|ui| {
                                    if ui.button("About PwGen").clicked() {
                                        self.show_about = true;
                                    }
                                    ui.small("Version info and credits");
                                });
                            });
                            
                            ui.add_space(8.0);
                            
                            // External links
                            ui.horizontal(|ui| {
                                ui.label("🔗");
                                ui.vertical(|ui| {
                                    ui.horizontal(|ui| {
                                        if ui.small_button("Kief Studio").clicked() {
                                            let _ = open::that("https://kief.studio");
                                        }
                                        if ui.small_button("TRaViS ASM").clicked() {
                                            let _ = open::that("https://travisasm.com");
                                        }
                                    });
                                    ui.small("Developer websites");
                                });
                            });
                        });
                    });
                });
        });
    }
    
    fn show_settings_tab(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙ Settings");
        ui.separator();
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.group(|ui| {
                ui.label("🔒 Security");
                ui.separator();
                
                egui::Grid::new("security_settings").num_columns(2).show(ui, |ui| {
                    ui.label("Auto-lock after (minutes):");
                    ui.add(egui::DragValue::new(&mut self.auto_lock_minutes).clamp_range(1..=60));
                    ui.end_row();
                    
                    ui.label("Minimize to tray:");
                    ui.checkbox(&mut self.minimize_to_tray, "");
                    ui.end_row();
                    
                    ui.label("Show system tray:");
                    ui.checkbox(&mut self.show_system_tray, "");
                    ui.end_row();
                });
            });
            
            ui.add_space(10.0);
            
            ui.group(|ui| {
                ui.label("🎨 Appearance");
                ui.separator();
                
                egui::Grid::new("appearance_settings").num_columns(2).show(ui, |ui| {
                    ui.label("Entries per page:");
                    ui.add(egui::DragValue::new(&mut self.entries_per_page).clamp_range(10..=500));
                    ui.end_row();
                    
                    ui.label("Theme:");
                    ui.label("System (auto)");
                    ui.end_row();
                    
                    ui.label("Font:");
                    ui.label("Fira Code");
                    ui.end_row();
                });
            });
            
            ui.add_space(10.0);
            
            ui.group(|ui| {
                ui.label("💾 Data");
                ui.separator();
                
                if ui.button("🔄 Reload Entries").clicked() {
                    self.load_entries();
                    self.success_message = "Entries reloaded".to_string();
                }
                
                ui.add_space(5.0);
                
                if ui.button("💾 Backup Now").clicked() {
                    self.show_backup = true;
                }
                
                if ui.button("📥 Import Data").clicked() {
                    self.show_import = true;
                }
            });
            
            ui.add_space(10.0);
            
            ui.group(|ui| {
                ui.label("ℹ Information");
                ui.separator();
                
                ui.label("PwGen v0.1.0");
                ui.label("🦀 Built with Rust + egui");
                ui.label("🔒 AES-256-GCM encryption");
                ui.label("💾 SQLite storage");
                
                ui.add_space(5.0);
                
                if ui.button("ℹ About").clicked() {
                    self.show_about = true;
                }
            });
        });
    }
    
    fn calculate_password_strength(&self, password: &str) -> &'static str {
        let length = password.len();
        let has_upper = password.chars().any(|c| c.is_uppercase());
        let has_lower = password.chars().any(|c| c.is_lowercase());
        let has_digit = password.chars().any(|c| c.is_numeric());
        let has_symbol = password.chars().any(|c| !c.is_alphanumeric());
        
        let variety = [has_upper, has_lower, has_digit, has_symbol].iter().filter(|&&x| x).count();
        
        match (length, variety) {
            (0..=7, _) => "Very Weak",
            (8..=11, 1..=2) => "Weak", 
            (8..=11, 3..=4) => "Fair",
            (12..=15, 1..=2) => "Fair",
            (12..=15, 3..=4) => "Good",
            (16.., 1..=2) => "Good",
            (16.., 3..=4) => "Strong",
            _ => "Weak",
        }
    }
    
    fn show_entry_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_add_dialog {
            return;
        }
        
        egui::Window::new(if self.edit_entry.is_some() { "Edit Entry" } else { "Add New Entry" })
            .collapsible(false)
            .resizable(false)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    egui::Grid::new("entry_form").num_columns(2).show(ui, |ui| {
                        ui.label("Website:");
                        ui.text_edit_singleline(&mut self.entry_site);
                        ui.end_row();
                        
                        ui.label("Username:");
                        ui.text_edit_singleline(&mut self.entry_username);
                        ui.end_row();
                        
                        ui.label("Password:");
                        ui.horizontal(|ui| {
                            if self.show_password {
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.entry_password)
                                        .font(egui::TextStyle::Monospace)
                                );
                            } else {
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.entry_password)
                                        .password(true)
                                        .font(egui::TextStyle::Monospace)
                                );
                            }
                            if ui.button(if self.show_password { "🙈" } else { "👁" }).clicked() {
                                self.show_password = !self.show_password;
                            }
                            if ui.button("🎲").on_hover_text("Generate").clicked() {
                                self.show_generator = true;
                                self.generate_password();
                            }
                        });
                        ui.end_row();
                        
                        ui.label("Notes:");
                        ui.add(egui::TextEdit::multiline(&mut self.entry_notes).desired_rows(3));
                        ui.end_row();
                        
                        ui.label("Tags:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.entry_tags)
                                .hint_text("Comma separated tags...")
                        );
                        ui.end_row();
                    });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("💾 Save").clicked() {
                            self.save_entry();
                        }
                        if ui.button("❌ Cancel").clicked() {
                            self.show_add_dialog = false;
                            self.clear_entry_form();
                        }
                    });
                });
            });
    }
    
    fn show_generator_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_generator {
            return;
        }
        
        egui::Window::new("Password Generator")
            .collapsible(false)
            .resizable(false)
            .default_width(350.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.generated_password)
                                    .desired_width(250.0)
                                    .font(egui::TextStyle::Monospace)
                                    .text_color(ui.visuals().strong_text_color())
                            );
                            if ui.button("📋").on_hover_text("Copy").clicked() {
                                self.copy_to_clipboard(&self.generated_password);
                                self.success_message = "Password copied!".to_string();
                            }
                            if ui.button("🔄").on_hover_text("Regenerate").clicked() {
                                self.generate_password();
                            }
                        });
                    });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        ui.label("Length:");
                        ui.add(egui::Slider::new(&mut self.gen_length, 8..=128));
                    });
                    
                    ui.checkbox(&mut self.gen_uppercase, "Uppercase (A-Z)");
                    ui.checkbox(&mut self.gen_lowercase, "Lowercase (a-z)");
                    ui.checkbox(&mut self.gen_numbers, "Numbers (0-9)");
                    ui.checkbox(&mut self.gen_symbols, "Symbols (!@#$%)");
                    ui.checkbox(&mut self.gen_exclude_ambiguous, "Exclude ambiguous (0O1lI)");
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("✅ Use This Password").clicked() {
                            self.entry_password = self.generated_password.clone();
                            self.show_generator = false;
                        }
                        if ui.button("❌ Close").clicked() {
                            self.show_generator = false;
                        }
                    });
                });
            });
    }
    
    fn show_settings_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_settings {
            return;
        }
        
        egui::Window::new("⚙ Settings")
            .collapsible(false)
            .resizable(false)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.label("Security Settings");
                        ui.horizontal(|ui| {
                            ui.label("Auto-lock after:");
                            ui.add(egui::Slider::new(&mut self.auto_lock_minutes, 1..=60).suffix(" min"));
                        });
                        ui.checkbox(&mut self.minimize_to_tray, "Minimize to system tray");
                    });
                    
                    ui.add_space(10.0);
                    
                    ui.group(|ui| {
                        ui.label("Generator Defaults");
                        ui.horizontal(|ui| {
                            ui.label("Default length:");
                            ui.add(egui::Slider::new(&mut self.gen_length, 8..=128));
                        });
                        ui.checkbox(&mut self.gen_uppercase, "Include uppercase by default");
                        ui.checkbox(&mut self.gen_numbers, "Include numbers by default");
                        ui.checkbox(&mut self.gen_symbols, "Include symbols by default");
                        ui.checkbox(&mut self.gen_exclude_ambiguous, "Exclude ambiguous characters");
                    });
                    
                    ui.separator();
                    
                    ui.horizontal(|ui| {
                        if ui.button("💾 Save").clicked() {
                            // TODO: Save settings to config file
                            self.success_message = "Settings saved!".to_string();
                            self.show_settings = false;
                        }
                        if ui.button("❌ Cancel").clicked() {
                            self.show_settings = false;
                        }
                    });
                });
            });
    }
    
    fn show_about_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_about {
            return;
        }
        
        egui::Window::new("ℹ About PwGen")
            .collapsible(false)
            .resizable(false)
            .default_width(450.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    if let Some(logo) = &self.logo_wide {
                        ui.add(
                            egui::Image::from_texture(logo)
                                .fit_to_exact_size(egui::vec2(150.0, 50.0))
                        );
                        ui.add_space(10.0);
                    }
                    
                    ui.heading("PwGen Password Manager");
                    ui.label("Version 1.0.0");
                    ui.add_space(10.0);
                    
                    ui.label("A secure, cross-platform password manager built in Rust");
                    ui.label("with military-grade encryption and modern UI.");
                    
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(15.0);
                    
                    ui.strong("Developed by HxHippy");
                    if ui.link("🌐 hxhippy.com").clicked() {
                        let _ = open::that("https://hxhippy.com");
                    }
                    
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("🤖 AI Consulting:");
                        if ui.link("Kief Studio").clicked() {
                            let _ = open::that("https://kief.studio");
                        }
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("🔍 Attack Surface Management:");
                        if ui.link("TRaViS").clicked() {
                            let _ = open::that("https://travisasm.com");
                        }
                    });
                    
                    ui.add_space(15.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label("🛡 Security:");
                        ui.label("AES-256-GCM, Argon2, Zero-Knowledge");
                    });
                    
                    ui.horizontal(|ui| {
                        ui.label("🦀 Built with:");
                        ui.label("Rust, egui, SQLite");
                    });
                    
                    ui.add_space(15.0);
                    
                    if ui.button("✅ Close").clicked() {
                        self.show_about = false;
                    }
                });
            });
    }
    
    fn show_statistics_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_statistics {
            return;
        }
        
        egui::Window::new("📊 Vault Statistics")
            .collapsible(false)
            .resizable(false)
            .default_width(350.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.label("📈 Entry Statistics");
                        ui.horizontal(|ui| {
                            ui.label("Total entries:");
                            ui.strong(format!("{}", self.entries.len()));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Entries shown:");
                            ui.strong(format!("{}", self.filtered_entries.len()));
                        });
                        
                        // Calculate some stats
                        let unique_sites: std::collections::HashSet<_> = self.entries.iter().map(|e| &e.site).collect();
                        ui.horizontal(|ui| {
                            ui.label("Unique sites:");
                            ui.strong(format!("{}", unique_sites.len()));
                        });
                        
                        let total_tags: std::collections::HashSet<_> = self.entries.iter().flat_map(|e| &e.tags).collect();
                        ui.horizontal(|ui| {
                            ui.label("Total tags:");
                            ui.strong(format!("{}", total_tags.len()));
                        });
                    });
                    
                    ui.add_space(10.0);
                    
                    ui.group(|ui| {
                        ui.label("🔒 Security Analysis");
                        
                        // Analyze password security
                        let weak_passwords = self.entries.iter()
                            .filter(|e| e.password.len() < 12)
                            .count();
                        let strong_passwords = self.entries.iter()
                            .filter(|e| e.password.len() >= 16)
                            .count();
                        
                        ui.horizontal(|ui| {
                            ui.label("Strong passwords (16+ chars):");
                            ui.colored_label(egui::Color32::from_rgb(50, 200, 50), format!("{}", strong_passwords));
                        });
                        ui.horizontal(|ui| {
                            ui.label("Weak passwords (<12 chars):");
                            ui.colored_label(egui::Color32::from_rgb(200, 50, 50), format!("{}", weak_passwords));
                        });
                        
                        // Password age analysis
                        let old_passwords = self.entries.iter()
                            .filter(|e| {
                                let age = chrono::Utc::now().signed_duration_since(e.password_changed_at);
                                age.num_days() > 90
                            })
                            .count();
                        
                        ui.horizontal(|ui| {
                            ui.label("Passwords >90 days old:");
                            if old_passwords > 0 {
                                ui.colored_label(egui::Color32::from_rgb(200, 150, 50), format!("{}", old_passwords));
                            } else {
                                ui.colored_label(egui::Color32::from_rgb(50, 200, 50), format!("{}", old_passwords));
                            }
                        });
                    });
                    
                    ui.add_space(15.0);
                    
                    if ui.button("✅ Close").clicked() {
                        self.show_statistics = false;
                    }
                });
            });
    }
    
    fn show_import_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_import {
            return;
        }
        
        egui::Window::new("📥 Import from Browser")
            .collapsible(false)
            .resizable(false)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("Import passwords from your web browser:");
                    ui.add_space(10.0);
                    
                    ui.group(|ui| {
                        ui.label("🌐 Supported Browsers");
                        
                        if ui.button("🟦 Google Chrome").clicked() {
                            self.import_from_browser("chrome");
                        }
                        if ui.button("🟧 Mozilla Firefox").clicked() {
                            self.import_from_browser("firefox");
                        }
                        if ui.button("🟦 Microsoft Edge").clicked() {
                            self.import_from_browser("edge");
                        }
                        if ui.button("🟣 Opera").clicked() {
                            self.import_from_browser("opera");
                        }
                        if ui.button("🟠 Brave").clicked() {
                            self.import_from_browser("brave");
                        }
                        if ui.button("🔵 Safari").clicked() {
                            self.import_from_browser("safari");
                        }
                    });
                    
                    ui.add_space(10.0);
                    ui.separator();
                    
                    ui.label("⚠ Note: Browser must be closed during import");
                    ui.label("🔒 All imported passwords are encrypted with your master password");
                    
                    ui.add_space(15.0);
                    
                    if ui.button("❌ Cancel").clicked() {
                        self.show_import = false;
                    }
                });
            });
    }
    
    fn show_backup_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_backup {
            return;
        }
        
        egui::Window::new("💾 Backup & Restore")
            .collapsible(false)
            .resizable(false)
            .default_width(400.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.group(|ui| {
                        ui.label("📤 Create Backup");
                        ui.label("Export your encrypted vault to a backup file");
                        ui.add_space(5.0);
                        
                        if ui.button("💾 Create Backup File").clicked() {
                            self.create_backup();
                        }
                    });
                    
                    ui.add_space(10.0);
                    
                    ui.group(|ui| {
                        ui.label("📥 Restore from Backup");
                        ui.label("Import entries from a previously created backup");
                        ui.add_space(5.0);
                        
                        if ui.button("📂 Select Backup File").clicked() {
                            self.restore_backup();
                        }
                    });
                    
                    ui.add_space(10.0);
                    ui.separator();
                    
                    ui.label("🔒 Backups are encrypted with your master password");
                    ui.label("📁 Keep backups in a secure location");
                    
                    ui.add_space(15.0);
                    
                    if ui.button("❌ Close").clicked() {
                        self.show_backup = false;
                    }
                });
            });
    }
    
    fn import_from_browser(&mut self, browser: &str) {
        // For now, show a file dialog to select CSV file from browser export
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("CSV Files", &["csv"])
            .set_title(format!("Select {} password export file", browser))
            .pick_file()
        {
            let storage_mutex = self.storage.clone();
            let runtime = self.runtime.clone();
            
            self.error_message.clear();
            
            let result = runtime.block_on(async {
                let storage_guard = storage_mutex.lock().unwrap();
                if let Some(storage) = storage_guard.as_ref() {
                    // Use the browser import functionality
                    let config = pwgen_core::browser_import::ImportConfig {
                        browser_type: match browser {
                            "chrome" => pwgen_core::browser_import::BrowserType::Chrome,
                            "firefox" => pwgen_core::browser_import::BrowserType::Firefox,
                            "edge" => pwgen_core::browser_import::BrowserType::Edge,
                            "opera" => pwgen_core::browser_import::BrowserType::Opera,
                            "brave" => pwgen_core::browser_import::BrowserType::Brave,
                            "safari" => pwgen_core::browser_import::BrowserType::Safari,
                            _ => pwgen_core::browser_import::BrowserType::Chrome,
                        },
                        format: pwgen_core::browser_import::ImportFormat::Csv,
                        skip_duplicates: true,
                        merge_duplicates: false,
                        import_folders_as_tags: true,
                        default_tags: vec![format!("imported-{}", browser)],
                        password_strength_check: false,
                        cleanup_urls: true,
                    };
                    
                    match pwgen_core::browser_import::BrowserImporter::import_from_file(path, config) {
                        Ok((imported_passwords, _result)) => {
                            // Create a simple config for conversion
                            let convert_config = pwgen_core::browser_import::ImportConfig::default();
                            match pwgen_core::browser_import::BrowserImporter::convert_to_entries(imported_passwords, &convert_config) {
                                Ok(entries) => {
                                    let mut imported_count = 0;
                                    for entry in entries {
                                        if storage.add_entry(&entry).await.is_ok() {
                                            imported_count += 1;
                                        }
                                    }
                                    Ok(imported_count)
                                }
                                Err(e) => Err(e.to_string())
                            }
                        }
                        Err(e) => Err(e.to_string())
                    }
                } else {
                    Err("Storage not initialized".to_string())
                }
            });
            
            match result {
                Ok(count) => {
                    self.success_message = format!("Successfully imported {} passwords from {}", count, browser);
                    self.load_entries();
                    self.show_import = false;
                }
                Err(e) => {
                    self.error_message = format!("Import failed: {}", e);
                }
            }
        }
    }
    
    fn create_backup(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON Backup", &["json"])
            .set_file_name(format!("pwgen_backup_{}.json", chrono::Utc::now().format("%Y%m%d_%H%M%S")))
            .save_file()
        {
            let storage_mutex = self.storage.clone();
            let runtime = self.runtime.clone();
            
            let result = runtime.block_on(async {
                let storage_guard = storage_mutex.lock().unwrap();
                if let Some(storage) = storage_guard.as_ref() {
                    // Get all entries and serialize to JSON
                    let filter = pwgen_core::models::SearchFilter {
                        query: None,
                        tags: None,
                        favorite_only: false,
                        sort_by: pwgen_core::models::SortField::Site,
                        sort_order: pwgen_core::models::SortOrder::Ascending,
                    };
                    
                    match storage.search_entries(&filter).await {
                        Ok(entries) => {
                            let backup_data = serde_json::to_string_pretty(&entries)
                                .map_err(|e| pwgen_core::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
                            
                            std::fs::write(&path, backup_data)
                                .map_err(pwgen_core::Error::Io)?;
                            
                            Ok(())
                        }
                        Err(e) => Err(e)
                    }
                } else {
                    Err(pwgen_core::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Storage not initialized")))
                }
            });
            
            match result {
                Ok(_) => {
                    self.success_message = "Backup created successfully!".to_string();
                    self.show_backup = false;
                }
                Err(e) => {
                    self.error_message = format!("Backup failed: {}", e);
                }
            }
        }
    }
    
    fn restore_backup(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON Backup", &["json"])
            .pick_file()
        {
            let storage_mutex = self.storage.clone();
            let runtime = self.runtime.clone();
            
            let result = runtime.block_on(async {
                let storage_guard = storage_mutex.lock().unwrap();
                if let Some(storage) = storage_guard.as_ref() {
                    // Read and deserialize the backup file
                    let backup_data = std::fs::read_to_string(&path)
                        .map_err(pwgen_core::Error::Io)?;
                    
                    let entries: Vec<pwgen_core::models::DecryptedPasswordEntry> = serde_json::from_str(&backup_data)
                        .map_err(|e| pwgen_core::Error::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())))?;
                    
                    let mut imported_count = 0;
                    for entry in entries {
                        if storage.add_entry(&entry).await.is_ok() {
                            imported_count += 1;
                        }
                    }
                    
                    Ok(imported_count)
                } else {
                    Err(pwgen_core::Error::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "Storage not initialized")))
                }
            });
            
            match result {
                Ok(count) => {
                    self.success_message = format!("Successfully restored {} entries from backup!", count);
                    self.load_entries();
                    self.show_backup = false;
                }
                Err(e) => {
                    self.error_message = format!("Restore failed: {}", e);
                }
            }
        }
    }
    
    fn show_tag_edit_dialog(&mut self, ctx: &egui::Context) {
        if let Some(entry_id) = &self.editing_tags_for_entry.clone() {
            egui::Window::new("Edit Tags")
                .collapsible(false)
                .resizable(false)
                .default_width(300.0)
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.label("Tags (comma-separated):");
                        let response = ui.add(
                            egui::TextEdit::singleline(&mut self.temp_tags)
                                .desired_width(ui.available_width() - 20.0)
                                .hint_text("tag1, tag2, tag3...")
                        );
                        
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.save_tags(entry_id.clone());
                        }
                        
                        ui.add_space(10.0);
                        
                        ui.horizontal(|ui| {
                            if ui.button("Save").clicked() {
                                self.save_tags(entry_id.clone());
                            }
                            if ui.button("Cancel").clicked() {
                                self.editing_tags_for_entry = None;
                                self.temp_tags.clear();
                            }
                        });
                    });
                });
        }
    }
    
    fn save_tags(&mut self, entry_id: String) {
        let storage_mutex = self.storage.clone();
        let runtime = self.runtime.clone();
        
        // Parse tags from temp_tags
        let new_tags: Vec<String> = self.temp_tags
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        
        // Find and update the entry
        if let Some(entry) = self.entries.iter_mut().find(|e| e.id == entry_id) {
            entry.tags = new_tags.clone();
            entry.updated_at = chrono::Utc::now();
            
            // Save to storage
            let entry_to_save = entry.clone();
            runtime.block_on(async {
                let storage_guard = storage_mutex.lock().unwrap();
                if let Some(storage) = storage_guard.as_ref() {
                    let _ = storage.update_entry(&entry_to_save).await;
                }
            });
            
            self.success_message = "Tags updated successfully!".to_string();
            self.filter_entries(); // Refresh the display
        }
        
        // Clear editing state
        self.editing_tags_for_entry = None;
        self.temp_tags.clear();
    }
    
    fn show_secrets_view(&mut self, ctx: &egui::Context) {
        if !self.show_secrets_view {
            return;
        }
        
        egui::Window::new("🔐 Secrets Manager")
            .collapsible(false)
            .resizable(true)
            .default_size([800.0, 600.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header with close button
                    ui.horizontal(|ui| {
                        ui.heading("Secrets Manager");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("✖ Close").clicked() {
                                self.show_secrets_view = false;
                            }
                        });
                    });
                    
                    ui.separator();
                    
                    // Tab bar for different secret types
                    ui.horizontal(|ui| {
                        ui.label("Type:");
                        if ui.selectable_label(matches!(self.current_secret_tab, SecretType::ApiKey), "🔑 API Keys").clicked() {
                            self.current_secret_tab = SecretType::ApiKey;
                        }
                        if ui.selectable_label(matches!(self.current_secret_tab, SecretType::SshKey), "🔐 SSH Keys").clicked() {
                            self.current_secret_tab = SecretType::SshKey;
                        }
                        if ui.selectable_label(matches!(self.current_secret_tab, SecretType::Document), "📄 Documents").clicked() {
                            self.current_secret_tab = SecretType::Document;
                        }
                        if ui.selectable_label(matches!(self.current_secret_tab, SecretType::Configuration), "⚙ Config/Env").clicked() {
                            self.current_secret_tab = SecretType::Configuration;
                        }
                        if ui.selectable_label(matches!(self.current_secret_tab, SecretType::SecureNote), "📝 Secure Notes").clicked() {
                            self.current_secret_tab = SecretType::SecureNote;
                        }
                        if ui.selectable_label(matches!(self.current_secret_tab, SecretType::ConnectionString), "🔗 Database").clicked() {
                            self.current_secret_tab = SecretType::ConnectionString;
                        }
                    });
                    
                    ui.separator();
                    
                    // Add new secret button
                    if ui.button(format!("➕ Add New {}", match self.current_secret_tab {
                        SecretType::ApiKey => "API Key",
                        SecretType::SshKey => "SSH Key", 
                        SecretType::Document => "Document",
                        SecretType::Configuration => "Configuration",
                        SecretType::SecureNote => "Secure Note",
                        SecretType::ConnectionString => "Database Connection",
                        _ => "Secret",
                    })).clicked() {
                        self.selected_secret_type = self.current_secret_tab.clone();
                        self.show_add_secret_dialog = true;
                    }
                    
                    ui.add_space(10.0);
                    
                    // Secrets list for current tab
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.label("Secrets list will be implemented next...");
                        
                        // Placeholder for now - we'll implement the actual list next
                        match self.current_secret_tab {
                            SecretType::ApiKey => {
                                ui.label("API Keys will be listed here");
                            }
                            SecretType::SshKey => {
                                ui.label("SSH Keys will be listed here");
                            }
                            SecretType::Document => {
                                ui.label("Documents will be listed here");
                            }
                            SecretType::Configuration => {
                                ui.label("Environment variables and config will be listed here");
                            }
                            SecretType::SecureNote => {
                                ui.label("Secure notes will be listed here");
                            }
                            SecretType::ConnectionString => {
                                ui.label("Database connections will be listed here");
                            }
                            _ => {
                                ui.label("Other secrets will be listed here");
                            }
                        }
                    });
                });
            });
    }
    
    fn show_add_secret_dialog(&mut self, ctx: &egui::Context) {
        if !self.show_add_secret_dialog {
            return;
        }
        
        let dialog_title = match self.selected_secret_type {
            SecretType::ApiKey => "Add API Key",
            SecretType::SshKey => "Add SSH Key",
            SecretType::Document => "Add Document",
            SecretType::Configuration => "Add Configuration",
            SecretType::SecureNote => "Add Secure Note",
            SecretType::ConnectionString => "Add Database Connection",
            _ => "Add Secret",
        };
        
        egui::Window::new(dialog_title)
            .collapsible(false)
            .resizable(true)
            .default_size([600.0, 500.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Common fields for all secret types
                    egui::Grid::new("secret_common_fields").num_columns(2).show(ui, |ui| {
                        ui.label("Name:");
                        ui.text_edit_singleline(&mut self.secret_name);
                        ui.end_row();
                        
                        ui.label("Description:");
                        ui.text_edit_singleline(&mut self.secret_description);
                        ui.end_row();
                        
                        ui.label("Tags:");
                        ui.add(
                            egui::TextEdit::singleline(&mut self.secret_tags)
                                .hint_text("Comma-separated tags...")
                        );
                        ui.end_row();
                    });
                    
                    ui.separator();
                    
                    // Type-specific fields
                    match self.selected_secret_type {
                        SecretType::ApiKey => self.show_api_key_fields(ui),
                        SecretType::SshKey => self.show_ssh_key_fields(ui),
                        SecretType::Document => self.show_document_fields(ui),
                        SecretType::Configuration => self.show_config_fields(ui),
                        SecretType::SecureNote => self.show_note_fields(ui),
                        SecretType::ConnectionString => self.show_database_fields(ui),
                        _ => {
                            ui.label("Secret type not yet implemented");
                        }
                    }
                    
                    ui.separator();
                    
                    // Action buttons
                    ui.horizontal(|ui| {
                        if ui.button("💾 Save Secret").clicked() {
                            self.save_secret();
                        }
                        if ui.button("❌ Cancel").clicked() {
                            self.cancel_secret_creation();
                        }
                    });
                });
            });
    }
    
    fn show_api_key_fields(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔑 API Key Details");
        egui::Grid::new("api_key_fields").num_columns(2).show(ui, |ui| {
            ui.label("Provider:");
            ui.add(
                egui::TextEdit::singleline(&mut self.api_provider)
                    .hint_text("e.g., AWS, Google Cloud, OpenAI...")
            );
            ui.end_row();
            
            ui.label("Key ID:");
            ui.text_edit_singleline(&mut self.api_key_id);
            ui.end_row();
            
            ui.label("API Key:");
            ui.add(
                egui::TextEdit::singleline(&mut self.api_key)
                    .password(true)
                    .font(egui::TextStyle::Monospace)
            );
            ui.end_row();
            
            ui.label("Secret Key:");
            ui.add(
                egui::TextEdit::singleline(&mut self.api_secret)
                    .password(true)
                    .font(egui::TextStyle::Monospace)
            );
            ui.end_row();
            
            ui.label("Environment:");
            egui::ComboBox::from_id_source("api_environment")
                .selected_text(&self.api_environment)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.api_environment, "development".to_string(), "Development");
                    ui.selectable_value(&mut self.api_environment, "staging".to_string(), "Staging");
                    ui.selectable_value(&mut self.api_environment, "production".to_string(), "Production");
                });
            ui.end_row();
            
            ui.label("Endpoint URL:");
            ui.text_edit_singleline(&mut self.api_endpoint);
            ui.end_row();
        });
    }
    
    fn show_ssh_key_fields(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔐 SSH Key Details");
        egui::Grid::new("ssh_key_fields").num_columns(2).show(ui, |ui| {
            ui.label("Key Type:");
            egui::ComboBox::from_id_source("ssh_key_type")
                .selected_text(&self.ssh_key_type)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.ssh_key_type, "RSA".to_string(), "RSA");
                    ui.selectable_value(&mut self.ssh_key_type, "ECDSA".to_string(), "ECDSA");
                    ui.selectable_value(&mut self.ssh_key_type, "Ed25519".to_string(), "Ed25519");
                });
            ui.end_row();
            
            ui.label("Private Key:");
            ui.add(
                egui::TextEdit::multiline(&mut self.ssh_private_key)
                    .desired_rows(8)
                    .font(egui::TextStyle::Monospace)
                    .hint_text("-----BEGIN OPENSSH PRIVATE KEY-----\n...")
            );
            ui.end_row();
            
            ui.label("Public Key:");
            ui.add(
                egui::TextEdit::multiline(&mut self.ssh_public_key)
                    .desired_rows(3)
                    .font(egui::TextStyle::Monospace)
                    .hint_text("ssh-rsa AAAAB3NzaC1yc2E...")
            );
            ui.end_row();
            
            ui.label("Passphrase:");
            ui.add(
                egui::TextEdit::singleline(&mut self.ssh_passphrase)
                    .password(true)
            );
            ui.end_row();
            
            ui.label("Comment:");
            ui.text_edit_singleline(&mut self.ssh_comment);
            ui.end_row();
        });
    }
    
    fn show_document_fields(&mut self, ui: &mut egui::Ui) {
        ui.heading("📄 Document Details");
        egui::Grid::new("document_fields").num_columns(2).show(ui, |ui| {
            ui.label("Filename:");
            ui.text_edit_singleline(&mut self.document_filename);
            ui.end_row();
            
            ui.label("Upload File:");
            if ui.button("📁 Choose File").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    match std::fs::read(&path) {
                        Ok(content) => {
                            self.document_content = content;
                            self.document_filename = path.file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                        }
                        Err(e) => {
                            self.error_message = format!("Failed to read file: {}", e);
                        }
                    }
                }
            }
            ui.end_row();
            
            if !self.document_content.is_empty() {
                ui.label("File Size:");
                ui.label(format!("{} bytes", self.document_content.len()));
                ui.end_row();
            }
        });
    }
    
    fn show_config_fields(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙ Configuration Variables");
        ui.label("Environment Variables (KEY=VALUE format, one per line):");
        ui.add(
            egui::TextEdit::multiline(&mut self.config_variables)
                .desired_rows(10)
                .font(egui::TextStyle::Monospace)
                .hint_text("DATABASE_URL=postgresql://...\nAPI_KEY=your_key_here\nDEBUG=true")
        );
    }
    
    fn show_note_fields(&mut self, ui: &mut egui::Ui) {
        ui.heading("📝 Secure Note");
        egui::Grid::new("note_fields").num_columns(2).show(ui, |ui| {
            ui.label("Title:");
            ui.text_edit_singleline(&mut self.note_title);
            ui.end_row();
        });
        
        ui.add_space(5.0);
        ui.label("Content:");
        ui.add(
            egui::TextEdit::multiline(&mut self.note_content)
                .desired_rows(12)
                .hint_text("Enter your secure note content here...")
        );
    }
    
    fn show_database_fields(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔗 Database Connection");
        egui::Grid::new("database_fields").num_columns(2).show(ui, |ui| {
            ui.label("Database Type:");
            egui::ComboBox::from_id_source("db_type")
                .selected_text(&self.db_type)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.db_type, "PostgreSQL".to_string(), "PostgreSQL");
                    ui.selectable_value(&mut self.db_type, "MySQL".to_string(), "MySQL");
                    ui.selectable_value(&mut self.db_type, "SQLite".to_string(), "SQLite");
                    ui.selectable_value(&mut self.db_type, "MongoDB".to_string(), "MongoDB");
                    ui.selectable_value(&mut self.db_type, "Redis".to_string(), "Redis");
                });
            ui.end_row();
            
            ui.label("Host:");
            ui.text_edit_singleline(&mut self.db_host);
            ui.end_row();
            
            ui.label("Port:");
            ui.text_edit_singleline(&mut self.db_port);
            ui.end_row();
            
            ui.label("Database Name:");
            ui.text_edit_singleline(&mut self.db_name);
            ui.end_row();
            
            ui.label("Username:");
            ui.text_edit_singleline(&mut self.db_username);
            ui.end_row();
            
            ui.label("Password:");
            ui.add(
                egui::TextEdit::singleline(&mut self.db_password)
                    .password(true)
            );
            ui.end_row();
        });
    }
    
    fn save_secret(&mut self) {
        if self.secret_name.trim().is_empty() {
            self.error_message = "Secret name is required".to_string();
            return;
        }

        let secrets_storage = self.secrets_storage.clone();
        let current_secret_type = self.selected_secret_type.clone();
        
        // Create the secret data based on the selected type
        let secret_data = match current_secret_type {
            SecretType::ApiKey => {
                use pwgen_core::api_keys::{ApiKeyProvider, ApiKeyPermissions, RotationInfo, UsageStats};
                
                let provider = match self.api_provider.as_str() {
                    "AWS" => ApiKeyProvider::AWS,
                    "Google Cloud" => ApiKeyProvider::GCP,
                    "Azure" => ApiKeyProvider::Azure,
                    "GitHub" => ApiKeyProvider::GitHub,
                    "Stripe" => ApiKeyProvider::Stripe,
                    "Twilio" => ApiKeyProvider::Twilio,
                    "SendGrid" => ApiKeyProvider::SendGrid,
                    "Slack" => ApiKeyProvider::Slack,
                    "Discord" => ApiKeyProvider::Discord,
                    _ => ApiKeyProvider::Custom(self.api_provider.clone()),
                };
                
                SecretData::ApiKey {
                    provider,
                    key_id: self.api_key_id.clone(),
                    api_key: self.api_key.clone(),
                    api_secret: if self.api_secret.is_empty() { None } else { Some(self.api_secret.clone()) },
                    token_type: "Bearer".to_string(),
                    permissions: ApiKeyPermissions {
                        read: true,
                        write: false,
                        admin: false,
                        scopes: vec![],
                        resource_access: std::collections::HashMap::new(),
                    },
                    environment: self.api_environment.clone(),
                    endpoint_url: if self.api_endpoint.is_empty() { None } else { Some(self.api_endpoint.clone()) },
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
            },
            SecretType::SshKey => {
                use pwgen_core::secrets::SshKeyType;
                
                let key_type = match self.ssh_key_type.as_str() {
                    "RSA" => SshKeyType::Rsa,
                    "Ed25519" => SshKeyType::Ed25519,
                    "ECDSA" => SshKeyType::Ecdsa,
                    "DSA" => SshKeyType::Dsa,
                    _ => SshKeyType::Rsa,
                };
                
                SecretData::SshKey {
                    key_type,
                    private_key: if self.ssh_private_key.is_empty() { None } else { Some(self.ssh_private_key.clone()) },
                    public_key: if self.ssh_public_key.is_empty() { None } else { Some(self.ssh_public_key.clone()) },
                    passphrase: if self.ssh_passphrase.is_empty() { None } else { Some(self.ssh_passphrase.clone()) },
                    comment: if self.ssh_comment.is_empty() { None } else { Some(self.ssh_comment.clone()) },
                    fingerprint: None, // Could be calculated from the key
                }
            },
            SecretType::Document => {
                use sha2::{Sha256, Digest};
                
                let mut hasher = Sha256::new();
                hasher.update(&self.document_content);
                let checksum = format!("{:x}", hasher.finalize());
                
                SecretData::Document {
                    filename: self.document_filename.clone(),
                    content_type: "application/octet-stream".to_string(),
                    content: self.document_content.clone(),
                    checksum,
                }
            },
            SecretType::Configuration => {
                use pwgen_core::secrets::ConfigFormat;
                
                // Parse environment variables from KEY=VALUE format
                let mut variables = std::collections::HashMap::new();
                for line in self.config_variables.lines() {
                    if let Some((key, value)) = line.split_once('=') {
                        variables.insert(key.trim().to_string(), value.trim().to_string());
                    }
                }
                
                SecretData::Configuration {
                    format: ConfigFormat::EnvFile,
                    variables,
                    template: None,
                }
            },
            SecretType::SecureNote => {
                use pwgen_core::secrets::NoteFormat;
                
                SecretData::SecureNote {
                    title: self.note_title.clone(),
                    content: self.note_content.clone(),
                    format: NoteFormat::PlainText,
                }
            },
            SecretType::ConnectionString => {
                use pwgen_core::secrets::DatabaseType;
                
                let database_type = match self.db_type.as_str() {
                    "PostgreSQL" => DatabaseType::PostgreSQL,
                    "MySQL" => DatabaseType::MySQL,
                    "SQLite" => DatabaseType::SQLite,
                    "MongoDB" => DatabaseType::MongoDB,
                    "Redis" => DatabaseType::Redis,
                    "Oracle" => DatabaseType::Oracle,
                    "SQL Server" => DatabaseType::SQLServer,
                    _ => DatabaseType::Custom(self.db_type.clone()),
                };
                
                let port = self.db_port.parse::<u16>().unwrap_or(5432);
                let connection_string = format!(
                    "{}://{}:{}@{}:{}/{}",
                    self.db_type.to_lowercase(),
                    self.db_username,
                    self.db_password,
                    self.db_host,
                    port,
                    self.db_name
                );
                
                SecretData::ConnectionString {
                    database_type,
                    host: self.db_host.clone(),
                    port: Some(port),
                    database: self.db_name.clone(),
                    username: self.db_username.clone(),
                    password: self.db_password.clone(),
                    connection_string,
                    ssl_config: None,
                }
            },
            _ => {
                self.error_message = "Unsupported secret type".to_string();
                return;
            }
        };

        // Parse tags
        let tags: Vec<String> = self.secret_tags
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Create the DecryptedSecretEntry
        let secret_entry = DecryptedSecretEntry {
            id: uuid::Uuid::new_v4().to_string(),
            name: self.secret_name.clone(),
            description: if self.secret_description.is_empty() { None } else { Some(self.secret_description.clone()) },
            secret_type: current_secret_type,
            data: secret_data,
            metadata: pwgen_core::secrets::SecretMetadata::default(),
            tags,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            last_accessed: None,
            expires_at: None,
            favorite: false,
        };

        // Save the secret to storage
        let runtime = self.runtime.clone();
        let secret_entry_clone = secret_entry.clone();
        
        let _ = runtime.block_on(async {
            let secrets_storage_guard = secrets_storage.lock().unwrap();
            if let Some(storage) = secrets_storage_guard.as_ref() {
                if let Err(e) = storage.add_secret(&secret_entry_clone).await {
                    eprintln!("Failed to save secret: {}", e);
                    return Err(e);
                }
            }
            Ok(())
        });

        // Reload secrets from database
        self.load_secrets();

        self.success_message = "Secret saved successfully!".to_string();
        self.error_message.clear();
        self.cancel_secret_creation();
    }
    
    fn refresh_secrets_list(&mut self) {
        self.filtered_secrets = self.secrets.clone();
    }
    
    fn cancel_secret_creation(&mut self) {
        self.show_add_secret_dialog = false;
        // Clear all form fields
        self.secret_name.clear();
        self.secret_description.clear();
        self.secret_tags.clear();
        self.api_provider.clear();
        self.api_key_id.clear();
        self.api_key.clear();
        self.api_secret.clear();
        self.api_environment = "production".to_string();
        self.api_endpoint.clear();
        self.ssh_key_type = "RSA".to_string();
        self.ssh_private_key.clear();
        self.ssh_public_key.clear();
        self.ssh_passphrase.clear();
        self.ssh_comment.clear();
        self.document_filename.clear();
        self.document_content.clear();
        self.config_variables.clear();
        self.note_title.clear();
        self.note_content.clear();
        self.db_type = "PostgreSQL".to_string();
        self.db_host.clear();
        self.db_port = "5432".to_string();
        self.db_name.clear();
        self.db_username.clear();
        self.db_password.clear();
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // Load Fira Code fonts
    if let Ok(fira_regular) = std::fs::read("assets/fonts/FiraCode-Regular.ttf") {
        fonts.font_data.insert(
            "fira_code".to_owned(),
            egui::FontData::from_owned(fira_regular),
        );
        
        // Add to monospace family for code/passwords
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .insert(0, "fira_code".to_owned());
    }
    
    if let Ok(fira_medium) = std::fs::read("assets/fonts/FiraCode-Medium.ttf") {
        fonts.font_data.insert(
            "fira_medium".to_owned(),
            egui::FontData::from_owned(fira_medium),
        );
        
        // Add to proportional family for UI text
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "fira_medium".to_owned());
    }
    
    ctx.set_fonts(fonts);
}

fn load_logo_wide(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let logo_bytes = include_bytes!("../../ui/PWGenLogo-Wide.png");
    
    // Parse PNG image
    let decoder = png::Decoder::new(&logo_bytes[..]);
    match decoder.read_info() {
        Ok(mut reader) => {
            let mut buf = vec![0; reader.output_buffer_size()];
            let info = reader.next_frame(&mut buf).ok()?;
            
            // Convert to RGBA if needed
            let pixels = match info.color_type {
                png::ColorType::Rgba => buf,
                png::ColorType::Rgb => {
                    let mut rgba_buf = Vec::with_capacity(buf.len() * 4 / 3);
                    for chunk in buf.chunks(3) {
                        rgba_buf.extend_from_slice(chunk);
                        rgba_buf.push(255); // Alpha
                    }
                    rgba_buf
                },
                _ => return None, // Unsupported format
            };
            let (width, height) = (info.width as usize, info.height as usize);
            
            // Convert to egui::ColorImage
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [width, height],
                &pixels,
            );
            
            // Load as texture with filtering for smooth scaling
            Some(ctx.load_texture(
                "logo-wide", 
                color_image, 
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Linear,
                    minification: egui::TextureFilter::Linear,
                    wrap_mode: egui::TextureWrapMode::ClampToEdge,
                }
            ))
        }
        Err(e) => {
            eprintln!("Failed to load wide logo: {}", e);
            None
        }
    }
}

fn load_logo_square(ctx: &egui::Context) -> Option<egui::TextureHandle> {
    let logo_bytes = include_bytes!("../../ui/PWGenLogo.png");
    
    // Parse PNG image
    let decoder = png::Decoder::new(&logo_bytes[..]);
    match decoder.read_info() {
        Ok(mut reader) => {
            let mut buf = vec![0; reader.output_buffer_size()];
            let info = reader.next_frame(&mut buf).ok()?;
            
            // Convert to RGBA if needed
            let pixels = match info.color_type {
                png::ColorType::Rgba => buf,
                png::ColorType::Rgb => {
                    let mut rgba_buf = Vec::with_capacity(buf.len() * 4 / 3);
                    for chunk in buf.chunks(3) {
                        rgba_buf.extend_from_slice(chunk);
                        rgba_buf.push(255); // Alpha
                    }
                    rgba_buf
                },
                _ => return None, // Unsupported format
            };
            let (width, height) = (info.width as usize, info.height as usize);
            
            // Convert to egui::ColorImage
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [width, height],
                &pixels,
            );
            
            // Load as texture with filtering for smooth scaling
            Some(ctx.load_texture(
                "logo-square", 
                color_image, 
                egui::TextureOptions {
                    magnification: egui::TextureFilter::Linear,
                    minification: egui::TextureFilter::Linear,
                    wrap_mode: egui::TextureWrapMode::ClampToEdge,
                }
            ))
        }
        Err(e) => {
            eprintln!("Failed to load square logo: {}", e);
            None
        }
    }
}

fn create_tray_icon() -> Option<()> {
    // TODO: Implement system tray - currently disabled due to missing system dependencies
    // For now, return None to disable tray functionality
    None
}

fn handle_tray_event(_app: &mut PwGenApp, _ctx: &egui::Context) {
    // TODO: Implement tray event handling when system tray is enabled
}

fn main() -> eframe::Result<()> {
    // Load icon
    let icon_bytes = include_bytes!("../../ui/PWGenLogo.png");
    let icon = eframe::icon_data::from_png_bytes(icon_bytes).unwrap();
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([1000.0, 600.0])
            .with_icon(icon)
            .with_resizable(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "PwGen Password Manager",
        native_options,
        Box::new(|cc| Box::new(PwGenApp::new(cc))),
    )
}