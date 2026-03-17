<div align="left">

<details>
<summary><b>Chapter Navigation ▼</b></summary>

### Part I: Foundations & Core Implementation

1. <a href="../01-Introduction.md">Chapter 1: Introduction & Overview</a>
2. <a href="../bitcoin-blockchain/README.md">Chapter 2: Introduction to Blockchain</a>
3. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Summary.md">Chapter 3: Bitcoin Whitepaper</a>
4. <a href="../bitcoin-blockchain/whitepaper-rust/00-Bitcoin-Whitepaper-Rust-Encoding-Summary.md">Chapter 4: Bitcoin Whitepaper In Rust</a>
5. <a href="../bitcoin-blockchain/Rust-Project-Index.md">Chapter 5: Rust Blockchain Project</a>
6. <a href="../bitcoin-blockchain/primitives/README.md">Chapter 6: Primitives</a>
7. <a href="../bitcoin-blockchain/util/README.md">Chapter 7: Utilities</a>
8. <a href="../bitcoin-blockchain/crypto/README.md">Chapter 8: Cryptography</a>
9. <a href="../bitcoin-blockchain/chain/README.md">Chapter 9: Blockchain (Technical Foundations)</a>
10. <a href="../bitcoin-blockchain/chain/10-Whitepaper-Step-5-Block-Acceptance.md">Chapter 10: Block Acceptance</a>
11. <a href="../bitcoin-blockchain/store/README.md">Chapter 11: Storage Layer</a>
12. <a href="../bitcoin-blockchain/net/README.md">Chapter 12: Network Layer</a>
13. <a href="../bitcoin-blockchain/node/README.md">Chapter 13: Node Orchestration</a>
14. <a href="../bitcoin-blockchain/wallet/README.md">Chapter 14: Wallet System</a>
15. <a href="../bitcoin-blockchain/web/README.md">Chapter 15: Web API Architecture</a>
16. <a href="../bitcoin-desktop-ui-iced/04.1-Desktop-Admin-UI-Iced.md">Chapter 16: Desktop Admin (Iced)</a>
17. <a href="../bitcoin-desktop-ui-iced/04.1A-Desktop-Admin-UI-Code-Walkthrough.md">16A: Code Walkthrough</a>
18. <a href="../bitcoin-desktop-ui-iced/04.1B-Desktop-Admin-UI-Update-Loop.md">16B: Update Loop</a>
19. <a href="../bitcoin-desktop-ui-iced/04.1C-Desktop-Admin-UI-View-Layer.md">16C: View Layer</a>
20. <a href="../bitcoin-desktop-ui-tauri/04.2-Desktop-Admin-UI-Tauri.md">Chapter 17: Desktop Admin (Tauri)</a>
21. <a href="../bitcoin-desktop-ui-tauri/04.2A-Tauri-Admin-Rust-Backend.md">17A: Rust Backend</a>
22. <a href="../bitcoin-desktop-ui-tauri/04.2B-Tauri-Admin-Frontend-Infrastructure.md">17B: Frontend Infrastructure</a>
23. <a href="../bitcoin-desktop-ui-tauri/04.2C-Tauri-Admin-Frontend-Pages.md">17C: Frontend Pages</a>
24. <a href="../bitcoin-wallet-ui-iced/05.1-Wallet-UI-Iced.md">Chapter 18: Wallet UI (Iced)</a>
25. <a href="../bitcoin-wallet-ui-iced/05.1A-Wallet-UI-Code-Listings.md">18A: Code Listings</a>
26. <a href="../bitcoin-wallet-ui-tauri/05.2-Wallet-UI-Tauri.md">Chapter 19: Wallet UI (Tauri)</a>
27. <a href="../bitcoin-wallet-ui-tauri/05.2A-Tauri-Wallet-Rust-Backend.md">19A: Rust Backend</a>
28. <a href="../bitcoin-wallet-ui-tauri/05.2B-Tauri-Wallet-Frontend-Infrastructure.md">19B: Frontend Infrastructure</a>
29. <a href="../bitcoin-wallet-ui-tauri/05.2C-Tauri-Wallet-Frontend-Pages.md">19C: Frontend Pages</a>
30. <a href="../embedded-database/06-Embedded-Database.md">Chapter 20: Embedded Database</a>
31. <a href="../embedded-database/06A-Embedded-Database-Code-Listings.md">20A: Code Listings</a>
32. <a href="06-Web-Admin-UI.md">Chapter 21: Web Admin Interface</a>
33. **21A: Code Listings** ← *You are here*

### Part II: Deployment & Operations

34. <a href="../ci/docker-compose/01-Introduction.md">Chapter 22: Docker Compose Deployment</a>
35. <a href="../ci/docker-compose/01A-Docker-Compose-Code-Listings.md">22A: Code Listings</a>
36. <a href="../ci/kubernetes/README.md">Chapter 23: Kubernetes Deployment</a>
37. <a href="../ci/kubernetes/01A-Kubernetes-Code-Listings.md">23A: Code Listings</a>

### Part III: Language Reference

38. <a href="../rust/README.md">Chapter 24: Rust Language Guide</a>

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../README.md)**

</div>

---

## Chapter 21A: Web Admin Interface — Complete Code Listings

This companion chapter contains **complete, verbatim listings** of the `bitcoin-web-ui` codebase files that Chapter 21 references.

Chapter 21 contains the narrative walkthrough (architecture, flow, and annotated readings). This chapter is kept **unmodified** so you can cross-check the exact implementation without opening the repository.

---

## How to use this chapter

Before each listing, you will find a short guide explaining:

- what the file is used for in the Web Admin UI,
- which parts are most important to understand,
- and where the corresponding behavior is handled elsewhere (routing, hooks, API client, etc.).

---

## Listing 7.1: `src/main.tsx`

This is the **browser entry point**. It is responsible for mounting React into the DOM and importing global styles (`index.css`).

Important to understand:

- The root element is `#root` (from `index.html`).
- `React.StrictMode` is enabled in development; it can cause some effects to run twice (by design) to surface side-effect bugs.
- All application wiring happens inside `App` (routing, providers, layout).

> **Methods involved**
> - `ReactDOM.createRoot(...).render(...)` (entrypoint call)

```typescript
import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css' // Global Tailwind layers + shared component classes

// Create a React 18 root and mount the application.
// The non-null assertion (`!`) reflects the contract of `index.html`: it must contain <div id="root" />.
ReactDOM.createRoot(document.getElementById('root')!).render(
  // StrictMode helps surface accidental side effects in development (it may double-invoke some lifecycles).
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)
```

---

## Listing 7.2: `src/App.tsx`

This is the **composition root** of the application. It wires together the major frameworks used by the UI:

- **React Router** (`BrowserRouter`, `Routes`, `Route`) for client-side routing.
- **React Query** (`QueryClientProvider`) for server-state caching and async lifecycle.
- **API configuration context** (`ApiConfigProvider`) so the user can set base URL and API key once for all requests.
- **Layout** wrapper (navbar + sidebar) so every screen shares consistent chrome.
- **Toasts** (`react-hot-toast`) for consistent feedback for mutations and errors.

Important to understand:

- This file defines the **routing table** of the UI (what screens exist and what URL path activates them).
- The `queryClient` options define global behaviors like retry policies and focus refetch.
- The providers establish the “ambient services” used everywhere (queries, API config).

> **Methods involved**
> - `App` (React component function)

```typescript
import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Toaster } from 'react-hot-toast';
import { ApiConfigProvider } from './contexts/ApiConfigContext';
import { Layout } from './components/Layout/Layout';
import { Home } from './pages/Home';

// Route components are organized by feature area (the URL table lives here).
import { BlockchainInfo } from './components/Blockchain/BlockchainInfo';
import { LatestBlocks } from './components/Blockchain/LatestBlocks';
import { AllBlocks } from './components/Blockchain/AllBlocks';
import { BlockByHash } from './components/Blockchain/BlockByHash';

import { CreateWallet } from './components/Wallet/CreateWallet';
import { WalletInfo } from './components/Wallet/WalletInfo';
import { Balance } from './components/Wallet/Balance';
import { SendTransaction } from './components/Wallet/SendTransaction';
import { TransactionHistory } from './components/Wallet/TransactionHistory';
import { AllAddresses } from './components/Wallet/AllAddresses';

import { Mempool } from './components/Transactions/Mempool';
import { MempoolTx } from './components/Transactions/MempoolTx';
import { AllTransactions } from './components/Transactions/AllTransactions';
import { AddressTransactions } from './components/Transactions/AddressTransactions';

import { MiningInfo } from './components/Mining/MiningInfo';
import { GenerateBlocks } from './components/Mining/GenerateBlocks';

import { HealthCheck } from './components/Health/HealthCheck';
import { Liveness } from './components/Health/Liveness';
import { Readiness } from './components/Health/Readiness';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      // Keep the UX responsive, but don’t hide persistent failures behind many retries.
      retry: 1,
      // Avoid surprising “UI changes while I’m reading” when the tab regains focus.
      refetchOnWindowFocus: false,
    },
  },
});

function App() {
  return (
    // React Query provides cache + loading/error lifecycle for server state.
    <QueryClientProvider client={queryClient}>
      {/* ApiConfigProvider persists baseURL/apiKey and reconfigures the API client singleton. */}
      <ApiConfigProvider>
        {/* BrowserRouter enables client-side routing (the Rust server serves index.html for non-API routes). */}
        <BrowserRouter>
          {/* Layout is the stable chrome around every route (navbar + sidebar). */}
          <Layout>
            <Routes>
              <Route path="/" element={<Home />} />
              
              {/* Blockchain routes */}
              <Route path="/blockchain/info" element={<BlockchainInfo />} />
              <Route path="/blockchain/latest" element={<LatestBlocks />} />
              <Route path="/blockchain/all" element={<AllBlocks />} />
              <Route path="/blockchain/hash" element={<BlockByHash />} />
              
              {/* Wallet routes */}
              <Route path="/wallet/create" element={<CreateWallet />} />
              <Route path="/wallet/info" element={<WalletInfo />} />
              <Route path="/wallet/balance" element={<Balance />} />
              <Route path="/wallet/send" element={<SendTransaction />} />
              <Route path="/wallet/history" element={<TransactionHistory />} />
              <Route path="/wallet/addresses" element={<AllAddresses />} />
              
              {/* Transaction routes */}
              <Route path="/transactions/mempool" element={<Mempool />} />
              <Route path="/transactions/mempool-tx" element={<MempoolTx />} />
              <Route path="/transactions/all" element={<AllTransactions />} />
              <Route path="/transactions/address" element={<AddressTransactions />} />
              
              {/* Mining routes */}
              <Route path="/mining/info" element={<MiningInfo />} />
              <Route path="/mining/generate" element={<GenerateBlocks />} />
              
              {/* Health routes */}
              <Route path="/health/check" element={<HealthCheck />} />
              <Route path="/health/liveness" element={<Liveness />} />
              <Route path="/health/readiness" element={<Readiness />} />
            </Routes>
          </Layout>
        </BrowserRouter>
        {/* A single global toaster instance keeps notifications consistent across the app. */}
        <Toaster
          position="top-right"
          toastOptions={{
            duration: 4000,
            style: {
              background: '#1f2937',
              color: '#fff',
              border: '1px solid #374151',
            },
            success: {
              iconTheme: {
                primary: '#f7931a',
                secondary: '#fff',
              },
            },
            error: {
              iconTheme: {
                primary: '#ef4444',
                secondary: '#fff',
              },
            },
          }}
        />
      </ApiConfigProvider>
    </QueryClientProvider>
  );
}

export default App;
```

