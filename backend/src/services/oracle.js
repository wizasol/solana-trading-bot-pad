/**
 * Oracle Service for Spotify/Royalty API Integration
 * 
 * This service can be extended to integrate with:
 * - Spotify API for streaming data
 * - Music distribution platforms (DistroKid, CD Baby, etc.)
 * - Royalty collection societies (ASCAP, BMI, etc.)
 */

import axios from 'axios';
import cron from 'node-cron';

class OracleService {
  constructor() {
    this.apiKeys = {
      spotify: process.env.SPOTIFY_CLIENT_ID,
      spotifySecret: process.env.SPOTIFY_CLIENT_SECRET,
    };
    this.isRunning = false;
  }

  /**
   * Fetch streaming data from Spotify API
   * @param {string} artistId - Spotify artist ID
   * @param {string} accessToken - Spotify access token
   * @returns {Promise<Object>} Streaming data
   */
  async fetchSpotifyData(artistId, accessToken) {
    try {
      // Example: Get artist's top tracks
      const response = await axios.get(
        `https://api.spotify.com/v1/artists/${artistId}/top-tracks?market=US`,
        {
          headers: {
            Authorization: `Bearer ${accessToken}`,
          },
        }
      );

      // Calculate estimated royalties (simplified)
      // In production, you'd need to integrate with actual royalty APIs
      const totalStreams = response.data.tracks.reduce(
        (sum, track) => sum + (track.popularity || 0) * 1000,
        0
      );

      // Estimate: ~$0.003 per stream (varies by region, label, etc.)
      const estimatedRevenue = totalStreams * 0.003;

      return {
        streams: totalStreams,
        estimatedRevenue,
        tracks: response.data.tracks.length,
        timestamp: new Date().toISOString(),
      };
    } catch (error) {
      console.error('Error fetching Spotify data:', error);
      throw error;
    }
  }

  /**
   * Get Spotify access token using client credentials
   * @returns {Promise<string>} Access token
   */
  async getSpotifyAccessToken() {
    try {
      const response = await axios.post(
        'https://accounts.spotify.com/api/token',
        new URLSearchParams({
          grant_type: 'client_credentials',
        }),
        {
          headers: {
            'Content-Type': 'application/x-www-form-urlencoded',
            Authorization: `Basic ${Buffer.from(
              `${this.apiKeys.spotify}:${this.apiKeys.spotifySecret}`
            ).toString('base64')}`,
          },
        }
      );

      return response.data.access_token;
    } catch (error) {
      console.error('Error getting Spotify access token:', error);
      throw error;
    }
  }

  /**
   * Calculate royalty distribution amount
   * @param {number} totalRevenue - Total revenue from streaming
   * @param {number} royaltyPercentage - Percentage to distribute (0-100)
   * @returns {number} Amount to distribute in lamports
   */
  calculateDistributionAmount(totalRevenue, royaltyPercentage) {
    const distributionAmount = (totalRevenue * royaltyPercentage) / 100;
    // Convert to lamports (1 SOL = 1e9 lamports)
    // Assuming 1 USD = 0.01 SOL (adjust based on current rates)
    return Math.floor(distributionAmount * 0.01 * 1e9);
  }

  /**
   * Trigger royalty distribution for a project
   * @param {string} projectAddress - Project PDA address
   * @param {number} amount - Amount in lamports
   * @param {string} source - Source of the distribution (e.g., 'spotify', 'manual')
   */
  async triggerDistribution(projectAddress, amount, source = 'oracle') {
    try {
      // In production, this would call the smart contract
      const response = await axios.post('http://localhost:3001/api/oracle/distribute', {
        projectAddress,
        amount,
        source,
      });

      console.log(`Distribution triggered: ${amount} lamports to ${projectAddress}`);
      return response.data;
    } catch (error) {
      console.error('Error triggering distribution:', error);
      throw error;
    }
  }

  /**
   * Start automated royalty distribution cron job
   * Runs daily at midnight UTC
   * @param {Array<Object>} projects - Array of projects to monitor
   */
  startCronJob(projects = []) {
    if (this.isRunning) {
      console.log('Oracle cron job already running');
      return;
    }

    // Run daily at midnight UTC
    cron.schedule('0 0 * * *', async () => {
      console.log('Running oracle cron job...');
      
      for (const project of projects) {
        try {
          if (!project.spotifyArtistId) {
            console.log(`Skipping project ${project.address}: no Spotify artist ID`);
            continue;
          }

          // Get access token
          const accessToken = await this.getSpotifyAccessToken();
          
          // Fetch streaming data
          const spotifyData = await this.fetchSpotifyData(
            project.spotifyArtistId,
            accessToken
          );

          // Calculate distribution amount
          const distributionAmount = this.calculateDistributionAmount(
            spotifyData.estimatedRevenue,
            project.royaltyPercentage
          );

          if (distributionAmount > 0) {
            // Trigger distribution
            await this.triggerDistribution(
              project.address,
              distributionAmount,
              'spotify'
            );
          }

          console.log(`Processed project ${project.address}: ${distributionAmount} lamports`);
        } catch (error) {
          console.error(`Error processing project ${project.address}:`, error);
        }
      }
    });

    this.isRunning = true;
    console.log('Oracle cron job started (runs daily at midnight UTC)');
  }

  /**
   * Stop the cron job
   */
  stopCronJob() {
    // In a real implementation, you'd store the cron job reference
    this.isRunning = false;
    console.log('Oracle cron job stopped');
  }
}

export const oracleService = new OracleService();
