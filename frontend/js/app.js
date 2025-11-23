/**
 * Safari Skills Passport - Frontend Application
 * Modern, Accessible, Feature-Rich UI
 */

// IMMEDIATE TEST - This should show in console when page loads
console.log('🚀 JAVASCRIPT FILE LOADED SUCCESSFULLY!');
console.log('Current URL:', window.location.href);
console.log('DOM readyState:', document.readyState);
const CONFIG = {
    // Use 127.0.0.1 to avoid IPv6 localhost resolution issues in some browsers
    API_BASE_URL: 'http://127.0.0.1:8080/api',
    STORAGE_KEY: 'ssp_auth_token',
    USER_KEY: 'ssp_user_data',
    TOAST_DURATION: 5000,
    ANIMATION_DURATION: 300
};

// State Management
const state = {
    token: null,
    user: null,
    credentials: [],
    qrScanner: null
};

// ==================== Utility Functions ====================

/**
 * Show toast notification
 */
function showToast(message, type = 'info') {
    const container = document.getElementById('toast-container');
    const toast = document.createElement('div');
    toast.className = `toast ${type}`;
    
    const icons = {
        success: 'fa-check-circle',
        error: 'fa-exclamation-circle',
        warning: 'fa-exclamation-triangle',
        info: 'fa-info-circle'
    };
    
    toast.innerHTML = `
        <div class="toast-icon">
            <i class="fas ${icons[type]}"></i>
        </div>
        <div class="toast-content">
            <p>${message}</p>
        </div>
    `;
    
    container.appendChild(toast);
    
    // Auto remove
    setTimeout(() => {
        toast.style.animation = 'slideIn 0.3s ease-out reverse';
        setTimeout(() => toast.remove(), 300);
    }, CONFIG.TOAST_DURATION);
}

/**
 * Show/hide loading overlay
 */
function setLoading(loading, message = 'Loading...') {
    const overlay = document.getElementById('loading-overlay');
    if (loading) {
        overlay.querySelector('p').textContent = message;
        overlay.style.display = 'flex';
    } else {
        overlay.style.display = 'none';
    }
}

/**
 * Make API request
 */
async function apiRequest(endpoint, options = {}) {
    const url = `${CONFIG.API_BASE_URL}${endpoint}`;
    const headers = {
        'Content-Type': 'application/json',
        ...options.headers
    };
    
    if (state.token) {
        headers['Authorization'] = `Bearer ${state.token}`;
    }
    
    try {
        const response = await fetch(url, {
            ...options,
            headers
        });
        
        let data;
        const contentType = response.headers.get('content-type');
        
        if (contentType && contentType.includes('application/json')) {
            data = await response.json();
        } else {
            // For non-JSON responses (like error pages), try to extract text
            const text = await response.text();
            data = { error: text || 'Request failed' };
        }
        
        if (!response.ok) {
            throw new Error(data.error || data.message || 'Request failed');
        }
        
        return data;
    } catch (error) {
        console.error('API Error:', error);
        throw error;
    }
}

/**
 * Format date
 */
function formatDate(dateString) {
    const date = new Date(dateString);
    return date.toLocaleDateString('en-US', {
        year: 'numeric',
        month: 'long',
        day: 'numeric'
    });
}

/**
 * Clear auth data from localStorage and reset state
 */
function clearAuthData() {
    localStorage.removeItem(CONFIG.STORAGE_KEY);
    localStorage.removeItem(CONFIG.USER_KEY);
    state.token = null;
    state.user = null;
    state.credentials = [];
}

/**
 * Save auth data to localStorage
 */
function saveAuthData(token, user) {
    localStorage.setItem(CONFIG.STORAGE_KEY, token);
    localStorage.setItem(CONFIG.USER_KEY, JSON.stringify(user));
    state.token = token;
    state.user = user;
}

/**
 * Load auth data from localStorage
 */
function loadAuthData() {
    const token = localStorage.getItem(CONFIG.STORAGE_KEY);
    const userData = localStorage.getItem(CONFIG.USER_KEY);
    
    if (token && userData) {
        state.token = token;
        state.user = JSON.parse(userData);
        return true;
    }
    return false;
}

/**
 * Update navigation based on authentication state
 */
function updateNavigation() {
    const navAuthButtons = document.getElementById('nav-auth-buttons');
    const navUserInfo = document.getElementById('nav-user-info');
    const userNameElement = document.getElementById('nav-user-name');
    
    if (state.token && state.user) {
        // User is logged in - show user info, hide auth buttons
        if (navAuthButtons) navAuthButtons.style.display = 'none';
        if (navUserInfo) navUserInfo.style.display = 'flex';
        if (userNameElement && state.user.name) {
            userNameElement.textContent = state.user.name;
        } else if (userNameElement && state.user.email) {
            userNameElement.textContent = state.user.email.split('@')[0]; // Show username part of email
        }
    } else {
        // User is not logged in - show auth buttons, hide user info
        if (navAuthButtons) navAuthButtons.style.display = 'flex';
        if (navUserInfo) navUserInfo.style.display = 'none';
    }
}

// ==================== Navigation & UI ====================

/**
 * Show specific section, hide others
 */
function showSection(sectionId) {
    console.log('showSection called with:', sectionId);

    // Hide all main sections first
    const allSections = ['hero', 'features', 'how-it-works', 'verify', 'about', 'dashboard'];
    allSections.forEach(id => {
        const section = document.getElementById(id);
        if (section) {
            section.style.display = 'none';
        }
    });

    // Show the requested section
    const targetSection = document.getElementById(sectionId);
    if (targetSection) {
        targetSection.style.display = 'block';
        console.log(`Section ${sectionId} shown successfully`);
    } else {
        console.warn(`Section ${sectionId} not found in DOM`);
    }

    // Scroll to top
    window.scrollTo({ top: 0, behavior: 'smooth' });
}/**
 * Toggle mobile navigation
 */
function initMobileNav() {
    const toggle = document.querySelector('.nav-toggle');
    const menu = document.querySelector('.nav-menu');
    
    if (toggle && menu) {
        toggle.addEventListener('click', () => {
            const isExpanded = toggle.getAttribute('aria-expanded') === 'true';
            toggle.setAttribute('aria-expanded', !isExpanded);
            menu.classList.toggle('active');
        });
    }
}

/**
 * Initialize smooth scroll for anchor links
 */
function initSmoothScroll() {
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function (e) {
            const href = this.getAttribute('href');
            if (href === '#') return;
            
            e.preventDefault();
            const target = document.querySelector(href);
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
}

/**
 * Animate counter numbers
 */
function animateCounters() {
    const counters = document.querySelectorAll('.stat-number[data-target]');
    
    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                const counter = entry.target;
                const target = parseInt(counter.getAttribute('data-target'));
                const duration = 2000;
                const increment = target / (duration / 16);
                let current = 0;
                
                const updateCounter = () => {
                    current += increment;
                    if (current < target) {
                        counter.textContent = Math.floor(current);
                        requestAnimationFrame(updateCounter);
                    } else {
                        counter.textContent = target + (target === 10 ? 's' : '');
                    }
                };
                
                updateCounter();
                observer.unobserve(counter);
            }
        });
    }, { threshold: 0.5 });
    
    counters.forEach(counter => observer.observe(counter));
}

// ==================== Authentication ====================

/**
 * Handle logout
 */
function handleLogout() {
    clearAuthData();
    closeModal(); // Close modal if open
    updateNavigation();
    showToast('Logged out successfully', 'info');

    // Show hero section (main landing page) after logout
    showSection('hero');
}

// ==================== Dashboard ====================

/**
 * Show dashboard
 */
async function showDashboard() {
    // Ensure modal is closed before showing dashboard
    closeModal();
    
    showSection('dashboard');
    
    // Update welcome message
    const welcome = document.getElementById('user-welcome');
    if (welcome && state.user) {
        welcome.textContent = `Welcome back, ${state.user.name || state.user.email}!`;
    }
    
    // Update dashboard based on user role
    updateDashboardForRole();
    
    // Load credentials
    await loadCredentials();
}

