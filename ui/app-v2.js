// Tauri v2 approach - use window.__TAURI_INTERNALS__
console.log('App starting...');

// Global variables for Tauri functions
let tauriInvoke = null;

// Initialize on DOM ready
document.addEventListener('DOMContentLoaded', async () => {
    console.log('DOM loaded, initializing...');
    
    // For Tauri v2, the invoke function is available directly
    if (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke) {
        tauriInvoke = window.__TAURI_INTERNALS__.invoke;
        console.log('Tauri v2 invoke found!');
        
        // Test the connection
        try {
            const exists = await tauriInvoke('vault_exists');
            console.log('Test invoke successful! Vault exists:', exists);
            
            // Show the UI
            document.getElementById('login-screen').classList.add('active');
            
            // Set up event listeners
            setupEventListeners();
        } catch (error) {
            console.error('Test invoke failed:', error);
        }
    } else {
        console.error('Tauri invoke not found');
        console.log('Available window properties:', Object.keys(window));
    }
});

// Wrapper for invoke to make it easier to use
async function invoke(cmd, args = {}) {
    if (!tauriInvoke) {
        throw new Error('Tauri not initialized');
    }
    return tauriInvoke(cmd, args);
}

// Simple clipboard write
async function writeText(text) {
    return invoke('copy_to_clipboard', { text });
}

// Set up event listeners
function setupEventListeners() {
    const masterPasswordInput = document.getElementById('master-password');
    const unlockBtn = document.getElementById('unlock-btn');
    const initBtn = document.getElementById('init-btn');
    
    unlockBtn.addEventListener('click', async () => {
        console.log('Unlock clicked');
        const password = masterPasswordInput.value;
        if (!password) {
            showError('Please enter your master password');
            return;
        }
        
        try {
            await invoke('unlock_vault', { password });
            console.log('Vault unlocked!');
            showMainScreen();
        } catch (error) {
            console.error('Unlock failed:', error);
            showError('Invalid master password');
        }
    });
    
    initBtn.addEventListener('click', async () => {
        console.log('Init clicked');
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
            await invoke('init_vault', { password });
            console.log('Vault initialized!');
            unlockBtn.style.display = 'block';
            showMainScreen();
        } catch (error) {
            console.error('Init failed:', error);
            showError('Failed to initialize vault: ' + error);
        }
    });
    
    // Enter key support
    masterPasswordInput.addEventListener('keypress', (e) => {
        if (e.key === 'Enter') {
            if (unlockBtn.style.display !== 'none') {
                unlockBtn.click();
            } else {
                initBtn.click();
            }
        }
    });
}

function showError(message) {
    const loginError = document.getElementById('login-error');
    if (loginError) {
        loginError.textContent = message;
    }
}

function showMainScreen() {
    const loginScreen = document.getElementById('login-screen');
    const mainScreen = document.getElementById('main-screen');
    
    if (loginScreen) loginScreen.classList.remove('active');
    if (mainScreen) mainScreen.classList.add('active');
    
    console.log('Switched to main screen');
}