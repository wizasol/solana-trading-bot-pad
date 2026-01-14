import React, { useState, useEffect } from 'react';
import { useWallet, useConnection } from '@solana/wallet-adapter-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';
import { PublicKey } from '@solana/web3.js';
import axios from 'axios';
import './App.css';

function App() {
  const { publicKey, connected } = useWallet();
  const { connection } = useConnection();
  const [projects, setProjects] = useState([]);
  const [selectedProject, setSelectedProject] = useState(null);
  const [buyAmount, setBuyAmount] = useState('');
  const [claimableAmount, setClaimableAmount] = useState(0);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (connected && publicKey) {
      fetchProjects();
      if (selectedProject) {
        fetchClaimableAmount();
      }
    }
  }, [connected, publicKey, selectedProject]);

  const fetchProjects = async () => {
    try {
      const response = await axios.get('/api/projects');
      setProjects(response.data.projects || []);
    } catch (error) {
      console.error('Error fetching projects:', error);
    }
  };

  const fetchClaimableAmount = async () => {
    if (!publicKey || !selectedProject) return;
    try {
      const response = await axios.get(
        `/api/royalties/claimable/${publicKey.toString()}/${selectedProject.address}`
      );
      setClaimableAmount(response.data.claimableAmount || 0);
    } catch (error) {
      console.error('Error fetching claimable amount:', error);
    }
  };

  const handleBuyTokens = async () => {
    if (!connected || !publicKey || !buyAmount) {
      alert('Please connect wallet and enter amount');
      return;
    }

    setLoading(true);
    try {
      // In production, this would call the smart contract
      alert(`Buying tokens: ${buyAmount} SOL`);
      // TODO: Implement actual smart contract call
    } catch (error) {
      console.error('Error buying tokens:', error);
      alert('Error buying tokens: ' + error.message);
    } finally {
      setLoading(false);
    }
  };

  const handleClaimRoyalties = async () => {
    if (!connected || !publicKey || !selectedProject) {
      alert('Please connect wallet and select a project');
      return;
    }

    setLoading(true);
    try {
      // In production, this would call the smart contract
      alert('Claiming royalties...');
      // TODO: Implement actual smart contract call
    } catch (error) {
      console.error('Error claiming royalties:', error);
      alert('Error claiming royalties: ' + error.message);
    } finally {
      setLoading(false);
    }
  };

  const handleCreateProject = () => {
    alert('Project creation feature coming soon!');
  };

  return (
    <div className="app">
      <header className="app-header">
        <h1>🎵 Royalty Tokenization Platform</h1>
        <p>Tokenize your music royalties on Solana</p>
        <WalletMultiButton />
      </header>

      <main className="app-main">
        {!connected ? (
          <div className="connect-prompt">
            <h2>Connect Your Wallet</h2>
            <p>Connect a Solana wallet to get started</p>
          </div>
        ) : (
          <>
            <section className="projects-section">
              <div className="section-header">
                <h2>Royalty Projects</h2>
                <button onClick={handleCreateProject} className="btn-primary">
                  Create Project
                </button>
              </div>

              {projects.length === 0 ? (
                <div className="empty-state">
                  <p>No projects found. Create one to get started!</p>
                </div>
              ) : (
                <div className="projects-grid">
                  {projects.map((project) => (
                    <div
                      key={project.address}
                      className={`project-card ${
                        selectedProject?.address === project.address ? 'selected' : ''
                      }`}
                      onClick={() => setSelectedProject(project)}
                    >
                      <h3>{project.name}</h3>
                      <p className="symbol">{project.symbol}</p>
                      <div className="project-info">
                        <div>
                          <span className="label">Total Supply:</span>
                          <span>{project.totalSupply?.toLocaleString()}</span>
                        </div>
                        <div>
                          <span className="label">Royalty %:</span>
                          <span>{project.royaltyPercentage}%</span>
                        </div>
                        <div>
                          <span className="label">Distributed:</span>
                          <span>{(project.totalDistributed / 1e9).toFixed(2)} SOL</span>
                        </div>
                      </div>
                    </div>
                  ))}
                </div>
              )}
            </section>

            {selectedProject && (
              <section className="actions-section">
                <h2>Actions</h2>
                <div className="action-cards">
                  <div className="action-card">
                    <h3>Buy Tokens</h3>
                    <div className="input-group">
                      <input
                        type="number"
                        placeholder="Amount (SOL)"
                        value={buyAmount}
                        onChange={(e) => setBuyAmount(e.target.value)}
                        min="0"
                        step="0.1"
                      />
                      <button
                        onClick={handleBuyTokens}
                        disabled={loading || !buyAmount}
                        className="btn-primary"
                      >
                        {loading ? 'Processing...' : 'Buy Tokens'}
                      </button>
                    </div>
                  </div>

                  <div className="action-card">
                    <h3>Claim Royalties</h3>
                    <div className="claim-info">
                      <p>
                        Claimable: <strong>{(claimableAmount / 1e9).toFixed(4)} SOL</strong>
                      </p>
                      <button
                        onClick={handleClaimRoyalties}
                        disabled={loading || claimableAmount === 0}
                        className="btn-secondary"
                      >
                        {loading ? 'Processing...' : 'Claim'}
                      </button>
                    </div>
                  </div>
                </div>
              </section>
            )}
          </>
        )}
      </main>
    </div>
  );
}

export default App;
