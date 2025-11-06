import { Link } from 'react-router-dom'
import { Wallet, LogOut, Plus, Home } from 'lucide-react'

function Navbar({ walletAddress, onConnect, onDisconnect, isConnecting }) {
  const shortenAddress = (address) => {
    if (!address) return ''
    return `${address.slice(0, 4)}...${address.slice(-4)}`
  }

  return (
    <nav className="gradient-bg shadow-lg">
      <div className="container mx-auto px-4">
        <div className="flex items-center justify-between h-16">
          <div className="flex items-center space-x-8">
            <Link to="/" className="text-white text-2xl font-bold flex items-center space-x-2">
              <svg className="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
              <span>OpenTender</span>
            </Link>
            
            <div className="hidden md:flex space-x-4">
              <Link 
                to="/" 
                className="text-white hover:bg-white hover:bg-opacity-20 px-3 py-2 rounded-md text-sm font-medium flex items-center space-x-1"
              >
                <Home size={18} />
                <span>Home</span>
              </Link>
              
              {walletAddress && (
                <Link 
                  to="/create" 
                  className="text-white hover:bg-white hover:bg-opacity-20 px-3 py-2 rounded-md text-sm font-medium flex items-center space-x-1"
                >
                  <Plus size={18} />
                  <span>Create Tender</span>
                </Link>
              )}
            </div>
          </div>

          <div className="flex items-center">
            {walletAddress ? (
              <div className="flex items-center space-x-2">
                <div className="bg-white bg-opacity-20 px-4 py-2 rounded-md flex items-center space-x-2">
                  <Wallet size={18} className="text-white" />
                  <span className="text-white text-sm font-medium">
                    {shortenAddress(walletAddress)}
                  </span>
                </div>
                <button
                  onClick={onDisconnect}
                  className="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded-md flex items-center space-x-1"
                >
                  <LogOut size={18} />
                  <span className="hidden md:inline">Disconnect</span>
                </button>
              </div>
            ) : (
              <button
                onClick={onConnect}
                disabled={isConnecting}
                className="bg-white hover:bg-gray-100 text-purple-600 px-6 py-2 rounded-md font-medium flex items-center space-x-2 disabled:opacity-50"
              >
                <Wallet size={18} />
                <span>{isConnecting ? 'Connecting...' : 'Connect Wallet'}</span>
              </button>
            )}
          </div>
        </div>
      </div>
    </nav>
  )
}

export default Navbar