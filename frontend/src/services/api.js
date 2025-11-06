import axios from 'axios'

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080/api'

const api = axios.create({
  baseURL: API_URL,
  headers: {
    'Content-Type': 'application/json',
  },
})

// Response interceptor to handle API responses
api.interceptors.response.use(
  (response) => {
    // If response has success field, return data or throw error
    if (response.data.success === false) {
      throw new Error(response.data.error || 'API request failed')
    }
    return response.data.data !== undefined ? response.data.data : response.data
  },
  (error) => {
    console.error('API Error:', error)
    throw error
  }
)

// Tender APIs
export const getAllTenders = async () => {
  const response = await api.get('/tenders')
  return response
}

export const getTender = async (id) => {
  const response = await api.get(`/tenders/${id}`)
  return response
}

export const createTender = async (tenderData) => {
  const response = await api.post('/tenders', tenderData)
  return response
}

export const closeTender = async (id, caller) => {
  const response = await api.post(`/tenders/${id}/close`, { tender_id: id, caller })
  return response
}

export const getWinner = async (id) => {
  const response = await api.get(`/tenders/${id}/winner`)
  return response
}

export const getTenderBidders = async (id) => {
  const response = await api.get(`/tenders/${id}/bidders`)
  return response
}

// Bid APIs
export const submitBid = async (bidData) => {
  const response = await api.post('/bids/submit', bidData)
  return response
}

export const revealBid = async (revealData) => {
  const response = await api.post('/bids/reveal', revealData)
  return response
}

export const getBid = async (tenderId, bidder) => {
  const response = await api.get(`/bids/${tenderId}/${bidder}`)
  return response
}

// Encryption APIs (for server-side encryption if needed)
export const encryptAmount = async (amount) => {
  const response = await api.post('/crypto/encrypt', { amount })
  return response
}

export const decryptAmount = async (encryptedAmount, decryptionKey) => {
  const response = await api.post('/crypto/decrypt', {
    encrypted_amount: encryptedAmount,
    decryption_key: decryptionKey,
  })
  return response
}

export default api