/**
 * Update dashboard based on user role
 */
function updateDashboardForRole() {
    const role = state.user.role;
    
    // Hide all role dashboards
    const roleDashboards = ['professional-dashboard', 'employer-dashboard', 'institution-dashboard'];
    roleDashboards.forEach(id => {
        const dashboard = document.getElementById(id);
        if (dashboard) dashboard.style.display = 'none';
    });
    
    // Show the appropriate dashboard
    const activeDashboard = document.getElementById(`${role}-dashboard`);
    if (activeDashboard) {
        activeDashboard.style.display = 'block';
        
        // Load role-specific data
        loadRoleSpecificData(role);
    }
}

/**
 * Load role-specific data
 */
async function loadRoleSpecificData(role) {
    switch (role) {
        case 'professional':
            await loadCredentials();
            break;
        case 'employer':
            await loadEmployerData();
            break;
        case 'institution':
            await loadInstitutionData();
            break;
    }
}

/**
 * Load employer-specific data
 */
async function loadEmployerData() {
    if (!state.token) return;
    
    setLoading(true, 'Loading employer data...');
    
    try {
        // Load verification history
        const historyResponse = await apiRequest('/verifications/history');
        const history = historyResponse.verifications || [];
        
        // If no history from API, use mock data
        const mockHistory = history.length > 0 ? history : [
            {
                id: 'mock-ver-1',
                candidate_name: 'John Doe',
                credential_type: 'degree',
                status: 'verified',
                created_at: '2024-11-01T10:30:00Z'
            },
            {
                id: 'mock-ver-2',
                candidate_name: 'Jane Smith',
                credential_type: 'certificate',
                status: 'verified',
                created_at: '2024-11-02T14:15:00Z'
            },
            {
                id: 'mock-ver-3',
                candidate_name: 'Bob Johnson',
                credential_type: 'license',
                status: 'pending',
                created_at: '2024-11-03T09:45:00Z'
            }
        ];
        
        // Update stats
        updateEmployerStats(mockHistory);
        
        // Render recent verifications
        renderRecentVerifications(mockHistory);
    } catch (error) {
        console.error('Failed to load employer data:', error);
        // Use mock data on API failure
        const mockHistory = [
            {
                id: 'mock-ver-1',
                candidate_name: 'John Doe',
                credential_type: 'degree',
                status: 'verified',
                created_at: '2024-11-01T10:30:00Z'
            },
            {
                id: 'mock-ver-2',
                candidate_name: 'Jane Smith',
                credential_type: 'certificate',
                status: 'verified',
                created_at: '2024-11-02T14:15:00Z'
            },
            {
                id: 'mock-ver-3',
                candidate_name: 'Bob Johnson',
                credential_type: 'license',
                status: 'pending',
                created_at: '2024-11-03T09:45:00Z'
            }
        ];
        
        updateEmployerStats(mockHistory);
        renderRecentVerifications(mockHistory);
        showToast('Using demo data - API not available', 'info');
    } finally {
        setLoading(false);
    }
}

/**
 * Load institution-specific data
 */
async function loadInstitutionData() {
    if (!state.token) return;
    
    setLoading(true, 'Loading institution data...');
    
    try {
        // Load issued credentials
        const issuedResponse = await apiRequest('/credentials/issued');
        const issuedCredentials = issuedResponse.credentials || [];
        
        // If no credentials from API, use mock data
        const mockCredentials = issuedCredentials.length > 0 ? issuedCredentials : [
            {
                id: 'mock-issued-1',
                title: 'Software Engineering Diploma',
                holder_name: 'Alice Cooper',
                credential_type: 'certificate',
                issued_at: '2024-10-15T00:00:00Z',
                status: 'verified'
            },
            {
                id: 'mock-issued-2',
                title: 'Data Science Certification',
                holder_name: 'Charlie Brown',
                credential_type: 'certificate',
                issued_at: '2024-10-20T00:00:00Z',
                status: 'verified'
            },
            {
                id: 'mock-issued-3',
                title: 'Project Management Professional',
                holder_name: 'Diana Prince',
                credential_type: 'certificate',
                issued_at: '2024-10-25T00:00:00Z',
                status: 'pending'
            },
            {
                id: 'mock-issued-4',
                title: 'Master of Business Administration',
                holder_name: 'Edward Norton',
                credential_type: 'degree',
                issued_at: '2024-09-30T00:00:00Z',
                status: 'verified'
            }
        ];
        
        // Update stats
        updateInstitutionStats(mockCredentials);
        
        // Render issued credentials
        renderIssuedCredentials(mockCredentials);
    } catch (error) {
        console.error('Failed to load institution data:', error);
        // Use mock data on API failure
        const mockCredentials = [
            {
                id: 'mock-issued-1',
                title: 'Software Engineering Diploma',
                holder_name: 'Alice Cooper',
                credential_type: 'certificate',
                issued_at: '2024-10-15T00:00:00Z',
                status: 'verified'
            },
            {
                id: 'mock-issued-2',
                title: 'Data Science Certification',
                holder_name: 'Charlie Brown',
                credential_type: 'certificate',
                issued_at: '2024-10-20T00:00:00Z',
                status: 'verified'
            },
            {
                id: 'mock-issued-3',
                title: 'Project Management Professional',
                holder_name: 'Diana Prince',
                credential_type: 'certificate',
                issued_at: '2024-10-25T00:00:00Z',
                status: 'pending'
            },
            {
                id: 'mock-issued-4',
                title: 'Master of Business Administration',
                holder_name: 'Edward Norton',
                credential_type: 'degree',
                issued_at: '2024-09-30T00:00:00Z',
                status: 'verified'
            }
        ];
        
        updateInstitutionStats(mockCredentials);
        renderIssuedCredentials(mockCredentials);
        showToast('Using demo data - API not available', 'info');
    } finally {
        setLoading(false);
    }
}

/**
 * Update employer statistics
 */
function updateEmployerStats(history) {
    const today = new Date().toDateString();
    const todayVerifications = history.filter(v => new Date(v.created_at).toDateString() === today).length;
    const pendingVerifications = history.filter(v => v.status === 'pending').length;
    const uniqueCandidates = new Set(history.map(v => v.candidate_id)).size;
    
    document.getElementById('verifications-today').textContent = todayVerifications;
    document.getElementById('pending-verifications').textContent = pendingVerifications;
    document.getElementById('total-candidates').textContent = uniqueCandidates;
}

/**
 * Update institution statistics
 */
function updateInstitutionStats(credentials) {
    const totalIssued = credentials.length;
    const uniqueRecipients = new Set(credentials.map(c => c.holder_id)).size;
    const verifiedCredentials = credentials.filter(c => c.status === 'verified').length;
    const verificationRate = totalIssued > 0 ? Math.round((verifiedCredentials / totalIssued) * 100) : 0;
    
    document.getElementById('issued-credentials').textContent = totalIssued;
    document.getElementById('total-recipients').textContent = uniqueRecipients;
    document.getElementById('verification-rate').textContent = `${verificationRate}%`;
}

/**
 * Render recent verifications for employer
 */
function renderRecentVerifications(history) {
    const container = document.getElementById('recent-verifications');
    if (!container) return;
    
    if (history.length === 0) {
        container.innerHTML = `
            <div class="empty-state">
                <i class="fas fa-history"></i>
                <h3>No recent verifications</h3>
                <p>Your verification history will appear here</p>
            </div>
        `;
        return;
    }
    
    container.innerHTML = history.slice(0, 5).map(verification => `
        <div class="verification-item">
            <div class="verification-icon">
                <i class="fas ${verification.status === 'verified' ? 'fa-check-circle' : 'fa-clock'}"></i>
            </div>
            <div class="verification-info">
                <h4>${verification.candidate_name || 'Unknown Candidate'}</h4>
                <p>${verification.credential_type || 'Credential'} - ${formatDate(verification.created_at)}</p>
                <span class="verification-status ${verification.status}">${verification.status}</span>
            </div>
        </div>
    `).join('');
}

