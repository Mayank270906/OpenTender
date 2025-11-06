import { useState, useEffect } from 'react'
import { useParams, useNavigate } from 'react-router-dom'
import { Calendar, DollarSign, User, Award, Clock } from 'lucide-react'
import { getTender, getWinner, getTenderBidders, closeTender } from '../services/api'
import { formatDate, formatAmount } from '../utils/helpers'
import SubmitBid from './SubmitBid'
import RevealBid from './RevealBid'

function TenderDetail({ walletAddress }) {
  const { id } = useParams()
  const navigate = useNavigate()
  const [tender, setTender] = useState(null)
  const [winner, setWinner] = useState(null)
  const [bidders, setBidders] = useState([])
  const [loading, setLoading] = useState(true)
  const [showBidForm, setShowBidForm] = useState(false)
  const [showRevealForm, setShowRevealForm] = useState(false)
  const [closing, setClosing] = useState(false)

  useEffect(() => {
    loadTenderDetails()
  }, [id])

  const loadTenderDetails = async () => {
    try {
      const tenderData = await getTender(id)
      setTender(tenderData)

      if (tenderData.is_closed) {
        const winnerData = await getWinner(id)
        setWinner(winnerData)
      }

      const biddersData = await getTenderBidders(id)
      setBidders(biddersData)
    } catch (error) {
      console.error('Failed to load tender:', error)
    } finally {
      setLoading(false)
    }
  }

  const handleCloseTender = async () => {
    if (!window.confirm('Are you sure you want to close this tender?')) return

    setClosing(true)
    try {
      await closeTender(Number(id), walletAddress)
      alert('Tender closed successfully!')
      loadTenderDetails()
    } catch (error) {
      console.error('Failed to close tender:', error)
      alert('Failed to close tender: ' + error.message)
    } finally {
      setClosing(false)
    }
  }

  const getTenderPhase = () => {
    if (!tender) return null
    const now = Date.now() / 1000

    if (tender.is_closed) return 'closed'
    if (now < tender.deadline) return 'bidding'
    if (now >= tender.deadline && now < tender.reveal_deadline) return 'reveal'
    return 'awaiting_closure'
  }

  const canCloseTender = () => {
    if (!tender || !walletAddress) return false
    const now = Date.now() / 1000
    return (
      !tender.is_closed &&
      now >= tender.reveal_deadline &&
      walletAddress === tender.creator
    )
  }

  if (loading) {
    return (
      <div className="flex justify-center items-center min-h-[400px]">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-600"></div>
      </div>
    )
  }

  if (!tender) {
    return (
      <div className="text-center py-12">
        <h2 className="text-2xl font-bold text-gray-900 mb-4">Tender not found</h2>
        <button
          onClick={() => navigate('/')}
          className="bg-purple-600 hover:bg-purple-700 text-white px-6 py-2 rounded-md"
        >
          Back to Home
        </button>
      </div>
    )
  }

  const phase = getTenderPhase()

  return (
    <div className="max-w-4xl mx-auto">
      <button
        onClick={() => navigate('/')}
        className="mb-4 text-purple-600 hover:text-purple-700 flex items-center"
      >
        ← Back to Tenders
      </button>

      <div className="bg-white rounded-lg shadow-md overflow-hidden">
        <div className="gradient-bg p-6 text-white">
          <h1 className="text-3xl font-bold mb-2">{tender.title}</h1>
          <p className="opacity-90">Tender ID: #{tender.id}</p>
        </div>

        <div className="p-6 space-y-6">
          {/* Status Banner */}
          <div className={`p-4 rounded-md ${
            phase === 'closed' ? 'bg-gray-100' :
            phase === 'bidding' ? 'bg-green-100' :
            phase === 'reveal' ? 'bg-yellow-100' :
            'bg-blue-100'
          }`}>
            <div className="flex items-center justify-between">
              <div>
                <h3 className="font-semibold text-lg">
                  {phase === 'closed' && 'Tender Closed'}
                  {phase === 'bidding' && 'Open for Bidding'}
                  {phase === 'reveal' && 'Reveal Phase'}
                  {phase === 'awaiting_closure' && 'Awaiting Closure'}
                </h3>
                <p className="text-sm">
                  {phase === 'closed' && 'This tender has been closed and winner announced'}
                  {phase === 'bidding' && 'Submit your encrypted bid before the deadline'}
                  {phase === 'reveal' && 'Reveal your bid with the decryption key'}
                  {phase === 'awaiting_closure' && 'Waiting for creator to close tender'}
                </p>
              </div>
              <Clock size={32} />
            </div>
          </div>

          {/* Description */}
          <div>
            <h3 className="text-xl font-semibold mb-2">Description</h3>
            <p className="text-gray-700 whitespace-pre-wrap">{tender.description}</p>
          </div>

          {/* Details Grid */}
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
            <div className="border border-gray-200 rounded-lg p-4">
              <div className="flex items-center text-gray-700 mb-2">
                <DollarSign size={20} className="mr-2 text-purple-600" />
                <span className="font-semibold">Minimum Bid</span>
              </div>
              <p className="text-2xl font-bold text-purple-600">
                {formatAmount(tender.min_bid)}
              </p>
            </div>

            <div className="border border-gray-200 rounded-lg p-4">
              <div className="flex items-center text-gray-700 mb-2">
                <Calendar size={20} className="mr-2 text-purple-600" />
                <span className="font-semibold">Bidding Deadline</span>
              </div>
              <p className="text-lg">{formatDate(tender.deadline)}</p>
            </div>

            <div className="border border-gray-200 rounded-lg p-4">
              <div className="flex items-center text-gray-700 mb-2">
                <Calendar size={20} className="mr-2 text-purple-600" />
                <span className="font-semibold">Reveal Deadline</span>
              </div>
              <p className="text-lg">{formatDate(tender.reveal_deadline)}</p>
            </div>

            <div className="border border-gray-200 rounded-lg p-4">
              <div className="flex items-center text-gray-700 mb-2">
                <User size={20} className="mr-2 text-purple-600" />
                <span className="font-semibold">Total Bidders</span>
              </div>
              <p className="text-2xl font-bold">{bidders.length}</p>
            </div>
          </div>

          {/* Winner Section */}
          {winner && (
            <div className="border-2 border-green-500 rounded-lg p-6 bg-green-50">
              <div className="flex items-center mb-4">
                <Award size={24} className="text-green-600 mr-2" />
                <h3 className="text-xl font-semibold text-green-900">Winner Announced</h3>
              </div>
              <div className="space-y-2">
                <p className="text-gray-700">
                  <span className="font-semibold">Bidder:</span> {winner.bidder}
                </p>
                <p className="text-gray-700">
                  <span className="font-semibold">Winning Bid:</span> {formatAmount(winner.amount)}
                </p>
                <p className="text-gray-700">
                  <span className="font-semibold">Selected At:</span> {formatDate(winner.selected_at)}
                </p>
              </div>
            </div>
          )}

          {/* Action Buttons */}
          {walletAddress && !tender.is_closed && (
            <div className="space-y-4">
              {phase === 'bidding' && (
                <button
                  onClick={() => setShowBidForm(!showBidForm)}
                  className="w-full bg-purple-600 hover:bg-purple-700 text-white py-3 rounded-md font-medium"
                >
                  {showBidForm ? 'Hide Bid Form' : 'Submit Bid'}
                </button>
              )}

              {phase === 'reveal' && (
                <button
                  onClick={() => setShowRevealForm(!showRevealForm)}
                  className="w-full bg-yellow-600 hover:bg-yellow-700 text-white py-3 rounded-md font-medium"
                >
                  {showRevealForm ? 'Hide Reveal Form' : 'Reveal Bid'}
                </button>
              )}

              {canCloseTender() && (
                <button
                  onClick={handleCloseTender}
                  disabled={closing}
                  className="w-full bg-red-600 hover:bg-red-700 text-white py-3 rounded-md font-medium disabled:opacity-50"
                >
                  {closing ? 'Closing...' : 'Close Tender & Announce Winner'}
                </button>
              )}
            </div>
          )}

          {/* Bid/Reveal Forms */}
          {showBidForm && (
            <SubmitBid
              tenderId={tender.id}
              walletAddress={walletAddress}
              onSuccess={() => {
                setShowBidForm(false)
                loadTenderDetails()
              }}
            />
          )}

          {showRevealForm && (
            <RevealBid
              tenderId={tender.id}
              walletAddress={walletAddress}
              onSuccess={() => {
                setShowRevealForm(false)
                loadTenderDetails()
              }}
            />
          )}
        </div>
      </div>
    </div>
  )
}

export default TenderDetail