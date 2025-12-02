import init, { BrowserFingerprinter } from '../pkg/browser_fingerprint.js';

let fingerprinter = null;

async function initWasm() {
    try {
        await init();
        fingerprinter = new BrowserFingerprinter();
        console.log('🦀 WASM module loaded successfully');
    } catch (error) {
        console.error('Failed to load WASM module:', error);
        showError('Failed to load WebAssembly module. Please ensure the project is built.');
    }
}

function showError(message) {
    const results = document.getElementById('results');
    results.innerHTML = `
        <div class="error-message" style="text-align: center; padding: 2rem; color: var(--accent-magenta);">
            <p style="font-size: 1.25rem; margin-bottom: 1rem;">⚠️ Error</p>
            <p>${message}</p>
        </div>
    `;
    results.classList.remove('hidden');
}

async function collectFingerprint() {
    const scanBtn = document.getElementById('scan-btn');
    const loading = document.getElementById('loading');
    const results = document.getElementById('results');
    
    // Update button state
    scanBtn.disabled = true;
    scanBtn.querySelector('.btn-text').textContent = 'Scanning...';
    
    // Show loading
    loading.classList.remove('hidden');
    results.classList.add('hidden');
    
    try {
        // Small delay for effect
        await new Promise(resolve => setTimeout(resolve, 800));
        
        // Collect fingerprint from Rust/WASM
        const jsonData = fingerprinter.collect();
        const data = JSON.parse(jsonData);
        
        // Update UI with fingerprint data
        updateUI(data);
        
        // Show results
        loading.classList.add('hidden');
        results.classList.remove('hidden');
        
    } catch (error) {
        console.error('Fingerprinting failed:', error);
        loading.classList.add('hidden');
        showError('Failed to collect browser fingerprint: ' + error.message);
    } finally {
        scanBtn.disabled = false;
        scanBtn.querySelector('.btn-text').textContent = 'Analyze Browser';
    }
}

function updateUI(data) {
    // Hash
    document.getElementById('hash-value').textContent = data.fingerprint_hash;
    
    // Browser info
    document.getElementById('user-agent').textContent = truncate(data.user_agent, 80);
    document.getElementById('user-agent').title = data.user_agent;
    document.getElementById('language').textContent = data.language + (data.languages.length > 1 ? ` (+${data.languages.length - 1} more)` : '');
    document.getElementById('platform').textContent = data.platform;
    document.getElementById('cookies').innerHTML = statusBadge(data.cookie_enabled);
    document.getElementById('dnt').textContent = data.do_not_track || 'Not set';
    document.getElementById('online').innerHTML = statusBadge(data.online, 'Online', 'Offline');
    
    // Hardware
    document.getElementById('cpu-cores').textContent = data.hardware_concurrency || 'Unknown';
    document.getElementById('device-memory').textContent = data.device_memory ? `${data.device_memory} GB` : 'Unknown';
    document.getElementById('touch-points').textContent = data.max_touch_points;
    
    // Screen
    document.getElementById('resolution').textContent = `${data.screen_width} × ${data.screen_height}`;
    document.getElementById('avail-resolution').textContent = `${data.screen_avail_width} × ${data.screen_avail_height}`;
    document.getElementById('color-depth').textContent = `${data.screen_color_depth}-bit`;
    document.getElementById('pixel-ratio').textContent = data.device_pixel_ratio.toFixed(2);
    
    // Timezone
    document.getElementById('timezone').textContent = data.timezone;
    document.getElementById('tz-offset').textContent = formatTimezoneOffset(data.timezone_offset);
    
    // Storage
    document.getElementById('local-storage').innerHTML = statusBadge(data.local_storage);
    document.getElementById('session-storage').innerHTML = statusBadge(data.session_storage);
    document.getElementById('indexed-db').innerHTML = statusBadge(data.indexed_db);
    
    // Canvas
    document.getElementById('canvas-hash').textContent = data.canvas_fingerprint;
    
    // WebGL
    document.getElementById('webgl-vendor').textContent = data.webgl_vendor;
    document.getElementById('webgl-renderer').textContent = data.webgl_renderer;
    document.getElementById('webgl-version').textContent = data.webgl_version;
    document.getElementById('webgl-shading').textContent = data.webgl_shading_language_version;
    document.getElementById('webgl-extensions').textContent = data.webgl_extensions.length > 0 
        ? data.webgl_extensions.join(', ') 
        : 'None detected';
    
    // Plugins
    const pluginsList = document.getElementById('plugins-list');
    if (data.plugins.length > 0) {
        pluginsList.innerHTML = data.plugins.map(p => `<div class="item">${escapeHtml(p)}</div>`).join('');
    } else {
        pluginsList.innerHTML = '<div class="item">No plugins detected</div>';
    }
    
    // JSON output
    document.getElementById('json-output').textContent = JSON.stringify(data, null, 2);
}

function statusBadge(value, trueText = 'Yes', falseText = 'No') {
    const className = value ? 'status-true' : 'status-false';
    const text = value ? trueText : falseText;
    return `<span class="${className}">${text}</span>`;
}

function truncate(str, maxLength) {
    if (str.length <= maxLength) return str;
    return str.substring(0, maxLength) + '...';
}

function formatTimezoneOffset(offsetMinutes) {
    const hours = Math.floor(Math.abs(offsetMinutes) / 60);
    const minutes = Math.abs(offsetMinutes) % 60;
    const sign = offsetMinutes <= 0 ? '+' : '-';
    return `UTC${sign}${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}`;
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

function copyHash() {
    const hash = document.getElementById('hash-value').textContent;
    navigator.clipboard.writeText(hash).then(() => {
        const btn = document.getElementById('copy-btn');
        const originalText = btn.textContent;
        btn.textContent = 'Copied!';
        btn.style.borderColor = 'var(--accent-green)';
        btn.style.color = 'var(--accent-green)';
        setTimeout(() => {
            btn.textContent = originalText;
            btn.style.borderColor = '';
            btn.style.color = '';
        }, 2000);
    }).catch(err => {
        console.error('Failed to copy:', err);
    });
}

function toggleJson() {
    const output = document.getElementById('json-output');
    const btn = document.getElementById('toggle-json');
    
    if (output.classList.contains('hidden')) {
        output.classList.remove('hidden');
        btn.textContent = 'Hide';
    } else {
        output.classList.add('hidden');
        btn.textContent = 'Show';
    }
}

// Event listeners
document.addEventListener('DOMContentLoaded', () => {
    initWasm();
    
    document.getElementById('scan-btn').addEventListener('click', collectFingerprint);
    document.getElementById('copy-btn').addEventListener('click', copyHash);
    document.getElementById('toggle-json').addEventListener('click', toggleJson);
});