/**
 * Render issued credentials for institution
 */
function renderIssuedCredentials(credentials) {
    const container = document.getElementById('issued-credentials-container');
    if (!container) return;
    
    if (credentials.length === 0) {
        container.innerHTML = `
            <div class="empty-state">
                <i class="fas fa-graduation-cap"></i>
                <h3>No credentials issued yet</h3>
                <p>Start issuing credentials to professionals</p>
            </div>
        `;
        return;
    }
    
    container.innerHTML = credentials.slice(0, 10).map(cred => `
        <div class="credential-item">
            <div class="credential-icon">
                <i class="fas ${getCredentialIcon(cred.credential_type)}"></i>
            </div>
            <div class="credential-info">
                <h3>${cred.title || cred.credential_type}</h3>
                <p>Issued to: ${cred.holder_name || 'Unknown Recipient'}</p>
                <span class="credential-date">${formatDate(cred.issued_at)}</span>
                <span class="credential-status ${cred.status}">${cred.status}</span>
            </div>
            <div class="credential-actions">
                <button class="btn btn-secondary" onclick="viewCredential('${cred.id}')">
                    <i class="fas fa-eye"></i> View
                </button>
                <button class="btn btn-primary" onclick="revokeCredential('${cred.id}')">
                    <i class="fas fa-ban"></i> Revoke
                </button>
            </div>
        </div>
    `).join('');
}

/**
 * Load user credentials
 */
async function loadCredentials() {
    if (!state.token) return;
    
    setLoading(true, 'Loading credentials...');
    
    try {
        const response = await apiRequest('/credentials/my');
        state.credentials = response.credentials || [];
        
        // If no credentials from API, use mock data for demo
        if (state.credentials.length === 0) {
            state.credentials = [
                {
                    id: 'mock-cred-1',
                    title: 'Bachelor of Science in Computer Science',
                    credential_type: 'degree',
                    issuer_name: 'University of Nairobi',
                    issued_at: '2023-06-15T00:00:00Z',
                    status: 'verified',
                    views: 12
                },
                {
                    id: 'mock-cred-2',
                    title: 'AWS Certified Solutions Architect',
                    credential_type: 'certificate',
                    issuer_name: 'Amazon Web Services',
                    issued_at: '2024-01-20T00:00:00Z',
                    status: 'verified',
                    views: 8
                }
            ];
        }
        
        // Update stats
        updateDashboardStats();
        
        // Render credentials list
        renderCredentialsList();
    } catch (error) {
        console.error('Failed to load credentials:', error);
        // Use mock data on API failure
        state.credentials = [
            {
                id: 'mock-cred-1',
                title: 'Bachelor of Science in Computer Science',
                credential_type: 'degree',
                issuer_name: 'University of Nairobi',
                issued_at: '2023-06-15T00:00:00Z',
                status: 'verified',
                views: 12
            },
            {
                id: 'mock-cred-2',
                title: 'AWS Certified Solutions Architect',
                credential_type: 'certificate',
                issuer_name: 'Amazon Web Services',
                issued_at: '2024-01-20T00:00:00Z',
                status: 'verified',
                views: 8
            }
        ];
        
        updateDashboardStats();
        renderCredentialsList();
        showToast('Using demo data - API not available', 'info');
    } finally {
        setLoading(false);
    }
}

/**
 * Update dashboard statistics
 */
function updateDashboardStats() {
    const total = document.getElementById('total-credentials');
    const verified = document.getElementById('verified-credentials');
    const views = document.getElementById('view-count');
    
    if (total) total.textContent = state.credentials.length;
    if (verified) {
        const verifiedCount = state.credentials.filter(c => c.status === 'verified').length;
        verified.textContent = verifiedCount;
    }
    if (views) {
        const totalViews = state.credentials.reduce((sum, c) => sum + (c.views || 0), 0);
        views.textContent = totalViews;
    }
}

/**
 * Render credentials list
 */
function renderCredentialsList() {
    const container = document.getElementById('credentials-container');
    if (!container) return;
    
    if (state.credentials.length === 0) {
        container.innerHTML = `
            <div class="empty-state">
                <i class="fas fa-inbox"></i>
                <h3>No credentials yet</h3>
                <p>Get started by requesting a credential from an institution</p>
            </div>
        `;
        return;
    }
    
    container.innerHTML = state.credentials.map(cred => `
        <div class="credential-item">
            <div class="credential-icon">
                <i class="fas ${getCredentialIcon(cred.credential_type)}"></i>
            </div>
            <div class="credential-info">
                <h3>${cred.title || cred.credential_type}</h3>
                <p>${cred.issuer_name || 'Unknown Issuer'}</p>
                <span class="credential-date">${formatDate(cred.issued_at)}</span>
            </div>
            <div class="credential-actions">
                <button class="btn btn-secondary" onclick="viewCredential('${cred.id}')">
                    <i class="fas fa-eye"></i> View
                </button>
                ${cred.id.startsWith('mock-') ? 
                    `<button class="btn btn-primary" disabled title="QR codes only available for real credentials">
                        <i class="fas fa-qrcode"></i> QR Code
                    </button>` :
                    `<button class="btn btn-primary" onclick="downloadQR('${cred.id}')">
                        <i class="fas fa-qrcode"></i> QR Code
                    </button>`
                }
            </div>
        </div>
    `).join('');
}

/**
 * Get icon for credential type
 */
function getCredentialIcon(type) {
    const icons = {
        'degree': 'fa-graduation-cap',
        'certificate': 'fa-certificate',
        'license': 'fa-id-card',
        'transcript': 'fa-file-alt',
        'award': 'fa-trophy'
    };
    return icons[type] || 'fa-certificate';
}

/**
 * View credential details
 */
async function viewCredential(credentialId) {
    setLoading(true, 'Loading credential...');
    
    try {
        const credential = await apiRequest(`/credentials/${credentialId}`);
        showCredentialModal(credential);
    } catch (error) {
        console.error('Failed to load credential:', error);
        // Use mock data on API failure
        const mockCredential = {
            id: credentialId,
            title: 'Demo Credential',
            credential_type: 'certificate',
            issuer_name: 'Demo Institution',
            issued_at: '2024-10-15T00:00:00Z',
            status: 'verified',
            description: 'This is a demo credential showing blockchain-verified professional certification.'
        };
        
        showCredentialModal(mockCredential);
        showToast('Using demo data - API not available', 'info');
    } finally {
        setLoading(false);
    }
}

/**
 * Download QR code
 */
async function downloadQR(credentialId) {
    // Check if this is a mock credential
    if (credentialId.startsWith('mock-')) {
        showToast('QR codes are only available for real credentials. Please issue a credential first.', 'info');
        return;
    }
    
    setLoading(true, 'Generating QR code...');
    
    try {
        const response = await fetch(`${CONFIG.API_BASE_URL}/credentials/${credentialId}/qr`, {
            headers: {
                'Authorization': `Bearer ${state.token}`
            }
        });
        
        if (!response.ok) throw new Error('Failed to generate QR code');
        
        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `credential-${credentialId}-qr.png`;
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
        window.URL.revokeObjectURL(url);
        
        showToast('QR code downloaded', 'success');
    } catch (error) {
        showToast('Failed to download QR code', 'error');
    } finally {
        setLoading(false);
    }
}

// ==================== Role-Based Dashboard Functions ====================

/**
 * Add credential (for professionals)
 */
function addCredential() {
    showModal('add-credential-modal');
}

/**
 * Issue credential (for professionals and institutions)
 */
function issueCredential() {
    showModal('issue-credential-modal');
}

/**
 * Verify credential (for professionals and employers)
 */
function verifyCredential() {
    // Redirect to verify section
    showSection('verify');
    showToast('Scan QR code or enter credential ID to verify', 'info');
}

/**
 * Manage credentials (for institutions)
 */
function manageCredentials() {
    showModal('manage-credentials-modal');
}

/**
 * Search candidates (for employers)
 */
function searchCandidates() {
    showModal('search-candidates-modal');
}

