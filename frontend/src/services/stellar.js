import { isConnected, getPublicKey, signTransaction } from '@stellar/freighter-api'

export const isFreighterInstalled = async () => {
  try {
    return await isConnected()
  } catch (error) {
    return false
  }
}

export const connectWallet = async () => {
  try {
    const isInstalled = await isFreighterInstalled()
    
    if (!isInstalled) {
      throw new Error('Freighter wallet is not installed. Please install it from https://www.freighter.app/')
    }

    const publicKey = await getPublicKey()
    return publicKey
  } catch (error) {
    console.error('Failed to connect wallet:', error)
    throw error
  }
}

export const signAndSubmitTransaction = async (xdr) => {
  try {
    const networkPassphrase = import.meta.env.VITE_NETWORK_PASSPHRASE || 'Test SDF Network ; September 2015'
    
    const signedXDR = await signTransaction(xdr, {
      networkPassphrase,
      accountToSign: await getPublicKey(),
    })

    // In production, submit the signed transaction to Stellar network
    // For now, we return the signed XDR
    return signedXDR
  } catch (error) {
    console.error('Failed to sign transaction:', error)
    throw error
  }
}

// Helper to build transaction XDR (placeholder)
// In production, you'd use stellar-sdk to build proper transactions
export const buildCreateTenderTx = (creatorAddress, tenderData) => {
  // This would build a proper Stellar transaction with Soroban contract invocation
  // For now, return a placeholder
  return 'PLACEHOLDER_XDR'
}

export const buildSubmitBidTx = (bidderAddress, bidData) => {
  return 'PLACEHOLDER_XDR'
}

export const buildRevealBidTx = (bidderAddress, revealData) => {
  return 'PLACEHOLDER_XDR'
}

export const buildCloseTenderTx = (callerAddress, tenderId) => {
  return 'PLACEHOLDER_XDR'
}

export default {
  isFreighterInstalled,
  connectWallet,
  signAndSubmitTransaction,
  buildCreateTenderTx,
  buildSubmitBidTx,
  buildRevealBidTx,
  buildCloseTenderTx,
}