# Architecture Overview

## System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Client Layer                            │
│  (Mobile Apps, Web Apps, QR Scanners, Employer Portals)       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ↓ HTTPS/REST API
┌─────────────────────────────────────────────────────────────────┐
│                      API Server (Axum)                          │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐         │
│  │  Auth Layer  │  │  Handlers    │  │  Middleware  │         │
│  │  (JWT)       │  │  (Routes)    │  │  (CORS, etc) │         │
│  └──────────────┘  └──────────────┘  └──────────────┘         │
└─────────────────────────────────────────────────────────────────┘
                              │
                    ┌─────────┼─────────┐
                    ↓         ↓         ↓
        ┌───────────────┐  ┌────────────────┐  ┌──────────────┐
        │   Services    │  │   Database     │  │  Blockchain  │
        │   Layer       │  │   Layer        │  │  Layer       │
        │               │  │                │  │              │
        │ • Auth        │  │ PostgreSQL     │  │ Substrate    │
        │ • Credential  │  │ Repositories   │  │ Pallets      │
        │ • IPFS        │  │ Migrations     │  │              │
        │ • Blockchain  │  │                │  │ • Users      │
        └───────────────┘  └────────────────┘  │ • Institutions│
                                                │ • Credentials│
                                                └──────────────┘
                    │                   │
                    ↓                   ↓
        ┌───────────────┐  ┌────────────────┐
        │   IPFS Node   │  │  Substrate Node│
        │  (Off-chain   │  │  (On-chain     │
        │   Storage)    │  │   Records)     │
        └───────────────┘  └────────────────┘
```

## Data Flow

### Credential Issuance Flow

```
1. Institution Issues Credential
   ↓
2. API Server Validates Institution Accreditation
   ↓
3. Document Uploaded to IPFS → Returns Hash
   ↓
4. Credential Hash Recorded on Blockchain
   ↓
5. Metadata Stored in PostgreSQL
   ↓
6. QR Code Generated
   ↓
7. Credential ID Returned to Institution
```

### Credential Verification Flow

```
1. Employer Scans QR Code / Enters Credential ID
   ↓
2. API Server Queries PostgreSQL for Metadata
   ↓
3. Verifies Hash on Blockchain
   ↓
4. Checks Credential Status (Issued/Revoked)
   ↓
5. Returns Verification Result with Details
```

## Component Details

### 1. Common Crate
**Purpose**: Shared types, models, and error handling

**Key Components**:
- Data models (User, Institution, Credential)
- DTOs (Request/Response objects)
- Error types (AppError, AppResult)
- Enums (UserRole, CredentialType, CredentialStatus)

### 2. Database Crate
**Purpose**: PostgreSQL data persistence

**Key Components**:
- Repository pattern implementation
- Database models
- SQL migrations
- Connection pooling

**Tables**:
- `users`: User accounts with wallet addresses
- `institutions`: Institution details and accreditation
- `credentials`: Credential metadata and references

### 3. Blockchain Crate
**Purpose**: Substrate pallets for on-chain logic

**Pallets**:
- **Users Pallet**: User registration, wallet management, verification
- **Institutions Pallet**: Institution registration, accreditation
- **Credentials Pallet**: Credential issuance, revocation, verification

**Storage**:
- Users: `AccountId → User`
- Institutions: `AccountId → Institution`
- Credentials: `CredentialId → Credential`

### 4. API Server Crate
**Purpose**: REST API and business logic

**Modules**:
- **Handlers**: HTTP request handlers for each endpoint
- **Services**: Business logic (Auth, Credential, IPFS, Blockchain)
- **Middleware**: JWT authentication
- **Utils**: Helper functions (QR code generation)

## Security Architecture

### Authentication & Authorization

```
1. User Registration
   ↓
   Password → bcrypt → Hash → Database
   ↓
   Generate Substrate Wallet Address
   ↓
   Create User Record

2. User Login
   ↓
   Verify Password → bcrypt
   ↓
   Generate JWT Token (24h expiry)
   ↓
   Return Token to Client

3. Authenticated Requests
   ↓
   Extract JWT from Header
   ↓
   Verify Signature
   ↓
   Extract User ID & Role
   ↓
   Check Authorization
   ↓
   Process Request
