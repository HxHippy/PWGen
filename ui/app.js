// Wait for Tauri to be ready
let invoke;
let writeText;

// Wait for window to load and Tauri API to be available
window.addEventListener('DOMContentLoaded', async () => {
    console.log('DOM loaded, checking for Tauri API...');
    
    // Give the module script time to load
    setTimeout(async () => {
        // Check if our global Tauri functions are available
        if (window.__TAURI_INVOKE__ && window.__TAURI_WRITE_TEXT__) {
            console.log('Tauri API found via module script');
            invoke = window.__TAURI_INVOKE__;
            writeText = window.__TAURI_WRITE_TEXT__;
            init();
        } 
        // Fallback to checking for __TAURI__ (v1 style)
        else if (window.__TAURI__) {
            console.log('Tauri API found (v1 style)');
            invoke = window.__TAURI__.tauri.invoke;
            writeText = window.__TAURI__.clipboard.writeText;
            init();
        }
        // Try direct invoke
        else if (window.invoke) {
            console.log('Direct invoke found');
            invoke = window.invoke;
            writeText = async (text) => {
                await invoke('copy_to_clipboard', { text });
            };
            init();
        }
        else {
            console.error('Tauri API not found after all checks');
            document.body.innerHTML = '<div style="display: flex; justify-content: center; align-items: center; height: 100vh; color: red;">Tauri API not available. Make sure you are running through: cargo run -p pwgen-gui</div>';
        }
    }, 100); // Small delay to ensure module script loads
});

// App state
let currentEntries = [];
let currentSecrets = [];
let selectedEntryId = null;
let selectedSecretId = null;
let isEditMode = false;
let currentView = 'passwords'; // 'passwords' or 'secrets'
let secretTypes = [];

// DOM elements
const loginScreen = document.getElementById('login-screen');
const mainScreen = document.getElementById('main-screen');
const masterPasswordInput = document.getElementById('master-password');
const unlockBtn = document.getElementById('unlock-btn');
const initBtn = document.getElementById('init-btn');
const loginError = document.getElementById('login-error');
const searchInput = document.getElementById('search-input');
const addEntryBtn = document.getElementById('add-entry-btn');
const generatePasswordBtn = document.getElementById('generate-password-btn');
const lockBtn = document.getElementById('lock-btn');
const entriesContainer = document.getElementById('entries-container');
const entryModal = document.getElementById('entry-modal');
const generatorModal = document.getElementById('generator-modal');
const entryForm = document.getElementById('entry-form');
const cancelEntryBtn = document.getElementById('cancel-entry');
const closeGeneratorBtn = document.getElementById('close-generator');
const passwordLengthSlider = document.getElementById('password-length');
const lengthValue = document.getElementById('length-value');
const regenerateBtn = document.getElementById('regenerate');
const copyGeneratedBtn = document.getElementById('copy-generated');
const useGeneratedBtn = document.getElementById('use-generated');
const generateForEntryBtn = document.getElementById('generate-for-entry');
const togglePasswordBtn = document.getElementById('toggle-password-visibility');
const entryDetails = document.getElementById('entry-details');
const closeDetailsBtn = document.getElementById('close-details');
const secretModal = document.getElementById('secret-modal');
const secretForm = document.getElementById('secret-form');
const cancelSecretBtn = document.getElementById('cancel-secret');
const secretTypeSelect = document.getElementById('secret-type');
const secretFieldsDiv = document.getElementById('secret-fields');

// Initialize
async function init() {
    console.log('Initializing app...');
    try {
        const vaultExists = await invoke('vault_exists');
        console.log('Vault exists:', vaultExists);
        
        if (!vaultExists) {
            unlockBtn.style.display = 'none';
        }
        
        // Check if already unlocked
        const isUnlocked = await invoke('is_vault_unlocked');
        if (isUnlocked) {
            showMainScreen();
            loadEntries();
            loadSecrets();
            loadSecretTypes();
        }
    } catch (error) {
        console.error('Initialization error:', error);
    }
}

