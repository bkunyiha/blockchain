import { useState } from 'react';
import { useSendTransaction } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { JsonViewer } from '../common/JsonViewer';

export function SendTransaction() {
  const [fromAddress, setFromAddress] = useState('');
  const [toAddress, setToAddress] = useState('');
  const [amount, setAmount] = useState('');
  const sendTransaction = useSendTransaction();

  const handleSend = () => {
    if (!fromAddress.trim() || !toAddress.trim() || !amount.trim()) {
      return;
    }

    const amountNum = parseInt(amount);
    if (isNaN(amountNum) || amountNum <= 0) {
      return;
    }

    sendTransaction.mutate({
      from_address: fromAddress.trim(),
      to_address: toAddress.trim(),
      amount: amountNum,
    });
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Send Bitcoin</h1>
      
      <div className="card mb-6">
        <div className="space-y-4">
          <div>
            <label className="block text-sm text-gray-300 mb-2">From Address</label>
            <input
              type="text"
              value={fromAddress}
              onChange={(e) => setFromAddress(e.target.value)}
              placeholder="Enter sender address"
              className="input-field"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-300 mb-2">To Address</label>
            <input
              type="text"
              value={toAddress}
              onChange={(e) => setToAddress(e.target.value)}
              placeholder="Enter recipient address"
              className="input-field"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-300 mb-2">Amount (satoshis)</label>
            <input
              type="number"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="1000000"
              className="input-field"
              min="1"
            />
          </div>
          <button
            onClick={handleSend}
            className="btn-primary"
            disabled={sendTransaction.isPending || !fromAddress.trim() || !toAddress.trim() || !amount.trim()}
          >
            {sendTransaction.isPending ? 'Sending...' : 'Send Transaction'}
          </button>
        </div>
      </div>

      {sendTransaction.isPending && <LoadingSpinner />}
      
      {sendTransaction.data?.data && (
        <div>
          <div className="card mb-4 bg-green-900/20 border-green-500/50">
            <p className="text-green-400 font-semibold mb-2">Transaction Sent Successfully!</p>
            <p className="text-sm text-gray-300">
              <span className="text-gray-400">Transaction ID: </span>
              <span className="font-mono break-all">{sendTransaction.data.data.txid}</span>
            </p>
          </div>
          <JsonViewer data={sendTransaction.data.data} title="Transaction Response" />
        </div>
      )}
    </div>
  );
}

