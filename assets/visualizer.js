let currentEntries = [];

async function fetchStoreContents() {
    try {
        const response = await fetch('/api/store-contents');
        const data = await response.json();
        if (data.status === 'success') {
            currentEntries = data.entries;
            renderEntries(currentEntries);
        } else {
            console.error('Failed to fetch store contents');
        }
    } catch (error) {
        console.error('Error fetching store contents:', error);
    }
}

function renderEntries(entries) {
    const container = document.querySelector('.store-entries');
    container.innerHTML = '';

    entries.forEach(entry => {
        const div = document.createElement('div');
        div.className = 'entry';
        
        // Try to parse the value as JSON for preview
        let valuePreview = '';
        try {
            const value = new Uint8Array(entry.value);
            const text = new TextDecoder().decode(value);
            const parsed = JSON.parse(text);
            valuePreview = JSON.stringify(parsed).slice(0, 100) + '...';
        } catch (e) {
            valuePreview = 'Binary data';
        }

        div.innerHTML = `
            <strong>${entry.key}</strong>
            <div class="value-preview">${valuePreview}</div>
        `;

        div.addEventListener('click', () => showDetails(entry));
        container.appendChild(div);
    });
}

let selectedEntry = null;

function showDetails(entry) {
    selectedEntry = entry;
    // Update selected state
    document.querySelectorAll('.entry').forEach(el => el.classList.remove('selected'));
    // Find the entry by looking at all entries and matching the key
    const entries = document.querySelectorAll('.entry');
    entries.forEach(el => {
        if (el.querySelector('strong').textContent === entry.key) {
            el.classList.add('selected');
        }
    });

    const detailsContent = document.querySelector('.details-content');
    
    try {
        const value = new Uint8Array(entry.value);
        const text = new TextDecoder().decode(value);
        const parsed = JSON.parse(text);
        
        detailsContent.innerHTML = `
            <h3>Key: ${entry.key}</h3>
            <pre>${JSON.stringify(parsed, null, 2)}</pre>
        `;
    } catch (e) {
        detailsContent.innerHTML = `
            <h3>Key: ${entry.key}</h3>
            <p>Binary data:</p>
            <pre>${Array.from(entry.value).join(', ')}</pre>
        `;
    }

    // Update action buttons
    const actionButtons = document.querySelector('.action-buttons');
    actionButtons.innerHTML = `
        <button onclick="copyToClipboard('${entry.key}')">Copy ID</button>
    `;
}

function filterEntries(searchText) {
    const filtered = currentEntries.filter(entry => 
        entry.key.toLowerCase().includes(searchText.toLowerCase())
    );
    renderEntries(filtered);
}

// Event Listeners
document.getElementById('refresh').addEventListener('click', fetchStoreContents);
document.getElementById('search').addEventListener('input', (e) => filterEntries(e.target.value));

async function copyToClipboard(text) {
    try {
        await navigator.clipboard.writeText(text);
        const button = document.querySelector('.action-buttons button');
        button.textContent = 'Copied!';
        button.classList.add('success');
        setTimeout(() => {
            button.textContent = 'Copy ID';
            button.classList.remove('success');
        }, 2000);
    } catch (err) {
        console.error('Failed to copy text: ', err);
    }
}

// Initial load
fetchStoreContents();