// Event Listeners
unlockBtn.addEventListener('click', unlockVault);
initBtn.addEventListener('click', initializeVault);
lockBtn.addEventListener('click', lockVault);
addEntryBtn.addEventListener('click', () => showEntryModal());
generatePasswordBtn.addEventListener('click', () => showGeneratorModal());
searchInput.addEventListener('input', debounce(searchEntries, 300));
entryForm.addEventListener('submit', saveEntry);
cancelEntryBtn.addEventListener('click', hideEntryModal);
closeGeneratorBtn.addEventListener('click', hideGeneratorModal);
passwordLengthSlider.addEventListener('input', updateLengthDisplay);
regenerateBtn.addEventListener('click', generatePassword);
copyGeneratedBtn.addEventListener('click', copyGeneratedPassword);
useGeneratedBtn.addEventListener('click', useGeneratedPassword);
generateForEntryBtn.addEventListener('click', generateForEntry);
togglePasswordBtn.addEventListener('click', togglePasswordVisibility);
closeDetailsBtn.addEventListener('click', hideEntryDetails);
secretForm.addEventListener('submit', saveSecret);
cancelSecretBtn.addEventListener('click', hideSecretModal);
secretTypeSelect.addEventListener('change', updateSecretFields);

masterPasswordInput.addEventListener('keypress', (e) => {
    if (e.key === 'Enter') {
        if (unlockBtn.style.display !== 'none') {
            unlockVault();
        } else {
            initializeVault();
        }
    }
});

// Navigation
document.querySelectorAll('.nav-item').forEach(item => {
    item.addEventListener('click', (e) => {
        e.preventDefault();
        document.querySelectorAll('.nav-item').forEach(i => i.classList.remove('active'));
        e.target.classList.add('active');
        const filter = e.target.dataset.filter;
        if (currentView === 'passwords') {
            filterEntries(filter);
        } else {
            filterSecrets(filter);
        }
    });
});

// View switching
document.querySelectorAll('.view-btn').forEach(btn => {
    btn.addEventListener('click', (e) => {
        e.preventDefault();
        document.querySelectorAll('.view-btn').forEach(b => b.classList.remove('active'));
        e.target.classList.add('active');
        currentView = e.target.dataset.view;
        switchView(currentView);
    });
});

// Vault operations
async function unlockVault() {
    console.log('Unlock vault clicked');
    const password = masterPasswordInput.value;
    if (!password) {
        showError('Please enter your master password');
        return;
    }
    
    console.log('Attempting to unlock vault...');
    try {
        await invoke('unlock_vault', { password });
        console.log('Vault unlocked successfully');
        showMainScreen();
        loadEntries();
        loadSecrets();
        loadSecretTypes();
    } catch (error) {
        console.error('Unlock failed:', error);
        showError('Invalid master password');
    }
}

async function initializeVault() {
    console.log('Initialize vault clicked');
    const password = masterPasswordInput.value;
    if (!password) {
        showError('Please enter a master password');
        return;
    }
    
    if (password.length < 12) {
        showError('Master password must be at least 12 characters');
        return;
    }
    
    console.log('Attempting to initialize vault...');
    try {
        await invoke('init_vault', { password });
        console.log('Vault initialized successfully');
        unlockBtn.style.display = 'block';
        showMainScreen();
        loadEntries();
        loadSecrets();
        loadSecretTypes();
    } catch (error) {
        console.error('Init failed:', error);
        showError('Failed to initialize vault: ' + error);
    }
}

async function lockVault() {
    try {
        await invoke('lock_vault');
        showLoginScreen();
        masterPasswordInput.value = '';
        currentEntries = [];
        selectedEntryId = null;
    } catch (error) {
        console.error('Failed to lock vault:', error);
    }
}

// Screen management
function showMainScreen() {
    loginScreen.classList.remove('active');
    mainScreen.classList.add('active');
    masterPasswordInput.value = '';
    loginError.textContent = '';
}

function showLoginScreen() {
    mainScreen.classList.remove('active');
    loginScreen.classList.add('active');
}

function showError(message) {
    loginError.textContent = message;
}

