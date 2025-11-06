import { useState } from 'react'
import { Lock, Download } from 'lucide-react'
import { submitBid } from '../services/api'
import { encryptBid } from '../services/encryption'

function SubmitBid({ tenderId, walletAddress, onSuccess }) {
  const [bidAmount, setBidAmount] = useState('')
  const [loading, setLoading] = useState(false)
  const [encryptedData, setEncryptedData] = useState(null)

  const handleEncrypt = () => {
    if (!bidAmount || parseInt(bidAmount) <= 0) {
      alert('Please enter a valid bid amount')
      return
    }

    try {
      const { encrypted, key } = encryptBid(parseInt(bidAmount))
      setEncryptedData({ encrypted, key })
    } catch (error) {
      console.error('Encryption failed:', error)
      alert('Failed to encrypt bid')
    }
  }

  const handleSubmit = async () => {
    if (!encryptedData) {
      alert('Please encrypt your bid first')
      return
    }

    setLoading(true)
    try {
      await submitBid({
        tender_id: tenderId,
        bidder: walletAddress,
        encrypted_amount: encryptedData.encrypted
      })

      // Download decryption key
      downloadKey()

      alert('Bid submitted successfully! Please save your decryption key.')
      onSuccess()
    } catch (error) {
      console.error('Failed to submit bid:', error)
      alert('Failed to submit bid: ' + error.message)
    } finally {
      setLoading(false)
    }
  }

  const downloadKey = () => {
    const keyData = {
      tender_id: tenderId,
      bidder: walletAddress,
      decryption_key: encryptedData.key,
      bid_amount: bidAmount,
      timestamp: new Date().toISOString()
    }

    const blob = new Blob([JSON.stringify(keyData, null, 2)], { type: 'application/json' })
    const url = URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `tender-${tenderId}-decryption-key.json`
    document.body.appendChild(a)
    a.click()
    document.body.removeChild(a)
    URL.revokeObjectURL(url)
  }

  return (
    <div className="border-2 border-purple-200 rounded-lg p-6 bg-purple-50">
      <h3 className="text-xl font-semibold mb-4 flex items-center">
        <Lock className="mr-2" />
        Submit Encrypted Bid
      </h3>

      <div className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Your Bid Amount (in Stroops) *
          </label>
          <input
            type="number"
            value={bidAmount}
            onChange={(e) => setBidAmount(e.target.value)}
            disabled={!!encryptedData}
            placeholder="Enter your bid amount"
            className="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-purple-600 focus:border-transparent disabled:bg-gray-100"
          />
          <p className="text-xs text-gray-600 mt-1">
            Your bid will be encrypted before submission
          </p>
        </div>

        {!encryptedData ? (
          <button
            onClick={handleEncrypt}
            className="w-full bg-purple-600 hover:bg-purple-700 text-white py-2 rounded-md font-medium"
          >
            Encrypt Bid
          </button>
        ) : (
          <div>
            <div className="bg-white border border-green-300 rounded-md p-4 mb-4">
              <p className="text-sm font-semibold text-green-700 mb-2">✓ Bid Encrypted Successfully</p>
              <div className="space-y-2 text-xs">
                <div>
                  <span className="font-medium">Encrypted Data:</span>
                  <p className="font-mono bg-gray-100 p-2 rounded mt-1 break-all">
                    {encryptedData.encrypted.substring(0, 50)}...
                  </p>
                </div>
                <div>
                  <span className="font-medium">Decryption Key:</span>
                  <p className="font-mono bg-gray-100 p-2 rounded mt-1 break-all">
                    {encryptedData.key}
                  </p>
                </div>
              </div>
            </div>

            <div className="bg-yellow-50 border-l-4 border-yellow-400 p-4 mb-4">
              <div className="flex">
                <div className="flex-shrink-0">
                  <svg className="h-5 w-5 text-yellow-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                  </svg>
                </div>
                <div className="ml-3">
                  <p className="text-sm text-yellow-700">
                    <strong>Important:</strong> Save your decryption key! You'll need it to reveal your bid later.
                    The key will be automatically downloaded when you submit.
                  </p>
                </div>
              </div>
            </div>

            <div className="flex space-x-2">
              <button
                onClick={downloadKey}
                className="flex-1 bg-gray-200 hover:bg-gray-300 text-gray-800 py-2 rounded-md font-medium flex items-center justify-center"
              >
                <Download size={18} className="mr-2" />
                Download Key
              </button>
              <button
                onClick={handleSubmit}
                disabled={loading}
                className="flex-1 bg-green-600 hover:bg-green-700 text-white py-2 rounded-md font-medium disabled:opacity-50"
              >
                {loading ? 'Submitting...' : 'Submit Bid'}
              </button>
            </div>

            <button
              onClick={() => setEncryptedData(null)}
              className="w-full mt-2 text-sm text-gray-600 hover:text-gray-800"
            >
              Start Over
            </button>
          </div>
        )}
      </div>
    </div>
  )
}

export default SubmitBid