import CryptoJS from 'crypto-js'

// Generate a random encryption key
const generateKey = () => {
  return CryptoJS.lib.WordArray.random(32).toString()
}

// Encrypt bid amount
export const encryptBid = (amount) => {
  try {
    // Generate a unique key for this bid
    const key = generateKey()
    
    // Convert amount to string
    const amountStr = amount.toString()
    
    // Encrypt using AES
    const encrypted = CryptoJS.AES.encrypt(amountStr, key).toString()
    
    return {
      encrypted,
      key,
    }
  } catch (error) {
    console.error('Encryption failed:', error)
    throw new Error('Failed to encrypt bid')
  }
}

// Decrypt bid amount
export const decryptBid = (encryptedAmount, key) => {
  try {
    // Decrypt using AES
    const bytes = CryptoJS.AES.decrypt(encryptedAmount, key)
    const decryptedStr = bytes.toString(CryptoJS.enc.Utf8)
    
    if (!decryptedStr) {
      throw new Error('Invalid decryption key')
    }
    
    return parseInt(decryptedStr)
  } catch (error) {
    console.error('Decryption failed:', error)
    throw new Error('Failed to decrypt bid. Please check your decryption key.')
  }
}

// Verify that encryption/decryption works correctly
export const verifyEncryption = (amount, encrypted, key) => {
  try {
    const decrypted = decryptBid(encrypted, key)
    return decrypted === amount
  } catch (error) {
    return false
  }
}

export default {
  encryptBid,
  decryptBid,
  verifyEncryption,
}