/**
 * View verification history (for employers)
 */
function viewVerificationHistory() {
    showModal('verification-history-modal');
}

/**
 * Bulk issue credentials (for institutions)
 */
function bulkIssueCredentials() {
    showModal('bulk-issue-modal');
}

/**
 * Revoke credential (for institutions)
 */
async function revokeCredential(credentialId) {
    if (!confirm('Are you sure you want to revoke this credential? This action cannot be undone.')) {
        return;
    }
    
    setLoading(true, 'Revoking credential...');
    
    try {
        await apiRequest(`/credentials/${credentialId}/revoke`, {
            method: 'POST'
        });
        
        showToast('Credential revoked successfully', 'success');
        // Reload institution data
        await loadInstitutionData();
    } catch (error) {
        showToast('Failed to revoke credential', 'error');
    } finally {
        setLoading(false);
    }
}

// Close modal
function closeModal() {
    const modals = document.querySelectorAll('.modal-overlay');
    modals.forEach(modal => {
        modal.style.display = 'none';
        modal.style.visibility = 'hidden';
        modal.style.opacity = '0';
        document.body.style.overflow = 'auto';
        // Remove modal from DOM after animation
        setTimeout(() => {
            if (modal.parentNode) {
                modal.parentNode.removeChild(modal);
            }
        }, 300);
    });

    // Clear any form errors (for auth modals)
    clearFormErrors();
}

/**
 * Show modal by ID
 */
function showModal(modalId) {
    const modal = document.getElementById(modalId);
    if (modal) {
        modal.style.display = 'flex';
        modal.style.visibility = 'visible';
        modal.style.opacity = '1';
        document.body.style.overflow = 'hidden';
    } else {
        // Create modal if it doesn't exist
        createModal(modalId);
    }
}
function createModal(modalId) {
    const modal = document.createElement('div');
    modal.id = modalId;
    modal.className = 'modal-overlay';
    modal.innerHTML = getModalContent(modalId);
    document.body.appendChild(modal);
    
    // Add close functionality
    const closeBtn = modal.querySelector('.modal-close');
    if (closeBtn) {
        closeBtn.addEventListener('click', () => closeModal());
    }
    
    // Add form event listeners
    const form = modal.querySelector('form');
    if (form) {
        switch (modalId) {
            case 'add-credential-modal':
                form.addEventListener('submit', handleAddCredential);
                break;
            case 'issue-credential-modal':
                form.addEventListener('submit', handleIssueCredential);
                break;
            case 'search-candidates-modal':
                form.addEventListener('submit', handleSearchCandidates);
                break;
        }
    }
    
    // Show modal
    modal.style.display = 'flex';
    modal.style.visibility = 'visible';
    modal.style.opacity = '1';
    document.body.style.overflow = 'hidden';
}

/**
 * Get modal content based on modal ID
 */
function getModalContent(modalId) {
    switch (modalId) {
        case 'add-credential-modal':
            return `
                <div class="modal-content">
                    <div class="modal-header">
                        <h2>Request New Credential</h2>
                        <button class="modal-close">&times;</button>
                    </div>
                    <div class="modal-body">
                        <form id="add-credential-form" class="auth-form">
                            <div class="form-group">
                                <label for="credential-type">Credential Type</label>
                                <select id="credential-type" class="form-input" required>
                                    <option value="">Select credential type</option>
                                    <option value="degree">Degree</option>
                                    <option value="certificate">Certificate</option>
                                    <option value="license">License</option>
                                    <option value="transcript">Transcript</option>
                                    <option value="award">Award</option>
                                </select>
                            </div>
                            <div class="form-group">
                                <label for="institution-email">Institution Email</label>
                                <input type="email" id="institution-email" class="form-input" 
                                       placeholder="institution@example.com" required>
                            </div>
                            <div class="form-group">
                                <label for="credential-title">Title</label>
                                <input type="text" id="credential-title" class="form-input" 
                                       placeholder="Bachelor of Science in Computer Science" required>
                            </div>
                            <div class="form-group">
                                <label for="request-message">Message to Institution</label>
                                <textarea id="request-message" class="form-input" rows="4" 
                                          placeholder="Please provide details about this credential request..."></textarea>
                            </div>
                            <button type="submit" class="btn btn-primary btn-lg btn-block">
                                <i class="fas fa-paper-plane"></i> Send Request
                            </button>
                        </form>
                    </div>
                </div>
            `;
        
        case 'issue-credential-modal':
            return `
                <div class="modal-content">
                    <div class="modal-header">
                        <h2>Issue Credential</h2>
                        <button class="modal-close">&times;</button>
                    </div>
                    <div class="modal-body">
                        <form id="issue-credential-form" class="auth-form">
                            <div class="form-group">
                                <label for="recipient-email">Recipient Email</label>
                                <input type="email" id="recipient-email" name="recipient-email" class="form-input" 
                                       placeholder="recipient@example.com" required>
                            </div>
                            <div class="form-group">
                                <label for="issue-credential-type">Credential Type</label>
                                <select id="issue-credential-type" name="issue-credential-type" class="form-input" required>
                                    <option value="">Select credential type</option>
                                    <option value="degree">Degree</option>
                                    <option value="certificate">Certificate</option>
                                    <option value="license">License</option>
                                    <option value="transcript">Transcript</option>
                                    <option value="award">Award</option>
                                </select>
                            </div>
                            <div class="form-group">
                                <label for="issue-title">Title</label>
                                <input type="text" id="issue-title" name="issue-title" class="form-input" 
                                       placeholder="Bachelor of Science in Computer Science" required>
                            </div>
                            <div class="form-group">
                                <label for="issue-description">Description</label>
                                <textarea id="issue-description" name="issue-description" class="form-input" rows="3" 
                                          placeholder="Detailed description of the credential..." required></textarea>
                            </div>
                            <div class="form-group">
                                <label for="issue-date">Issue Date</label>
                                <input type="date" id="issue-date" name="issue-date" class="form-input" required>
                            </div>
                            <button type="submit" class="btn btn-primary btn-lg btn-block">
                                <i class="fas fa-certificate"></i> Issue Credential
                            </button>
                        </form>
                    </div>
                </div>
            `;
        
        case 'search-candidates-modal':
            return `
                <div class="modal-content">
                    <div class="modal-header">
                        <h2>Search Candidates</h2>
                        <button class="modal-close">&times;</button>
                    </div>
                    <div class="modal-body">
                        <form id="search-candidates-form" class="auth-form">
                            <div class="form-group">
                                <label for="search-skills">Skills</label>
                                <input type="text" id="search-skills" class="form-input" 
                                       placeholder="e.g., JavaScript, Python, React" required>
                            </div>
                            <div class="form-group">
                                <label for="search-location">Location</label>
                                <input type="text" id="search-location" class="form-input" 
                                       placeholder="e.g., Nairobi, Lagos, Cape Town">
                            </div>
                            <div class="form-group">
                                <label for="search-credential-type">Credential Type</label>
                                <select id="search-credential-type" class="form-input">
                                    <option value="">Any type</option>
                                    <option value="degree">Degree</option>
                                    <option value="certificate">Certificate</option>
                                    <option value="license">License</option>
                                </select>
                            </div>
                            <button type="submit" class="btn btn-primary btn-lg btn-block">
                                <i class="fas fa-search"></i> Search Candidates
                            </button>
                        </form>
                        <div id="search-results" style="margin-top: var(--spacing-xl); display: none;">
                            <h3>Search Results</h3>
                            <div id="candidates-list"></div>
                        </div>
                    </div>
                </div>
            `;
        
        default:
            return `
                <div class="modal-content">
                    <div class="modal-header">
                        <h2>Feature Coming Soon</h2>
                        <button class="modal-close">&times;</button>
                    </div>
                    <div class="modal-body">
                        <p>This feature is currently under development.</p>
                        <button onclick="closeModal()" class="btn btn-primary">Close</button>
                    </div>
                </div>
            `;
    }
}

/**
 * Handle add credential form submission
 */
