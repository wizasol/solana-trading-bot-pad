import express from 'express';
import { Connection, PublicKey } from '@solana/web3.js';

const router = express.Router();

// Get project details
router.get('/:address', async (req, res) => {
  try {
    const { address } = req.params;
    
    // Validate address
    let projectPubkey;
    try {
      projectPubkey = new PublicKey(address);
    } catch (error) {
      return res.status(400).json({ error: 'Invalid project address' });
    }

    // In production, fetch from on-chain account
    // For now, return mock data
    res.json({
      address: address,
      name: 'Sample Artist Project',
      symbol: 'ART',
      totalSupply: 1000000,
      royaltyPercentage: 50,
      totalDistributed: 0,
      createdAt: new Date().toISOString(),
    });
  } catch (error) {
    console.error('Error fetching project:', error);
    res.status(500).json({ error: error.message });
  }
});

// List all projects
router.get('/', async (req, res) => {
  try {
    // In production, fetch from on-chain accounts
    res.json({
      projects: [],
      total: 0,
    });
  } catch (error) {
    console.error('Error listing projects:', error);
    res.status(500).json({ error: error.message });
  }
});

export { router as projectRoutes };
