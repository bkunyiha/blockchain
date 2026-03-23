import { useState, useMemo } from 'react';
import {
  ArrowUp,
  ArrowDown,
  Search,
  ChevronLeft,
  ChevronRight,
} from 'lucide-react';

export interface Column {
  key: string;
  label: string;
  sortable?: boolean;
  width?: string;
  render?: (value: any, row: any) => React.ReactNode;
}

interface DataTableProps {
  columns: Column[];
  data: any[];
  sortable?: boolean;
  searchable?: boolean;
  pageSize?: number;
  renderActions?: (row: any) => React.ReactNode;
}

function DataTable({
  columns,
  data,
  sortable = true,
  searchable = true,
  pageSize = 10,
  renderActions,
}: DataTableProps) {
  const [searchTerm, setSearchTerm] = useState('');
  const [sortKey, setSortKey] = useState<string | null>(null);
  const [sortOrder, setSortOrder] = useState<'asc' | 'desc'>('asc');
  const [currentPage, setCurrentPage] = useState(0);

  const filteredData = useMemo(() => {
    if (!searchTerm) return data;

    const term = searchTerm.toLowerCase();
    return data.filter((row) =>
      columns.some((col) => {
        const value = row[col.key];
        return String(value).toLowerCase().includes(term);
      })
    );
  }, [data, searchTerm, columns]);

  const sortedData = useMemo(() => {
    if (!sortKey || !sortable) return filteredData;

    return [...filteredData].sort((a, b) => {
      const aVal = a[sortKey];
      const bVal = b[sortKey];

      if (aVal === null || aVal === undefined) return 1;
      if (bVal === null || bVal === undefined) return -1;

      if (typeof aVal === 'number' && typeof bVal === 'number') {
        return sortOrder === 'asc' ? aVal - bVal : bVal - aVal;
      }

      const aStr = String(aVal).toLowerCase();
      const bStr = String(bVal).toLowerCase();
      return sortOrder === 'asc'
        ? aStr.localeCompare(bStr)
        : bStr.localeCompare(aStr);
    });
  }, [filteredData, sortKey, sortOrder, sortable]);

  const paginatedData = useMemo(() => {
    const start = currentPage * pageSize;
    return sortedData.slice(start, start + pageSize);
  }, [sortedData, currentPage, pageSize]);

  const totalPages = Math.ceil(sortedData.length / pageSize);

  const handleSort = (key: string) => {
    if (!sortable) return;

    if (sortKey === key) {
      setSortOrder(sortOrder === 'asc' ? 'desc' : 'asc');
    } else {
      setSortKey(key);
      setSortOrder('asc');
    }
    setCurrentPage(0);
  };

  if (data.length === 0) {
    return (
      <div className="rounded-lg border border-slate-700 bg-slate-800 p-8 text-center">
        <p className="text-slate-400">No data available</p>
      </div>
    );
  }

  return (
    <div className="rounded-lg border border-slate-700 bg-slate-800">
      {searchable && (
        <div className="border-b border-slate-700 p-4">
          <div className="relative">
            <Search className="absolute left-3 top-1/2 h-4 w-4 -translate-y-1/2 text-slate-500" />
            <input
              type="text"
              placeholder="Search..."
              value={searchTerm}
              onChange={(e) => {
                setSearchTerm(e.target.value);
                setCurrentPage(0);
              }}
              className="w-full rounded-lg border border-slate-600 bg-slate-700 pl-10 pr-4 py-2 text-slate-100 placeholder-slate-400 focus:border-yellow-500 focus:outline-none"
            />
          </div>
        </div>
      )}

      <div className="overflow-x-auto">
        <table className="w-full">
          <thead>
            <tr className="border-b border-slate-700 bg-slate-900">
              {columns.map((col) => (
                <th
                  key={col.key}
                  onClick={() => handleSort(col.key)}
                  className={`px-6 py-3 text-left text-sm font-semibold ${
                    col.sortable && sortable
                      ? 'cursor-pointer hover:bg-slate-800'
                      : ''
                  }`}
                  style={col.width ? { width: col.width } : {}}
                >
                  <div className="flex items-center gap-2">
                    {col.label}
                    {col.sortable && sortable && sortKey === col.key && (
                      <span>
                        {sortOrder === 'asc' ? (
                          <ArrowUp className="h-4 w-4" />
                        ) : (
                          <ArrowDown className="h-4 w-4" />
                        )}
                      </span>
                    )}
                  </div>
                </th>
              ))}
              {renderActions && <th className="px-6 py-3">Actions</th>}
            </tr>
          </thead>
          <tbody>
            {paginatedData.map((row, idx) => (
              <tr
                key={idx}
                className="border-b border-slate-700 hover:bg-slate-700/50"
              >
                {columns.map((col) => (
                  <td
                    key={col.key}
                    className="px-6 py-4 text-sm"
                    style={col.width ? { width: col.width } : {}}
                  >
                    {col.render
                      ? col.render(row[col.key], row)
                      : String(row[col.key] ?? '-')}
                  </td>
                ))}
                {renderActions && (
                  <td className="px-6 py-4">{renderActions(row)}</td>
                )}
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      {totalPages > 1 && (
        <div className="flex items-center justify-between border-t border-slate-700 px-6 py-4">
          <span className="text-sm text-slate-400">
            Page {currentPage + 1} of {totalPages}
          </span>
          <div className="flex gap-2">
            <button
              onClick={() =>
                setCurrentPage(Math.max(0, currentPage - 1))
              }
              disabled={currentPage === 0}
              className="rounded-lg bg-slate-700 p-2 disabled:opacity-50"
            >
              <ChevronLeft className="h-4 w-4" />
            </button>
            <button
              onClick={() =>
                setCurrentPage(Math.min(totalPages - 1, currentPage + 1))
              }
              disabled={currentPage === totalPages - 1}
              className="rounded-lg bg-slate-700 p-2 disabled:opacity-50"
            >
              <ChevronRight className="h-4 w-4" />
            </button>
          </div>
        </div>
      )}
    </div>
  );
}

export default DataTable;