async function handleAddCredential(e) {
    e.preventDefault();
    
    const formData = new FormData(e.target);
    const credentialData = {
        credential_type: formData.get('credential-type'),
        institution_email: formData.get('institution-email'),
        title: formData.get('credential-title'),
        message: formData.get('request-message')
    };
    
    setLoading(true, 'Sending credential request...');
    
    try {
        await apiRequest('/credentials/request', {
            method: 'POST',
            body: JSON.stringify(credentialData)
        });
        
        showToast('Credential request sent successfully!', 'success');
        closeModal();
        // Reload credentials
        await loadCredentials();
    } catch (error) {
        showToast(error.message || 'Failed to send credential request', 'error');
    } finally {
        setLoading(false);
    }
}

/**
 * Handle issue credential form submission
 */
async function handleIssueCredential(e) {
    e.preventDefault();
    
    const formData = new FormData(e.target);
    const credentialData = {
        holder_email: String(formData.get('recipient-email')).trim(),
        credential_type: String(formData.get('issue-credential-type')).trim(),
        title: String(formData.get('issue-title')).trim(),
        description: String(formData.get('issue-description')).trim(),
        issue_date: formData.get('issue-date') + 'T00:00:00Z',
        expiry_date: null,
        metadata: null,
        document_data: btoa('Sample credential document content') // Base64 encoded placeholder
    };
    
    setLoading(true, 'Issuing credential...');
    
    try {
        const response = await apiRequest('/credentials/issue', {
            method: 'POST',
            body: JSON.stringify(credentialData)
        });
        
        // Show success modal with QR code
        showCredentialIssuedModal(response);
        
        showToast('Credential issued successfully!', 'success');
        closeModal();
        // Reload institution data
        await loadInstitutionData();
    } catch (error) {
        showToast(error.message || 'Failed to issue credential', 'error');
    } finally {
        setLoading(false);
    }
}

/**
 * Handle search candidates form submission
 */
async function handleSearchCandidates(e) {
    e.preventDefault();
    
    const formData = new FormData(e.target);
    const searchData = {
        skills: formData.get('search-skills'),
        location: formData.get('search-location'),
        credential_type: formData.get('search-credential-type')
    };
    
    setLoading(true, 'Searching candidates...');
    
    try {
        const response = await apiRequest('/candidates/search', {
            method: 'POST',
            body: JSON.stringify(searchData)
        });
        
        renderSearchResults(response.candidates || []);
        document.getElementById('search-results').style.display = 'block';
    } catch (error) {
        console.error('Search failed:', error);
        // Use mock data on API failure
        const mockCandidates = [
            {
                id: 'mock-cand-1',
                name: 'Sarah Johnson',
                location: 'Nairobi, Kenya',
                skills: ['JavaScript', 'React', 'Node.js', 'Python'],
                credentials: [
                    {
                        title: 'Full Stack Developer Certification',
                        issuer_name: 'Tech Academy Nairobi',
                        issued_at: '2024-08-15T00:00:00Z'
                    }
                ]
            },
            {
                id: 'mock-cand-2',
                name: 'Michael Oduya',
                location: 'Lagos, Nigeria',
                skills: ['Python', 'Django', 'PostgreSQL', 'AWS'],
                credentials: [
                    {
                        title: 'AWS Certified Developer',
                        issuer_name: 'Amazon Web Services',
                        issued_at: '2024-07-20T00:00:00Z'
                    }
                ]
            },
            {
                id: 'mock-cand-3',
                name: 'Grace Nkosi',
                location: 'Cape Town, South Africa',
                skills: ['Java', 'Spring Boot', 'MySQL', 'Docker'],
                credentials: [
                    {
                        title: 'Software Engineering Degree',
                        issuer_name: 'University of Cape Town',
                        issued_at: '2023-12-10T00:00:00Z'
                    }
                ]
            }
        ];
        
        renderSearchResults(mockCandidates);
        document.getElementById('search-results').style.display = 'block';
        showToast('Using demo data - API not available', 'info');
    } finally {
        setLoading(false);
    }
}

/**
 * Render search results
 */
function renderSearchResults(candidates) {
    const container = document.getElementById('candidates-list');
    if (!container) return;
    
    if (candidates.length === 0) {
        container.innerHTML = `
            <div class="empty-state">
                <i class="fas fa-users"></i>
                <h3>No candidates found</h3>
                <p>Try adjusting your search criteria</p>
            </div>
        `;
        return;
    }
    
    container.innerHTML = candidates.map(candidate => `
        <div class="candidate-item">
            <div class="candidate-info">
                <h4>${candidate.name || 'Anonymous Candidate'}</h4>
                <p>${candidate.location || 'Location not specified'}</p>
                <div class="candidate-skills">
                    ${candidate.skills ? candidate.skills.map(skill => 
                        `<span class="skill-tag">${skill}</span>`
                    ).join('') : ''}
                </div>
            </div>
            <div class="candidate-actions">
                <button class="btn btn-secondary" onclick="viewCandidate('${candidate.id}')">
                    <i class="fas fa-eye"></i> View Profile
                </button>
                <button class="btn btn-primary" onclick="contactCandidate('${candidate.id}')">
                    <i class="fas fa-envelope"></i> Contact
                </button>
            </div>
        </div>
    `).join('');
}

/**
 * View candidate profile
 */
async function viewCandidate(candidateId) {
    setLoading(true, 'Loading candidate profile...');
    
    try {
        const candidate = await apiRequest(`/candidates/${candidateId}`);
        showCandidateModal(candidate);
    } catch (error) {
        console.error('Failed to load candidate profile:', error);
        // Use mock data on API failure
        const mockCandidate = {
            id: candidateId,
            name: 'Demo Candidate',
            location: 'Nairobi, Kenya',
            skills: ['JavaScript', 'React', 'Node.js', 'Python', 'AWS'],
            credentials: [
                {
                    id: 'mock-cred-1',
                    title: 'Full Stack Developer Certification',
                    issuer_name: 'Tech Academy Nairobi',
                    issued_at: '2024-08-15T00:00:00Z'
                },
                {
                    id: 'mock-cred-2',
                    title: 'AWS Certified Developer',
                    issuer_name: 'Amazon Web Services',
                    issued_at: '2024-07-20T00:00:00Z'
                }
            ]
        };
        
        showCandidateModal(mockCandidate);
        showToast('Using demo data - API not available', 'info');
    } finally {
        setLoading(false);
    }
}

/**
 * Contact candidate
 */
function contactCandidate(candidateId) {
    showModal('contact-candidate-modal');
}

/**
 * Show credential modal
 */
function showCredentialModal(credential) {
    const modal = document.createElement('div');
    modal.id = 'credential-modal';
    modal.className = 'modal-overlay';
    modal.innerHTML = `
        <div class="modal-content">
            <div class="modal-header">
                <h2>${credential.title || 'Credential Details'}</h2>
                <button class="modal-close">&times;</button>
            </div>
            <div class="modal-body">
                <div class="credential-details-modal">
                    <div class="detail-row">
                        <span class="detail-label">Type</span>
                        <span class="detail-value">${credential.credential_type}</span>
                    </div>
                    <div class="detail-row">
                        <span class="detail-label">Issuer</span>
                        <span class="detail-value">${credential.issuer_name || 'N/A'}</span>
                    </div>
                    <div class="detail-row">
                        <span class="detail-label">Issued Date</span>
                        <span class="detail-value">${formatDate(credential.issued_at)}</span>
                    </div>
                    <div class="detail-row">
                        <span class="detail-label">Status</span>
                        <span class="detail-value">
                            <span class="status-badge ${credential.status}">${credential.status}</span>
                        </span>
                    </div>
                    ${credential.description ? `
                        <div class="detail-row">
                            <span class="detail-label">Description</span>
                            <span class="detail-value">${credential.description}</span>
                        </div>
                    ` : ''}
                </div>
                <div class="modal-actions">
                    ${credential.id.startsWith('mock-') ? 
                        `<button class="btn btn-primary" disabled title="QR codes only available for real credentials">
                            <i class="fas fa-qrcode"></i> Download QR Code
                        </button>` :
                        `<button onclick="downloadQR('${credential.id}')" class="btn btn-primary">
                            <i class="fas fa-qrcode"></i> Download QR Code
                        </button>`
                    }
                    <button onclick="closeModal()" class="btn btn-secondary">Close</button>
                </div>
            </div>
        </div>
    `;
    document.body.appendChild(modal);
    
    // Add close functionality
    const closeBtn = modal.querySelector('.modal-close');
    if (closeBtn) {
        closeBtn.addEventListener('click', () => closeModal());
    }
    
    // Show modal
    modal.style.display = 'flex';
    modal.style.visibility = 'visible';
    modal.style.opacity = '1';
    document.body.style.overflow = 'hidden';
}

/**
 * Show credential issued success modal with QR code
 */
function showCredentialIssuedModal(response) {
    const modal = document.createElement('div');
    modal.id = 'credential-issued-modal';
    modal.className = 'modal-overlay';
    modal.innerHTML = `
        <div class="modal-content">
            <div class="modal-header">
                <h2><i class="fas fa-check-circle" style="color: var(--success-color);"></i> Credential Issued Successfully!</h2>
                <button class="modal-close">&times;</button>
            </div>
            <div class="modal-body">
                <div class="credential-issued-summary">
                    <div class="success-message">
                        <p>The credential has been issued and recorded on the blockchain.</p>
                    </div>
                    
                    <div class="credential-details">
                        <div class="detail-row">
                            <span class="detail-label">Credential ID</span>
                            <span class="detail-value">${response.credential_id}</span>
                        </div>
                        <div class="detail-row">
                            <span class="detail-label">IPFS Hash</span>
                            <span class="detail-value">${response.ipfs_hash.substring(0, 20)}...</span>
                        </div>
                        <div class="detail-row">
                            <span class="detail-label">Blockchain Hash</span>
                            <span class="detail-value">${response.chain_hash.substring(0, 20)}...</span>
                        </div>
                    </div>
                    
                    <div class="qr-code-section">
                        <h3>QR Code for Verification</h3>
                        <div class="qr-preview">
                            <img src="data:image/png;base64,${response.qr_code}" alt="QR Code" style="max-width: 200px; max-height: 200px;">
                        </div>
                        <p class="qr-description">This QR code contains the credential ID and can be used for instant verification.</p>
                    </div>
                    
                    <div class="modal-actions">
                        <button onclick="downloadQR('${response.credential_id}')" class="btn btn-primary">
                            <i class="fas fa-download"></i> Download QR Code
                        </button>
                        <button onclick="closeModal()" class="btn btn-secondary">Close</button>
                    </div>
                </div>
            </div>
        </div>
    `;
    document.body.appendChild(modal);
    
    // Add close functionality
    const closeBtn = modal.querySelector('.modal-close');
    if (closeBtn) {
        closeBtn.addEventListener('click', () => closeModal());
    }
    
    // Show modal
    modal.style.display = 'flex';
    modal.style.visibility = 'visible';
    modal.style.opacity = '1';
    document.body.style.overflow = 'hidden';
}

// ==================== Verification ====================

/**
 * Initialize verification tabs
 */
function initVerifyTabs() {
    const tabs = document.querySelectorAll('.tab-btn');
    const contents = document.querySelectorAll('.tab-content');
    
    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            const targetId = tab.getAttribute('data-tab');
            
            // Update tabs
            tabs.forEach(t => t.classList.remove('active'));
            tab.classList.add('active');
            
            // Update content
            contents.forEach(c => c.classList.remove('active'));
            const targetContent = document.getElementById(targetId);
            if (targetContent) {
                targetContent.classList.add('active');
                
                // Start QR scanner if switching to QR tab
                if (targetId === 'qr-scan') {
                    initQRScanner();
                } else {
                    stopQRScanner();
                }
            }
        });
    });
}

/**
 * Initialize QR code scanner
 */
function initQRScanner() {
    if (state.qrScanner) return;
    
    const qrReader = document.getElementById('qr-reader');
    if (!qrReader || typeof Html5Qrcode === 'undefined') return;
    
    state.qrScanner = new Html5Qrcode("qr-reader");
    
    state.qrScanner.start(
        { facingMode: "environment" },
        {
            fps: 10,
            qrbox: { width: 250, height: 250 }
        },
        onScanSuccess,
        onScanError
    ).catch(err => {
        console.error('QR Scanner error:', err);
        showToast('Unable to start camera', 'error');
    });
}

/**
 * Stop QR scanner
 */
function stopQRScanner() {
    if (state.qrScanner) {
        state.qrScanner.stop().then(() => {
            state.qrScanner = null;
        }).catch(console.error);
    }
}

/**
 * QR scan success handler
 */
function onScanSuccess(decodedText) {
    stopQRScanner();
    verifyCredentialById(decodedText);
}

/**
 * QR scan error handler (silent)
 */
function onScanError(error) {
    // Silent - scanning errors are normal
}

/**
 * Handle manual verification form
 */
async function handleVerifyForm(e) {
    e.preventDefault();
    
    const credentialId = document.getElementById('credential-id').value.trim();
    if (!credentialId) {
        showToast('Please enter a credential ID', 'warning');
        return;
    }
    
    verifyCredentialById(credentialId);
}

/**
 * Verify credential by ID
 */
async function verifyCredentialById(credentialId) {
    setLoading(true, 'Verifying credential...');
    
    try {
        const response = await apiRequest(`/verify/${credentialId}`);
        showVerificationResult(response, true);
    } catch (error) {
        showVerificationResult({ error: error.message }, false);
    } finally {
        setLoading(false);
    }
}

/**
 * Show verification result
 */
function showVerificationResult(data, isValid) {
    const resultDiv = document.getElementById('verification-result');
    if (!resultDiv) return;
    
    if (isValid) {
        resultDiv.className = 'verification-result success';
        resultDiv.innerHTML = `
            <div class="result-header">
                <div class="result-icon success">
                    <i class="fas fa-check-circle"></i>
                </div>
                <div class="result-info">
                    <h3>Credential Verified ✓</h3>
                    <p>This credential is authentic and valid</p>
                </div>
            </div>
            <div class="credential-details">
                <div class="detail-row">
                    <span class="detail-label">Credential ID</span>
                    <span class="detail-value">${data.credential_id}</span>
                </div>
                <div class="detail-row">
                    <span class="detail-label">Type</span>
                    <span class="detail-value">${data.credential_type || 'N/A'}</span>
                </div>
                <div class="detail-row">
                    <span class="detail-label">Issuer</span>
                    <span class="detail-value">${data.issuer_name || 'N/A'}</span>
                </div>
                <div class="detail-row">
                    <span class="detail-label">Issued Date</span>
                    <span class="detail-value">${data.issued_at ? formatDate(data.issued_at) : 'N/A'}</span>
                </div>
                <div class="detail-row">
                    <span class="detail-label">Status</span>
                    <span class="detail-value">
                        <span style="color: var(--success-color); font-weight: 600;">
                            <i class="fas fa-check"></i> Valid
                        </span>
                    </span>
                </div>
            </div>
        `;
    } else {
        resultDiv.className = 'verification-result error';
        resultDiv.innerHTML = `
            <div class="result-header">
                <div class="result-icon error">
                    <i class="fas fa-times-circle"></i>
                </div>
                <div class="result-info">
                    <h3>Verification Failed</h3>
                    <p>${data.error || 'Credential not found or invalid'}</p>
                </div>
            </div>
        `;
    }
    
    resultDiv.style.display = 'block';
    resultDiv.scrollIntoView({ behavior: 'smooth', block: 'nearest' });
}

// ==================== Auth Tabs ====================

/**
 * Toggle password visibility
 */
function togglePassword(inputId) {
    const input = document.getElementById(inputId);
    const toggleBtn = input.nextElementSibling;
    const icon = toggleBtn.querySelector('i');
    
    if (input.type === 'password') {
        input.type = 'text';
        icon.className = 'fas fa-eye-slash';
        toggleBtn.setAttribute('aria-label', 'Hide password');
    } else {
        input.type = 'password';
        icon.className = 'fas fa-eye';
        toggleBtn.setAttribute('aria-label', 'Show password');
    }
}

