# API Usage Examples

This document provides practical examples for using the Safari Skills Passport API.

## Setup

```bash
# Base URL
BASE_URL="http://localhost:8080"

# Save tokens
TOKEN=""
```

## 1. Register Users

### Register a Professional
```bash
curl -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john.doe@example.com",
    "password": "SecurePass123!",
    "name": "John Doe",
    "role": "professional"
  }'

# Save the token from response
TOKEN="<token-from-response>"
```

### Register an Institution
```bash
curl -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@university.edu",
    "password": "SecurePass123!",
    "name": "University Admin",
    "role": "institution"
  }'

INSTITUTION_TOKEN="<token-from-response>"
```

### Register an Employer
```bash
curl -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "hr@company.com",
    "password": "SecurePass123!",
    "name": "HR Manager",
    "role": "employer"
  }'

EMPLOYER_TOKEN="<token-from-response>"
```

## 2. Login

```bash
curl -X POST "$BASE_URL/api/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john.doe@example.com",
    "password": "SecurePass123!"
  }'
```

## 3. Register Institution

```bash
curl -X POST "$BASE_URL/api/institutions/register" \
  -H "Authorization: Bearer $INSTITUTION_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "institution_name": "University of Nairobi",
    "institution_type": "University",
    "country": "Kenya",
    "accreditation_number": "UON-ACC-2024"
  }'
```

## 4. Get My Institution

```bash
curl -X GET "$BASE_URL/api/institutions/me" \
  -H "Authorization: Bearer $INSTITUTION_TOKEN"
```

## 5. Issue Credential

First, encode a document in base64:
```bash
# Encode a PDF or image
base64 -w 0 certificate.pdf > certificate_base64.txt
DOCUMENT_DATA=$(cat certificate_base64.txt)
```

Then issue the credential:
```bash
curl -X POST "$BASE_URL/api/credentials/issue" \
  -H "Authorization: Bearer $INSTITUTION_TOKEN" \
  -H "Content-Type: application/json" \
  -d "{
    \"holder_email\": \"john.doe@example.com\",
    \"credential_type\": \"degree\",
    \"title\": \"Bachelor of Science in Computer Science\",
    \"description\": \"Graduated with First Class Honors\",
    \"issue_date\": \"2024-12-15T00:00:00Z\",
    \"expiry_date\": null,
    \"metadata\": {
      \"gpa\": \"3.9\",
      \"honors\": \"First Class\",
      \"major\": \"Computer Science\"
    },
    \"document_data\": \"$DOCUMENT_DATA\"
  }"

# Save credential_id from response
CREDENTIAL_ID="<credential-id-from-response>"
```

## 6. Verify Credential (Public - No Auth Required)

```bash
curl -X GET "$BASE_URL/api/credentials/verify/$CREDENTIAL_ID"
```

## 7. Verify QR Code

```bash
curl -X POST "$BASE_URL/api/credentials/verify-qr" \
  -H "Content-Type: application/json" \
  -d "{
    \"qr_data\": \"$CREDENTIAL_ID\"
  }"
```

## 8. Get My Credentials

```bash
curl -X GET "$BASE_URL/api/credentials/my" \
  -H "Authorization: Bearer $TOKEN"
```

## 9. Get Issued Credentials (Institution)

```bash
curl -X GET "$BASE_URL/api/credentials/issued" \
  -H "Authorization: Bearer $INSTITUTION_TOKEN"
```

## 10. Get Specific Credential

```bash
curl -X GET "$BASE_URL/api/credentials/$CREDENTIAL_ID" \
  -H "Authorization: Bearer $TOKEN"
```

## 11. Revoke Credential (Issuer Only)

```bash
curl -X POST "$BASE_URL/api/credentials/$CREDENTIAL_ID/revoke" \
  -H "Authorization: Bearer $INSTITUTION_TOKEN"
```

## 12. Health Check

```bash
curl -X GET "$BASE_URL/health"
```

## Complete Workflow Example

