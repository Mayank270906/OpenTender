import { useState } from 'react'
import { Unlock, Upload } from 'lucide-react'
import { revealBid } from '../services/api'

function RevealBid({ tenderId, walletAddress, onSuccess }) {
  const [bidAmount, setBidAmount] = useState('')
  const [decryptionKey, setDecryptionKey] = useState('')
  const [loading, setLoading] = useState(false)

  const handleFileUpload = (e) => {
    const file = e.target.files[0]
    if (!file) return

    const reader = new FileReader()
    reader.onload = (event) => {
      try {
        const data = JSON.parse(event.target.result)
        if (data.decryption_key && data.bid_amount) {
          setDecryptionKey(data.decryption_key)
          setBidAmount(data.bid_amount.toString())
        } else {
          alert('Invalid key file format')
        }
      } catch (error) {
        alert('Failed to read key file')
      }
    }
    reader.readAsText(file)
  }

  const handleSubmit = async (e) => {
    e.preventDefault()
    
    if (!bidAmount || !decryptionKey) {
      alert('Please provide both bid amount and decryption key')
      return
    }

    setLoading(true)
    try {
      await revealBid({
        tender_id: tenderId,
        bidder: walletAddress,
        actual_amount: parseInt(bidAmount),
        decryption_key: decryptionKey
      })

      alert('Bid revealed successfully!')
      onSuccess()
    } catch (error) {
      console.error('Failed to reveal bid:', error)
      alert('Failed to reveal bid: ' + error.message)
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="border-2 border-yellow-200 rounded-lg p-6 bg-yellow-50">
      <h3 className="text-xl font-semibold mb-4 flex items-center">
        <Unlock className="mr-2" />
        Reveal Your Bid
      </h3>

      <form onSubmit={handleSubmit} className="space-y-4">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Upload Decryption Key File
          </label>
          <div className="flex items-center justify-center w-full">
            <label className="flex flex-col items-center justify-center w-full h-32 border-2 border-gray-300 border-dashed rounded-lg cursor-pointer bg-white hover:bg-gray-50">
              <div className="flex flex-col items-center justify-center pt-5 pb-6">
                <Upload className="w-8 h-8 mb-2 text-gray-400" />
                <p className="mb-2 text-sm text-gray-500">
                  <span className="font-semibold">Click to upload</span> or drag and drop
                </p>
                <p className="text-xs text-gray-500">JSON key file</p>
              </div>
              <input
                type="file"
                accept=".json"
                onChange={handleFileUpload}
                className="hidden"
              />
            </label>
          </div>
        </div>

        <div className="text-center text-gray-500">OR</div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Bid Amount (Stroops) *
          </label>
          <input
            type="number"
            value={bidAmount}
            onChange={(e) => setBidAmount(e.target.value)}
            required
            placeholder="Enter your original bid amount"
            className="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-yellow-600 focus:border-transparent"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Decryption Key *
          </label>
          <textarea
            value={decryptionKey}
            onChange={(e) => setDecryptionKey(e.target.value)}
            required
            rows="3"
            placeholder="Paste your decryption key here"
            className="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-yellow-600 focus:border-transparent font-mono text-sm"
          />
        </div>

        <div className="bg-blue-50 border-l-4 border-blue-400 p-4">
          <div className="flex">
            <div className="flex-shrink-0">
              <svg className="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
              </svg>
            </div>
            <div className="ml-3">
              <p className="text-sm text-blue-700">
                Enter the exact bid amount you submitted and provide your decryption key.
                This information was saved when you submitted your bid.
              </p>
            </div>
          </div>
        </div>

        <button
          type="submit"
          disabled={loading}
          className="w-full bg-yellow-600 hover:bg-yellow-700 text-white py-3 rounded-md font-medium disabled:opacity-50"
        >
          {loading ? 'Revealing...' : 'Reveal Bid'}
        </button>
      </form>
    </div>
  )
}

export default RevealBid