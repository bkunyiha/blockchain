import { Copy } from 'lucide-react';
import { useClipboard } from '../hooks/useClipboard';
import { formatBTC, truncateHash, formatDate, formatNumber } from '../lib/utils';

export interface CardItem {
  label: string;
  value: string | number;
  copyable?: boolean;
  format?: 'btc' | 'hash' | 'date' | 'number';
}

interface DataCardProps {
  title: string;
  items: CardItem[];
}

function DataCard({ title, items }: DataCardProps) {
  const { copy } = useClipboard();

  const formatValue = (value: string | number, format?: string): string => {
    if (format === 'btc' && typeof value === 'number') {
      return formatBTC(value);
    }
    if (format === 'hash' && typeof value === 'string') {
      return truncateHash(value);
    }
    if (format === 'date' && typeof value === 'string') {
      return formatDate(value);
    }
    if (format === 'number' && typeof value === 'number') {
      return formatNumber(value);
    }
    return String(value);
  };

  return (
    <div className="rounded-lg border border-slate-700 bg-slate-800 p-6">
      <h2 className="mb-6 text-lg font-bold">{title}</h2>

      <div className="grid gap-4 sm:grid-cols-2">
        {items.map((item, idx) => (
          <div
            key={idx}
            className="rounded-lg bg-slate-900 p-4"
          >
            <p className="mb-2 text-sm font-medium text-slate-400">
              {item.label}
            </p>
            <div className="flex items-center justify-between gap-2">
              <p className="break-all text-lg font-semibold text-slate-100">
                {formatValue(item.value, item.format)}
              </p>
              {item.copyable && (
                <button
                  onClick={() => copy(String(item.value))}
                  className="flex-shrink-0 rounded-lg bg-slate-800 p-2 text-slate-400 hover:bg-slate-700 hover:text-slate-200 transition-colors"
                  title="Copy"
                >
                  <Copy className="h-4 w-4" />
                </button>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

export default DataCard;
