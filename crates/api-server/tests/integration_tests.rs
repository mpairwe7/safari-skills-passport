// Integration tests for Safari Skills Passport API
// Run with: cargo test --package api-server --test integration_tests

#[cfg(test)]
mod tests {
    use serde_json::json;
    
    // Note: These tests require a running database and server
    // In a real scenario, you'd use a test database and test fixtures
    
    fn base_url() -> String {
        std::env::var("BASE_URL").unwrap_or_else(|_| "http://localhost:8081".to_string())
    }
    
    #[tokio::test]
    async fn test_health_check() {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}/health", base_url()))
            .send()
            .await
            .expect("Failed to send request");
        
        assert_eq!(response.status(), 200);
        
        let body: serde_json::Value = response
            .json()
            .await
            .expect("Failed to parse response");
        
        assert_eq!(body["status"], "healthy");
    }
    
    #[tokio::test]
    async fn test_user_registration_and_login() {
        let client = reqwest::Client::new();
        
        // Register user
        let register_payload = json!({
            "email": format!("test_{}@example.com", uuid::Uuid::new_v4()),
            "password": "TestPass123!",
            "name": "Test User",
            "role": "professional"
        });
        
        let register_response = client
            .post(format!("{}/api/auth/register", base_url()))
            .json(&register_payload)
            .send()
            .await
            .expect("Failed to register");
        
        assert_eq!(register_response.status(), 200);
        
        let register_body: serde_json::Value = register_response
            .json()
            .await
            .expect("Failed to parse register response");
        
        assert!(register_body["token"].is_string());
        assert!(register_body["user"]["id"].is_string());
        
        // Login
        let login_payload = json!({
            "email": register_payload["email"],
            "password": "TestPass123!"
        });
        
        let login_response = client
            .post(format!("{}/api/auth/login", base_url()))
            .json(&login_payload)
            .send()
            .await
            .expect("Failed to login");
        
        assert_eq!(login_response.status(), 200);
        
        let login_body: serde_json::Value = login_response
            .json()
            .await
            .expect("Failed to parse login response");
        
        assert!(login_body["token"].is_string());
    }
    
    #[tokio::test]
    async fn test_institution_registration() {
        let client = reqwest::Client::new();
        
        // Register institution user
        let register_payload = json!({
            "email": format!("institution_{}@example.com", uuid::Uuid::new_v4()),
            "password": "TestPass123!",
            "name": "Test Institution",
            "role": "institution"
        });
        
        let register_response = client
            .post(format!("{}/api/auth/register", base_url()))
            .json(&register_payload)
            .send()
            .await
            .expect("Failed to register institution");
        
        let register_body: serde_json::Value = register_response
            .json()
            .await
            .expect("Failed to parse response");
        
        let token = register_body["token"].as_str().unwrap();
        
        // Register institution details
        let institution_payload = json!({
            "institution_name": "Test University",
            "institution_type": "University",
            "country": "Kenya",
            "accreditation_number": "TU-ACC-2024"
        });
        
        let institution_response = client
            .post(format!("{}/api/institutions/register", base_url()))
            .header("Authorization", format!("Bearer {}", token))
            .json(&institution_payload)
            .send()
            .await
            .expect("Failed to register institution details");
        
        assert_eq!(institution_response.status(), 200);
        
        let institution_body: serde_json::Value = institution_response
            .json()
            .await
            .expect("Failed to parse institution response");
        
        assert_eq!(institution_body["institution_name"], "Test University");
        assert_eq!(institution_body["is_accredited"], false);
    }
    
    #[tokio::test]
    async fn test_credential_verification_not_found() {
        let client = reqwest::Client::new();
        
        let verify_response = client
            .get(format!("{}/api/credentials/verify/SSP-nonexistent", base_url()))
            .send()
            .await
            .expect("Failed to verify credential");
        
        assert_eq!(verify_response.status(), 200);
        
        let verify_body: serde_json::Value = verify_response
            .json()
            .await
            .expect("Failed to parse verify response");
        
        assert_eq!(verify_body["valid"], false);
        assert_eq!(verify_body["message"], "Credential not found");
    }
    
    #[tokio::test]
    async fn test_unauthorized_access() {
        let client = reqwest::Client::new();
        
        // Try to access protected endpoint without token
        let response = client
            .get(format!("{}/api/credentials/my", base_url()))
            .send()
            .await
            .expect("Failed to send request");
        
        assert_eq!(response.status(), 401);
    }
    
    #[tokio::test]
    async fn test_invalid_token() {
        let client = reqwest::Client::new();
        
        let response = client
            .get(format!("{}/api/credentials/my", base_url()))
            .header("Authorization", "Bearer invalid_token")
            .send()
            .await
            .expect("Failed to send request");
        
        assert_eq!(response.status(), 401);
    }
}

// Unit tests for individual components
#[cfg(test)]
mod unit_tests {
    use common::{UserRole, CredentialType, CredentialStatus};
    
    #[test]
    fn test_user_role_serialization() {
        let role = UserRole::Professional;
        let serialized = serde_json::to_string(&role).unwrap();
        assert_eq!(serialized, "\"professional\"");
    }
    
    #[test]
    fn test_credential_type_serialization() {
        let cred_type = CredentialType::Certificate;
        let serialized = serde_json::to_string(&cred_type).unwrap();
        assert_eq!(serialized, "\"certificate\"");
    }
    
    #[test]
    fn test_credential_status_serialization() {
        let status = CredentialStatus::Issued;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, "\"issued\"");
    }
}
