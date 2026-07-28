import { invoke } from '@tauri-apps/api/core';

window.addEventListener('DOMContentLoaded', async () => {
    const inputEl = document.querySelector('#journalInput');
    const saveBtn = document.querySelector('#saveBtn');
    const statusEl = document.querySelector('#status');
    const logContainer = document.querySelector('#logContainer');

    const renderEntries = (entries) => {
        logContainer.innerHTML = '';
        if (!entries || entries.length === 0) {
            logContainer.innerHTML = `<p class="empty-log">No entries saved yet. Start writing above!</p>`;
            return;
        }

        entries.forEach(entry => {
            const logItem = document.createElement('div');
            logItem.className = 'log-item';
            logItem.innerHTML = `
                <span class="log-date">📅 ${entry.date}</span>
                <p>${escapeHtml(entry.content)}</p>
                <span class="log-score">⭐ AI Score: ${entry.ai_score} / 10</span>
            `;
            logContainer.appendChild(logItem);
        });
    };

    try {
        const savedEntries = await invoke('get_journal_entries');
        renderEntries(savedEntries);
    } catch (error) {
        console.error('Failed to load entries:', error);
    }

  
    saveBtn.addEventListener('click', async () => {
        const text = inputEl.value.trim();
        if (!text) {
            statusEl.style.color = '#f38ba8';
            statusEl.textContent = 'Entry cannot be empty!';
            return;
        }

        try {
            await invoke('add_journal_entry', { content: text });
            
            statusEl.style.color = '#a6e3a1';
            statusEl.textContent = 'Saved successfully to disk! ✨';
            inputEl.value = '';

            
            const updatedEntries = await invoke('get_journal_entries');
            renderEntries(updatedEntries);

        } catch (error) {
            statusEl.style.color = '#f38ba8';
            statusEl.textContent = 'Error: ' + error;
        }
    });
});

function escapeHtml(str) {
    return str.replace(/[&<>'"]/g, 
        tag => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', "'": '&#39;', '"': '&quot;' }[tag] || tag)
    );
}