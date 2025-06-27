// Tauri v2 compatible password manager app
console.log('PwGen starting...');

// Global state
let currentEntries = [];
let selectedEntryId = null;

// Wait for DOM to load
document.addEventListener('DOMContentLoaded', async () => {
    console.log('DOM loaded');
    
    // Wait a bit for Tauri to initialize
    setTimeout(async () => {
        try {
            // Check if Tauri invoke is available
            if (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke) {
                console.log('Tauri v2 API found');
                await initializeApp();
            } else {
                console.error('Tauri API not found');
                showApiError();
            }
        } catch (error) {
            console.error('Initialization error:', error);
            showApiError();
        }
    }, 100);
});

// Initialize app
async function initializeApp() {
    console.log('Initializing app...');
    
    try {
        // Check if vault exists
        const vaultExists = await window.__TAURI_INTERNALS__.invoke('vault_exists');
        console.log('Vault exists:', vaultExists);
        
        // Set up UI based on vault status
        const unlockBtn = document.getElementById('unlock-btn');
        const initBtn = document.getElementById('init-btn');
        
        if (vaultExists) {
            unlockBtn.style.display = 'block';
            initBtn.style.display = 'none';
        } else {
            unlockBtn.style.display = 'none';
            initBtn.style.display = 'block';
        }
        
        // Set up event listeners
        setupEventListeners();
        
    } catch (error) {
        console.error('App initialization failed:', error);
        showError('Failed to initialize app: ' + error.message);
    }
}

// Set up all event listeners
function setupEventListeners() {
    console.log('Setting up event listeners...');
    
    const masterPasswordInput = document.getElementById('master-password');
    const unlockBtn = document.getElementById('unlock-btn');
    const initBtn = document.getElementById('init-btn');
    const lockBtn = document.getElementById('lock-btn');
    const addEntryBtn = document.getElementById('add-entry-btn');
    
    // Unlock vault
    unlockBtn.addEventListener('click', async () => {
        console.log('Unlock button clicked');
        const password = masterPasswordInput.value;
        
        if (!password) {
            showError('Please enter your master password');
            return;
        }
        
        try {
            console.log('Attempting to unlock vault...');
            await window.__TAURI_INTERNALS__.invoke('unlock_vault', { password });
            console.log('Vault unlocked successfully');
            
            showMainScreen();
            await loadEntries();
            
        } catch (error) {
            console.error('Unlock failed:', error);
            showError('Invalid master password');
        }
    });
    
    // Initialize vault
    initBtn.addEventListener('click', async () => {
        console.log('Initialize button clicked');
        const password = masterPasswordInput.value;
        
        if (!password) {
            showError('Please enter a master password');
            return;
        }
        
        if (password.length < 12) {
            showError('Master password must be at least 12 characters');
            return;
        }
        
        try {
            console.log('Attempting to initialize vault...');
            await window.__TAURI_INTERNALS__.invoke('init_vault', { password });
            console.log('Vault initialized successfully');
            
            showMainScreen();
            await loadEntries();
            
        } catch (error) {
            console.error('Initialize failed:', error);
            const errorMessage = error.message || error.toString() || 'Unknown error';
            showError('Failed to initialize vault: ' + errorMessage);
        }
    });
    
    // Enter key support
    masterPasswordInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            const unlockVisible = unlockBtn.style.display !== 'none';
            if (unlockVisible) {
                unlockBtn.click();
            } else {
                initBtn.click();
            }
        }
    });
    
    // Lock vault
    if (lockBtn) {
        lockBtn.addEventListener('click', async () => {
            try {
                await window.__TAURI_INTERNALS__.invoke('lock_vault');
                showLoginScreen();
                masterPasswordInput.value = '';
                currentEntries = [];
                selectedEntryId = null;
            } catch (error) {
                console.error('Lock failed:', error);
                showError('Failed to lock vault');
            }
        });
    }
    
    // Add entry button
    if (addEntryBtn) {
        addEntryBtn.addEventListener('click', () => {
            showEntryModal();
        });
    }
    
    console.log('Event listeners set up');
}

// Screen management
function showMainScreen() {
    console.log('Showing main screen');
    document.getElementById('login-screen').classList.remove('active');
    document.getElementById('main-screen').classList.add('active');
    document.getElementById('master-password').value = '';
    hideError();
}

function showLoginScreen() {
    console.log('Showing login screen');
    document.getElementById('main-screen').classList.remove('active');
    document.getElementById('login-screen').classList.add('active');
}

// Error handling
function showError(message) {
    const errorEl = document.getElementById('login-error');
    if (errorEl) {
        errorEl.textContent = message;
        console.error('Error:', message);
    }
}

function hideError() {
    const errorEl = document.getElementById('login-error');
    if (errorEl) {
        errorEl.textContent = '';
    }
}

function showApiError() {
    document.body.innerHTML = `
        <div style="display: flex; flex-direction: column; justify-content: center; align-items: center; height: 100vh; color: red; text-align: center; padding: 20px;">
            <h2>Tauri API Not Available</h2>
            <p>This application must be run through Tauri.</p>
            <p>Please use: <code>cargo run -p pwgen-gui</code></p>
        </div>
    `;
}

