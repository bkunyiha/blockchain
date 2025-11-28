import { useState } from 'react';
import { useBalance } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function Balance() {
  const [address, setAddress] = useState('');
  const { data, error, isLoading, refetch } = useBalance(address);

  const handleLoad = () => {
    if (address.trim()) {
      refetch();
    }
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Balance</h1>
      
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
            Load Balance
          </button>
        </div>
      </div>

      {isLoading && <LoadingSpinner />}
      
      {error && <ErrorMessage error={error as Error} />}
      
      {data?.data && (
        <div>
          <div className="card mb-4">
            <h2 className="text-lg font-semibold text-white mb-4">Balance Summary</h2>
            <div className="space-y-2">
              {data.data.confirmed !== undefined && (
                <div className="flex justify-between">
                  <span className="text-gray-400">Confirmed:</span>
                  <span className="text-white font-mono">{data.data.confirmed} satoshis</span>
                </div>
              )}
              {data.data.unconfirmed !== undefined && (
                <div className="flex justify-between">
                  <span className="text-gray-400">Unconfirmed:</span>
                  <span className="text-white font-mono">{data.data.unconfirmed} satoshis</span>
                </div>
              )}
            </div>
          </div>
          <JsonViewer data={data.data} title="Balance Details" />
        </div>
      )}
      
      {!address && !isLoading && !data && (
        <div className="text-gray-400">Enter a wallet address to view balance</div>
      )}
    </div>
  );
}