// Entry management
async function loadEntries() {
    try {
        currentEntries = await invoke('search_entries', { 
            request: { 
                query: null, 
                tags: null, 
                favorite_only: false 
            } 
        });
        displayEntries(currentEntries);
        updateTagsList();
    } catch (error) {
        console.error('Failed to load entries:', error);
    }
}

function displayEntries(entries) {
    entriesContainer.innerHTML = '';
    
    if (entries.length === 0) {
        entriesContainer.innerHTML = '<p class="empty-state">No passwords saved yet</p>';
        return;
    }
    
    entries.forEach(entry => {
        const entryEl = document.createElement('div');
        entryEl.className = 'password-entry';
        if (entry.id === selectedEntryId) {
            entryEl.classList.add('selected');
        }
        
        entryEl.innerHTML = `
            <div class="entry-info">
                <h4>${escapeHtml(entry.site)}</h4>
                <p>${escapeHtml(entry.username)}</p>
            </div>
            <div class="entry-actions">
                <button class="icon-btn small" onclick="copyPassword('${entry.id}')" title="Copy Password">📋</button>
                ${entry.favorite ? '⭐' : ''}
            </div>
        `;
        
        entryEl.addEventListener('click', () => selectEntry(entry));
        entriesContainer.appendChild(entryEl);
    });
}

async function selectEntry(entry) {
    selectedEntryId = entry.id;
    displayEntries(currentEntries);
    showEntryDetails(entry);
}

function showEntryDetails(entry) {
    entryDetails.classList.add('active');
    document.getElementById('detail-site').textContent = entry.site;
    
    const detailContent = document.getElementById('detail-content');
    detailContent.innerHTML = `
        <div class="detail-field">
            <label>Username</label>
            <div class="detail-field-value">
                <input type="text" value="${escapeHtml(entry.username)}" readonly>
                <button class="icon-btn small" onclick="copyToClipboard('${escapeHtml(entry.username)}')">📋</button>
            </div>
        </div>
        <div class="detail-field">
            <label>Password</label>
            <div class="detail-field-value">
                <input type="password" value="${escapeHtml(entry.password)}" readonly id="detail-password-${entry.id}">
                <button class="icon-btn small" onclick="toggleDetailPassword('${entry.id}')">👁️</button>
                <button class="icon-btn small" onclick="copyPassword('${entry.id}')">📋</button>
            </div>
        </div>
        ${entry.notes ? `
        <div class="detail-field">
            <label>Notes</label>
            <textarea readonly>${escapeHtml(entry.notes)}</textarea>
        </div>
        ` : ''}
        ${entry.tags.length > 0 ? `
        <div class="detail-field">
            <label>Tags</label>
            <div>
                ${entry.tags.map(tag => `<span class="tag">${escapeHtml(tag)}</span>`).join('')}
            </div>
        </div>
        ` : ''}
        <div class="detail-field">
            <label>Last Used</label>
            <p>${entry.last_used ? new Date(entry.last_used).toLocaleString() : 'Never'}</p>
        </div>
        <div class="detail-actions">
            <button class="primary" onclick="editEntry('${entry.id}')">Edit</button>
            <button class="secondary danger" onclick="deleteEntry('${entry.id}')">Delete</button>
        </div>
    `;
}

function hideEntryDetails() {
    entryDetails.classList.remove('active');
    selectedEntryId = null;
    displayEntries(currentEntries);
}

async function searchEntries() {
    const query = searchInput.value.trim();
    
    try {
        currentEntries = await invoke('search_entries', { 
            request: { 
                query: query || null, 
                tags: null, 
                favorite_only: false 
            } 
        });
        displayEntries(currentEntries);
    } catch (error) {
        console.error('Search failed:', error);
    }
}

function filterEntries(filter) {
    switch (filter) {
        case 'favorites':
            displayEntries(currentEntries.filter(e => e.favorite));
            break;
        case 'recent':
            const sorted = [...currentEntries].sort((a, b) => {
                const aTime = a.last_used || a.created_at;
                const bTime = b.last_used || b.created_at;
                return new Date(bTime) - new Date(aTime);
            });
            displayEntries(sorted.slice(0, 10));
            break;
        default:
            displayEntries(currentEntries);
    }
}

