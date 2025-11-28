import { useState } from 'react';
import { useWalletInfo } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function WalletInfo() {
  const [address, setAddress] = useState('');
  const { data, error, isLoading, refetch } = useWalletInfo(address);

  const handleLoad = () => {
    if (address.trim()) {
      refetch();
    }
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Wallet Info</h1>
      
      <div className="mb-6">
        <div className="flex gap-4">
          <input
            type="text"
            value={address}
            onChange={(e) => setAddress(e.target.value)}
            placeholder="Enter wallet address"
            className="input-field flex-1"
            onKeyPress={(e) => e.key === 'Enter' && handleLoad()}
          />
          <button onClick={handleLoad} className="btn-primary" disabled={!address.trim() || isLoading}>
            Load Info
          </button>
        </div>
      </div>

      {isLoading && <LoadingSpinner />}
      
      {error && <ErrorMessage error={error as Error} />}
      
      {data?.data && (
        <JsonViewer data={data.data} title="Wallet Information" />
      )}
      
      {!address && !isLoading && !data && (
        <div className="text-gray-400">Enter a wallet address to view information</div>
      )}
    </div>
  );
}

