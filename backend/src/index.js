import express from 'express';
import cors from 'cors';
import dotenv from 'dotenv';
import { Connection, PublicKey, Keypair } from '@solana/web3.js';
import { Program, AnchorProvider, Wallet } from '@coral-xyz/anchor';
import { royaltyRoutes } from './routes/royalty.js';
import { projectRoutes } from './routes/project.js';
import { oracleService } from './services/oracle.js';

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3001;

// Middleware
app.use(cors());
app.use(express.json());

// Solana connection
const connection = new Connection(
  process.env.SOLANA_RPC_URL || 'http://127.0.0.1:8899',
  'confirmed'
);

// Initialize Anchor provider
let provider;
let program;

try {
  // For local development, use a dummy wallet
  // In production, use a proper keypair
  const wallet = process.env.PRIVATE_KEY
    ? new Wallet(Keypair.fromSecretKey(
        Buffer.from(JSON.parse(process.env.PRIVATE_KEY))
      ))
    : new Wallet(Keypair.generate());

  provider = new AnchorProvider(connection, wallet, {
    commitment: 'confirmed',
  });

  // Load program IDL (generated after anchor build)
  // For now, we'll use a placeholder - you'll need to import the actual IDL
  console.log('Anchor provider initialized');
} catch (error) {
  console.error('Error initializing Anchor provider:', error);
}

// Routes
app.get('/health', (req, res) => {
  res.json({ status: 'ok', timestamp: new Date().toISOString() });
});

app.use('/api/projects', projectRoutes);
app.use('/api/royalties', royaltyRoutes);

// Oracle endpoint for Spotify/royalty API integration
app.post('/api/oracle/distribute', async (req, res) => {
  try {
    const { projectAddress, amount, source } = req.body;
    
    // Validate input
    if (!projectAddress || !amount || amount <= 0) {
      return res.status(400).json({ error: 'Invalid parameters' });
    }

    // In production, this would:
    // 1. Verify the oracle signature
    // 2. Call the smart contract to distribute royalties
    // 3. Log the transaction

    res.json({
      success: true,
      message: 'Royalty distribution initiated',
      projectAddress,
      amount,
      source: source || 'manual',
    });
  } catch (error) {
    console.error('Oracle distribution error:', error);
    res.status(500).json({ error: error.message });
  }
});

// Oracle endpoints
app.post('/api/oracle/spotify/fetch', async (req, res) => {
  try {
    const { artistId } = req.body;
    
    if (!artistId) {
      return res.status(400).json({ error: 'Artist ID required' });
    }

    const accessToken = await oracleService.getSpotifyAccessToken();
    const data = await oracleService.fetchSpotifyData(artistId, accessToken);

    res.json({
      success: true,
      data,
    });
  } catch (error) {
    console.error('Spotify fetch error:', error);
    res.status(500).json({ error: error.message });
  }
});

app.post('/api/oracle/cron/start', async (req, res) => {
  try {
    const { projects } = req.body;
    oracleService.startCronJob(projects || []);
    res.json({ success: true, message: 'Cron job started' });
  } catch (error) {
    console.error('Cron start error:', error);
    res.status(500).json({ error: error.message });
  }
});

// Error handling
app.use((err, req, res, next) => {
  console.error(err.stack);
  res.status(500).json({ error: 'Something went wrong!' });
});

app.listen(PORT, () => {
  console.log(`🚀 Backend server running on port ${PORT}`);
  console.log(`📡 Solana RPC: ${connection.rpcEndpoint}`);
  console.log(`🔮 Oracle service ready`);
});

export { app, connection, provider, program };