// Modal management
function showEntryModal(entry = null) {
    isEditMode = !!entry;
    document.getElementById('modal-title').textContent = isEditMode ? 'Edit Entry' : 'Add New Entry';
    
    if (entry) {
        document.getElementById('entry-site').value = entry.site;
        document.getElementById('entry-username').value = entry.username;
        document.getElementById('entry-password').value = entry.password;
        document.getElementById('entry-notes').value = entry.notes || '';
        document.getElementById('entry-tags').value = entry.tags.join(', ');
    } else {
        entryForm.reset();
    }
    
    entryModal.classList.add('active');
}

function hideEntryModal() {
    entryModal.classList.remove('active');
    entryForm.reset();
    isEditMode = false;
}

async function saveEntry(e) {
    e.preventDefault();
    
    const site = document.getElementById('entry-site').value;
    const username = document.getElementById('entry-username').value;
    const password = document.getElementById('entry-password').value;
    const notes = document.getElementById('entry-notes').value;
    const tags = document.getElementById('entry-tags').value
        .split(',')
        .map(t => t.trim())
        .filter(t => t);
    
    try {
        if (isEditMode) {
            const entry = currentEntries.find(e => e.site === site && e.username === username);
            await invoke('update_entry', {
                entry: {
                    ...entry,
                    password,
                    notes: notes || null,
                    tags,
                    updated_at: new Date().toISOString()
                }
            });
        } else {
            await invoke('add_entry', {
                request: { site, username, password, notes: notes || null, tags }
            });
        }
        
        hideEntryModal();
        loadEntries();
    } catch (error) {
        alert('Failed to save entry: ' + error);
    }
}

async function deleteEntry(id) {
    if (!confirm('Are you sure you want to delete this entry?')) return;
    
    try {
        await invoke('delete_entry', { id });
        hideEntryDetails();
        loadEntries();
    } catch (error) {
        alert('Failed to delete entry: ' + error);
    }
}

window.editEntry = function(id) {
    const entry = currentEntries.find(e => e.id === id);
    if (entry) {
        showEntryModal(entry);
    }
};

window.deleteEntry = deleteEntry;

// Password generation
function showGeneratorModal() {
    generatorModal.classList.add('active');
    generatePassword();
}

function hideGeneratorModal() {
    generatorModal.classList.remove('active');
}

async function generatePassword() {
    const config = {
        length: parseInt(passwordLengthSlider.value),
        include_uppercase: document.getElementById('include-uppercase').checked,
        include_lowercase: document.getElementById('include-lowercase').checked,
        include_numbers: document.getElementById('include-numbers').checked,
        include_symbols: document.getElementById('include-symbols').checked,
        exclude_ambiguous: document.getElementById('exclude-ambiguous').checked
    };
    
    try {
        const password = await invoke('generate_password', { request: config });
        document.getElementById('generated-password').value = password;
    } catch (error) {
        console.error('Failed to generate password:', error);
    }
}

function updateLengthDisplay() {
    lengthValue.textContent = passwordLengthSlider.value;
    generatePassword();
}

async function copyGeneratedPassword() {
    const password = document.getElementById('generated-password').value;
    await copyToClipboard(password);
}

function useGeneratedPassword() {
    const password = document.getElementById('generated-password').value;
    document.getElementById('entry-password').value = password;
    hideGeneratorModal();
}

async function generateForEntry() {
    await generatePassword();
    showGeneratorModal();
    useGeneratedBtn.style.display = 'block';
}

function togglePasswordVisibility() {
    const passwordInput = document.getElementById('entry-password');
    const type = passwordInput.type === 'password' ? 'text' : 'password';
    passwordInput.type = type;
    togglePasswordBtn.textContent = type === 'password' ? '👁️' : '🙈';
}

window.toggleDetailPassword = function(id) {
    const input = document.getElementById(`detail-password-${id}`);
    input.type = input.type === 'password' ? 'text' : 'password';
};

