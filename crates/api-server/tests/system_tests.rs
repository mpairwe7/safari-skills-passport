// System tests - End-to-end tests with containerized services
// Run with: BASE_URL=http://localhost:8082 cargo test --package api-server --test system_tests

use serde_json::json;
use std::env;

const DEFAULT_BASE_URL: &str = "http://localhost:8080";

fn get_base_url() -> String {
    env::var("BASE_URL").unwrap_or_else(|_| DEFAULT_BASE_URL.to_string())
}

#[tokio::test]
async fn test_e2e_complete_workflow() {
    let base_url = get_base_url();
    let client = reqwest::Client::new();

    // Step 1: Health check
    println!("🔍 Step 1: Health check");
    let health_response = client
        .get(format!("{}/health", base_url))
        .send()
        .await
        .expect("Health check failed");
    
    assert_eq!(health_response.status(), 200);
    println!("✅ Health check passed");

    // Step 2: Register professional
    println!("👤 Step 2: Register professional");
    let professional_email = format!("professional_{}@example.com", uuid::Uuid::new_v4());
    let prof_register = json!({
        "email": professional_email,
        "password": "SecurePass123!",
        "name": "Jane Doe",
        "role": "professional"
    });

    let prof_response = client
        .post(format!("{}/api/auth/register", base_url))
        .json(&prof_register)
        .send()
        .await
        .expect("Professional registration failed");

    assert_eq!(prof_response.status(), 200);
    let prof_data: serde_json::Value = prof_response.json().await.unwrap();
    let prof_token = prof_data["token"].as_str().unwrap();
    println!("✅ Professional registered: {}", professional_email);

    // Step 3: Register institution
    println!("🏛️ Step 3: Register institution");
    let institution_email = format!("institution_{}@example.com", uuid::Uuid::new_v4());
    let inst_register = json!({
        "email": institution_email,
        "password": "SecurePass123!",
        "name": "Test University",
        "role": "institution"
    });

    let inst_response = client
        .post(format!("{}/api/auth/register", base_url))
        .json(&inst_register)
        .send()
        .await
        .expect("Institution registration failed");

    assert_eq!(inst_response.status(), 200);
    let inst_data: serde_json::Value = inst_response.json().await.unwrap();
    let inst_token = inst_data["token"].as_str().unwrap();
    println!("✅ Institution registered: {}", institution_email);

    // Step 4: Register institution details
    println!("📋 Step 4: Register institution details");
    let inst_details = json!({
        "institution_name": "E2E Test University",
        "institution_type": "University",
        "country": "Kenya",
        "accreditation_number": "E2E-ACC-2024"
    });

    let inst_details_response = client
        .post(format!("{}/api/institutions/register", base_url))
        .header("Authorization", format!("Bearer {}", inst_token))
        .json(&inst_details)
        .send()
        .await
        .expect("Institution details registration failed");

    assert_eq!(inst_details_response.status(), 200);
    println!("✅ Institution details registered");

    // Step 5: Get my institution
    println!("🔍 Step 5: Get institution details");
    let get_inst_response = client
        .get(format!("{}/api/institutions/me", base_url))
        .header("Authorization", format!("Bearer {}", inst_token))
        .send()
        .await
        .expect("Failed to get institution");

    assert_eq!(get_inst_response.status(), 200);
    let inst_info: serde_json::Value = get_inst_response.json().await.unwrap();
    assert_eq!(inst_info["institution_name"], "E2E Test University");
    assert_eq!(inst_info["is_accredited"], false);
    println!("✅ Institution details retrieved");

    // Step 6: Register employer
    println!("💼 Step 6: Register employer");
    let employer_email = format!("employer_{}@example.com", uuid::Uuid::new_v4());
    let emp_register = json!({
        "email": employer_email,
        "password": "SecurePass123!",
        "name": "HR Manager",
        "role": "employer"
    });

    let emp_response = client
        .post(format!("{}/api/auth/register", base_url))
        .json(&emp_register)
        .send()
        .await
        .expect("Employer registration failed");

    assert_eq!(emp_response.status(), 200);
    let emp_data: serde_json::Value = emp_response.json().await.unwrap();
    let _emp_token = emp_data["token"].as_str().unwrap();
    println!("✅ Employer registered: {}", employer_email);

    // Step 7: Login as professional
    println!("🔐 Step 7: Test login");
    let login_payload = json!({
        "email": professional_email,
        "password": "SecurePass123!"
    });

    let login_response = client
        .post(format!("{}/api/auth/login", base_url))
        .json(&login_payload)
        .send()
        .await
        .expect("Login failed");

    assert_eq!(login_response.status(), 200);
    println!("✅ Login successful");

    // Step 8: Get my credentials (should be empty)
    println!("📜 Step 8: Get credentials (should be empty)");
    let creds_response = client
        .get(format!("{}/api/credentials/my", base_url))
        .header("Authorization", format!("Bearer {}", prof_token))
        .send()
        .await
        .expect("Failed to get credentials");

    assert_eq!(creds_response.status(), 200);
    let creds_data: serde_json::Value = creds_response.json().await.unwrap();
    assert_eq!(creds_data["total"], 0);
    println!("✅ No credentials found (as expected)");

    // Note: We cannot issue credentials without accrediting the institution
    // In a real test, you would need to manually accredit or have an admin endpoint

    println!("🎉 End-to-end workflow test completed successfully!");
}

