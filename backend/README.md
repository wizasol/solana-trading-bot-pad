# Backend API

Express server for the Royalty Tokenization platform.

## Setup

```bash
npm install
cp .env.example .env
# Edit .env with your configuration
```

## Environment Variables

- `PORT`: Server port (default: 3001)
- `SOLANA_RPC_URL`: Solana RPC endpoint
- `PRIVATE_KEY`: Private key for signing transactions (JSON array format)
- `PROGRAM_ID`: Anchor program ID

## API Endpoints

### Health
- `GET /health` - Health check

### Projects
- `GET /api/projects` - List all projects
- `GET /api/projects/:address` - Get project details

### Royalties
- `GET /api/royalties/:projectAddress` - Get distribution history
- `GET /api/royalties/claimable/:walletAddress/:projectAddress` - Get claimable amount

### Oracle
- `POST /api/oracle/distribute` - Trigger royalty distribution (oracle endpoint)

## Development

```bash
npm run dev
```