---

## Listing 7.3: `src/contexts/ApiConfigContext.tsx`

This module defines **global API configuration** (base URL and API key) and persists it in `localStorage`.

Important to understand:

- The provider is the single authority for base URL and API key.
- The API client singleton is (re)created when config changes via `updateApiClient(...)`.
- `useApiConfig()` enforces correct usage by throwing if called outside the provider.

> **Methods involved**
> - `ApiConfigProvider`
> - `useApiConfig`

```typescript
import { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { updateApiClient } from '../services/api';

interface ApiConfigContextType {
  baseURL: string;
  apiKey: string;
  setBaseURL: (url: string) => void;
  setApiKey: (key: string) => void;
  isConfigured: boolean;
}

// `undefined` forces correct usage: consumers must be wrapped in ApiConfigProvider.
const ApiConfigContext = createContext<ApiConfigContextType | undefined>(undefined);

export function ApiConfigProvider({ children }: { children: ReactNode }) {
  // Initialize from localStorage so refreshes preserve configuration.
  const [baseURL, setBaseURLState] = useState(() => {
    return localStorage.getItem('api_base_url') || 'http://127.0.0.1:8080';
  });
  const [apiKey, setApiKeyState] = useState(() => {
    return localStorage.getItem('api_key') || '';
  });

  useEffect(() => {
    // Keep the API client singleton aligned with the latest config.
    // This matters because hooks/components call getApiClient() at request time.
    updateApiClient(baseURL, apiKey || undefined);
  }, [baseURL, apiKey]);

  const setBaseURL = (url: string) => {
    setBaseURLState(url);
    // Persist so the admin doesn’t have to retype on reload.
    localStorage.setItem('api_base_url', url);
    // Immediate reconfiguration so subsequent requests use the new URL.
    updateApiClient(url, apiKey || undefined);
  };

  const setApiKey = (key: string) => {
    setApiKeyState(key);
    if (key) {
      localStorage.setItem('api_key', key);
    } else {
      // Treat empty string as “no auth configured”.
      localStorage.removeItem('api_key');
    }
    // Immediate reconfiguration so subsequent requests include/remove auth.
    updateApiClient(baseURL, key || undefined);
  };

  return (
    <ApiConfigContext.Provider
      value={{
        baseURL,
        apiKey,
        setBaseURL,
        setApiKey,
        isConfigured: !!apiKey,
      }}
    >
      {children}
    </ApiConfigContext.Provider>
  );
}

export function useApiConfig() {
  const context = useContext(ApiConfigContext);
  if (context === undefined) {
    // This is a deliberate fail-fast: it prevents components from silently operating without config.
    throw new Error('useApiConfig must be used within an ApiConfigProvider');
  }
  return context;
}
```

---

## Listing 7.4: `src/services/api.ts`

This module is the **HTTP boundary** of the Web Admin UI:

- `ApiClient` wraps Axios and defines one method per backend endpoint.
- `getApiClient()` provides a lazily-created singleton configured from `localStorage`.
- `updateApiClient()` recreates that singleton when configuration changes.

Important to understand:

- The UI does not call `fetch` directly. It calls `getApiClient().someMethod(...)`.
- React Query hooks depend on these methods for actual HTTP transport.
- Authentication is handled via the `X-API-Key` header when configured.

> **Methods involved**
> - `ApiClient` constructor
> - `ApiClient` methods (one per endpoint)
> - `getApiClient`
> - `updateApiClient`

```typescript
import axios, { AxiosInstance } from 'axios';
import type {
  ApiResponse,
  BlockchainInfo,
  BlockSummary,
  CreateWalletRequest,
  CreateWalletResponse,
  SendTransactionRequest,
  SendTransactionResponse,
  JsonValue,
  MiningRequest,
  PaginatedResponse,
} from '../types/api';

export class ApiClient {
  private client: AxiosInstance;

  constructor(baseURL: string, apiKey?: string) {
    // Create a configured Axios instance so every request consistently shares base URL + headers.
    this.client = axios.create({
      baseURL,
      headers: {
        'Content-Type': 'application/json',
        // The Rust admin API expects `X-API-Key` for authorization when enabled.
        ...(apiKey && { 'X-API-Key': apiKey }),
      },
    });
  }

  updateApiKey(apiKey: string) {
    // Convenience: update default header for all future requests made through this Axios instance.
    this.client.defaults.headers['X-API-Key'] = apiKey;
  }

  removeApiKey() {
    // When the key is removed, omit the header entirely (not an empty string).
    delete this.client.defaults.headers['X-API-Key'];
  }

  // Blockchain endpoints
  async getBlockchainInfo(): Promise<ApiResponse<BlockchainInfo>> {
    const response = await this.client.get('/api/admin/blockchain');
    return response.data;
  }

  async getLatestBlocks(): Promise<ApiResponse<BlockSummary[]>> {
    const response = await this.client.get('/api/admin/blockchain/blocks/latest');
    return response.data;
  }

  async getAllBlocks(page?: number, limit?: number): Promise<ApiResponse<PaginatedResponse<BlockSummary>>> {
    // Build the query string explicitly to keep optional params out of the URL when undefined.
    const params = new URLSearchParams();
    if (page !== undefined) params.append('page', page.toString());
    if (limit !== undefined) params.append('limit', limit.toString());
    const queryString = params.toString();
    const url = `/api/admin/blockchain/blocks${queryString ? `?${queryString}` : ''}`;
    const response = await this.client.get(url);
    return response.data;
  }

  async getBlockByHash(hash: string): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get(`/api/admin/blockchain/blocks/${hash}`);
    return response.data;
  }

  // Wallet endpoints
  async createWallet(req: CreateWalletRequest): Promise<ApiResponse<CreateWalletResponse>> {
    const response = await this.client.post('/api/admin/wallet', req);
    return response.data;
  }

  async getAddresses(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/wallet/addresses');
    return response.data;
  }

  async getWalletInfo(address: string): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get(`/api/admin/wallet/${address}`);
    return response.data;
  }

  async getBalance(address: string): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get(`/api/admin/wallet/${address}/balance`);
    return response.data;
  }

  async sendTransaction(req: SendTransactionRequest): Promise<ApiResponse<SendTransactionResponse>> {
    const response = await this.client.post('/api/admin/transactions', req);
    return response.data;
  }

  async getAddressTransactions(address: string): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get(`/api/admin/transactions/address/${address}`);
    return response.data;
  }

  // Transaction endpoints
  async getMempool(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/transactions/mempool');
    return response.data;
  }

  async getMempoolTransaction(txid: string): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get(`/api/admin/transactions/mempool/${txid}`);
    return response.data;
  }

  async getAllTransactions(page: number = 1, limit: number = 100): Promise<ApiResponse<PaginatedResponse<JsonValue>>> {
    const response = await this.client.get('/api/admin/transactions', {
      params: { page, limit },
    });
    return response.data;
  }

  // Mining endpoints
  async getMiningInfo(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/mining/info');
    return response.data;
  }

  async generateToAddress(req: MiningRequest): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.post('/api/admin/mining/generatetoaddress', req);
    return response.data;
  }

  // Health endpoints
  async getHealth(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/health');
    return response.data;
  }

  async getLiveness(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/health/live');
    return response.data;
  }

  async getReadiness(): Promise<ApiResponse<JsonValue>> {
    const response = await this.client.get('/api/admin/health/ready');
    return response.data;
  }
}

// Singleton instance
let apiClient: ApiClient | null = null;

export function getApiClient(): ApiClient {
  if (!apiClient) {
    // Lazy initialize from persisted configuration so callers can just “get and use”.
    const baseURL = localStorage.getItem('api_base_url') || 'http://127.0.0.1:8080';
    const apiKey = localStorage.getItem('api_key') || undefined;
    apiClient = new ApiClient(baseURL, apiKey);
  }
  return apiClient;
}

export function updateApiClient(baseURL: string, apiKey?: string) {
  // Replace the singleton. This is the “bridge” between UI configuration and HTTP behavior.
  apiClient = new ApiClient(baseURL, apiKey);
  if (apiKey) {
    localStorage.setItem('api_key', apiKey);
  } else {
    localStorage.removeItem('api_key');
  }
  localStorage.setItem('api_base_url', baseURL);
}
```