```bash
#!/bin/bash
BASE_URL="http://localhost:8080"

# 1. Register institution
echo "1. Registering institution..."
INST_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "admin@techuni.edu",
    "password": "SecurePass123!",
    "name": "Tech University Admin",
    "role": "institution"
  }')

INSTITUTION_TOKEN=$(echo $INST_RESPONSE | jq -r '.token')
echo "Institution token: $INSTITUTION_TOKEN"

# 2. Register institution details
echo "2. Registering institution details..."
curl -s -X POST "$BASE_URL/api/institutions/register" \
  -H "Authorization: Bearer $INSTITUTION_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "institution_name": "Tech University",
    "institution_type": "University",
    "country": "Kenya",
    "accreditation_number": "TU-ACC-2024"
  }' | jq '.'

# 3. Register professional
echo "3. Registering professional..."
PROF_RESPONSE=$(curl -s -X POST "$BASE_URL/api/auth/register" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "alice@example.com",
    "password": "SecurePass123!",
    "name": "Alice Johnson",
    "role": "professional"
  }')

PROF_TOKEN=$(echo $PROF_RESPONSE | jq -r '.token')
echo "Professional token: $PROF_TOKEN"

# 4. Issue credential (you need to manually accredit the institution first in database)
echo "4. Issuing credential..."
CRED_RESPONSE=$(curl -s -X POST "$BASE_URL/api/credentials/issue" \
  -H "Authorization: Bearer $INSTITUTION_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "holder_email": "alice@example.com",
    "credential_type": "certificate",
    "title": "Web Development Certificate",
    "description": "Completed advanced web development course",
    "issue_date": "2024-12-15T00:00:00Z",
    "expiry_date": null,
    "metadata": {
      "grade": "A",
      "course_duration": "6 months"
    },
    "document_data": "'"$(echo "Sample Certificate" | base64)"'"
  }')

CREDENTIAL_ID=$(echo $CRED_RESPONSE | jq -r '.credential_id')
echo "Credential ID: $CREDENTIAL_ID"

# 5. Verify credential
echo "5. Verifying credential..."
curl -s -X GET "$BASE_URL/api/credentials/verify/$CREDENTIAL_ID" | jq '.'

# 6. Get professional's credentials
echo "6. Getting professional's credentials..."
curl -s -X GET "$BASE_URL/api/credentials/my" \
  -H "Authorization: Bearer $PROF_TOKEN" | jq '.'

echo "Complete!"
```

## Python Example

```python
import requests
import base64
import json

BASE_URL = "http://localhost:8080"

# Register institution
def register_institution():
    response = requests.post(
        f"{BASE_URL}/api/auth/register",
        json={
            "email": "admin@university.edu",
            "password": "SecurePass123!",
            "name": "University Admin",
            "role": "institution"
        }
    )
    return response.json()

# Issue credential
def issue_credential(token, holder_email, document_path):
    with open(document_path, 'rb') as f:
        document_data = base64.b64encode(f.read()).decode('utf-8')
    
    response = requests.post(
        f"{BASE_URL}/api/credentials/issue",
        headers={
            "Authorization": f"Bearer {token}",
            "Content-Type": "application/json"
        },
        json={
            "holder_email": holder_email,
            "credential_type": "certificate",
            "title": "Professional Certificate",
            "description": "Completed professional training",
            "issue_date": "2024-12-15T00:00:00Z",
            "expiry_date": None,
            "metadata": {"level": "Advanced"},
            "document_data": document_data
        }
    )
    return response.json()

# Verify credential
def verify_credential(credential_id):
    response = requests.get(
        f"{BASE_URL}/api/credentials/verify/{credential_id}"
    )
    return response.json()

# Usage
if __name__ == "__main__":
    # Register
    result = register_institution()
    token = result['token']
    print(f"Token: {token}")
    
    # Issue credential
    # cred = issue_credential(token, "holder@example.com", "certificate.pdf")
    # print(f"Credential: {cred}")
```

## JavaScript/Node.js Example

```javascript
const axios = require('axios');
const fs = require('fs');

const BASE_URL = 'http://localhost:8080';

// Register institution
async function registerInstitution() {
  const response = await axios.post(`${BASE_URL}/api/auth/register`, {
    email: 'admin@university.edu',
    password: 'SecurePass123!',
    name: 'University Admin',
    role: 'institution'
  });
  return response.data;
}

// Issue credential
async function issueCredential(token, holderEmail, documentPath) {
  const documentBuffer = fs.readFileSync(documentPath);
  const documentData = documentBuffer.toString('base64');
  
  const response = await axios.post(
    `${BASE_URL}/api/credentials/issue`,
    {
      holder_email: holderEmail,
      credential_type: 'certificate',
      title: 'Professional Certificate',
      description: 'Completed professional training',
      issue_date: new Date().toISOString(),
      expiry_date: null,
      metadata: { level: 'Advanced' },
      document_data: documentData
    },
    {
      headers: {
        Authorization: `Bearer ${token}`,
        'Content-Type': 'application/json'
      }
    }
  );
  return response.data;
}

// Verify credential
async function verifyCredential(credentialId) {
  const response = await axios.get(
    `${BASE_URL}/api/credentials/verify/${credentialId}`
  );
  return response.data;
}

// Usage
(async () => {
  const result = await registerInstitution();
  console.log('Token:', result.token);
  
  // const cred = await issueCredential(result.token, 'holder@example.com', 'certificate.pdf');
  // console.log('Credential:', cred);
})();
```

## Error Handling

Common error responses:

```json
{
  "error": "Error message here"
}
```

HTTP Status Codes:
- `200 OK`: Success
- `400 Bad Request`: Validation error
- `401 Unauthorized`: Authentication failed
- `403 Forbidden`: Authorization failed
- `404 Not Found`: Resource not found
- `409 Conflict`: Resource already exists
- `500 Internal Server Error`: Server error
