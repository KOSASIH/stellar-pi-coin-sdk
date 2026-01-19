# GodHead Nexus Autonomous AI

## Overview
This module implements the most advanced AI for Pi Coin: In-contract simulation + off-chain Hyper AI via TensorFlow.

## Setup
1. Run off-chain AI: `python hyper_ai.py` (requires TensorFlow).
2. Deploy oracle contract to bridge API.
3. Update contract with oracle address.
4. Deploy Pi Coin contract on Stellar.

## Usage
- On-chain: Call `AiSimulation::predict` for bounded predictions.
- Off-chain: Query `/predict` API for full AI.
- Autonomous: Oracle fetches predictions for contract decisions.

## Evolution
AI evolves via feedback loops: On-chain votes trigger off-chain retraining.