/**
 * Initialize auth tabs
 */
function initAuthTabs() {
    const tabs = document.querySelectorAll('.auth-tab');
    const contents = document.querySelectorAll('.auth-content');
    
    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            const targetId = tab.getAttribute('data-tab');
            
            // Update tabs
            tabs.forEach(t => t.classList.remove('active'));
            tab.classList.add('active');
            
            // Update content
            contents.forEach(c => c.classList.remove('active'));
            const targetContent = document.getElementById(targetId);
            if (targetContent) {
                targetContent.classList.add('active');
            }
        });
    });
}

/**
 * Initialize the application
 */
function init() {
    console.log('🚀 Initializing Safari Skills Passport...');
    console.log('Current DOM readyState:', document.readyState);
    
    // Check if user is already logged in
    if (loadAuthData()) {
        console.log('User already logged in');
        updateNavigation();
        showDashboard();
    }
    
    // Initialize UI components
    initMobileNav();
    initSmoothScroll();
    animateCounters();
    initVerifyTabs();
    initPasswordToggles();
    initFormEnhancements();
    
    // Event listeners
    const loginForm = document.getElementById('login-form');
    if (loginForm) {
        loginForm.addEventListener('submit', handleLogin);
    }
    
    const registerForm = document.getElementById('register-form');
    if (registerForm) {
        registerForm.addEventListener('submit', handleRegister);
    }
    
    const verifyForm = document.getElementById('verify-form');
    if (verifyForm) {
        verifyForm.addEventListener('submit', handleVerifyForm);
    }
    
    const logoutBtn = document.getElementById('logout-btn');
    if (logoutBtn) {
        logoutBtn.addEventListener('click', handleLogout);
    }
    
    const getStartedBtn = document.querySelector('.nav-auth-buttons .btn:last-child');
    console.log('Looking for sign up button:', getStartedBtn);
    if (getStartedBtn) {
        console.log('Found sign up button, no conflicting event listeners needed');
    } else {
        console.error('Sign up button not found in DOM');
    }
    
    console.log('Safari Skills Passport initialized successfully');
}

// Start the application when DOM is ready
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
} else {
    init();
}

// Global functions for inline event handlers
window.showSection = showSection;
window.viewCredential = viewCredential;
window.downloadQR = downloadQR;
window.handleLogout = handleLogout;
window.handleGetStarted = function() {
    console.log('handleGetStarted called');

    if (state.token) {
        console.log('User is logged in, showing dashboard');
        showDashboard();
    } else {
        console.log('User not logged in, showing register modal');
        showRegisterModal();
    }
};

// Role-based dashboard functions
window.addCredential = addCredential;
window.issueCredential = issueCredential;
window.verifyCredential = verifyCredential;
window.manageCredentials = manageCredentials;
window.searchCandidates = searchCandidates;
window.viewVerificationHistory = viewVerificationHistory;
window.bulkIssueCredentials = bulkIssueCredentials;
window.revokeCredential = revokeCredential;
window.viewCandidate = viewCandidate;
window.contactCandidate = contactCandidate;

// Auth functions
window.togglePassword = togglePassword;

// Show login modal
function showLoginModal() {
    const modal = document.getElementById('auth-modal');
    if (modal) {
        console.log('Opening login modal...');
        modal.style.display = 'flex';
        modal.style.visibility = 'visible';
        modal.style.opacity = '1';
        document.body.style.overflow = 'hidden'; // Prevent background scrolling

        // Update modal title
        const title = modal.querySelector('#modal-title');
        if (title) title.textContent = 'Login to Safari Skills Passport';

        // Show login content, hide register content
        const loginContent = modal.querySelector('#login-content');
        const registerContent = modal.querySelector('#register-content');

        if (loginContent) {
            loginContent.classList.add('active');
            loginContent.style.display = 'block';
            loginContent.style.opacity = '1';
            loginContent.style.transform = 'translateY(0)';
        }
        if (registerContent) {
            registerContent.classList.remove('active');
            registerContent.style.display = 'none';
        }

        // Focus management for accessibility
        const firstFocusable = modal.querySelector('#login-email');
        if (firstFocusable) {
            setTimeout(() => firstFocusable.focus(), 100);
        }

        // Add focus trap
        trapFocus(modal);

        console.log('Login modal shown successfully');
    } else {
        console.error('Auth modal not found');
    }
}

// Show register modal
function showRegisterModal() {
    const modal = document.getElementById('auth-modal');
    if (modal) {
        console.log('Opening register modal...');
        modal.style.display = 'flex';
        modal.style.visibility = 'visible';
        modal.style.opacity = '1';
        document.body.style.overflow = 'hidden'; // Prevent background scrolling

        // Update modal title
        const title = modal.querySelector('#modal-title');
        if (title) title.textContent = 'Join Safari Skills Passport';

        // Show register content, hide login content
        const loginContent = modal.querySelector('#login-content');
        const registerContent = modal.querySelector('#register-content');

        if (registerContent) {
            registerContent.classList.add('active');
            registerContent.style.display = 'block';
            registerContent.style.opacity = '1';
            registerContent.style.transform = 'translateY(0)';
        }
        if (loginContent) {
            loginContent.classList.remove('active');
            loginContent.style.display = 'none';
        }

        // Focus management for accessibility
        const firstFocusable = modal.querySelector('#register-name');
        if (firstFocusable) {
            setTimeout(() => firstFocusable.focus(), 100);
        }

        // Add focus trap
        trapFocus(modal);

        console.log('Register modal shown successfully');
    } else {
        console.error('Auth modal not found');
    }
}

// Focus trap function for accessibility
function trapFocus(container) {
    const focusableElements = container.querySelectorAll(
        'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
    );
    const firstElement = focusableElements[0];
    const lastElement = focusableElements[focusableElements.length - 1];

    function handleTabKey(e) {
        if (e.key === 'Tab') {
            if (e.shiftKey) {
                if (document.activeElement === firstElement) {
                    e.preventDefault();
                    lastElement.focus();
                }
            } else {
                if (document.activeElement === lastElement) {
                    e.preventDefault();
                    firstElement.focus();
                }
            }
        }
    }

    container.addEventListener('keydown', handleTabKey);

    // Remove listener when modal closes
    const observer = new MutationObserver((mutations) => {
        mutations.forEach((mutation) => {
            if (mutation.type === 'attributes' && mutation.attributeName === 'style') {
                if (container.style.display === 'none') {
                    container.removeEventListener('keydown', handleTabKey);
                    observer.disconnect();
                }
            }
        });
    });

    observer.observe(container, { attributes: true });
}

// Clear form errors
function clearFormErrors() {
    const formGroups = document.querySelectorAll('.auth-form .form-group');
    formGroups.forEach(group => {
        group.classList.remove('error', 'success');
        const errorMsg = group.querySelector('.form-error');
        const successMsg = group.querySelector('.form-success');
        if (errorMsg) errorMsg.style.display = 'none';
        if (successMsg) successMsg.style.display = 'none';
    });
}

// Enhanced form validation
function validateForm(form) {
    let isValid = true;
    const inputs = form.querySelectorAll('input[required], select[required]');

    inputs.forEach(input => {
        const formGroup = input.closest('.form-group');
        if (!formGroup) {
            console.warn('Input element not properly wrapped in .form-group:', input);
            return; // Skip this input
        }
        
        const errorMsg = formGroup.querySelector('.form-error');

        // Clear previous states
        formGroup.classList.remove('error', 'success');
        if (errorMsg) errorMsg.style.display = 'none';

        // Check validity
        if (!input.checkValidity()) {
            isValid = false;
            formGroup.classList.add('error');
            if (errorMsg) {
                errorMsg.textContent = getValidationMessage(input);
                errorMsg.style.display = 'block';
            }
        } else {
            formGroup.classList.add('success');
        }
    });

    return isValid;
}