// Clipboard operations
async function copyToClipboard(text) {
    try {
        await invoke('copy_to_clipboard', { text });
        showToast('Copied to clipboard');
    } catch (error) {
        console.error('Failed to copy:', error);
        showToast('Failed to copy to clipboard');
    }
}

window.copyToClipboard = copyToClipboard;

async function copyPassword(id) {
    try {
        const entry = await invoke('get_entry', { id });
        await copyToClipboard(entry.password);
    } catch (error) {
        console.error('Failed to copy password:', error);
    }
}

window.copyPassword = copyPassword;

// Tags management
function updateTagsList() {
    const allTags = new Set();
    currentEntries.forEach(entry => {
        entry.tags.forEach(tag => allTags.add(tag));
    });
    
    const tagsList = document.getElementById('tags-list');
    tagsList.innerHTML = '';
    
    Array.from(allTags).sort().forEach(tag => {
        const tagEl = document.createElement('a');
        tagEl.href = '#';
        tagEl.className = 'tag';
        tagEl.textContent = tag;
        tagEl.onclick = (e) => {
            e.preventDefault();
            searchByTag(tag);
        };
        tagsList.appendChild(tagEl);
    });
}

async function searchByTag(tag) {
    try {
        currentEntries = await invoke('search_entries', { 
            request: { 
                query: null, 
                tags: [tag], 
                favorite_only: false 
            } 
        });
        displayEntries(currentEntries);
    } catch (error) {
        console.error('Search by tag failed:', error);
    }
}

// Utilities
function debounce(func, wait) {
    let timeout;
    return function executedFunction(...args) {
        const later = () => {
            clearTimeout(timeout);
            func(...args);
        };
        clearTimeout(timeout);
        timeout = setTimeout(later, wait);
    };
}

