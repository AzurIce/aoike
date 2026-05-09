let statsChart = null;

async function fetchStats() {
    try {
        const response = await fetch('/api/stats');
        if (!response.ok) throw new Error('Failed to fetch stats');
        const data = await response.json();
        updateUI(data);
    } catch (error) {
        console.error('Error fetching stats:', error);
    }
}

function updateUI(data) {
    const entries = Object.entries(data.counts)
        .sort((a, b) => b[1] - a[1]);
    
    // Update stat cards
    document.getElementById('total-files').textContent = data.total;
    document.getElementById('total-types').textContent = entries.length;
    document.getElementById('top-type').textContent = entries.length > 0 
        ? `.${entries[0][0]}` 
        : '-';
    
    // Update table
    const tbody = document.getElementById('stats-tbody');
    tbody.innerHTML = '';
    
    entries.forEach(([ext, count]) => {
        const percentage = data.total > 0 
            ? ((count / data.total) * 100).toFixed(1) + '%'
            : '0%';
        
        const row = document.createElement('tr');
        row.innerHTML = `
            <td><span class="badge">.${ext}</span></td>
            <td class="count">${count}</td>
            <td class="percentage">${percentage}</td>
        `;
        tbody.appendChild(row);
    });
    
    // Update chart
    updateChart(entries);
}

function updateChart(entries) {
    const ctx = document.getElementById('stats-chart').getContext('2d');
    
    const labels = entries.map(([ext]) => `.${ext}`);
    const data = entries.map(([_, count]) => count);
    
    // Minimal color palette
    const colors = [
        '#18181b', '#3f3f46', '#71717a', '#a1a1aa',
        '#d4d4d8', '#e4e4e7', '#f4f4f5', '#fafafa'
    ];
    
    if (statsChart) {
        statsChart.destroy();
    }
    
    const container = document.querySelector('.chart-container');
    const isDark = false;
    
    statsChart = new Chart(ctx, {
        type: 'doughnut',
        data: {
            labels: labels,
            datasets: [{
                data: data,
                backgroundColor: colors.slice(0, entries.length),
                borderWidth: 2,
                borderColor: '#ffffff',
                hoverOffset: 4
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: false,
            cutout: '65%',
            plugins: {
                legend: {
                    display: false
                },
                tooltip: {
                    backgroundColor: '#18181b',
                    padding: 12,
                    cornerRadius: 8,
                    titleFont: {
                        size: 13,
                        family: '-apple-system, BlinkMacSystemFont, sans-serif'
                    },
                    bodyFont: {
                        size: 13,
                        family: '-apple-system, BlinkMacSystemFont, sans-serif'
                    },
                    callbacks: {
                        label: (context) => {
                            const total = context.dataset.data.reduce((a, b) => a + b, 0);
                            const percentage = ((context.raw / total) * 100).toFixed(1);
                            return ` ${context.raw} files (${percentage}%)`;
                        }
                    }
                }
            },
            animation: {
                duration: 400
            }
        }
    });
}

// Initial load
fetchStats();

// Setup SSE for real-time updates
function setupSSE() {
    const eventSource = new EventSource('/api/events');
    
    eventSource.onmessage = (event) => {
        try {
            const data = JSON.parse(event.data);
            updateUI(data);
        } catch (error) {
            console.error('Error parsing SSE data:', error);
        }
    };
    
    eventSource.onerror = (error) => {
        console.error('SSE error:', error);
        document.getElementById('status-indicator').style.background = '#ef4444';
        setTimeout(() => {
            eventSource.close();
            setupSSE();
        }, 3000);
    };
    
    eventSource.onopen = () => {
        document.getElementById('status-indicator').style.background = '#22c55e';
    };
}

setupSSE();

// Handle window resize
window.addEventListener('resize', () => {
    if (statsChart) {
        statsChart.resize();
    }
});
