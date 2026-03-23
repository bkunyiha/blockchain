import React, { useState } from "react";
import { Copy, Check, Trash2, Edit2, X } from "lucide-react";
import { WalletAddress } from "../types";
import { truncateAddress, formatDate, cn } from "../lib/utils";
import { writeText } from "@tauri-apps/plugin-clipboard-manager";
import { useToastStore } from "../store/toastStore";
import { useWalletStore } from "../store/walletStore";

interface WalletCardProps {
  wallet: WalletAddress;
  isActive: boolean;
  onSelect: () => void;
  onDelete: () => void;
  onRename: (newLabel: string) => void;
}

export default function WalletCard({ wallet, isActive, onSelect, onDelete, onRename }: WalletCardProps) {
  const [copied, setCopied] = useState(false);
  const [editing, setEditing] = useState(false);
  const [editLabel, setEditLabel] = useState(wallet.label || "");
  const [showDeleteConfirm, setShowDeleteConfirm] = useState(false);
  const addToast = useToastStore((s) => s.addToast);
  const theme = useWalletStore((s) => s.theme);

  const handleCopy = async (e: React.MouseEvent) => {
    e.stopPropagation();
    try {
      await writeText(wallet.address);
      setCopied(true);
      addToast("success", "Address copied to clipboard");
      setTimeout(() => setCopied(false), 2000);
    } catch {
      addToast("error", "Failed to copy address");
    }
  };

  const handleSaveRename = () => {
    onRename(editLabel);
    setEditing(false);
  };

  return (
    <div
      className={cn(
        "rounded-lg border p-4 transition-all cursor-pointer hover:shadow-md",
        isActive
          ? "border-blue-500 ring-1 ring-blue-500/20"
          : theme === "light"
            ? "border-gray-200 hover:border-gray-300"
            : "border-gray-700 hover:border-gray-600",
        theme === "light" ? "bg-white" : "bg-gray-800"
      )}
      onClick={onSelect}
    >
      {/* Label */}
      <div className="flex items-center justify-between mb-2">
        {editing ? (
          <div className="flex items-center gap-1 flex-1" onClick={(e) => e.stopPropagation()}>
            <input
              value={editLabel}
              onChange={(e) => setEditLabel(e.target.value)}
              className={cn("text-sm font-bold px-2 py-1 rounded border flex-1",
                theme === "light" ? "bg-white border-gray-300" : "bg-gray-700 border-gray-600"
              )}
              autoFocus
              onKeyDown={(e) => e.key === "Enter" && handleSaveRename()}
            />
            <button onClick={handleSaveRename} className="p-1 text-green-400 hover:text-green-300">
              <Check size={14} />
            </button>
            <button onClick={() => setEditing(false)} className="p-1 text-gray-400 hover:text-gray-300">
              <X size={14} />
            </button>
          </div>
        ) : (
          <>
            <span className="font-bold text-sm truncate">{wallet.label || "Unnamed Wallet"}</span>
            {isActive && (
              <span className="text-[10px] bg-blue-500/20 text-blue-400 px-2 py-0.5 rounded-full font-medium">
                Active
              </span>
            )}
          </>
        )}
      </div>

      {/* Address */}
      <div className="flex items-center gap-1 mb-2">
        <code className={cn("text-xs font-mono truncate", theme === "light" ? "text-gray-500" : "text-gray-400")}
          title={wallet.address}
        >
          {truncateAddress(wallet.address, 12)}
        </code>
        <button
          onClick={handleCopy}
          className="p-0.5 hover:text-bitcoin-orange transition-colors flex-shrink-0"
          title="Copy address"
        >
          {copied ? <Check size={12} className="text-green-400" /> : <Copy size={12} />}
        </button>
      </div>

      {/* Date */}
      <div className={cn("text-xs mb-3", theme === "light" ? "text-gray-400" : "text-gray-500")}
        title={wallet.created_at}
      >
        Created {formatDate(wallet.created_at)}
      </div>

      {/* Actions */}
      <div className="flex gap-2" onClick={(e) => e.stopPropagation()}>
        <button
          onClick={onSelect}
          className={cn(
            "flex-1 text-xs py-1.5 rounded-md font-medium transition-colors",
            isActive
              ? "bg-blue-600 text-white"
              : "bg-bitcoin-orange text-white hover:bg-bitcoin-orange-dark"
          )}
        >
          {isActive ? "Selected" : "Select"}
        </button>
        <button
          onClick={() => setEditing(true)}
          className={cn("p-1.5 rounded-md transition-colors",
            theme === "light" ? "hover:bg-gray-100" : "hover:bg-gray-700"
          )}
          title="Rename"
        >
          <Edit2 size={14} />
        </button>
        {showDeleteConfirm ? (
          <div className="flex gap-1">
            <button
              onClick={onDelete}
              className="px-2 py-1 text-xs bg-red-600 text-white rounded-md hover:bg-red-700"
            >
              Confirm
            </button>
            <button
              onClick={() => setShowDeleteConfirm(false)}
              className={cn("px-2 py-1 text-xs rounded-md", theme === "light" ? "bg-gray-200" : "bg-gray-700")}
            >
              Cancel
            </button>
          </div>
        ) : (
          <button
            onClick={() => setShowDeleteConfirm(true)}
            className="p-1.5 rounded-md hover:bg-red-900/30 text-red-400 transition-colors"
            title="Delete"
          >
            <Trash2 size={14} />
          </button>
        )}
      </div>
    </div>
  );
}
