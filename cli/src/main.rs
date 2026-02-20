mod commands;
mod config;
mod export;
mod fuzz;
mod import;
mod manifest;
mod multisig;
mod patch;
mod profiler;
mod sla;
mod test_framework;
mod wizard;

use anyhow::Result;
use clap::{Parser, Subcommand};
use patch::Severity;

/// Soroban Registry CLI — discover, publish, verify, and deploy Soroban contracts
#[derive(Debug, Parser)]
#[command(name = "soroban-registry", version, about, long_about = None)]
pub struct Cli {
    /// Registry API URL
    #[arg(
        long,
        env = "SOROBAN_REGISTRY_API_URL",
        default_value = "http://localhost:3001"
    )]
    pub api_url: String,

    /// Stellar network to use (mainnet | testnet | futurenet)
    #[arg(long, global = true)]
    pub network: Option<String>,

    /// Enable verbose output (shows HTTP requests, responses, and debug info)
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Search for contracts in the registry
    Search {
        /// Search query
        query: String,
        /// Only show verified contracts
        #[arg(long)]
        verified_only: bool,
    },

    /// Get detailed information about a contract
    Info {
        /// Contract ID to look up
        contract_id: String,
    },

    /// Publish a new contract to the registry
    Publish {
        /// On-chain contract ID
        #[arg(long)]
        contract_id: String,

        /// Human-readable contract name
        #[arg(long)]
        name: String,

        /// Optional description
        #[arg(long)]
        description: Option<String>,

        /// Network (mainnet, testnet, futurenet)
        #[arg(long, default_value = "Testnet")]
        network: String,

        /// Category
        #[arg(long)]
        category: Option<String>,

        /// Comma-separated tags
        #[arg(long)]
        tags: Option<String>,

        /// Publisher Stellar address
        #[arg(long)]
        publisher: