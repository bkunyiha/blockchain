import { useState } from 'react';
import { Dialog, Transition } from '@headlessui/react';
import { Fragment } from 'react';
import { useAddresses, useWalletInfo, useBalance, useAddressTransactions } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function AllAddresses() {
  const { data, error, isLoading, refetch } = useAddresses();
  const [selectedAddress, setSelectedAddress] = useState<string | null>(null);
  const [viewType, setViewType] = useState<'info' | 'balance' | 'history' | null>(null);

  const addresses = data?.data?.addresses || (Array.isArray(data?.data) ? data.data : []);

  const handleAction = (address: string, type: 'info' | 'balance' | 'history') => {
    setSelectedAddress(address);
    setViewType(type);
  };

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">All Addresses</h1>
        <button onClick={() => refetch()} className="btn-primary" disabled={isLoading}>
          Refresh
        </button>
      </div>

      {isLoading && <LoadingSpinner />}
      
      {error && <ErrorMessage error={error as Error} />}
      
      {addresses.length > 0 && (
        <div className="card mb-6">
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead>
                <tr className="border-b border-gray-700">
                  <th className="text-left py-3 px-4 text-gray-300 font-semibold">Address</th>
                  <th className="text-center py-3 px-4 text-gray-300 font-semibold">Info</th>
                  <th className="text-center py-3 px-4 text-gray-300 font-semibold">Balance</th>
                  <th className="text-center py-3 px-4 text-gray-300 font-semibold">History</th>
                </tr>
              </thead>
              <tbody>
                {addresses.map((address: string, index: number) => (
                  <tr key={index} className="border-b border-gray-700/50 hover:bg-gray-700/30">
                    <td className="py-3 px-4">
                      <span className="font-mono text-sm break-all text-gray-300">{address}</span>
                    </td>
                    <td className="py-3 px-4 text-center">
                      <button
                        onClick={() => handleAction(address, 'info')}
                        className="btn-secondary text-xs py-1 px-3"
                      >
                        Info
                      </button>
                    </td>
                    <td className="py-3 px-4 text-center">
                      <button
                        onClick={() => handleAction(address, 'balance')}
                        className="btn-secondary text-xs py-1 px-3"
                      >
                        Balance
                      </button>
                    </td>
                    <td className="py-3 px-4 text-center">
                      <button
                        onClick={() => handleAction(address, 'history')}
                        className="btn-secondary text-xs py-1 px-3"
                      >
                        History
                      </button>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}

      {selectedAddress && viewType && (
        <AddressDetailsModal
          address={selectedAddress}
          viewType={viewType}
          onClose={() => {
            setSelectedAddress(null);
            setViewType(null);
          }}
        />
      )}

      {addresses.length === 0 && !isLoading && !error && (
        <div className="text-gray-400">No addresses found. Create a wallet first.</div>
      )}
    </div>
  );
}

function AddressDetailsModal({
  address,
  viewType,
  onClose,
}: {
  address: string;
  viewType: 'info' | 'balance' | 'history';
  onClose: () => void;
}) {
  const { data: infoData, isLoading: infoLoading, error: infoError } = useWalletInfo(viewType === 'info' ? address : '');
  const { data: balanceData, isLoading: balanceLoading, error: balanceError } = useBalance(viewType === 'balance' ? address : '');
  const { data: historyData, isLoading: historyLoading, error: historyError } = useAddressTransactions(viewType === 'history' ? address : '');

  const isLoading = infoLoading || balanceLoading || historyLoading;
  const data = infoData || balanceData || historyData;
  const error = infoError || balanceError || historyError;

  const title =
    viewType === 'info'
      ? 'Wallet Info'
      : viewType === 'balance'
      ? 'Balance'
      : 'Transaction History';

  return (
    <Transition appear show={true} as={Fragment}>
      <Dialog as="div" className="relative z-50" onClose={onClose}>
        <Transition.Child
          as={Fragment}
          enter="ease-out duration-300"
          enterFrom="opacity-0"
          enterTo="opacity-100"
          leave="ease-in duration-200"
          leaveFrom="opacity-100"
          leaveTo="opacity-0"
        >
          <div className="fixed inset-0 bg-black/50" />
        </Transition.Child>

        <div className="fixed inset-0 overflow-y-auto">
          <div className="flex min-h-full items-center justify-center p-4">
            <Transition.Child
              as={Fragment}
              enter="ease-out duration-300"
              enterFrom="opacity-0 scale-95"
              enterTo="opacity-100 scale-100"
              leave="ease-in duration-200"
              leaveFrom="opacity-100 scale-100"
              leaveTo="opacity-0 scale-95"
            >
              <Dialog.Panel className="w-full max-w-4xl transform overflow-hidden rounded-lg bg-gray-800 border border-gray-700 p-6 shadow-xl transition-all">
                <div className="flex items-center justify-between mb-4">
                  <Dialog.Title className="text-xl font-bold text-white">
                    {title}
                  </Dialog.Title>
                  <button
                    onClick={onClose}
                    className="text-gray-400 hover:text-white transition-colors"
                    aria-label="Close"
                  >
                    <svg
                      className="w-6 h-6"
                      fill="none"
                      stroke="currentColor"
                      viewBox="0 0 24 24"
                    >
                      <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M6 18L18 6M6 6l12 12"
                      />
                    </svg>
                  </button>
                </div>

                <div className="mb-4">
                  <p className="text-sm font-mono text-gray-400 break-all">{address}</p>
                </div>

                <div className="max-h-[70vh] overflow-y-auto">
                  {isLoading ? (
                    <LoadingSpinner />
                  ) : error ? (
                    <ErrorMessage error={error as Error} />
                  ) : data?.data ? (
                    <>
                      {viewType === 'history' && data.data.items ? (
                        <>
                          {data.data.items.length > 0 ? (
                            <>
                              <JsonViewer data={data.data.items} title={`${title} Details`} />
                              {data.data.total > 0 && (
                                <div className="mt-4 text-sm text-gray-400">
                                  Showing {data.data.items.length} of {data.data.total} transactions
                                </div>
                              )}
                            </>
                          ) : (
                            <div className="text-gray-400">No transactions found for this address.</div>
                          )}
                        </>
                      ) : (
                        <JsonViewer data={data.data} title={`${title} Details`} />
                      )}
                    </>
                  ) : (
                    <div className="text-gray-400">No data available</div>
                  )}
                </div>
              </Dialog.Panel>
            </Transition.Child>
          </div>
        </div>
      </Dialog>
    </Transition>
  );
}

