# Architecture Overview

## System Components

### 1. Smart Contract (Anchor/Solana)

**Location**: `programs/royalty-tokenization/`

**Key Functions**:
- `initialize_project`: Creates a new royalty token project
- `buy_tokens`: Allows investors to purchase royalty tokens
- `distribute_royalties`: Distributes streaming income to the treasury
- `claim_royalties`: Allows token holders to claim their share

**Accounts**:
- `Project`: Stores project metadata and configuration
- `Mint`: SPL token mint for the royalty tokens
- `Treasury`: PDA that holds SOL for distribution

### 2. Backend API (Express/Node.js)

**Location**: `backend/`

**Key Features**:
- RESTful API for project and royalty management
- Solana/Anchor integration
- Oracle service for Spotify API integration
- Automated cron jobs for royalty distribution

**Endpoints**:
- `GET /health` - Health check
- `GET /api/projects` - List projects
- `GET /api/projects/:address` - Get project details
- `GET /api/royalties/:projectAddress` - Get distribution history
- `GET /api/royalties/claimable/:wallet/:project` - Get claimable amount
- `POST /api/oracle/distribute` - Trigger distribution
- `POST /api/oracle/spotify/fetch` - Fetch Spotify data
- `POST /api/oracle/cron/start` - Start automated distribution

### 3. Frontend (React + Vite)

**Location**: `frontend/`

**Key Features**:
- Wallet connection (Phantom, Solflare)
- Project browsing and selection
- Token purchase interface
- Royalty claiming interface
- Modern, responsive UI

## Data Flow

### Token Purchase Flow
1. User connects wallet in frontend
2. User selects project and enters purchase amount
3. Frontend calls smart contract `buy_tokens` function
4. SOL is transferred to treasury PDA
5. Tokens are minted to user's wallet

### Royalty Distribution Flow
1. Oracle service fetches streaming data (Spotify API)
2. Revenue is calculated based on streams
3. Distribution amount is calculated (royalty_percentage * revenue)
4. Backend calls smart contract `distribute_royalties` function
5. SOL is transferred to treasury PDA
6. Project's `total_distributed` is updated

### Claim Flow
1. User connects wallet and selects project
2. Frontend calculates claimable amount (proportional to holdings)
3. User clicks "Claim"
4. Frontend calls smart contract `claim_royalties` function
5. SOL is transferred from treasury to user's wallet

## Security Considerations

1. **Oracle Authorization**: Only artist or authorized oracle can distribute royalties
2. **PDA Signing**: Treasury operations use PDA seeds for signing
3. **Math Safety**: All calculations use checked arithmetic to prevent overflow
4. **Input Validation**: All functions validate inputs before processing

## Future Enhancements

1. **Multi-source Royalties**: Support for Apple Music, YouTube, etc.
2. **Secondary Market**: Token trading on DEX
3. **Governance**: Token holder voting on project decisions
4. **Analytics Dashboard**: Detailed revenue and distribution analytics
5. **Automated Oracle**: Chainlink or Pyth integration for price feeds
