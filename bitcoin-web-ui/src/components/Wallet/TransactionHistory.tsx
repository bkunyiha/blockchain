import { useState } from 'react';
import { useAddressTransactions } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function TransactionHistory() {
  const [address, setAddress] = useState('');
  const { data, error, isLoading, refetch } = useAddressTransactions(address);

  const handleLoad = () => {
    if (address.trim()) {
      refetch();
    }
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Transaction History</h1>
      
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
            Load History
          </button>
        </div>
      </div>

      {isLoading && <LoadingSpinner />}
      
      {error && <ErrorMessage error={error as Error} />}
      
      {data?.data && (
        <>
          {data.data.items && data.data.items.length > 0 ? (
            <JsonViewer data={data.data.items} title={`Transaction History (${data.data.total} total)`} />
          ) : (
            <div className="text-gray-400">
              No transactions found for this address.
              {data.data.total === 0 && ' This address has no transaction history.'}
            </div>
          )}
          {data.data.total > 0 && (
            <div className="mt-4 text-sm text-gray-400">
              Showing {data.data.items?.length || 0} of {data.data.total} transactions
              {data.data.total_pages > 1 && ` (Page ${data.data.page} of ${data.data.total_pages})`}
            </div>
          )}
        </>
      )}
      
      {!address && !isLoading && !data && (
        <div className="text-gray-400">Enter a wallet address to view transaction history</div>
      )}
    </div>
  );
}