// Load entries from backend
async function loadEntries() {
    console.log('Loading entries...');
    try {
        const entries = await window.__TAURI_INTERNALS__.invoke('search_entries', {
            request: {
                query: null,
                tags: null,
                favorite_only: false
            }
        });
        
        console.log('Loaded entries:', entries.length);
        currentEntries = entries;
        displayEntries(entries);
        
    } catch (error) {
        console.error('Failed to load entries:', error);
        showNotification('Failed to load entries', 'error');
    }
}

// Display entries in the UI
function displayEntries(entries) {
    const container = document.getElementById('entries-container');
    if (!container) return;
    
    container.innerHTML = '';
    
    if (entries.length === 0) {
        container.innerHTML = '<div class="empty-state">No passwords saved yet. Click + to add one.</div>';
        return;
    }
    
    entries.forEach(entry => {
        const entryEl = document.createElement('div');
        entryEl.className = 'password-entry';
        entryEl.dataset.id = entry.id;
        
        if (entry.id === selectedEntryId) {
            entryEl.classList.add('selected');
        }
        
        entryEl.innerHTML = `
            <div class="entry-info">
                <h4>${escapeHtml(entry.site)}</h4>
                <p>${escapeHtml(entry.username)}</p>
            </div>
            <div class="entry-actions">
                <button class="icon-btn copy-password-btn" title="Copy Password">🔑</button>
                <button class="icon-btn copy-username-btn" title="Copy Username">👤</button>
            </div>
        `;
        
        // Click to select entry
        entryEl.addEventListener('click', () => {
            selectEntry(entry);
        });
        
        // Copy password button
        entryEl.querySelector('.copy-password-btn').addEventListener('click', async (e) => {
            e.stopPropagation();
            await copyToClipboard(entry.password);
        });
        
        // Copy username button
        entryEl.querySelector('.copy-username-btn').addEventListener('click', async (e) => {
            e.stopPropagation();
            await copyToClipboard(entry.username);
        });
        
        container.appendChild(entryEl);
    });
}

// Select an entry
function selectEntry(entry) {
    selectedEntryId = entry.id;
    
    // Update visual selection
    document.querySelectorAll('.password-entry').forEach(el => {
        el.classList.remove('selected');
        if (el.dataset.id === entry.id) {
            el.classList.add('selected');
        }
    });
    
    // Show entry details
    showEntryDetails(entry);
}

// Show entry details
function showEntryDetails(entry) {
    const detailsPanel = document.getElementById('entry-details');
    const detailsContent = document.getElementById('detail-content');
    const detailSite = document.getElementById('detail-site');
    
    if (!detailsPanel || !detailsContent || !detailSite) return;
    
    detailSite.textContent = entry.site;
    
    detailsContent.innerHTML = `
        <div class="detail-field">
            <label>Username:</label>
            <div class="detail-field-value">
                <span>${escapeHtml(entry.username)}</span>
                <button class="icon-btn copy-btn" title="Copy Username">📋</button>
            </div>
        </div>
        <div class="detail-field">
            <label>Password:</label>
            <div class="detail-field-value">
                <span class="password-hidden">••••••••</span>
                <button class="icon-btn show-password-btn" title="Show Password">👁️</button>
                <button class="icon-btn copy-btn" title="Copy Password">📋</button>
            </div>
        </div>
        ${entry.notes ? `
        <div class="detail-field">
            <label>Notes:</label>
            <p>${escapeHtml(entry.notes)}</p>
        </div>
        ` : ''}
        <div class="detail-field">
            <label>Created:</label>
            <p>${new Date(entry.created_at).toLocaleString()}</p>
        </div>
    `;
    
    // Set up detail buttons
    const copyBtns = detailsContent.querySelectorAll('.copy-btn');
    copyBtns.forEach((btn, index) => {
        btn.addEventListener('click', async () => {
            const text = index === 0 ? entry.username : entry.password;
            await copyToClipboard(text);
        });
    });
    
    const showPasswordBtn = detailsContent.querySelector('.show-password-btn');
    if (showPasswordBtn) {
        showPasswordBtn.addEventListener('click', () => {
            const passwordSpan = detailsContent.querySelector('.password-hidden');
            if (passwordSpan.textContent === '••••••••') {
                passwordSpan.textContent = entry.password;
                showPasswordBtn.textContent = '🙈';
                showPasswordBtn.title = 'Hide Password';
            } else {
                passwordSpan.textContent = '••••••••';
                showPasswordBtn.textContent = '👁️';
                showPasswordBtn.title = 'Show Password';
            }
        });
    }
    
    detailsPanel.style.display = 'block';
}

// Copy to clipboard
async function copyToClipboard(text) {
    try {
        await window.__TAURI_INTERNALS__.invoke('copy_to_clipboard', { text });
        showNotification('Copied to clipboard!');
    } catch (error) {
        console.error('Copy failed:', error);
        showNotification('Failed to copy to clipboard', 'error');
    }
}

// Show notification
function showNotification(message, type = 'success') {
    const notification = document.createElement('div');
    notification.className = 'notification';
    notification.textContent = message;
    notification.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        background: ${type === 'error' ? '#ff4444' : '#44ff44'};
        color: white;
        padding: 10px 20px;
        border-radius: 4px;
        z-index: 10000;
        font-size: 14px;
    `;
    
    document.body.appendChild(notification);
    
    setTimeout(() => {
        notification.remove();
    }, 3000);
}

// Show entry modal (placeholder)
function showEntryModal() {
    showNotification('Add entry functionality coming soon!');
}

// Utility function
function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

console.log('PwGen app.js loaded');