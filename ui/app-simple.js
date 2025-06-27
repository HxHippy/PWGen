// Simple test to check if Tauri is working
console.log('App.js loaded');

// Check what's available in the window object
console.log('Window object keys:', Object.keys(window));
console.log('Looking for Tauri API...');

// Function to test Tauri invoke
async function testTauri() {
    try {
        // Try the most basic Tauri v2 approach
        if (window.__TAURI_INTERNALS__) {
            console.log('Found __TAURI_INTERNALS__');
            console.log('Keys:', Object.keys(window.__TAURI_INTERNALS__));
        }
        
        // Check for window.__TAURI__
        if (window.__TAURI__) {
            console.log('Found window.__TAURI__');
            console.log('Keys:', Object.keys(window.__TAURI__));
        }
        
        // Try to find invoke function
        if (window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke) {
            console.log('Found invoke in __TAURI_INTERNALS__');
            const result = await window.__TAURI_INTERNALS__.invoke('vault_exists');
            console.log('Vault exists result:', result);
        }
        
    } catch (error) {
        console.error('Error testing Tauri:', error);
    }
}

// Run test when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    console.log('DOM loaded');
    testTauri();
});