// Get validation error messages
function getValidationMessage(input) {
    if (input.validity.valueMissing) {
        return `${input.previousElementSibling ? input.previousElementSibling.textContent : 'This field'} is required`;
    }
    if (input.validity.typeMismatch) {
        return input.type === 'email' ? 'Please enter a valid email address' : 'Please enter a valid value';
    }
    if (input.validity.tooShort) {
        return `Minimum ${input.minLength} characters required`;
    }
    if (input.validity.patternMismatch) {
        return 'Please enter a valid format';
    }
    return 'Please check your input';
}

// Enhanced login handler with validation
async function handleLogin(e) {
    e.preventDefault();

    const form = e.target;
    if (!validateForm(form)) {
        showToast('Please correct the errors below', 'warning');
        return;
    }

    const email = document.getElementById('login-email').value;
    const password = document.getElementById('login-password').value;

    // Add loading state
    form.classList.add('submitting');
    setLoading(true, 'Logging in...');

    try {
        const response = await apiRequest('/auth/login', {
            method: 'POST',
            body: JSON.stringify({ email, password })
        });

        saveAuthData(response.token, {
            id: response.user.id,
            email: response.user.email,
            role: response.user.role,
            name: response.user.name
        });

        showToast('Login successful!', 'success');
        
        // Close modal immediately and update navigation
        closeModal();
        updateNavigation();
        
        // Show dashboard
        showDashboard();
        
    } catch (error) {
        showToast(error.message || 'Login failed', 'error');

        // Highlight password field on auth error
        const passwordGroup = document.getElementById('login-password').closest('.form-group');
        if (passwordGroup && error.message.includes('password')) {
            passwordGroup.classList.add('error');
            const errorMsg = passwordGroup.querySelector('.form-error');
            if (errorMsg) {
                errorMsg.textContent = 'Invalid email or password';
                errorMsg.style.display = 'block';
            }
        }
    } finally {
        form.classList.remove('submitting');
        setLoading(false);
    }
}

// Enhanced register handler with validation
async function handleRegister(e) {
    e.preventDefault();

    const form = e.target;
    if (!validateForm(form)) {
        showToast('Please correct the errors below', 'warning');
        return;
    }

    // Check terms agreement
    const termsCheckbox = document.getElementById('register-terms');
    if (termsCheckbox && !termsCheckbox.checked) {
        showToast('Please agree to the Terms of Service and Privacy Policy', 'warning');
        termsCheckbox.focus();
        return;
    }

    const name = document.getElementById('register-name').value;
    const email = document.getElementById('register-email').value;
    const password = document.getElementById('register-password').value;
    const role = document.getElementById('register-role').value;

    if (!role) {
        showToast('Please select your role', 'warning');
        return;
    }

    // Add loading state
    form.classList.add('submitting');
    setLoading(true, 'Creating account...');

    try {
        const response = await apiRequest('/auth/register', {
            method: 'POST',
            body: JSON.stringify({
                email,
                password,
                role,
                name
            })
        });

        saveAuthData(response.token, {
            id: response.user.id,
            email: response.user.email,
            role: response.user.role,
            name: response.user.name
        });

        showToast('Account created successfully!', 'success');
        
        // Close modal immediately and update navigation
        closeModal();
        updateNavigation();
        
        // Show dashboard
        showDashboard();
    } catch (error) {
        showToast(error.message || 'Registration failed', 'error');

        // Handle specific errors
        if (error.message.includes('email')) {
            const emailGroup = document.getElementById('register-email').closest('.form-group');
            if (emailGroup) {
                emailGroup.classList.add('error');
                const errorMsg = emailGroup.querySelector('.form-error');
                if (errorMsg) {
                    errorMsg.textContent = 'This email is already registered';
                    errorMsg.style.display = 'block';
                }
            }
        }
    } finally {
        form.classList.remove('submitting');
        setLoading(false);
    }
}

// Initialize auth tabs with better UX
function initAuthTabs() {
    const tabs = document.querySelectorAll('.auth-tab');
    const contents = document.querySelectorAll('.auth-content');

    tabs.forEach(tab => {
        tab.addEventListener('click', () => {
            const targetId = tab.getAttribute('data-tab');

            // Update tabs
            tabs.forEach(t => {
                t.classList.remove('active');
                t.setAttribute('aria-selected', 'false');
            });
            tab.classList.add('active');
            tab.setAttribute('aria-selected', 'true');

            // Update content with animation
            contents.forEach(c => {
                c.classList.remove('active');
                c.style.opacity = '0';
                c.style.transform = 'translateY(10px)';
            });

            const targetContent = document.getElementById(targetId);
            if (targetContent) {
                targetContent.classList.add('active');
                // Trigger animation
                setTimeout(() => {
                    targetContent.style.opacity = '1';
                    targetContent.style.transform = 'translateY(0)';
                }, 50);

                // Focus first input in the active form
                const firstInput = targetContent.querySelector('input');
                if (firstInput) {
                    setTimeout(() => firstInput.focus(), 100);
                }
            }

            // Clear any form errors when switching tabs
            clearFormErrors();
        });

        // Add keyboard support
        tab.addEventListener('keydown', (e) => {
            if (e.key === 'Enter' || e.key === ' ') {
                e.preventDefault();
                tab.click();
            }
        });
    });
}

// Add password toggle functionality
function initPasswordToggles() {
    const passwordInputs = document.querySelectorAll('input[type="password"]');

    passwordInputs.forEach(input => {
        const toggle = document.createElement('button');
        toggle.type = 'button';
        toggle.className = 'password-toggle';
        toggle.setAttribute('aria-label', 'Toggle password visibility');
        toggle.innerHTML = '<i class="fas fa-eye"></i>';

        const inputGroup = input.closest('.input-with-icon');
        if (inputGroup) {
            inputGroup.appendChild(toggle);

            toggle.addEventListener('click', () => {
                const isVisible = input.type === 'text';
                input.type = isVisible ? 'password' : 'text';
                toggle.innerHTML = isVisible ?
                    '<i class="fas fa-eye"></i>' :
                    '<i class="fas fa-eye-slash"></i>';
                toggle.setAttribute('aria-label',
                    isVisible ? 'Hide password' : 'Show password');
            });
        }
    });
}

// Add form field enhancements
function initFormEnhancements() {
    // Auto-focus next field on enter
    const inputs = document.querySelectorAll('.auth-form input');
    inputs.forEach((input, index) => {
        input.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                const nextInput = inputs[index + 1];
                if (nextInput) {
                    nextInput.focus();
                } else {
                    // Submit form
                    const form = input.closest('form');
                    if (form) form.dispatchEvent(new Event('submit'));
                }
            }
        });
    });

    // Real-time validation feedback
    const requiredInputs = document.querySelectorAll('.auth-form input[required]');
    requiredInputs.forEach(input => {
        input.addEventListener('blur', () => {
            const formGroup = input.closest('.form-group');
            if (formGroup) {
                if (input.checkValidity()) {
                    formGroup.classList.remove('error');
                    formGroup.classList.add('success');
                } else {
                    formGroup.classList.remove('success');
                    formGroup.classList.add('error');
                }
            }
        });

        input.addEventListener('input', () => {
            const formGroup = input.closest('.form-group');
            if (formGroup && formGroup.classList.contains('error')) {
                if (input.checkValidity()) {
                    formGroup.classList.remove('error');
                    formGroup.classList.add('success');
                }
            }
        });
    });
}

// Close modal when clicking outside
document.addEventListener('click', function(event) {
    const modals = document.querySelectorAll('.modal-overlay');
    modals.forEach(modal => {
        if (modal && event.target === modal) {
            closeModal();
        }
    });
});

// Close modal on Escape key
document.addEventListener('keydown', function(event) {
    if (event.key === 'Escape') {
        closeModal();
    }
});

// Switch to register from modal
function switchToRegisterFromModal() {
    showRegisterModal();
}

// Switch to login from modal
function switchToLoginFromModal() {
    showLoginModal();
}