function escapeHtml(text) {
    const map = {
        '&': '&amp;',
        '<': '&lt;',
        '>': '&gt;',
        '"': '&quot;',
        "'": '&#039;'
    };
    return text.replace(/[&<>"']/g, m => map[m]);
}

function showToast(message) {
    // Simple toast notification (you can enhance this)
    const toast = document.createElement('div');
    toast.className = 'toast';
    toast.textContent = message;
    toast.style.cssText = `
        position: fixed;
        bottom: 20px;
        right: 20px;
        background: var(--success-color);
        color: white;
        padding: 1rem 1.5rem;
        border-radius: 8px;
        z-index: 2000;
    `;
    document.body.appendChild(toast);
    setTimeout(() => toast.remove(), 3000);
}

// View switching functions
function switchView(view) {
    currentView = view;
    const secretTypesSection = document.getElementById('secret-types-section');
    
    if (view === 'secrets') {
        displaySecrets(currentSecrets);
        secretTypesSection.style.display = 'block';
        populateSecretTypesFilter();
        
        // Change the add button behavior
        addEntryBtn.onclick = () => showSecretModal();
        addEntryBtn.title = 'Add Secret';
    } else {
        displayEntries(currentEntries);
        secretTypesSection.style.display = 'none';
        
        // Restore original add button behavior  
        addEntryBtn.onclick = () => showEntryModal();
        addEntryBtn.title = 'Add Entry';
    }
    
    hideEntryDetails();
}

function populateSecretTypesFilter() {
    const container = document.getElementById('secret-types-list');
    container.innerHTML = '';
    
    const allTypes = ['all', ...secretTypes];
    allTypes.forEach(type => {
        const item = document.createElement('a');
        item.href = '#';
        item.className = 'nav-item';
        item.dataset.secretType = type;
        item.textContent = type === 'all' ? 'All Types' : type.replace('-', ' ').replace(/\b\w/g, l => l.toUpperCase());
        
        item.addEventListener('click', (e) => {
            e.preventDefault();
            document.querySelectorAll('#secret-types-list .nav-item').forEach(i => i.classList.remove('active'));
            e.target.classList.add('active');
            filterSecretsByType(type === 'all' ? null : type);
        });
        
        container.appendChild(item);
    });
    
    // Set first item as active
    if (container.firstChild) {
        container.firstChild.classList.add('active');
    }
}

function filterSecrets(filter) {
    switch (filter) {
        case 'favorites':
            displaySecrets(currentSecrets.filter(s => s.favorite));
            break;
        case 'recent':
            const sorted = [...currentSecrets].sort((a, b) => {
                const aTime = a.last_accessed || a.created_at;
                const bTime = b.last_accessed || b.created_at;
                return new Date(bTime) - new Date(aTime);
            });
            displaySecrets(sorted.slice(0, 10));
            break;
        default:
            displaySecrets(currentSecrets);
    }
}

function filterSecretsByType(type) {
    if (!type) {
        displaySecrets(currentSecrets);
        return;
    }
    
    const filtered = currentSecrets.filter(secret => {
        if (typeof secret.secret_type === 'string') {
            return secret.secret_type === type;
        }
        
        const typeKey = Object.keys(secret.secret_type)[0];
        if (typeKey === 'Custom') {
            return secret.secret_type[typeKey] === type;
        }
        
        return typeKey.toLowerCase().replace(/([A-Z])/g, '-$1').replace(/^-/, '') === type;
    });
    
    displaySecrets(filtered);
}

// Secret modal functions
function showSecretModal() {
    secretModal.classList.add('active');
    secretForm.reset();
    updateSecretFields();
    document.getElementById('secret-name').focus();
}

function hideSecretModal() {
    secretModal.classList.remove('active');
}

function updateSecretFields() {
    const selectedType = secretTypeSelect.value;
    secretFieldsDiv.innerHTML = '';
    
    if (selectedType === 'password') {
        secretFieldsDiv.innerHTML = `
            <input type="text" id="secret-username" placeholder="Username" required>
            <div class="password-input-group">
                <input type="password" id="secret-password" placeholder="Password" required>
                <button type="button" onclick="toggleSecretFormPassword()" class="icon-btn">👁️</button>
                <button type="button" onclick="generatePasswordForSecret()" class="icon-btn">⚡</button>
            </div>
            <input type="url" id="secret-url" placeholder="URL (optional)">
        `;
    } else if (selectedType === 'secure-note') {
        secretFieldsDiv.innerHTML = `
            <textarea id="secret-content" placeholder="Note content" rows="5" required></textarea>
        `;
    }
}

function toggleSecretFormPassword() {
    const input = document.getElementById('secret-password');
    if (input) {
        input.type = input.type === 'password' ? 'text' : 'password';
    }
}

async function generatePasswordForSecret() {
    try {
        const config = {
            length: 16,
            include_uppercase: true,
            include_lowercase: true,
            include_numbers: true,
            include_symbols: true,
            exclude_ambiguous: true
        };
        
        const password = await invoke('generate_password', { request: config });
        const input = document.getElementById('secret-password');
        if (input) {
            input.value = password;
        }
    } catch (error) {
        console.error('Password generation failed:', error);
    }
}

async function saveSecret(e) {
    e.preventDefault();
    
    const name = document.getElementById('secret-name').value;
    const description = document.getElementById('secret-description').value || null;
    const secretType = secretTypeSelect.value;
    const tags = document.getElementById('secret-tags').value
        .split(',')
        .map(tag => tag.trim())
        .filter(tag => tag.length > 0);
    
    let data = {};
    
    if (secretType === 'password') {
        data = {
            username: document.getElementById('secret-username').value,
            password: document.getElementById('secret-password').value,
            url: document.getElementById('secret-url').value || null
        };
    } else if (secretType === 'secure-note') {
        data = {
            content: document.getElementById('secret-content').value
        };
    }
    
    const request = {
        name,
        description,
        secret_type: secretType,
        data,
        tags
    };
    
    try {
        await invoke('add_secret', { request });
        await loadSecrets();
        hideSecretModal();
        showNotification('Secret added successfully');
    } catch (error) {
        console.error('Save failed:', error);
        showNotification('Failed to save secret', 'error');
    }
}

// New secrets management functions
async function loadSecrets() {
    try {
        currentSecrets = await invoke('search_secrets', {
            request: {
                query: null,
                secret_type: null,
                tags: [],
                favorites: false,
                project: null
            }
        });
        if (currentView === 'secrets') {
            displaySecrets(currentSecrets);
        }
    } catch (error) {
        console.error('Failed to load secrets:', error);
        currentSecrets = [];
    }
}

async function loadSecretTypes() {
    try {
        secretTypes = await invoke('get_secret_types');
    } catch (error) {
        console.error('Failed to load secret types:', error);
        secretTypes = ['password', 'secure-note'];
    }
}

function displaySecrets(secrets) {
    entriesContainer.innerHTML = '';
    
    if (secrets.length === 0) {
        entriesContainer.innerHTML = '<div class="empty-state">No secrets found</div>';
        return;
    }
    
    secrets.forEach(secret => {
        const secretEl = document.createElement('div');
        secretEl.className = `entry-item ${selectedSecretId === secret.id ? 'selected' : ''}`;
        
        const secretTypeIcon = getSecretTypeIcon(secret.secret_type);
        
        secretEl.innerHTML = `
            <div class="entry-info">
                <h3>${secretTypeIcon} ${escapeHtml(secret.name)}</h3>
                <p class="entry-type">${getSecretTypeDisplay(secret.secret_type)}</p>
                ${secret.description ? `<p class="entry-desc">${escapeHtml(secret.description)}</p>` : ''}
            </div>
            <div class="entry-actions">
                <button class="icon-btn small" onclick="copySecretData('${secret.id}')" title="Copy">📋</button>
                ${secret.favorite ? '⭐' : ''}
            </div>
        `;
        
        secretEl.addEventListener('click', () => selectSecret(secret));
        entriesContainer.appendChild(secretEl);
    });
}

function getSecretTypeIcon(secretType) {
    const icons = {
        'Password': '🔐',
        'SshKey': '🔑',
        'ApiKey': '🎯',
        'SecureNote': '📝',
        'Document': '📄',
        'Configuration': '⚙️',
        'Custom': '🔧'
    };
    
    if (typeof secretType === 'string') {
        return icons['Custom'] || '🔧';
    }
    
    return icons[Object.keys(secretType)[0]] || '🔧';
}

function getSecretTypeDisplay(secretType) {
    if (typeof secretType === 'string') {
        return secretType;
    }
    
    const typeKey = Object.keys(secretType)[0];
    if (typeKey === 'Custom') {
        return secretType[typeKey];
    }
    
    return typeKey.replace(/([A-Z])/g, ' $1').trim();
}

async function selectSecret(secret) {
    selectedSecretId = secret.id;
    selectedEntryId = null;
    displaySecrets(currentSecrets);
    showSecretDetails(secret);
}

function showSecretDetails(secret) {
    entryDetails.classList.add('active');
    document.getElementById('detail-site').textContent = secret.name;
    
    const detailContent = document.getElementById('detail-content');
    let contentHtml = `
        <div class="detail-field">
            <label>Type</label>
            <p>${getSecretTypeDisplay(secret.secret_type)}</p>
        </div>
    `;
    
    if (secret.description) {
        contentHtml += `
            <div class="detail-field">
                <label>Description</label>
                <p>${escapeHtml(secret.description)}</p>
            </div>
        `;
    }
    
    // Display secret data based on type
    if (secret.data) {
        if (secret.data.Password) {
            const data = secret.data.Password;
            contentHtml += `
                <div class="detail-field">
                    <label>Username</label>
                    <div class="detail-field-value">
                        <input type="text" value="${escapeHtml(data.username || '')}" readonly>
                        <button class="icon-btn small" onclick="copyToClipboard('${escapeHtml(data.username || '')}')">📋</button>
                    </div>
                </div>
                <div class="detail-field">
                    <label>Password</label>
                    <div class="detail-field-value">
                        <input type="password" value="${escapeHtml(data.password || '')}" readonly id="detail-secret-password-${secret.id}">
                        <button class="icon-btn small" onclick="toggleSecretPassword('${secret.id}')">👁️</button>
                        <button class="icon-btn small" onclick="copyToClipboard('${escapeHtml(data.password || '')}')">📋</button>
                    </div>
                </div>
            `;
            if (data.url) {
                contentHtml += `
                    <div class="detail-field">
                        <label>URL</label>
                        <div class="detail-field-value">
                            <input type="text" value="${escapeHtml(data.url)}" readonly>
                            <button class="icon-btn small" onclick="openUrl('${escapeHtml(data.url)}')">🔗</button>
                        </div>
                    </div>
                `;
            }
        } else if (secret.data.SecureNote) {
            const data = secret.data.SecureNote;
            contentHtml += `
                <div class="detail-field">
                    <label>Content</label>
                    <textarea readonly rows="6">${escapeHtml(data.content || '')}</textarea>
                </div>
            `;
        }
    }
    
    if (secret.tags.length > 0) {
        contentHtml += `
            <div class="detail-field">
                <label>Tags</label>
                <div>
                    ${secret.tags.map(tag => `<span class="tag">${escapeHtml(tag)}</span>`).join('')}
                </div>
            </div>
        `;
    }
    
    contentHtml += `
        <div class="detail-field">
            <label>Created</label>
            <p>${new Date(secret.created_at).toLocaleString()}</p>
        </div>
        <div class="detail-actions">
            <button class="primary" onclick="editSecret('${secret.id}')">Edit</button>
            <button class="secondary danger" onclick="deleteSecret('${secret.id}')">Delete</button>
        </div>
    `;
    
    detailContent.innerHTML = contentHtml;
}

async function copySecretData(secretId) {
    try {
        const secret = currentSecrets.find(s => s.id === secretId);
        if (!secret) return;
        
        let textToCopy = '';
        if (secret.data && secret.data.Password) {
            textToCopy = secret.data.Password.password || '';
        } else if (secret.data && secret.data.SecureNote) {
            textToCopy = secret.data.SecureNote.content || '';
        }
        
        if (textToCopy) {
            await writeText(textToCopy);
            showNotification('Copied to clipboard');
        }
    } catch (error) {
        console.error('Copy failed:', error);
        showNotification('Copy failed', 'error');
    }
}

function toggleSecretPassword(secretId) {
    const input = document.getElementById(`detail-secret-password-${secretId}`);
    if (input) {
        input.type = input.type === 'password' ? 'text' : 'password';
    }
}

function openUrl(url) {
    window.open(url, '_blank');
}

function showNotification(message, type = 'success') {
    // Simple notification - could be enhanced with a proper notification system
    const notification = document.createElement('div');
    notification.className = `notification ${type}`;
    notification.textContent = message;
    notification.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        background: ${type === 'error' ? '#ff4444' : '#44ff44'};
        color: white;
        padding: 10px 20px;
        border-radius: 4px;
        z-index: 9999;
    `;
    
    document.body.appendChild(notification);
    setTimeout(() => {
        notification.remove();
    }, 3000);
}

async function editSecret(secretId) {
    // TODO: Implement secret editing
    showNotification('Secret editing not yet implemented', 'error');
}

async function deleteSecret(secretId) {
    if (!confirm('Are you sure you want to delete this secret?')) {
        return;
    }
    
    try {
        await invoke('delete_secret', { id: secretId });
        await loadSecrets();
        hideEntryDetails();
        showNotification('Secret deleted successfully');
    } catch (error) {
        console.error('Delete failed:', error);
        showNotification('Failed to delete secret', 'error');
    }
}

// Make functions available globally for onclick handlers
window.copyPassword = copyPassword;
window.copyToClipboard = copyToClipboard;
window.toggleDetailPassword = toggleDetailPassword;
window.editEntry = editEntry;
window.deleteEntry = deleteEntry;
window.copySecretData = copySecretData;
window.toggleSecretPassword = toggleSecretPassword;
window.openUrl = openUrl;
window.editSecret = editSecret;
window.deleteSecret = deleteSecret;
window.toggleSecretFormPassword = toggleSecretFormPassword;
window.generatePasswordForSecret = generatePasswordForSecret;

// Initialize app is now called from initTauri().then() above