# Safari Skills Passport - User Guide

Welcome to Safari Skills Passport! This guide will help you navigate and use all features of the platform for managing blockchain-verified credentials.

## Table of Contents

1. [Getting Started](#getting-started)
2. [User Roles](#user-roles)
3. [Account Management](#account-management)
4. [Feature Guide](#feature-guide)
5. [Credential Management](#credential-management)
6. [Verification Process](#verification-process)
7. [Best Practices](#best-practices)
8. [FAQ](#faq)

---

## Getting Started

### Creating an Account

1. **Visit the Application**
   - Open your browser and navigate to the application URL (default: `http://localhost:3000`)

2. **Click "Get Started"**
   - Located in the top-right navigation bar

3. **Select Your Role**
   - **Professional**: Individuals holding credentials and seeking employment
   - **Employer**: Companies verifying candidate credentials
   - **Institution**: Schools/organizations issuing credentials

4. **Fill Registration Form**
   - Full Name
   - Email Address
   - Password (minimum 8 characters recommended)
   - Accept Terms of Service

5. **Complete Registration**
   - Verify your email (if required)
   - Log in with your credentials

### First Login

Upon successful login, you'll be directed to your role-specific dashboard with:
- Welcome message with your name
- Quick action buttons for your role
- Statistics and overview cards
- Recent activity or notifications

---

## User Roles

### 👤 Professional

**What Professionals Can Do:**
- View their issued credentials
- Request credentials from institutions
- Share credentials with employers
- Download QR codes for credentials
- Track credential views and shares
- Update their profile information

**Dashboard Features:**
- **My Credentials**: View all issued credentials with details
- **Request Credential**: Submit requests to institutions
- **Verify Credential**: Verify credentials from other professionals
- **Credential Statistics**: View total, verified, and shared credentials
- **Activity Log**: Track credential interactions

**Typical Workflow:**
1. Receive credential notification from institution
2. Accept and store credential
3. Share QR code with employers
4. Track who viewed the credential

---

### 🏢 Employer

**What Employers Can Do:**
- Search candidate profiles by skills
- Verify candidate credentials
- View verification history
- Track verification status (pending/verified)
- Download verification reports

**Dashboard Features:**
- **Search Candidates**: Find professionals by:
  - Skills (JavaScript, Python, AWS, etc.)
  - Location
  - Credential type
- **Verification History**: View all verifications performed
- **Recent Verifications**: Quick access to latest verifications
- **Statistics**:
  - Total verifications
  - Pending verifications
  - Unique candidates verified

**Typical Workflow:**
1. Search for candidates with specific skills
2. Review candidate profiles and credentials
3. Verify credentials by scanning QR code or entering credential ID
4. Track verification results
5. Access verification history for records

---

### 🏛️ Institution

**What Institutions Can Do:**
- Issue credentials to professionals
- Manage credential templates
- Bulk issue credentials
- Track issued credentials
- View credential verification statistics
- Revoke credentials if needed

**Dashboard Features:**
- **Issue Credential**: 
  - Select recipient email
  - Choose credential type
  - Add title and description
  - Set issue date
- **Manage Credentials**: View all issued credentials
- **Institution Details**: Configure organization information
- **Statistics**:
  - Total credentials issued
  - Unique recipients
  - Verification rate
  - Credential breakdown by type

**Typical Workflow:**
1. Register institution details
2. Select graduate/professional to credential
3. Fill credential form (type, title, date)
4. Issue credential (automatically recorded on blockchain)
5. Track verifications of issued credentials
6. Revoke if credential becomes invalid

---

## Account Management

### Updating Profile

1. **Access Settings**
   - Click your name in the top-right corner
   - Select "Settings" or "Profile"

2. **Edit Information**
   - Update name, email, or phone
   - Modify bio (Professionals)
   - Change institution details (Institutions)

3. **Save Changes**
   - Click "Save" button
   - Confirm if prompted

### Security Settings

#### Changing Password
1. Go to Settings → Security
2. Enter current password
3. Enter new password (8+ characters)
4. Confirm new password
5. Click "Update Password"

#### Logout

**Single Logout:**
- Click your name → Logout

**Logout from All Devices:**
- Settings → Security → "Logout from All Devices"

#### Deactivate Account
- Settings → Account → "Delete Account"
- ⚠️ **Warning**: This action is permanent and cannot be undone

---

## Feature Guide

### Dashboard Overview

#### Statistics Cards
- **Total Credentials** (Professionals): Number of credentials held
- **Verified Credentials**: Count of verified/validated credentials
- **View Count**: Total views received
- **Pending Verifications** (Employers): Awaiting confirmation
- **Verification Rate** (Institutions): Percentage of verified issued credentials

#### Quick Actions
- **New Credential Request** (Professional)
- **Issue Credential** (Institution)
- **Search Candidates** (Employer)
- **Verify Credential** (Employer/Professional)

#### Navigation Menu
- Home
- Dashboard
- Credentials/Verifications
- Settings
- Logout

### Search Functionality

#### Credential Search (Professionals/Employers)
```
Filters available:
- Credential Type (Degree, Certificate, License, etc.)
- Issuer Name
- Issue Date Range
- Status (Verified, Pending, Revoked)
```

#### Candidate Search (Employers)
```
Search by:
- Skills (comma-separated)
- Location
- Credential Type
- Education Level
```

---

## Credential Management

### Viewing Credentials

#### For Professionals
1. Go to **My Credentials** in dashboard
2. Click on any credential to view details:
   - Title and description
   - Issuing institution
   - Issue date and expiry (if applicable)
   - Status (Verified/Pending)
   - Number of views
3. Click **View Details** for full information

#### For Employers
1. Search for candidate
2. Click **View Profile**
3. See all candidate's verified credentials
4. Click credential for detailed information

### Requesting Credentials (Professional)

1. **Click "Request Credential"** button on dashboard
2. **Fill Request Form**:
   - Select credential type from dropdown
   - Enter institution email address
   - Add credential title
   - Write message to institution (optional)
3. **Submit Request**
4. **Track Status**:
   - Pending: Awaiting institution response
   - Approved: Credential issued
   - Declined: Institution declined request

### Issuing Credentials (Institution)

1. **Click "Issue Credential"** button
2. **Enter Recipient Email** - The professional receiving the credential
3. **Select Credential Type**:
   - Degree
   - Certificate
   - License
   - Transcript
   - Award
4. **Fill Credential Details**:
   - **Title**: Name of the credential (e.g., "Bachelor of Science in CS")
   - **Description**: Detailed information about the credential
   - **Issue Date**: When the credential was earned
   - **Expiry Date** (optional): When credential expires
5. **Blockchain Recording**:
   - Credential is automatically recorded on blockchain
   - IPFS hash generated for immutability
   - QR code created automatically
6. **Success Confirmation**:
   - View credential ID
   - Download QR code
   - View blockchain hash

### Downloading QR Codes

#### For Real Credentials
1. Open credential details
2. Click **"Download QR Code"** button
3. QR code PNG file downloads
4. Use for sharing with employers

#### For Mock Credentials (Demo Data)
- QR download disabled (informational message displayed)
- Use actual issued credentials for QR functionality

### Revoking Credentials (Institution)

1. Go to **Manage Credentials**
2. Find the credential to revoke
3. Click **Revoke** button
4. **Confirm revocation**:
   - ⚠️ Cannot be undone
   - Professionals will see status as "Revoked"
5. **Done** - Credential is now revoked

---

## Verification Process

### Scanning QR Code (Employer)

1. **Go to Verify Section**
   - Dashboard → Verify Credential
   - Or: Main menu → Verify

2. **Choose Verification Method**
   - **QR Scan Tab**: Uses device camera
   - **Manual Entry Tab**: Enter credential ID manually

3. **Scan QR Code**
   - Click "Allow Camera Access" if prompted
   - Point camera at QR code
   - System automatically scans and verifies

4. **View Verification Result**
   - ✅ **Verified**: Credential is authentic
     - Shows credential details
     - Displays issuer information
     - Confirms blockchain record
   - ❌ **Failed**: Credential not found or invalid
     - Check credential ID
     - Verify QR code is readable
     - Ensure credential hasn't been revoked

### Manual Credential Verification

1. **Get Credential ID**
   - From professional or QR code label
   - Format: UUID or alphanumeric code

2. **Enter in Manual Verification**
   - Tab: "Manual Entry"
   - Input field: "Credential ID"
   - Click "Verify"

3. **Review Results**
   - Same verification result display as QR scan
   - All credential information available

### Verification Record

Employers can:
- **View History** - All verifications performed
- **Export Report** - Download verification details
- **Share Results** - Send verification to stakeholders
- **Track Trends** - See verification patterns over time

---

## Best Practices

### For Professionals

✅ **Do:**
- Keep your profile updated with current information
- Request credentials promptly after completion
- Share QR codes only with trusted employers
- Monitor who has accessed your credentials
- Update password regularly
- Review credential details for accuracy
- Report suspicious activity

❌ **Don't:**
- Share credentials publicly without filtering
- Ignore verification requests
- Use outdated credentials
- Click suspicious links in credential shares
- Share account credentials with others
- Leave account logged in on public computers

### For Employers

✅ **Do:**
- Verify credentials before hiring decisions
- Keep verification records for compliance
- Search comprehensively for candidates
- Document verification processes
- Update search criteria regularly
- Report suspicious credentials
- Use bulk verification for multiple candidates

❌ **Don't:**
- Accept unverified credentials
- Share verification results inappropriately
- Make decisions based solely on credentials
- Store login credentials insecurely
- Skip verification steps
- Delay credential checks

### For Institutions

✅ **Do:**
- Verify recipient email addresses
- Maintain accurate credential records
- Use standardized credential titles
- Document credential standards
- Track credential issuance
- Monitor credential verification rates
- Revoke compromised credentials promptly

❌ **Don't:**
- Issue unearned credentials
- Use incorrect issue dates
- Delete credential records
- Ignore revocation requests
- Share private institution data
- Compromise credential security

---

## FAQ

### General Questions

**Q: Is this platform free to use?**
A: Yes, Safari Skills Passport is open-source and free to use.

**Q: How secure are my credentials?**
A: Credentials are secured using blockchain technology and cryptographic hashing, ensuring immutability and authenticity.

**Q: Can credentials be faked?**
A: No. The blockchain recording and cryptographic verification make credential tampering virtually impossible.

**Q: How long are credentials stored?**
A: Indefinitely, unless revoked by the issuing institution.

### Professional Questions

**Q: How do I request a credential I'm missing?**
A: Go to "Request Credential" → Fill form with institution email → Submit. The institution will receive and process your request.

**Q: Can I share my credentials privately?**
A: Yes, only share QR codes with people you trust. The QR code contains your credential ID.

**Q: What if I don't receive a requested credential?**
A: Follow up with the institution directly. Some requests may take time to process.

**Q: How many credentials can I hold?**
A: Unlimited. You can collect credentials throughout your career.

### Employer Questions

**Q: What information do I see when verifying?**
A: Credential type, issuer, issue date, status, and blockchain verification details.

**Q: Can I verify expired credentials?**
A: Yes, but they'll be marked as expired. Use your judgment for hiring.

**Q: How accurate are the verifications?**
A: Very accurate. The blockchain ensures credentials are genuine and unmodified.

**Q: Can I export verification reports?**
A: Yes, reports can be downloaded for record-keeping and compliance.

### Institution Questions

**Q: Can I bulk issue credentials?**
A: Yes, use the bulk issue feature to issue to multiple recipients at once.

**Q: What if I issued the wrong credential?**
A: Revoke it immediately. Contact the recipient to reissue the correct one.

**Q: How do I track my issued credentials?**
A: Use "Manage Credentials" to view all issued credentials with verification stats.

**Q: Can revoked credentials be re-issued?**
A: Yes, you can issue a new credential to the same recipient.

### Technical Questions

**Q: What browser should I use?**
A: Chrome, Firefox, Safari, or Edge (all modern versions). JavaScript must be enabled.

**Q: What if I forget my password?**
A: Click "Forgot Password" on login page and follow email instructions.

**Q: Can I use the same email for multiple accounts?**
A: No, one email = one account. Create a separate account for each role.

**Q: Is there an offline mode?**
A: No, you need internet connection to access the platform.

**Q: How do I delete my account?**
A: Settings → Account → Delete Account. ⚠️ This is permanent.

---

## Troubleshooting

### Can't Login
- Verify email and password
- Check caps lock
- Clear browser cookies: Settings → Clear Browsing Data
- Try different browser
- Reset password if needed

### Page Won't Load
- Check internet connection
- Refresh page (Ctrl+R or Cmd+R)
- Clear browser cache
- Disable browser extensions
- Check server status

### QR Code Won't Scan
- Ensure camera access is allowed
- Clean camera lens
- Increase lighting
- Hold phone steady
- Try manual entry instead

### Credential Not Found
- Verify credential ID is correct
- Check credential hasn't been revoked
- Ensure QR code is readable
- Try refreshing page
- Contact credential issuer

### Slow Performance
- Check internet speed
- Clear browser cache
- Close unnecessary tabs
- Restart browser
- Try during off-peak hours

---

## Getting Help

- **Technical Issues**: Contact system administrator
- **Bug Reports**: Open issue on GitHub
- **Feature Requests**: Submit on GitHub Discussions
- **General Questions**: Check FAQ or documentation
- **Security Issues**: Report privately to project maintainers

---

## Additional Resources

- **Installation Guide**: See `INSTALLATION.md`
- **Architecture**: See `ARCHITECTURE.md`
- **API Documentation**: See `API_EXAMPLES.md`
- **Project Status**: See `PROJECT_COMPLETE.md`
- **Quick Reference**: See `DEMO_REFERENCE_CARD.md`

---

## Tips & Tricks

### For Power Users

**Keyboard Shortcuts:**
- `Esc` - Close modal dialogs
- `Ctrl+K` - Search (if available)
- `Ctrl+/` - Show help menu (if available)

**Pro Tips:**
1. Use browser bookmarks for quick access
2. Download QR codes for offline reference
3. Take screenshots of important credentials
4. Subscribe to institution announcements
5. Review profile monthly for updates

---

## Support & Contact

**Project Repository:**
https://github.com/mpairwe7/safari-skills-passport

**Report Issues:**
https://github.com/mpairwe7/safari-skills-passport/issues

**Contribute:**
https://github.com/mpairwe7/safari-skills-passport/pulls

---

**Last Updated**: November 2025
**Version**: 1.0
**License**: MIT

Welcome to Safari Skills Passport! Enjoy secure, verified credential management. 🎓