---

## Listing 7.5: `src/hooks/useApi.ts`

This module defines the **application’s data access surface** as React Query hooks.

Important to understand:

- Components call hooks (e.g., `useBlockchainInfo`) rather than calling the API client directly.
- Hooks define **query keys** (cache identity) and **query functions** (what to fetch).
- Mutations (create wallet, send transaction, generate blocks) also define side effects:
  - cache invalidation,
  - toast notifications.

> **Methods involved**
> - `useBlockchainInfo`, `useLatestBlocks`, `useAllBlocks`, `useBlockByHash`
> - `useCreateWallet`, `useAddresses`, `useWalletInfo`, `useBalance`, `useSendTransaction`, `useAddressTransactions`
> - `useMempool`, `useMempoolTransaction`, `useAllTransactions`
> - `useMiningInfo`, `useGenerateBlocks`
> - `useHealth`, `useLiveness`, `useReadiness`

```typescript
import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { getApiClient } from '../services/api';
import type {
  CreateWalletRequest,
  SendTransactionRequest,
  MiningRequest,
} from '../types/api';
import toast from 'react-hot-toast';

// Blockchain queries
export function useBlockchainInfo(refetchInterval?: number) {
  return useQuery({
    // Query keys define cache identity; changing the key creates a distinct cached value.
    queryKey: ['blockchain', 'info'],
    // Query functions are the actual HTTP call (delegated to ApiClient).
    queryFn: () => getApiClient().getBlockchainInfo(),
    // Optional: drive dashboards / status screens without manual refresh.
    refetchInterval,
    retry: 1,
  });
}

export function useLatestBlocks() {
  return useQuery({
    queryKey: ['blockchain', 'latest-blocks'],
    queryFn: () => getApiClient().getLatestBlocks(),
    retry: 1,
  });
}

export function useAllBlocks(page?: number, limit?: number) {
  return useQuery({
    queryKey: ['blockchain', 'all-blocks', page, limit],
    queryFn: () => getApiClient().getAllBlocks(page, limit),
    // This screen is user-driven (“Load Page” / “Load All”), so we don’t fetch on mount.
    enabled: false,
    retry: 1,
  });
}

export function useBlockByHash(hash: string) {
  return useQuery({
    queryKey: ['blockchain', 'block', hash],
    queryFn: () => getApiClient().getBlockByHash(hash),
    enabled: !!hash,
    retry: 1,
  });
}

// Wallet queries
export function useCreateWallet() {
  // QueryClient lets us invalidate caches after mutations so list screens refetch fresh state.
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (req: CreateWalletRequest) => getApiClient().createWallet(req),
    onSuccess: () => {
      // Creating a wallet changes the address list; invalidate so consumers refetch.
      queryClient.invalidateQueries({ queryKey: ['wallet', 'addresses'] });
      toast.success('Wallet created successfully!');
    },
    onError: (error: any) => {
      // Axios errors often carry server payloads at error.response.data.
      toast.error(error.response?.data?.error || 'Failed to create wallet');
    },
  });
}

export function useAddresses() {
  return useQuery({
    queryKey: ['wallet', 'addresses'],
    queryFn: () => getApiClient().getAddresses(),
    retry: 1,
  });
}

export function useWalletInfo(address: string) {
  return useQuery({
    queryKey: ['wallet', 'info', address],
    queryFn: () => getApiClient().getWalletInfo(address),
    enabled: !!address,
    retry: 1,
  });
}

export function useBalance(address: string) {
  return useQuery({
    queryKey: ['wallet', 'balance', address],
    queryFn: () => getApiClient().getBalance(address),
    enabled: !!address,
    retry: 1,
  });
}

export function useSendTransaction() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (req: SendTransactionRequest) => getApiClient().sendTransaction(req),
    onSuccess: () => {
      // Sending changes both transaction views and wallet-related derived state.
      queryClient.invalidateQueries({ queryKey: ['transactions'] });
      queryClient.invalidateQueries({ queryKey: ['wallet'] });
      toast.success('Transaction sent successfully!');
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.error || 'Failed to send transaction');
    },
  });
}

export function useAddressTransactions(address: string) {
  return useQuery({
    queryKey: ['transactions', 'address', address],
    queryFn: () => getApiClient().getAddressTransactions(address),
    enabled: !!address,
    retry: 1,
  });
}

// Transaction queries
export function useMempool() {
  return useQuery({
    queryKey: ['transactions', 'mempool'],
    queryFn: () => getApiClient().getMempool(),
    retry: 1,
  });
}

export function useMempoolTransaction(txid: string) {
  return useQuery({
    queryKey: ['transactions', 'mempool', txid],
    queryFn: () => getApiClient().getMempoolTransaction(txid),
    enabled: !!txid,
    retry: 1,
  });
}

export function useAllTransactions(page: number = 1, limit: number = 100) {
  return useQuery({
    queryKey: ['transactions', 'all', page, limit],
    queryFn: () => getApiClient().getAllTransactions(page, limit),
    // Like AllBlocks, this screen loads on explicit user action.
    enabled: false,
    retry: 1,
  });
}

// Mining queries
export function useMiningInfo() {
  return useQuery({
    queryKey: ['mining', 'info'],
    queryFn: () => getApiClient().getMiningInfo(),
    retry: 1,
  });
}

export function useGenerateBlocks() {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: (req: MiningRequest) => getApiClient().generateToAddress(req),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['blockchain'] });
      queryClient.invalidateQueries({ queryKey: ['mining'] });
      toast.success('Blocks generated successfully!');
    },
    onError: (error: any) => {
      toast.error(error.response?.data?.error || 'Failed to generate blocks');
    },
  });
}

// Health queries
export function useHealth() {
  return useQuery({
    queryKey: ['health'],
    queryFn: () => getApiClient().getHealth(),
    retry: 1,
  });
}

export function useLiveness() {
  return useQuery({
    queryKey: ['health', 'liveness'],
    queryFn: () => getApiClient().getLiveness(),
    retry: 1,
  });
}

export function useReadiness() {
  return useQuery({
    queryKey: ['health', 'readiness'],
    queryFn: () => getApiClient().getReadiness(),
    retry: 1,
  });
}
```

---

## Listing 7.6: `src/components/Layout/Layout.tsx`

This is the **top-level layout** that wraps every routed page.

Important to understand:

- It establishes the “chrome”: `Navbar` on top, `Sidebar` on the left, and the main content area.
- It does not fetch data; it is purely structural.

> **Methods involved**
> - `Layout`

```tsx
import { ReactNode } from 'react';
import { Navbar } from './Navbar';
import { Sidebar } from './Sidebar';

interface LayoutProps {
  children: ReactNode;
}

export function Layout({ children }: LayoutProps) {
  return (
    // Layout is intentionally “dumb”: no data fetching, only structure and styling.
    <div className="min-h-screen bg-gray-900">
      {/* Navbar hosts API configuration and global actions. */}
      <Navbar />
      <div className="flex">
        {/* Sidebar provides navigation between feature areas. */}
        <Sidebar />
        {/* Routed pages render here. */}
        <main className="flex-1 p-8">
          {children}
        </main>
      </div>
    </div>
  );
}
```

---

## Listing 7.7: `src/components/Layout/Navbar.tsx`

The navbar provides the **API configuration UX**: a “Configure API” menu where the user sets base URL and API key.

Important to understand:

- It uses `useApiConfig()` (context) to read and write config.
- It uses Headless UI’s `Menu` + `Transition` for an accessible dropdown.
- Writing config updates `localStorage` and recreates the API client singleton.

> **Methods involved**
> - `Navbar`

```tsx
import { useApiConfig } from '../../contexts/ApiConfigContext';
import { Menu, Transition } from '@headlessui/react';
import { Fragment } from 'react';

export function Navbar() {
  // Read + write global API configuration (persisted in localStorage).
  const { baseURL, apiKey, setBaseURL, setApiKey } = useApiConfig();

  return (
    <nav className="bg-gray-800 border-b border-gray-700">
      <div className="max-w-7xl mx-auto px-4 sm:px-20 lg:px-8">
        <div className="flex items-center justify-between h-16">
          <div className="flex items-center">
            <h1 className="text-xl font-bold text-bitcoin-orange">Bitcoin Blockchain Admin</h1>
          </div>
          
          <div className="flex items-center gap-4">
            <Menu as="div" className="relative">
              <Menu.Button className="btn-secondary text-sm">
                Configure API
              </Menu.Button>
              <Transition
                as={Fragment}
                // Headless UI transition classes provide accessible animated dropdown behavior.
                enter="transition ease-out duration-100"
                enterFrom="transform opacity-0 scale-95"
                enterTo="transform opacity-100 scale-100"
                leave="transition ease-in duration-75"
                leaveFrom="transform opacity-100 scale-100"
                leaveTo="transform opacity-0 scale-95"
              >
                <Menu.Items className="absolute right-0 mt-2 w-80 bg-gray-800 border border-gray-700 rounded-lg shadow-lg p-4 z-50">
                  <div className="space-y-4">
                    <div>
                      <label className="block text-sm text-gray-300 mb-1">Base URL</label>
                      <input
                        type="text"
                        value={baseURL}
                        // This updates state + localStorage + ApiClient singleton via ApiConfigContext.
                        onChange={(e) => setBaseURL(e.target.value)}
                        className="input-field"
                        placeholder="http://127.0.0.1:8080"
                      />
                    </div>
                    <div>
                      <label className="block text-sm text-gray-300 mb-1">API Key</label>
                      <input
                        type="password"
                        value={apiKey}
                        // Set empty string to “deconfigure” authentication.
                        onChange={(e) => setApiKey(e.target.value)}
                        className="input-field"
                        placeholder="admin-secret"
                      />
                    </div>
                    {apiKey && (
                      <div className="text-xs text-green-400">
                        ✓ API configured
                      </div>
                    )}
                  </div>
                </Menu.Items>
              </Transition>
            </Menu>
          </div>
        </div>
      </div>
    </nav>
  );
}
```

