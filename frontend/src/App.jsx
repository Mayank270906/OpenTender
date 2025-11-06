import { useState, useEffect } from 'react'
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom'
import Navbar from './components/Navbar'
import TenderList from './components/TenderList'
import CreateTender from './components/CreateTender'
import TenderDetail from './components/TenderDetail'
import { connectWallet, isFreighterInstalled } from './services/stellar'

function App() {
  const [walletAddress, setWalletAddress] = useState(null)
  const [isConnecting, setIsConnecting] = useState(false)

  useEffect(() => {
    // Check if wallet is already connected
    const checkConnection = async () => {
      if (await isFreighterInstalled()) {
        // Try to get current address without prompting
        try {
          const address = localStorage.getItem('walletAddress')
          if (address) {
            setWalletAddress(address)
          }
        } catch (error) {
          console.error('Error checking wallet connection:', error)
        }
      }
    }
    checkConnection()
  }, [])

  const handleConnectWallet = async () => {
    setIsConnecting(true)
    try {
      const address = await connectWallet()
      setWalletAddress(address)
      localStorage.setItem('walletAddress', address)
    } catch (error) {
      console.error('Failed to connect wallet:', error)
      alert('Failed to connect wallet. Please make sure Freighter is installed.')
    } finally {
      setIsConnecting(false)
    }
  }

  const handleDisconnect = () => {
    setWalletAddress(null)
    localStorage.removeItem('walletAddress')
  }

  return (
    <Router>
      <div className="min-h-screen bg-gray-50">
        <Navbar 
          walletAddress={walletAddress}
          onConnect={handleConnectWallet}
          onDisconnect={handleDisconnect}
          isConnecting={isConnecting}
        />
        
        <main className="container mx-auto px-4 py-8">
          <Routes>
            <Route path="/" element={<TenderList walletAddress={walletAddress} />} />
            <Route 
              path="/create" 
              element={
                walletAddress ? 
                <CreateTender walletAddress={walletAddress} /> : 
                <Navigate to="/" />
              } 
            />
            <Route 
              path="/tender/:id" 
              element={<TenderDetail walletAddress={walletAddress} />} 
            />
          </Routes>
        </main>

        {!walletAddress && (
          <div className="fixed bottom-8 right-8 bg-yellow-100 border-l-4 border-yellow-500 p-4 rounded shadow-lg max-w-md">
            <div className="flex items-center">
              <div className="flex-shrink-0">
                <svg className="h-5 w-5 text-yellow-500" viewBox="0 0 20 20" fill="currentColor">
                  <path fillRule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clipRule="evenodd" />
                </svg>
              </div>
              <div className="ml-3">
                <p className="text-sm text-yellow-700">
                  Please connect your Stellar wallet to create tenders and submit bids.
                </p>
              </div>
            </div>
          </div>
        )}
      </div>
    </Router>
  )
}

export default App