```

### Data Security Layers

1. **Transport Layer**: HTTPS (in production)
2. **Authentication**: JWT tokens with expiry
3. **Authorization**: Role-based access control
4. **Data Encryption**: Password hashing with bcrypt
5. **Blockchain**: Immutable credential hashes
6. **IPFS**: Content-addressed storage

## Scalability Considerations

### Horizontal Scaling
- **API Servers**: Stateless design allows multiple instances
- **Load Balancer**: Distribute requests across servers
- **Database**: Connection pooling, read replicas
- **IPFS**: Clustered nodes for redundancy

### Vertical Scaling
- **Increase server resources**: CPU, RAM for API servers
- **Database optimization**: Indexes, query optimization
- **Caching**: Redis for frequently accessed data

### Performance Optimizations
- **Database indexes**: On frequently queried fields
- **Async operations**: Non-blocking I/O with Tokio
- **Connection pooling**: Reuse database connections
- **Batch operations**: Group blockchain transactions

## Technology Choices

### Why Rust?
- **Performance**: Near C++ speed, zero-cost abstractions
- **Safety**: Memory safety without garbage collection
- **Concurrency**: Fearless concurrency with async/await
- **Ecosystem**: Strong crypto and blockchain libraries

### Why Substrate?
- **Flexibility**: Custom blockchain logic via pallets
- **Upgradability**: Runtime upgrades without hard forks
- **Interoperability**: Cross-chain communication ready
- **Developer Experience**: Well-documented, active community

### Why PostgreSQL?
- **ACID compliance**: Strong consistency guarantees
- **JSONB support**: Flexible metadata storage
- **Performance**: Excellent query optimization
- **Reliability**: Battle-tested in production

### Why IPFS?
- **Decentralization**: No single point of failure
- **Content Addressing**: Immutable document storage
- **Cost Effective**: Distributed storage reduces costs
- **Global Availability**: Content accessible worldwide

### Why Axum?
- **Performance**: Built on Hyper and Tower
- **Type Safety**: Compile-time request validation
- **Ergonomics**: Clean API with extractors
- **Ecosystem**: Compatible with Tower middleware

## Deployment Architecture

### Production Setup

```
┌──────────────────────────────────────────────────────────┐
│                    Internet                              │
└──────────────────────────────────────────────────────────┘
                       │
                       ↓
┌──────────────────────────────────────────────────────────┐
│              Load Balancer (nginx/HAProxy)               │
└──────────────────────────────────────────────────────────┘
                       │
        ┌──────────────┼──────────────┐
        ↓              ↓              ↓
   ┌────────┐     ┌────────┐     ┌────────┐
   │ API 1  │     │ API 2  │     │ API 3  │
   └────────┘     └────────┘     └────────┘
        │              │              │
        └──────────────┼──────────────┘
                       │
        ┌──────────────┼──────────────┐
        ↓              ↓              ↓
   ┌──────────┐  ┌──────────┐  ┌──────────┐
   │PostgreSQL│  │   IPFS   │  │Substrate │
   │ Primary  │  │ Cluster  │  │   Node   │
   └──────────┘  └──────────┘  └──────────┘
        │
        ↓
   ┌──────────┐
   │PostgreSQL│
   │ Replica  │
   └──────────┘
```

## Monitoring & Observability

### Logging
- **Structured Logging**: JSON format with tracing
- **Log Levels**: DEBUG, INFO, WARN, ERROR
- **Correlation IDs**: Track requests across services

### Metrics
- **API Metrics**: Request count, latency, error rate
- **Database Metrics**: Connection pool, query time
- **Blockchain Metrics**: Transaction success rate
- **IPFS Metrics**: Upload/download success

### Alerting
- **High Error Rate**: > 5% errors in 5 minutes
- **High Latency**: P95 > 1000ms
- **Database Issues**: Connection pool exhaustion
- **Disk Space**: < 10% free space

## Future Architecture Enhancements

1. **Microservices**: Split into smaller services
2. **Message Queue**: RabbitMQ/Kafka for async processing
3. **Caching Layer**: Redis for session/credential cache
4. **CDN**: CloudFlare for static content
5. **GraphQL API**: Alternative to REST
6. **WebSocket**: Real-time notifications
7. **Mobile Backend**: Firebase for push notifications
8. **Analytics**: Data warehouse for insights
