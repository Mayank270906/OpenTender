import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { Clock, DollarSign, FileText, CheckCircle, XCircle } from 'lucide-react'
import { getAllTenders } from '../services/api'
import { formatDate, formatAmount } from '../utils/helpers'

function TenderList({ walletAddress }) {
  const [tenders, setTenders] = useState([])
  const [loading, setLoading] = useState(true)
  const [filter, setFilter] = useState('all') // all, open, closed

  useEffect(() => {
    loadTenders()
  }, [])

  const loadTenders = async () => {
    try {
      const data = await getAllTenders()
      setTenders(data)
    } catch (error) {
      console.error('Failed to load tenders:', error)
    } finally {
      setLoading(false)
    }
  }

  const filteredTenders = tenders.filter(tender => {
    if (filter === 'all') return true
    if (filter === 'open') return !tender.is_closed
    if (filter === 'closed') return tender.is_closed
    return true
  })

  const getStatusBadge = (tender) => {
    const now = Date.now() / 1000
    
    if (tender.is_closed) {
      return (
        <span className="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-gray-200 text-gray-800">
          <XCircle size={14} className="mr-1" />
          Closed
        </span>
      )
    }
    
    if (now < tender.deadline) {
      return (
        <span className="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-green-100 text-green-800">
          <CheckCircle size={14} className="mr-1" />
          Open for Bidding
        </span>
      )
    }
    
    if (now >= tender.deadline && now < tender.reveal_deadline) {
      return (
        <span className="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-yellow-100 text-yellow-800">
          <Clock size={14} className="mr-1" />
          Reveal Phase
        </span>
      )
    }
    
    return (
      <span className="inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-blue-100 text-blue-800">
        <Clock size={14} className="mr-1" />
        Awaiting Closure
      </span>
    )
  }

  if (loading) {
    return (
      <div className="flex justify-center items-center min-h-[400px]">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-600"></div>
      </div>
    )
  }

  return (
    <div>
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Active Tenders</h1>
        <p className="text-gray-600">Browse and participate in transparent, blockchain-based tenders</p>
      </div>

      <div className="mb-6 flex space-x-2">
        <button
          onClick={() => setFilter('all')}
          className={`px-4 py-2 rounded-md ${
            filter === 'all' 
              ? 'bg-purple-600 text-white' 
              : 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50'
          }`}
        >
          All Tenders
        </button>
        <button
          onClick={() => setFilter('open')}
          className={`px-4 py-2 rounded-md ${
            filter === 'open' 
              ? 'bg-purple-600 text-white' 
              : 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50'
          }`}
        >
          Open
        </button>
        <button
          onClick={() => setFilter('closed')}
          className={`px-4 py-2 rounded-md ${
            filter === 'closed' 
              ? 'bg-purple-600 text-white' 
              : 'bg-white text-gray-700 border border-gray-300 hover:bg-gray-50'
          }`}
        >
          Closed
        </button>
      </div>

      {filteredTenders.length === 0 ? (
        <div className="text-center py-12 bg-white rounded-lg shadow">
          <FileText size={48} className="mx-auto text-gray-400 mb-4" />
          <h3 className="text-lg font-medium text-gray-900 mb-2">No tenders found</h3>
          <p className="text-gray-600 mb-4">Be the first to create a tender!</p>
          {walletAddress && (
            <Link
              to="/create"
              className="inline-block bg-purple-600 hover:bg-purple-700 text-white px-6 py-2 rounded-md"
            >
              Create Tender
            </Link>
          )}
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
          {filteredTenders.map((tender) => (
            <Link
              key={tender.id}
              to={`/tender/${tender.id}`}
              className="bg-white rounded-lg shadow-md overflow-hidden card-hover"
            >
              <div className="p-6">
                <div className="flex justify-between items-start mb-4">
                  <h3 className="text-xl font-semibold text-gray-900 line-clamp-2">
                    {tender.title}
                  </h3>
                  {getStatusBadge(tender)}
                </div>

                <p className="text-gray-600 text-sm mb-4 line-clamp-3">
                  {tender.description}
                </p>

                <div className="space-y-2 text-sm">
                  <div className="flex items-center text-gray-700">
                    <DollarSign size={16} className="mr-2 text-purple-600" />
                    <span>Min Bid: {formatAmount(tender.min_bid)}</span>
                  </div>
                  
                  <div className="flex items-center text-gray-700">
                    <Clock size={16} className="mr-2 text-purple-600" />
                    <span>Deadline: {formatDate(tender.deadline)}</span>
                  </div>
                </div>

                <div className="mt-4 pt-4 border-t border-gray-200">
                  <button className="w-full bg-purple-600 hover:bg-purple-700 text-white py-2 rounded-md text-sm font-medium">
                    View Details
                  </button>
                </div>
              </div>
            </Link>
          ))}
        </div>
      )}
    </div>
  )
}

export default TenderList