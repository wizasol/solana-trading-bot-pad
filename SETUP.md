# Setup Guide

Complete setup instructions for the Royalty Tokenization Platform.

## Prerequisites

1. **Rust** (latest stable version)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Solana CLI** (v1.18+)
   ```bash
   sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
   ```

3. **Anchor Framework**
   ```bash
   cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
   avm install latest
   avm use latest
   ```

4. **Node.js** (v18+)
   ```bash
   # Using nvm (recommended)
   nvm install 18
   nvm use 18
   ```

## Project Setup

### 1. Build the Anchor Program

```bash
# Navigate to project root
cd /root/development/spotify-royalty-tokenization

# Build the program
anchor build

# Generate program ID
solana-keygen new -o target/deploy/royalty_tokenization-keypair.json

# Update Anchor.toml with the new program ID
# Update declare_id! in programs/royalty-tokenization/src/lib.rs
```

### 2. Start Local Validator

```bash
# In a separate terminal
solana-test-validator
```

### 3. Deploy the Program

```bash
# Set cluster to localnet
solana config set --url localhost

# Airdrop SOL for deployment
solana airdrop 10

# Deploy
anchor deploy
```

### 4. Setup Backend

```bash
cd backend
npm install
cp .env.example .env
# Edit .env with your configuration
npm run dev
```

### 5. Setup Frontend

```bash
cd frontend
npm install
npm run dev
```

## Environment Configuration

### Backend (.env)

```env
PORT=3001
SOLANA_RPC_URL=http://127.0.0.1:8899
PRIVATE_KEY=[Your private key as JSON array]
PROGRAM_ID=RoyTkn11111111111111111111111111111111111

# Optional: Spotify API for oracle
SPOTIFY_CLIENT_ID=your_spotify_client_id
SPOTIFY_CLIENT_SECRET=your_spotify_client_secret
```

### Frontend (.env)

```env
VITE_SOLANA_NETWORK=devnet
VITE_RPC_URL=https://api.devnet.solana.com
VITE_BACKEND_URL=http://localhost:3001
```

## Testing

### Test the Smart Contract

```bash
anchor test
```

### Test the Backend

```bash
cd backend
curl http://localhost:3001/health
```

### Test the Frontend

Open `http://localhost:5173` in your browser and connect a wallet.

## Production Deployment

### Deploy to Devnet

```bash
solana config set --url devnet
solana airdrop 2
anchor deploy
```

### Deploy to Mainnet

```bash
solana config set --url mainnet-beta
anchor deploy
```

## Troubleshooting

### Common Issues

1. **Anchor build fails**: Make sure Rust and Anchor are properly installed
2. **Deployment fails**: Ensure you have enough SOL in your wallet
3. **Backend connection errors**: Check that the local validator is running
4. **Frontend wallet connection**: Make sure you're using a compatible wallet (Phantom, Solflare)

## Next Steps

- Integrate with actual Spotify API
- Add more royalty sources (Apple Music, YouTube, etc.)
- Implement project creation UI
- Add transaction history
- Set up monitoring and alerts