---

## Listing 7.8: `src/components/Layout/Sidebar.tsx`

The sidebar provides **navigation** and a persistent UX: dropdown sections remain open when navigating within their own subtree.

Important to understand:

- The `navItems` table defines the information architecture.
- `openMenuPath` is derived from `location.pathname` so reloads/deep links open the correct menu.
- This is UI state only; all data fetching remains in routed components via hooks.

> **Methods involved**
> - `Sidebar`

```tsx
import { Link, useLocation } from 'react-router-dom';
import { Transition } from '@headlessui/react';
import { Fragment, useState, useEffect } from 'react';
import clsx from 'clsx';

interface NavItem {
  name: string;
  path: string;
  subItems?: { name: string; path: string }[];
}

// The navigation table is the “information architecture” of the admin UI.
// It is used both for rendering and for determining which dropdown should be open.
const navItems: NavItem[] = [
  { name: 'Dashboard', path: '/' },
  {
    name: 'Blockchain',
    path: '/blockchain',
    subItems: [
      { name: 'Info', path: '/blockchain/info' },
      { name: 'Latest Blocks', path: '/blockchain/latest' },
      { name: 'All Blocks', path: '/blockchain/all' },
      { name: 'Block by Hash', path: '/blockchain/hash' },
    ],
  },
  {
    name: 'Wallet',
    path: '/wallet',
    subItems: [
      { name: 'Create Wallet', path: '/wallet/create' },
      { name: 'Wallet Info', path: '/wallet/info' },
      { name: 'Balance', path: '/wallet/balance' },
      { name: 'Send Bitcoin', path: '/wallet/send' },
      { name: 'Transaction History', path: '/wallet/history' },
      { name: 'All Addresses', path: '/wallet/addresses' },
    ],
  },
  {
    name: 'Transactions',
    path: '/transactions',
    subItems: [
      { name: 'Mempool', path: '/transactions/mempool' },
      { name: 'Mempool Transaction', path: '/transactions/mempool-tx' },
      { name: 'All Transactions', path: '/transactions/all' },
      { name: 'Address Transactions', path: '/transactions/address' },
    ],
  },
  {
    name: 'Mining',
    path: '/mining',
    subItems: [
      { name: 'Mining Info', path: '/mining/info' },
      { name: 'Generate Blocks', path: '/mining/generate' },
    ],
  },
  {
    name: 'Health',
    path: '/health',
    subItems: [
      { name: 'Health Check', path: '/health/check' },
      { name: 'Liveness', path: '/health/liveness' },
      { name: 'Readiness', path: '/health/readiness' },
    ],
  },
];

export function Sidebar() {
  const location = useLocation();
  
  // Determine which menu should be open based on current location
  const getOpenMenuPath = (): string | null => {
    for (const item of navItems) {
      if (item.subItems) {
        // Check if current path matches this menu or any of its sub-items
        if (location.pathname === item.path || 
            item.subItems.some(sub => location.pathname === sub.path)) {
          return item.path;
        }
      }
    }
    return null;
  };

  // `openMenuPath` is derived from the URL so deep links keep the correct menu expanded.
  const [openMenuPath, setOpenMenuPath] = useState<string | null>(getOpenMenuPath());

  // Update open menu when location changes
  useEffect(() => {
    const newOpenMenuPath = getOpenMenuPath();
    setOpenMenuPath(newOpenMenuPath);
  }, [location.pathname]);

  return (
    <aside className="w-64 bg-gray-800 border-r border-gray-700 min-h-screen">
      <nav className="p-4 space-y-2">
        {navItems.map((item) => {
          const isActive = location.pathname === item.path || 
            (item.subItems && item.subItems.some(sub => location.pathname === sub.path));
          
          if (item.subItems) {
            const isOpen = openMenuPath === item.path;
            
            return (
              <div key={item.path} className="relative">
                <button
                  onClick={() => {
                    // Toggle menu, but keep it open if navigating within same menu
                    if (isOpen && openMenuPath === item.path) {
                      // If clicking the same menu button while it's open, keep it open
                      // (only close when navigating to different main menu)
                      return;
                    }
                    setOpenMenuPath(isOpen ? null : item.path);
                  }}
                  className={clsx(
                    'w-full text-left px-4 py-2 rounded-lg transition-colors',
                    isActive
                      ? 'bg-bitcoin-orange/20 text-bitcoin-orange border border-bitcoin-orange/30'
                      : 'text-gray-300 hover:bg-gray-700'
                  )}
                >
                  {item.name}
                </button>
                <Transition
                  as={Fragment}
                  show={isOpen}
                  // Transition makes dropdown open/close smooth but keeps DOM structure accessible.
                  enter="transition ease-out duration-100"
                  enterFrom="transform opacity-0 scale-95"
                  enterTo="transform opacity-100 scale-100"
                  leave="transition ease-in duration-75"
                  leaveFrom="transform opacity-100 scale-100"
                  leaveTo="transform opacity-0 scale-95"
                >
                  <div className="mt-1 ml-4 space-y-1">
                    {item.subItems.map((subItem) => {
                      const isSubItemActive = location.pathname === subItem.path;
                      return (
                        <Link
                          key={subItem.path}
                          to={subItem.path}
                          onClick={() => {
                            // Keep menu open when clicking sub-items
                            setOpenMenuPath(item.path);
                          }}
                          className={clsx(
                            'block px-4 py-2 rounded-lg text-sm transition-colors',
                            isSubItemActive
                              ? 'bg-bitcoin-orange/20 text-bitcoin-orange'
                              : 'text-gray-400 hover:bg-gray-700 hover:text-gray-200'
                          )}
                        >
                          {subItem.name}
                        </Link>
                      );
                    })}
                  </div>
                </Transition>
              </div>
            );
          }

          return (
            <Link
              key={item.path}
              to={item.path}
              className={clsx(
                'block px-4 py-2 rounded-lg transition-colors',
                isActive
                  ? 'bg-bitcoin-orange/20 text-bitcoin-orange border border-bitcoin-orange/30'
                  : 'text-gray-300 hover:bg-gray-700'
              )}
            >
              {item.name}
            </Link>
          );
        })}
      </nav>
    </aside>
  );
}
```

---

## Listing 7.9: `src/pages/Home.tsx`

The home route is intentionally thin: it renders the `Dashboard`.

> **Methods involved**
> - `Home`

```tsx
import { Dashboard } from '../components/Dashboard/Dashboard';

export function Home() {
  // Home is intentionally a thin alias for the dashboard route.
  return <Dashboard />;
}
```

---

## Listing 7.10: `src/components/Dashboard/Dashboard.tsx`

The dashboard is the canonical example of the “React Query + stat cards” pattern:

- fetch data via a hook (`useBlockchainInfo`),
- display loading/error states via shared components,
- render the result.

Important to understand:

- `useBlockchainInfo(5000)` uses a refetch interval to auto-refresh every 5 seconds.
- The view does not call the API client directly; it relies on `useApi` hooks.
- Date formatting is delegated to `formatDate` for robustness.

> **Methods involved**
> - `Dashboard`

```tsx
import { useBlockchainInfo } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { StatCard } from '../common/StatCard';
import { formatDate } from '../../utils/date';

export function Dashboard() {
  // Server state is managed by React Query; the component simply renders based on its status.
  const { data, error, isLoading } = useBlockchainInfo(5000); // Auto-refresh every 5 seconds

  if (isLoading) {
    // Standard loading state: keep the UX consistent across pages.
    return <LoadingSpinner />;
  }

  if (error) {
    // Standard error state: present a readable message.
    return <ErrorMessage error={error as Error} />;
  }

  // The Rust API wraps payloads as ApiResponse<T>; `data.data` is the actual value.
  const info = data?.data;

  if (!info) {
    // Defensive: if the request succeeded but contains no payload, render a clear fallback.
    return <div className="text-gray-400">No blockchain data available</div>;
  }

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Dashboard</h1>
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-20 mb-8">
        <StatCard
          label="Block Height"
          value={info.height}
          icon={<span className="text-2xl">📊</span>}
        />
        <StatCard
          label="Total Blocks"
          value={info.total_blocks}
          icon={<span className="text-2xl">🧱</span>}
        />
        <StatCard
          label="Total Transactions"
          value={info.total_transactions}
          icon={<span className="text-2xl">💸</span>}
        />
        <StatCard
          label="Mempool Size"
          value={info.mempool_size}
          icon={<span className="text-2xl">⏳</span>}
        />
      </div>
      
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        <div className="card">
          <h2 className="text-lg font-semibold text-white mb-4">Blockchain Info</h2>
          <div className="space-y-2 text-sm">
            <div className="flex justify-between">
              <span className="text-gray-400">Difficulty:</span>
              <span className="text-white font-mono">{info.difficulty}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Last Block Hash:</span>
              <span className="text-white font-mono text-xs break-all">{info.last_block_hash}</span>
            </div>
            <div className="flex justify-between">
              <span className="text-gray-400">Last Block Time:</span>
              {/* formatDate is defensive against malformed timestamps coming from the backend. */}
              <span className="text-white">{formatDate(info.last_block_timestamp)}</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
```

---

## Listing 7.11: `src/components/common/JsonViewer.tsx`

This shared component is how the UI displays “raw JSON” payloads in a readable, copyable form.

Important to understand:

