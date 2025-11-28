import { useState } from 'react';
import { useCreateWallet } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { JsonViewer } from '../common/JsonViewer';
import toast from 'react-hot-toast';

export function CreateWallet() {
  const [label, setLabel] = useState('');
  const createWallet = useCreateWallet();

  const handleCreate = async () => {
    if (!label.trim()) {
      toast.error('Please enter a wallet label');
      return;
    }
    
    createWallet.mutate({ label: label.trim() });
    setLabel('');
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Create Wallet</h1>
      
      <div className="card mb-6">
        <div className="space-y-4">
          <div>
            <label className="block text-sm text-gray-300 mb-2">Wallet Label (optional)</label>
            <input
              type="text"
              value={label}
              onChange={(e) => setLabel(e.target.value)}
              placeholder="My Wallet"
              className="input-field"
              onKeyPress={(e) => e.key === 'Enter' && handleCreate()}
            />
          </div>
          <button
            onClick={handleCreate}
            className="btn-primary"
            disabled={createWallet.isPending}
          >
            {createWallet.isPending ? 'Creating...' : 'Create Wallet'}
          </button>
        </div>
      </div>

      {createWallet.isPending && <LoadingSpinner />}
      
      {createWallet.data?.data && (
        <div>
          <div className="card mb-4 bg-green-900/20 border-green-500/50">
            <p className="text-green-400 font-semibold mb-2">Wallet Created Successfully!</p>
            <p className="text-sm text-gray-300">
              <span className="text-gray-400">Address: </span>
              <span className="font-mono break-all">{createWallet.data.data.address}</span>
            </p>
          </div>
          <JsonViewer data={createWallet.data.data} title="Wallet Response" />
        </div>
      )}
    </div>
  );
}