#[tokio::test]
async fn test_authentication_flow() {
    let base_url = get_base_url();
    let client = reqwest::Client::new();

    // Register user
    let email = format!("auth_test_{}@example.com", uuid::Uuid::new_v4());
    let register_payload = json!({
        "email": email,
        "password": "TestPass123!",
        "name": "Auth Test User",
        "role": "professional"
    });

    let register_response = client
        .post(format!("{}/api/auth/register", base_url))
        .json(&register_payload)
        .send()
        .await
        .expect("Registration failed");

    assert_eq!(register_response.status(), 200);
    let register_data: serde_json::Value = register_response.json().await.unwrap();
    let token = register_data["token"].as_str().unwrap();

    // Test protected endpoint with valid token
    let protected_response = client
        .get(format!("{}/api/credentials/my", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Protected endpoint request failed");

    assert_eq!(protected_response.status(), 200);

    // Test protected endpoint without token
    let no_token_response = client
        .get(format!("{}/api/credentials/my", base_url))
        .send()
        .await
        .expect("No token request failed");

    assert_eq!(no_token_response.status(), 401);

    // Test protected endpoint with invalid token
    let invalid_token_response = client
        .get(format!("{}/api/credentials/my", base_url))
        .header("Authorization", "Bearer invalid_token_here")
        .send()
        .await
        .expect("Invalid token request failed");

    assert_eq!(invalid_token_response.status(), 401);

    println!("✅ Authentication flow test passed");
}

#[tokio::test]
async fn test_credential_verification_public_endpoint() {
    let base_url = get_base_url();
    let client = reqwest::Client::new();

    // Test verification of non-existent credential (public endpoint - no auth required)
    let verify_response = client
        .get(format!("{}/api/credentials/verify/SSP-nonexistent-credential", base_url))
        .send()
        .await
        .expect("Verification request failed");

    assert_eq!(verify_response.status(), 200);
    let verify_data: serde_json::Value = verify_response.json().await.unwrap();
    assert_eq!(verify_data["valid"], false);
    assert_eq!(verify_data["message"], "Credential not found");

    println!("✅ Credential verification test passed");
}

#[tokio::test]
async fn test_role_based_access_control() {
    let base_url = get_base_url();
    let client = reqwest::Client::new();

    // Register professional
    let professional_email = format!("rbac_prof_{}@example.com", uuid::Uuid::new_v4());
    let prof_register = json!({
        "email": professional_email,
        "password": "SecurePass123!",
        "name": "RBAC Test Professional",
        "role": "professional"
    });

    let prof_response = client
        .post(format!("{}/api/auth/register", base_url))
        .json(&prof_register)
        .send()
        .await
        .expect("Professional registration failed");

    let prof_data: serde_json::Value = prof_response.json().await.unwrap();
    let prof_token = prof_data["token"].as_str().unwrap();

    // Try to register institution details as professional (should fail)
    let inst_details = json!({
        "institution_name": "Should Fail University",
        "institution_type": "University",
        "country": "Kenya",
        "accreditation_number": "FAIL-ACC-2024"
    });

    let inst_details_response = client
        .post(format!("{}/api/institutions/register", base_url))
        .header("Authorization", format!("Bearer {}", prof_token))
        .json(&inst_details)
        .send()
        .await
        .expect("Institution registration request failed");

    assert_eq!(inst_details_response.status(), 403);

    // Try to get issued credentials as professional (should fail)
    let issued_creds_response = client
        .get(format!("{}/api/credentials/issued", base_url))
        .header("Authorization", format!("Bearer {}", prof_token))
        .send()
        .await
        .expect("Issued credentials request failed");

    assert_eq!(issued_creds_response.status(), 403);

    println!("✅ Role-based access control test passed");
}

#[tokio::test]
async fn test_qr_code_verification() {
    let base_url = get_base_url();
    let client = reqwest::Client::new();

    // Test QR code verification with non-existent credential
    let qr_payload = json!({
        "qr_data": "SSP-nonexistent-qr-credential"
    });

    let qr_response = client
        .post(format!("{}/api/credentials/verify-qr", base_url))
        .json(&qr_payload)
        .send()
        .await
        .expect("QR verification request failed");

    assert_eq!(qr_response.status(), 200);
    let qr_data: serde_json::Value = qr_response.json().await.unwrap();
    assert_eq!(qr_data["valid"], false);

    println!("✅ QR code verification test passed");
}