- The component computes `jsonString` once per render and displays it in a `<pre>`.
- The user can expand/collapse height and copy to clipboard.
- Toast notifications confirm copy actions.

> **Methods involved**
> - `JsonViewer`
> - `copyToClipboard` (local helper)

```tsx
import { useState } from 'react';
import toast from 'react-hot-toast';

interface JsonViewerProps {
  data: any;
  title?: string;
}

export function JsonViewer({ data, title }: JsonViewerProps) {
  // Expand/collapse only changes presentation height; it does not change the underlying data.
  const [isExpanded, setIsExpanded] = useState(false);
  // Prettify JSON for readability. For very large payloads, this can be expensive, but it is acceptable here.
  const jsonString = JSON.stringify(data, null, 2);

  const copyToClipboard = () => {
    // Clipboard API writes plain text; we confirm the action via a toast.
    navigator.clipboard.writeText(jsonString);
    toast.success('Copied to clipboard!');
  };

  return (
    <div className="bg-gray-800 rounded-lg border border-gray-700">
      {title && (
        <div className="flex items-center justify-between px-4 py-2 border-b border-gray-700">
          <h3 className="text-sm font-semibold text-gray-300">{title}</h3>
          <div className="flex gap-2">
            <button
              onClick={() => setIsExpanded(!isExpanded)}
              className="px-15 py-1 text-xs bg-gray-700 hover:bg-gray-600 rounded text-gray-300"
            >
              {/* Toggling changes only the max height of the scroll container below. */}
              {isExpanded ? 'Collapse' : 'Expand'}
            </button>
            <button
              onClick={copyToClipboard}
              className="px-15 py-1 text-xs bg-bitcoin-orange hover:bg-bitcoin-gold rounded text-white"
            >
              Copy
            </button>
          </div>
        </div>
      )}
      <div className={`overflow-auto ${isExpanded ? 'max-h-[600px]' : 'max-h-[300px]'}`}>
        <pre className="p-4 text-xs text-gray-300 font-mono">
          {jsonString}
        </pre>
      </div>
    </div>
  );
}
```

---

## Listing 7.12: `src/components/common/LoadingSpinner.tsx`

Simple shared component used by nearly every screen to show “query is in flight”.

> **Methods involved**
> - `LoadingSpinner`

```tsx
export function LoadingSpinner() {
  return (
    <div className="flex items-center justify-center p-8">
      <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-bitcoin-orange"></div>
    </div>
  );
}
```

---

## Listing 7.13: `src/components/common/ErrorMessage.tsx`

Shared error presentation. It converts either an `Error` or a string into a display message.

> **Methods involved**
> - `ErrorMessage`

```tsx
interface ErrorMessageProps {
  error: Error | string;
}

export function ErrorMessage({ error }: ErrorMessageProps) {
  const message = error instanceof Error ? error.message : error;
  return (
    <div className="bg-red-900/20 border border-red-500/50 rounded-lg p-4 text-red-200">
      <p className="font-semibold">Error</p>
      <p className="text-sm mt-1">{message}</p>
    </div>
  );
}
```

---

## Listing 7.14: `src/components/common/StatCard.tsx`

Small reusable component used on the dashboard for consistent “label + value” rendering.

> **Methods involved**
> - `StatCard`

```tsx
interface StatCardProps {
  label: string;
  value: string | number;
  icon?: React.ReactNode;
}

export function StatCard({ label, value, icon }: StatCardProps) {
  return (
    // Stateless “presentational component”: renders consistent UI for (label, value, optional icon).
    <div className="bg-gray-800 rounded-lg border border-gray-700 p-20 hover:border-bitcoin-orange/50 transition-colors">
      <div className="flex items-center justify-between">
        <div>
          <p className="text-sm text-gray-400 mb-1">{label}</p>
          <p className="text-2xl font-bold text-white">{value}</p>
        </div>
        {icon && <div className="text-bitcoin-orange">{icon}</div>}
      </div>
    </div>
  );
}
```

---

## Listing 7.15: `src/utils/date.ts`

This utility exists because the UI consumes timestamps from a Rust backend, and production UIs must be resilient to malformed or unexpected date formats.

Important to understand:

- `formatDate` is defensive: it returns `N/A`, `Invalid Date`, or `Date format error` rather than throwing.
- The code includes a guard for a previously observed backend bug where a date’s year becomes unreasonably large.
- `formatDateWithTimezone` provides a “presentation-grade” format with timezone name.

> **Methods involved**
> - `formatDate`
> - `formatDateWithTimezone`

```typescript
/**
 * Safely format a date string or Date object to a localized string
 * @param date - Date string (ISO 8601) or Date object
 * @returns Formatted date string or "Invalid Date" if parsing fails
 */
export function formatDate(date: string | Date | null | undefined): string {
  if (!date) {
    return 'N/A';
  }

  try {
    let dateObj: Date;
    
    if (typeof date === 'string') {
      // Check if it's a malformed date (like "+57878-04-29T11:29:35Z" from backend bug)
      // This happens when milliseconds are treated as seconds
      if (date.startsWith('+') && date.includes('-')) {
        // Try to extract and fix: if year > 3000, it's likely milliseconds treated as seconds
        const yearMatch = date.match(/\+(\d+)-/);
        if (yearMatch) {
          // If year is unreasonably large (> 3000), this is likely a bug
          // For now, just show the raw timestamp or a fallback
          console.warn('Invalid date format detected (likely backend bug):', date);
          return 'Date format error';
        }
      }
      
      dateObj = new Date(date);
    } else {
      dateObj = date;
    }
    
    // Check if date is valid
    if (isNaN(dateObj.getTime())) {
      // Try to parse as Unix timestamp (milliseconds)
      if (typeof date === 'string') {
        const num = parseInt(date);
        if (!isNaN(num) && num > 0) {
          const timestampDate = new Date(num);
          if (!isNaN(timestampDate.getTime())) {
            return timestampDate.toLocaleString();
          }
        }
      }
      return 'Invalid Date';
    }

    // Check if date is unreasonably far in the future (likely a bug)
    const year = dateObj.getFullYear();
    if (year > 3000) {
      console.warn('Date is unreasonably far in the future (likely backend bug):', date, '->', dateObj);
      return 'Date format error';
    }

    return dateObj.toLocaleString();
  } catch (error) {
    console.error('Error formatting date:', error, date);
    return 'Invalid Date';
  }
}

/**
 * Format a date string to a more readable format
 * @param date - Date string (ISO 8601) or Date object
 * @returns Formatted date string with timezone
 */
export function formatDateWithTimezone(date: string | Date | null | undefined): string {
  if (!date) {
    return 'N/A';
  }

  try {
    const dateObj = typeof date === 'string' ? new Date(date) : date;
    
    // Check if date is valid
    if (isNaN(dateObj.getTime())) {
      return 'Invalid Date';
    }

    return dateObj.toLocaleString('en-US', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      timeZoneName: 'short',
    });
  } catch (error) {
    console.error('Error formatting date:', error, date);
    return 'Invalid Date';
  }
}
```

---

## Listing 7.16: `src/types/api.ts`

This file defines the **TypeScript mirror** of the Rust server’s response types. Its job is to make:

- hook return values type-safe,
- component rendering predictable,
- and refactoring safer (types are checked at compile time).

Important to understand:

- `ApiResponse<T>` matches the server wrapper (success/data/error/timestamp).
- `PaginatedResponse<T>` encodes pagination metadata used by UI components.

> **Methods involved**
> - Type definitions (interfaces and type aliases)

```typescript
// API Types matching Rust API responses

export interface ApiResponse<T> {
  // The Rust API wraps all payloads in this envelope for consistent success/error handling.
  success: boolean;
  data?: T;
  error?: string;
  timestamp: string;
}

export interface BlockchainInfo {
  height: number;
  difficulty: number;
  total_blocks: number;
  total_transactions: number;
  mempool_size: number;
  last_block_hash: string;
  last_block_timestamp: string;
}

export interface BlockSummary {
  hash: string;
  previous_hash: string;
  timestamp: string;
  height: number;
  nonce: number;
  difficulty: number;
  transaction_count: number;
  merkle_root: string;
  size_bytes: number;
}

export interface CreateWalletRequest {
  label?: string;
}

export interface CreateWalletResponse {
  address: string;
}

export interface SendTransactionRequest {
  from_address: string;
  to_address: string;
  amount: number;
}

export interface SendTransactionResponse {
  txid: string;
}

export interface BalanceResponse {
  address: string;
  confirmed: number;
  unconfirmed: number;
}

export interface MiningRequest {
  address: string;
  nblocks: number;
  maxtries?: number;
}

export type JsonValue = any;

export interface PaginatedResponse<T> {
  // `items` is the slice for the requested page.
  items: T[];
  // Pagination metadata supports UI paging controls and “load all pages” loops.
  page: number;
  limit: number;
  total: number;
  total_pages: number;
  has_next: boolean;
  has_prev: boolean;
}
```

---

## Listing 7.17: `src/components/Blockchain/BlockchainInfo.tsx`

“Fetch + refresh button + JSON viewer” is the simplest screen pattern. This component demonstrates:

- hook-driven data fetching (`useBlockchainInfo`),
- explicit refetch on demand,
- structured loading/error handling.

> **Methods involved**
> - `BlockchainInfo`

```tsx
import { useBlockchainInfo } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function BlockchainInfo() {
  // React Query returns a stable shape: loading/error/data plus imperative helpers like refetch().
  const { data, error, isLoading, refetch } = useBlockchainInfo();

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">Blockchain Info</h1>
        {/* Explicit refetch avoids forcing users to reload the whole SPA. */}
        <button onClick={() => refetch()} className="btn-primary">
          Refresh
        </button>
      </div>
      
      {/* Only render when the payload exists; the hook returns ApiResponse<T>. */}
      {data?.data && <JsonViewer data={data.data} title="Blockchain Information" />}
    </div>
  );
}
```

