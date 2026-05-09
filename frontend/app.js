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
    // Update total
    document.getElementById('total-files').textContent = data.total;
    
    // Update table
    const tbody = document.getElementById('stats-tbody');
    tbody.innerHTML = '';
    
    const sortedEntries = Object.entries(data.counts)
        .sort((a, b) => b[1] - a[1]);
    
    sortedEntries.forEach(([ext, count]) => {
        const percentage = ((count / data.total) * 100).toFixed(1);
        const row = document.createElement('tr');
        row.innerHTML = `
            <td><span class="extension-badge">.${ext}</span></td>
            <td>${count}</td>
            <td>${percentage}%</td>
        `;
        tbody.appendChild(row);
    });
    
    // Update chart
    updateChart(sortedEntries, data.total);
}

function updateChart(entries, total) {
    const ctx = document.getElementById('stats-chart').getContext('2d');
    
    const labels = entries.map(([ext]) => `.${ext}`);
    const data = entries.map(([_, count]) => count);
    
    const colors = [
        '#667eea', '#764ba2', '#f093fb', '#f5576c',
        '#4facfe', '#00f2fe', '#43e97b', '#38f9d7',
        '#fa709a', '#fee140', '#30cfd0', '#330867'
    ];
    
    if (statsChart) {
        statsChart.destroy();
    }
    
    statsChart = new Chart(ctx, {
        type: 'doughnut',
        data: {
            labels: labels,
            datasets: [{
                data: data,
                backgroundColor: colors.slice(0, entries.length),
                borderWidth: 2,
                borderColor: '#fff'
            }]
        },
        options: {
            responsive: true,
            maintainAspectRatio: true,
            plugins: {
                legend: {
                    position: 'bottom',
                    labels: {
                        padding: 15,
                        usePointStyle: true
                    }
                },
                title: {
                    display: true,
                    text: 'File Type Distribution',
                    font: {
                        size: 16
                    }
                }
            }
        }
    });
}

// Initial load
fetchStats();

// Refresh every 5 seconds
setInterval(fetchStats, 5000);
