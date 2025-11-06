// Format Unix timestamp to readable date
export const formatDate = (timestamp) => {
  const date = new Date(timestamp * 1000)
  return date.toLocaleString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  })
}

// Format amount (stroops to XLM)
export const formatAmount = (stroops) => {
  const xlm = stroops / 10000000
  return `${xlm.toLocaleString()} XLM (${stroops.toLocaleString()} stroops)`
}

// Shorten Stellar address
export const shortenAddress = (address, startChars = 4, endChars = 4) => {
  if (!address) return ''
  if (address.length <= startChars + endChars) return address
  return `${address.slice(0, startChars)}...${address.slice(-endChars)}`
}

// Calculate time remaining
export const getTimeRemaining = (deadline) => {
  const now = Math.floor(Date.now() / 1000)
  const remaining = deadline - now

  if (remaining <= 0) return 'Expired'

  const days = Math.floor(remaining / 86400)
  const hours = Math.floor((remaining % 86400) / 3600)
  const minutes = Math.floor((remaining % 3600) / 60)

  if (days > 0) return `${days}d ${hours}h remaining`
  if (hours > 0) return `${hours}h ${minutes}m remaining`
  return `${minutes}m remaining`
}

// Validate Stellar address
export const isValidStellarAddress = (address) => {
  return /^G[A-Z0-9]{55}$/.test(address)
}

// Convert XLM to stroops
export const xlmToStroops = (xlm) => {
  return Math.floor(xlm * 10000000)
}

// Convert stroops to XLM
export const stroopsToXlm = (stroops) => {
  return stroops / 10000000
}

export default {
  formatDate,
  formatAmount,
  shortenAddress,
  getTimeRemaining,
  isValidStellarAddress,
  xlmToStroops,
  stroopsToXlm,
}