---

## Listing 7.18: `src/components/Blockchain/LatestBlocks.tsx`

This screen demonstrates presenting a list view (latest blocks) while still providing access to raw JSON.

Important to understand:

- The list view is human-friendly; JSON viewer is an “escape hatch” for debugging and inspection.
- `formatDate` is used so timestamps are robustly displayed.

> **Methods involved**
> - `LatestBlocks`

```tsx
import { useLatestBlocks } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';
import { formatDate } from '../../utils/date';

export function LatestBlocks() {
  const { data, error, isLoading, refetch } = useLatestBlocks();

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  // The backend returns `BlockSummary[]` as the payload for this endpoint.
  const blocks = data?.data || [];

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">Latest Blocks</h1>
        <button onClick={() => refetch()} className="btn-primary">
          Refresh
        </button>
      </div>
      
      {blocks.length > 0 ? (
        <div className="space-y-4">
          {blocks.map((block) => (
            <div key={block.hash} className="card">
              <div className="flex items-start justify-between">
                <div className="flex-1">
                  <div className="flex items-center gap-4 mb-2">
                    <span className="text-bitcoin-orange font-semibold">Block #{block.height}</span>
                    <span className="text-sm text-gray-400">
                      {/* Timestamp formatting is defensive because the backend may emit unexpected formats. */}
                      {formatDate(block.timestamp)}
                    </span>
                  </div>
                  <div className="text-sm space-y-1">
                    <div>
                      <span className="text-gray-400">Hash: </span>
                      <span className="text-gray-300 font-mono text-xs break-all">{block.hash}</span>
                    </div>
                    <div>
                      <span className="text-gray-400">Transactions: </span>
                      <span className="text-white">{block.transaction_count}</span>
                    </div>
                    <div>
                      <span className="text-gray-400">Difficulty: </span>
                      <span className="text-white">{block.difficulty}</span>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          ))}
        </div>
      ) : (
        <div className="text-gray-400">No blocks found</div>
      )}
      
      {blocks.length > 0 && (
        <div className="mt-6">
          {/* Raw JSON is useful for debugging and for readers comparing fields to the API schema. */}
          <JsonViewer data={blocks} title="Raw JSON Data" />
        </div>
      )}
    </div>
  );
}
```

---

## Listing 7.19: `src/components/Blockchain/AllBlocks.tsx`

This screen demonstrates a “load on demand” query:

- the hook is created with `enabled: false`,
- and data is fetched when the user clicks “Load Page” or “Load All Blocks”.

Important to understand:

- `handleLoadAll` iterates pages by calling the API client directly. This is a conscious escape hatch for bulk loading that does not rely on React Query paging.
- The component keeps its own `allBlocks` state for “bulk loaded” datasets.

> **Methods involved**
> - `AllBlocks`
> - `handleLoad` (local helper)
> - `handleLoadAll` (local helper)

```tsx
import { useState, useEffect } from 'react';
import { useAllBlocks } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';
import type { BlockSummary } from '../../types/api';
import { getApiClient } from '../../services/api';

export function AllBlocks() {
  const [allBlocks, setAllBlocks] = useState<BlockSummary[]>([]);
  const [currentPage] = useState(0);
  const [limit] = useState(100); // Fetch 100 blocks per page
  const { data, error, isLoading, refetch } = useAllBlocks(currentPage, limit);
  const [isLoadingData, setIsLoadingData] = useState(false);
  const [isLoadingAll, setIsLoadingAll] = useState(false);

  const handleLoad = async () => {
    setIsLoadingData(true);
    await refetch();
    setIsLoadingData(false);
  };

  const handleLoadAll = async () => {
    setIsLoadingAll(true);
    setAllBlocks([]);
    
    try {
      const allBlocksData: BlockSummary[] = [];
      let page = 0;
      let hasMore = true;

      while (hasMore) {
        const response = await getApiClient().getAllBlocks(page, limit);
        if (response.data?.items) {
          allBlocksData.push(...response.data.items);
          hasMore = response.data.has_next;
          page++;
        } else {
          hasMore = false;
        }
      }

      setAllBlocks(allBlocksData);
    } catch (err) {
      console.error('Error loading all blocks:', err);
    } finally {
      setIsLoadingAll(false);
    }
  };

  // Update allBlocks when data changes
  useEffect(() => {
    if (data?.data?.items) {
      setAllBlocks(data.data.items);
    }
  }, [data]);

  if (isLoading || isLoadingData) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  const paginatedData = data?.data;
  const blocksToShow = allBlocks.length > 0 ? allBlocks : paginatedData?.items || [];

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">All Blocks</h1>
        <div className="flex gap-2">
          <button onClick={handleLoad} className="btn-primary" disabled={isLoading}>
            Load Page
          </button>
          <button onClick={handleLoadAll} className="btn-primary" disabled={isLoadingAll}>
            {isLoadingAll ? 'Loading All...' : 'Load All Blocks'}
          </button>
        </div>
      </div>

      {paginatedData && (
        <div className="mb-4 text-sm text-gray-400">
          Showing page {paginatedData.page + 1} of {paginatedData.total_pages} 
          ({paginatedData.total} total blocks)
        </div>
      )}
      
      {blocksToShow.length > 0 ? (
        <JsonViewer data={blocksToShow} title={allBlocks.length > 0 ? `All Blocks (${allBlocks.length})` : 'Blocks'} />
      ) : (
        <div className="text-gray-400">Click "Load All Blocks" to fetch data</div>
      )}
    </div>
  );
}
```

---

## Listing 7.20: `src/components/Blockchain/BlockByHash.tsx`

This screen demonstrates a “query parameterized by local state” pattern:

- the component owns the input field (`hash`),
- the hook is `enabled: !!hash` but the UI still uses `refetch()` on demand.

> **Methods involved**
> - `BlockByHash`
> - `handleSearch` (local helper)

```tsx
import { useState } from 'react';
import { useBlockByHash } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function BlockByHash() {
  // Local input state; the query depends on this value.
  const [hash, setHash] = useState('');
  const { data, error, isLoading, refetch } = useBlockByHash(hash);

  const handleSearch = () => {
    // Avoid unnecessary API calls for empty input.
    if (hash.trim()) {
      refetch();
    }
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Block by Hash</h1>
      
      <div className="mb-6">
        <div className="flex gap-4">
          <input
            type="text"
            value={hash}
            onChange={(e) => setHash(e.target.value)}
            placeholder="Enter block hash"
            className="input-field flex-1"
            // Convenience: Enter key triggers the same search action.
            onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
          />
          <button onClick={handleSearch} className="btn-primary" disabled={!hash.trim() || isLoading}>
            Search
          </button>
        </div>
      </div>

      {isLoading && <LoadingSpinner />}
      
      {error && <ErrorMessage error={error as Error} />}
      
      {data?.data && (
        <JsonViewer data={data.data} title="Block Details" />
      )}
      
      {!hash && !isLoading && !data && (
        <div className="text-gray-400">Enter a block hash to search</div>
      )}
    </div>
  );
}
```

---

## Listing 7.21: `src/components/Wallet/CreateWallet.tsx`

This component demonstrates a mutation flow:

- local form state (`label`),
- mutation hook (`useCreateWallet`),
- toasts + cache invalidation handled by the hook,
- and a success panel + JSON viewer.

> **Methods involved**
> - `CreateWallet`
> - `handleCreate` (local helper)

```tsx
import { useState } from 'react';
import { useCreateWallet } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { JsonViewer } from '../common/JsonViewer';
import toast from 'react-hot-toast';

export function CreateWallet() {
  // Local UI state (in-memory only). The actual wallet is created on the Rust server.
  const [label, setLabel] = useState('');
  // Mutation hook encapsulates: HTTP call + toasts + cache invalidation.
  const createWallet = useCreateWallet();

  const handleCreate = async () => {
    // Basic validation: require a non-empty label in this UI flow.
    if (!label.trim()) {
      toast.error('Please enter a wallet label');
      return;
    }
    
    // Trigger the mutation; results land in createWallet.data / createWallet.error.
    createWallet.mutate({ label: label.trim() });
    // Clear the input for fast repeated operations.
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
              // Keep the input controlled so the UI state is always the source of truth.
              onChange={(e) => setLabel(e.target.value)}
              placeholder="My Wallet"
              className="input-field"
              // Keyboard ergonomics: Enter creates the wallet.
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

      {/* Mutation-in-flight state; keep the component responsive during network calls. */}
      {createWallet.isPending && <LoadingSpinner />}
      
      {createWallet.data?.data && (
        <div>
          {/* A human-friendly success summary; JSON viewer below provides the exact payload. */}
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
```

---

## Listing 7.22: `src/components/Wallet/WalletInfo.tsx`

“Enter address → fetch → show JSON” screen. It is intentionally minimal and relies on shared primitives (hooks + JSON viewer).

> **Methods involved**
> - `WalletInfo`
> - `handleLoad` (local helper)

```tsx
import { useState } from 'react';
import { useWalletInfo } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function WalletInfo() {
  // Input-driven query: the hook is enabled only when `address` is non-empty.
  const [address, setAddress] = useState('');
  const { data, error, isLoading, refetch } = useWalletInfo(address);

  const handleLoad = () => {
    // Explicit fetch on button click / Enter key.
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
            // Keyboard ergonomics: Enter triggers the same as clicking “Load Info”.
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
```

---

## Listing 7.23: `src/components/Wallet/Balance.tsx`

This screen shows an important pattern: “render a summarized view, then provide raw JSON.”

> **Methods involved**
> - `Balance`
> - `handleLoad` (local helper)

```tsx
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
```

---

## Listing 7.24: `src/components/Wallet/SendTransaction.tsx`

This is the primary “mutation + validation” component:

