import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { FileText, Calendar, DollarSign } from 'lucide-react'
import { createTender } from '../services/api'

function CreateTender({ walletAddress }) {
  const navigate = useNavigate()
  const [loading, setLoading] = useState(false)
  const [formData, setFormData] = useState({
    title: '',
    description: '',
    ipfs_hash: '',
    deadline: '',
    reveal_deadline: '',
    min_bid: ''
  })

  const handleChange = (e) => {
    setFormData({
      ...formData,
      [e.target.name]: e.target.value
    })
  }

  const handleSubmit = async (e) => {
    e.preventDefault()
    setLoading(true)

    try {
      // Convert dates to Unix timestamps
      const deadline = Math.floor(new Date(formData.deadline).getTime() / 1000)
      const revealDeadline = Math.floor(new Date(formData.reveal_deadline).getTime() / 1000)

      const tenderData = {
        creator: walletAddress,
        title: formData.title,
        description: formData.description,
        ipfs_hash: formData.ipfs_hash || 'QmDefault',
        deadline: deadline,
        reveal_deadline: revealDeadline,
        min_bid: parseInt(formData.min_bid)
      }

      const tenderId = await createTender(tenderData)
      alert(`Tender created successfully! ID: ${tenderId}`)
      navigate('/')
    } catch (error) {
      console.error('Failed to create tender:', error)
      alert('Failed to create tender: ' + error.message)
    } finally {
      setLoading(false)
    }
  }

  // Get minimum date (today)
  const today = new Date().toISOString().split('T')[0]

  return (
    <div className="max-w-2xl mx-auto">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Create New Tender</h1>
        <p className="text-gray-600">Fill in the details to create a new transparent tender</p>
      </div>

      <form onSubmit={handleSubmit} className="bg-white rounded-lg shadow-md p-6 space-y-6">
        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Tender Title *
          </label>
          <div className="relative">
            <FileText className="absolute left-3 top-3 text-gray-400" size={20} />
            <input
              type="text"
              name="title"
              value={formData.title}
              onChange={handleChange}
              required
              placeholder="e.g., Road Construction Project"
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-purple-600 focus:border-transparent"
            />
          </div>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Description *
          </label>
          <textarea
            name="description"
            value={formData.description}
            onChange={handleChange}
            required
            rows="4"
            placeholder="Detailed description of the tender requirements..."
            className="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-purple-600 focus:border-transparent"
          />
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            IPFS Document Hash (Optional)
          </label>
          <input
            type="text"
            name="ipfs_hash"
            value={formData.ipfs_hash}
            onChange={handleChange}
            placeholder="QmXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
            className="w-full px-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-purple-600 focus:border-transparent"
          />
          <p className="text-xs text-gray-500 mt-1">
            Upload tender documents to IPFS and paste the hash here
          </p>
        </div>

        <div>
          <label className="block text-sm font-medium text-gray-700 mb-2">
            Minimum Bid Amount (in Stroops) *
          </label>
          <div className="relative">
            <DollarSign className="absolute left-3 top-3 text-gray-400" size={20} />
            <input
              type="number"
              name="min_bid"
              value={formData.min_bid}
              onChange={handleChange}
              required
              min="1"
              placeholder="100000"
              className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-purple-600 focus:border-transparent"
            />
          </div>
          <p className="text-xs text-gray-500 mt-1">
            1 XLM = 10,000,000 stroops
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Bidding Deadline *
            </label>
            <div className="relative">
              <Calendar className="absolute left-3 top-3 text-gray-400" size={20} />
              <input
                type="datetime-local"
                name="deadline"
                value={formData.deadline}
                onChange={handleChange}
                required
                min={today}
                className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-purple-600 focus:border-transparent"
              />
            </div>
          </div>

          <div>
            <label className="block text-sm font-medium text-gray-700 mb-2">
              Reveal Deadline *
            </label>
            <div className="relative">
              <Calendar className="absolute left-3 top-3 text-gray-400" size={20} />
              <input
                type="datetime-local"
                name="reveal_deadline"
                value={formData.reveal_deadline}
                onChange={handleChange}
                required
                min={formData.deadline || today}
                className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-md focus:ring-2 focus:ring-purple-600 focus:border-transparent"
              />
            </div>
          </div>
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
                <strong>Note:</strong> The reveal deadline must be after the bidding deadline. 
                Bidders will have time between these two deadlines to reveal their encrypted bids.
              </p>
            </div>
          </div>
        </div>

        <div className="flex space-x-4">
          <button
            type="button"
            onClick={() => navigate('/')}
            className="flex-1 bg-gray-200 hover:bg-gray-300 text-gray-800 py-3 rounded-md font-medium"
          >
            Cancel
          </button>
          <button
            type="submit"
            disabled={loading}
            className="flex-1 bg-purple-600 hover:bg-purple-700 text-white py-3 rounded-md font-medium disabled:opacity-50"
          >
            {loading ? 'Creating...' : 'Create Tender'}
          </button>
        </div>
      </form>
    </div>
  )
}

export default CreateTender