- validates required fields,
- parses and validates numeric input,
- sends the mutation,
- displays a success state.

> **Methods involved**
> - `SendTransaction`
> - `handleSend` (local helper)

```tsx
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
```

---

## Listing 7.25: `src/components/Wallet/TransactionHistory.tsx`

This screen demonstrates consuming a paginated response shape (when present) and presenting an empty state.

> **Methods involved**
> - `TransactionHistory`
> - `handleLoad` (local helper)

```tsx
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
```

---

## Listing 7.26: `src/components/Wallet/AllAddresses.tsx`

This is the most complex wallet screen: it combines a table view with a modal that can display info/balance/history for a selected address.

Important to understand:

- The main component uses `useAddresses()` for the address list.
- The modal component conditionally enables data hooks by passing either the address or an empty string (so the hook’s `enabled: !!address` gates requests).
- The history path expects a paginated response with `.items`.

> **Methods involved**
> - `AllAddresses`
> - `AddressDetailsModal` (local component function)
> - `handleAction` (local helper)

```tsx
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
                  <th className="text-left py-15 px-4 text-gray-300 font-semibold">Address</th>
                  <th className="text-center py-15 px-4 text-gray-300 font-semibold">Info</th>
                  <th className="text-center py-15 px-4 text-gray-300 font-semibold">Balance</th>
                  <th className="text-center py-15 px-4 text-gray-300 font-semibold">History</th>
                </tr>
              </thead>
              <tbody>
                {addresses.map((address: string, index: number) => (
                  <tr key={index} className="border-b border-gray-700/50 hover:bg-gray-700/30">
                    <td className="py-15 px-4">
                      <span className="font-mono text-sm break-all text-gray-300">{address}</span>
                    </td>
                    <td className="py-15 px-4 text-center">
                      <button
                        onClick={() => handleAction(address, 'info')}
                        className="btn-secondary text-xs py-1 px-3"
                      >
                        Info
                      </button>
                    </td>
                    <td className="py-15 px-4 text-center">
                      <button
                        onClick={() => handleAction(address, 'balance')}
                        className="btn-secondary text-xs py-1 px-3"
                      >
                        Balance
                      </button>
                    </td>
                    <td className="py-15 px-4 text-center">
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
              <Dialog.Panel className="w-full max-w-4xl transform overflow-hidden rounded-lg bg-gray-800 border border-gray-700 p-20 shadow-xl transition-all">
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
                      className="w-20 h-6"
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
```

---

## Listing 7.27: `src/components/Transactions/Mempool.tsx`

Minimal “query + refresh + JSON” pattern for mempool transactions.

> **Methods involved**
> - `Mempool`

```tsx
import { useMempool } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function Mempool() {
  const { data, error, isLoading, refetch } = useMempool();

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">Mempool</h1>
        <button onClick={() => refetch()} className="btn-primary">
          Refresh
        </button>
      </div>
      
      {data?.data && <JsonViewer data={data.data} title="Mempool Transactions" />}
    </div>
  );
}
```

---

## Listing 7.28: `src/components/Transactions/MempoolTx.tsx`

“Search by txid” screen. Demonstrates local state + `refetch` on demand.

> **Methods involved**
> - `MempoolTx`
> - `handleSearch` (local helper)

```tsx
import { useState } from 'react';
import { useMempoolTransaction } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function MempoolTx() {
  // Query-by-id pattern: local state drives the hook parameter.
  const [txid, setTxid] = useState('');
  const { data, error, isLoading, refetch } = useMempoolTransaction(txid);

  const handleSearch = () => {
    // Guard: do not query with empty txid.
    if (txid.trim()) {
      refetch();
    }
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Mempool Transaction</h1>
      
      <div className="mb-6">
        <div className="flex gap-4">
          <input
            type="text"
            value={txid}
            onChange={(e) => setTxid(e.target.value)}
            placeholder="Enter transaction ID"
            className="input-field flex-1"
            // Keyboard ergonomics.
            onKeyPress={(e) => e.key === 'Enter' && handleSearch()}
          />
          <button onClick={handleSearch} className="btn-primary" disabled={!txid.trim() || isLoading}>
            Search
          </button>
        </div>
      </div>

      {isLoading && <LoadingSpinner />}
      
      {error && <ErrorMessage error={error as Error} />}
      
      {data?.data && (
        <JsonViewer data={data.data} title="Transaction Details" />
      )}
      
      {!txid && !isLoading && !data && (
        <div className="text-gray-400">Enter a transaction ID to search</div>
      )}
    </div>
  );
}
```

---

## Listing 7.29: `src/components/Transactions/AllTransactions.tsx`

This screen demonstrates:

- explicit paging via state (`page`, `limit`),
- an “enabled: false” query that is triggered on demand,
- and a bulk “load all pages” mode that iterates using the API client directly.

> **Methods involved**
> - `AllTransactions`
> - `handleLoad`, `handleLoadPage`, `handleLoadAll` (local helpers)

```tsx
import { useState } from 'react';
import { useAllTransactions } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';
import { getApiClient } from '../../services/api';

export function AllTransactions() {
  // Pagination state for the “Load Page” flow.
  const [page, setPage] = useState(1);
  const [limit] = useState(100); // Higher default limit for longer display
  // This query is `enabled: false` in the hook; it runs only when we call refetch().
  const { data, error, isLoading, refetch } = useAllTransactions(page, limit);
  const [isLoadingData, setIsLoadingData] = useState(false);
  // When “Load All” is used, we store the aggregate list here (separate from React Query cache).
  const [allTransactions, setAllTransactions] = useState<any[]>([]);
  const [isLoadingAll, setIsLoadingAll] = useState(false);

  const handleLoad = async () => {
    // Trigger the disabled query for the current page.
    setIsLoadingData(true);
    await refetch();
    setIsLoadingData(false);
  };

  const handleLoadPage = async (pageNum: number) => {
    // Update page, then fetch that page’s slice.
    setPage(pageNum);
    setIsLoadingData(true);
    await refetch();
    setIsLoadingData(false);
  };

  const handleLoadAll = async () => {
    // Bulk-load loop: iteratively fetch pages until the backend reports no further pages.
    setIsLoadingAll(true);
    setAllTransactions([]);
    
    try {
      let currentPage = 1;
      let allItems: any[] = [];
      let hasMore = true;

      while (hasMore) {
        const response = await getApiClient().getAllTransactions(currentPage, limit);
        if (response.success && response.data) {
          const paginatedData = response.data;
          // Accumulate items across pages.
          allItems = [...allItems, ...paginatedData.items];
          
          if (paginatedData.has_next) {
            currentPage++;
          } else {
            hasMore = false;
          }
        } else {
          hasMore = false;
        }
      }

      setAllTransactions(allItems);
    } catch (err) {
      console.error('Error loading all transactions:', err);
    } finally {
      setIsLoadingAll(false);
    }
  };

  if (isLoading || isLoadingData) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  // Prefer bulk-loaded data when present; otherwise show current page slice.
  const displayData = allTransactions.length > 0 ? allTransactions : (data?.data?.items || []);
  // Pagination controls apply only when not in “Load All” mode.
  const paginationInfo = data?.data && allTransactions.length === 0 ? data.data : null;

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">All Transactions</h1>
        <div className="flex gap-2">
          <button onClick={handleLoad} className="btn-primary" disabled={isLoading}>
            Load Page {page}
          </button>
          <button onClick={handleLoadAll} className="btn-primary" disabled={isLoadingAll}>
            {isLoadingAll ? 'Loading All...' : 'Load All Transactions'}
          </button>
        </div>
      </div>

      {paginationInfo && allTransactions.length === 0 && (
        <div className="mb-4 text-sm text-gray-400">
          Showing {paginationInfo.items?.length || 0} of {paginationInfo.total} transactions
          {paginationInfo.total_pages > 1 && ` (Page ${paginationInfo.page} of ${paginationInfo.total_pages})`}
        </div>
      )}

      {paginationInfo && paginationInfo.total_pages > 1 && allTransactions.length === 0 && (
        <div className="mb-4 flex gap-2">
          <button
            onClick={() => handleLoadPage(paginationInfo.page - 1)}
            disabled={!paginationInfo.has_prev || isLoading}
            className="btn-secondary"
          >
            Previous
          </button>
          <button
            onClick={() => handleLoadPage(paginationInfo.page + 1)}
            disabled={!paginationInfo.has_next || isLoading}
            className="btn-secondary"
          >
            Next
          </button>
        </div>
      )}

      {allTransactions.length > 0 && (
        <div className="mb-4 text-sm text-gray-400">
          Loaded {allTransactions.length} transactions (all pages)
        </div>
      )}
      
      {displayData.length > 0 ? (
        <div className="max-h-[80vh] overflow-y-auto">
          <JsonViewer 
            data={displayData} 
            title={`All Transactions${allTransactions.length > 0 ? ` (${allTransactions.length} total)` : paginationInfo ? ` (${paginationInfo.total} total)` : ''}`} 
          />
        </div>
      ) : (
        <div className="text-gray-400">Click "Load Page {page}" or "Load All Transactions" to fetch data</div>
      )}
    </div>
  );
}
```

---

## Listing 7.30: `src/components/Transactions/AddressTransactions.tsx`

This screen is an address-driven query that displays the server’s response as JSON.

> **Methods involved**
> - `AddressTransactions`
> - `handleLoad` (local helper)

```tsx
import { useState } from 'react';
import { useAddressTransactions } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function AddressTransactions() {
  const [address, setAddress] = useState('');
  const { data, error, isLoading, refetch } = useAddressTransactions(address);

  const handleLoad = () => {
    if (address.trim()) {
      refetch();
    }
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Address Transactions</h1>
      
      <div className="mb-6">
        <div className="flex gap-4">
          <input
            type="text"
            value={address}
            onChange={(e) => setAddress(e.target.value)}
            placeholder="Enter address"
            className="input-field flex-1"
            onKeyPress={(e) => e.key === 'Enter' && handleLoad()}
          />
          <button onClick={handleLoad} className="btn-primary" disabled={!address.trim() || isLoading}>
            Load Transactions
          </button>
        </div>
      </div>

      {isLoading && <LoadingSpinner />}
      
      {error && <ErrorMessage error={error as Error} />}
      
      {data?.data && (
        <JsonViewer data={data.data} title="Address Transactions" />
      )}
      
      {!address && !isLoading && !data && (
        <div className="text-gray-400">Enter an address to view transactions</div>
      )}
    </div>
  );
}
```

---

## Listing 7.31: `src/components/Mining/MiningInfo.tsx`

Minimal “query + refresh + JSON” screen for mining info.

> **Methods involved**
> - `MiningInfo`

```tsx
import { useMiningInfo } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function MiningInfo() {
  const { data, error, isLoading, refetch } = useMiningInfo();

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">Mining Info</h1>
        <button onClick={() => refetch()} className="btn-primary">
          Refresh
        </button>
      </div>
      
      {data?.data && <JsonViewer data={data.data} title="Mining Information" />}
    </div>
  );
}
```

---

## Listing 7.32: `src/components/Mining/GenerateBlocks.tsx`

This is the mining mutation screen. It validates inputs and triggers `useGenerateBlocks()`.

> **Methods involved**
> - `GenerateBlocks`
> - `handleGenerate` (local helper)

```tsx
import { useState } from 'react';
import { useGenerateBlocks } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { JsonViewer } from '../common/JsonViewer';

export function GenerateBlocks() {
  const [address, setAddress] = useState('');
  const [nblocks, setNblocks] = useState('1');
  const [maxtries, setMaxtries] = useState('');
  const generateBlocks = useGenerateBlocks();

  const handleGenerate = () => {
    if (!address.trim() || !nblocks.trim()) {
      return;
    }

    const nblocksNum = parseInt(nblocks);
    if (isNaN(nblocksNum) || nblocksNum <= 0) {
      return;
    }

    const maxtriesNum = maxtries.trim() ? parseInt(maxtries) : undefined;
    if (maxtries.trim() && (isNaN(maxtriesNum!) || maxtriesNum! <= 0)) {
      return;
    }

    generateBlocks.mutate({
      address: address.trim(),
      nblocks: nblocksNum,
      maxtries: maxtriesNum,
    });
  };

  return (
    <div>
      <h1 className="text-3xl font-bold text-white mb-6">Generate Blocks</h1>
      
      <div className="card mb-6">
        <div className="space-y-4">
          <div>
            <label className="block text-sm text-gray-300 mb-2">Mining Reward Address</label>
            <input
              type="text"
              value={address}
              onChange={(e) => setAddress(e.target.value)}
              placeholder="Enter address to receive mining rewards"
              className="input-field"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-300 mb-2">Number of Blocks</label>
            <input
              type="number"
              value={nblocks}
              onChange={(e) => setNblocks(e.target.value)}
              placeholder="1"
              className="input-field"
              min="1"
            />
          </div>
          <div>
            <label className="block text-sm text-gray-300 mb-2">Max Tries (optional)</label>
            <input
              type="number"
              value={maxtries}
              onChange={(e) => setMaxtries(e.target.value)}
              placeholder="Leave empty for default"
              className="input-field"
              min="1"
            />
          </div>
          <button
            onClick={handleGenerate}
            className="btn-primary"
            disabled={generateBlocks.isPending || !address.trim() || !nblocks.trim()}
          >
            {generateBlocks.isPending ? 'Generating...' : 'Generate Blocks'}
          </button>
        </div>
      </div>

      {generateBlocks.isPending && <LoadingSpinner />}
      
      {generateBlocks.data?.data && (
        <div>
          <div className="card mb-4 bg-green-900/20 border-green-500/50">
            <p className="text-green-400 font-semibold mb-2">Blocks Generated Successfully!</p>
          </div>
          <JsonViewer data={generateBlocks.data.data} title="Generation Result" />
        </div>
      )}
    </div>
  );
}
```

---

## Listing 7.33: `src/components/Health/HealthCheck.tsx`

This is a health endpoint query with a summarized success panel plus raw JSON.

> **Methods involved**
> - `HealthCheck`

```tsx
import { useHealth } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function HealthCheck() {
  const { data, error, isLoading, refetch } = useHealth();

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">Health Check</h1>
        <button onClick={() => refetch()} className="btn-primary">
          Refresh
        </button>
      </div>
      
      {data?.data && (
        <div>
          <div className={`card mb-4 ${data.success ? 'bg-green-900/20 border-green-500/50' : 'bg-red-900/20 border-red-500/50'}`}>
            <p className={`font-semibold ${data.success ? 'text-green-400' : 'text-red-400'}`}>
              {data.success ? '✓ System is healthy' : '✗ System health check failed'}
            </p>
          </div>
          <JsonViewer data={data.data} title="Health Check Details" />
        </div>
      )}
    </div>
  );
}
```

---

## Listing 7.34: `src/components/Health/Liveness.tsx`

Health screen variant for liveness.

> **Methods involved**
> - `Liveness`

```tsx
import { useLiveness } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function Liveness() {
  const { data, error, isLoading, refetch } = useLiveness();

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">Liveness Check</h1>
        <button onClick={() => refetch()} className="btn-primary">
          Refresh
        </button>
      </div>
      
      {data?.data && (
        <div>
          <div className={`card mb-4 ${data.success ? 'bg-green-900/20 border-green-500/50' : 'bg-red-900/20 border-red-500/50'}`}>
            <p className={`font-semibold ${data.success ? 'text-green-400' : 'text-red-400'}`}>
              {data.success ? '✓ System is alive' : '✗ Liveness check failed'}
            </p>
          </div>
          <JsonViewer data={data.data} title="Liveness Check Details" />
        </div>
      )}
    </div>
  );
}
```

---

## Listing 7.35: `src/components/Health/Readiness.tsx`

Health screen variant for readiness.

> **Methods involved**
> - `Readiness`

```tsx
import { useReadiness } from '../../hooks/useApi';
import { LoadingSpinner } from '../common/LoadingSpinner';
import { ErrorMessage } from '../common/ErrorMessage';
import { JsonViewer } from '../common/JsonViewer';

export function Readiness() {
  const { data, error, isLoading, refetch } = useReadiness();

  if (isLoading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return <ErrorMessage error={error as Error} />;
  }

  return (
    <div>
      <div className="flex items-center justify-between mb-6">
        <h1 className="text-3xl font-bold text-white">Readiness Check</h1>
        <button onClick={() => refetch()} className="btn-primary">
          Refresh
        </button>
      </div>
      
      {data?.data && (
        <div>
          <div className={`card mb-4 ${data.success ? 'bg-green-900/20 border-green-500/50' : 'bg-red-900/20 border-red-500/50'}`}>
            <p className={`font-semibold ${data.success ? 'text-green-400' : 'text-red-400'}`}>
              {data.success ? '✓ System is ready' : '✗ Readiness check failed'}
            </p>
          </div>
          <JsonViewer data={data.data} title="Readiness Check Details" />
        </div>
      )}
    </div>
  );
}
```

---

## Listing 7.36: `vite.config.ts`

Vite config controls the developer experience and production output. The most important part is the dev server proxy:

- `/api/*` is proxied to the Rust server, avoiding CORS complexity in development.

> **Methods involved**
> - `defineConfig(...)` (configuration call)

```typescript
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:8080',
        changeOrigin: true,
      },
    },
  },
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
  },
})
```

---

## Listing 7.37: `tailwind.config.js`

Tailwind config defines where classnames are scanned and extends the theme with Bitcoin colors.

> **Methods involved**
> - configuration object export

```javascript
/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'bitcoin-orange': '#f7931a',
        'bitcoin-gold': '#ffb347',
      },
    },
  },
  plugins: [],
}
```

---

## Listing 7.38: `postcss.config.js`

PostCSS config wires Tailwind + Autoprefixer.

> **Methods involved**
> - configuration object export

```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

---

## Listing 7.39: `tsconfig.json`

TypeScript config controls compiler strictness, module resolution, and JSX transformation.

> **Methods involved**
> - configuration object

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "useDefineForClassFields": true,
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "skipLibCheck": true,
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "resolveJsonModule": true,
    "isolatedModules": true,
    "noEmit": true,
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "noFallthroughCasesInSwitch": true
  },
  "include": ["src"],
  "references": [{ "path": "./tsconfig.node.json" }]
}
```

---

## Listing 7.40: `src/index.css`

This is where Tailwind layers are imported and a small component class library is defined (`btn-primary`, `input-field`, etc.).

Important to understand:

- These classes are used across nearly all components to keep styling consistent.

> **Methods involved**
> - CSS component class definitions

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer base {
  body {
    @apply bg-gray-900 text-gray-100;
  }
}

@layer components {
  .btn-primary {
    @apply px-4 py-2 bg-bitcoin-orange hover:bg-bitcoin-gold text-white font-semibold rounded-lg transition-colors;
  }
  
  .btn-secondary {
    @apply px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white font-semibold rounded-lg transition-colors;
  }
  
  .input-field {
    @apply w-full px-4 py-2 bg-gray-800 border border-gray-700 rounded-lg text-white placeholder-gray-500 focus:outline-none focus:border-bitcoin-orange;
  }
  
  .card {
    @apply bg-gray-800 rounded-lg border border-gray-700 p-20;
  }
}
```

---

<div align="center">

**Reading order**

**[← Previous: Web Admin Interface](06-Web-Admin-UI.md)** | **[Next: Docker Compose Deployment →](../ci/docker-compose/01-Introduction.md)**

</div>

---

