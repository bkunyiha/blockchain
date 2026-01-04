<div align="left">

<details>
<summary><b>📑 Chapter Navigation ▼</b></summary>

### Part I: Core Blockchain Implementation

1. [Chapter 1: Introduction & Overview](../01-Introduction.md) - Book introduction, project structure, technical stack
2. [Chapter 1.2: Introduction to Bitcoin & Blockchain](../bitcoin-blockchain/README.md) - Bitcoin and blockchain fundamentals
3. [Chapter 1.3: Bitcoin Whitepaper](../bitcoin-blockchain/00-Bitcoin-Whitepaper-Summary.md) - Bitcoin Whitepaper
4. [Chapter 1.4: Bitcoin Whitepaper In Rust](../bitcoin-blockchain/whitepaper-rust/README.md) - Bitcoin Whitepaper In Rust
5. [Chapter 2.0: Rust Blockchain Project](../bitcoin-blockchain/Rust-Project-Index.md) - Blockchain Project
6. [Chapter 2.1: Primitives](../bitcoin-blockchain/primitives/README.md) - Core data structures
7. [Chapter 2.2: Utilities](../bitcoin-blockchain/util/README.md) - Utility functions and helpers
8. [Chapter 2.3: Cryptography](../bitcoin-blockchain/crypto/README.md) - Cryptographic primitives and libraries
9. [Chapter 2.4: Blockchain(POW & Block Acceptance)](../bitcoin-blockchain/chain/01-Technical-Foundations.md) - Proof Of Work
10. [Chapter 2.5: Storage Layer](../bitcoin-blockchain/store/README.md) - Persistent storage implementation
11. [Chapter 2.6: Blockchain(POW & Block Acceptance)](../bitcoin-blockchain/chain/02-Block-Acceptance-Whitepaper-Step-5.md) - Proof Of Work
12. [Chapter 2.7: Network Layer](../bitcoin-blockchain/net/README.md) - Peer-to-peer networking and protocol
13. [Chapter 2.8: Node Orchestration](../bitcoin-blockchain/node/README.md) - Node context and coordination
14. [Chapter 2.9: Wallet System](../bitcoin-blockchain/wallet/README.md) - Wallet implementation and key management
15. [Chapter 3: Web API Architecture](../bitcoin-blockchain/web/README.md) - REST API implementation
16. [Chapter 4: Desktop Admin Interface](../bitcoin-desktop-ui/03-Desktop-Admin-UI.md) - Iced framework architecture
17. [Chapter 5: Wallet User Interface](../bitcoin-wallet-ui/04-Wallet-UI.md) - Wallet UI implementation
18. [Chapter 6: Embedded Database & Persistence](../bitcoin-wallet-ui/05-Embedded-Database.md) - SQLCipher integration
19. **Chapter 7: Web Admin Interface** ← *You are here*

### Part II: Deployment & Operations

20. [Chapter 8: Docker Compose Deployment](../ci/docker-compose/01-Introduction.md) - Docker Compose guide
21. [Chapter 9: Kubernetes Deployment](../ci/kubernetes/README.md) - Kubernetes production guide
22. [Chapter 10: Rust Language Guide](../rust/README.md) - Rust programming language reference

</details>

</div>

---
<div align="right">

**[← Back to Main Book](../../README.md)**

</div>

---

# Chapter 7: Web Admin Interface

**Part I: Core Blockchain Implementation** | **Part II: Deployment & Operations**

<div align="center">

**📚 [← Chapter 6: Embedded Database](../bitcoin-wallet-ui/05-Embedded-Database.md)** | **Chapter 7: Web Admin Interface** | **[Chapter 8: Docker Compose →](../ci/docker-compose/01-Introduction.md)** 📚

</div>

---

## Table of Contents

1. [Overview](#overview)
2. [Architecture](#architecture)
3. [Technology Stack](#technology-stack)
4. [Project Structure](#project-structure)
5. [Core Concepts](#core-concepts)
   - [React Hooks](#react-hooks)
   - [State Management](#state-management)
   - [Data Flow](#data-flow)
6. [API Integration](#api-integration)
7. [Component Architecture](#component-architecture)
8. [Build and Deployment](#build-and-deployment)
9. [Configuration](#configuration)

---

## Overview

In this chapter, we'll explore the Bitcoin Web UI—a modern, single-page application (SPA) built with React and TypeScript that provides a comprehensive administrative interface for managing a Bitcoin blockchain node. This web interface represents a significant evolution from static HTML pages to a fully-featured, interactive web application. As we journey through this chapter, we'll understand how modern web technologies come together to create a powerful administrative tool.

### What We'll Build

The Web Admin Interface provides a rich set of features that make managing a blockchain node intuitive and efficient:

- **Real-time Dashboard**: We'll build an auto-refreshing dashboard that shows blockchain statistics, giving administrators immediate insight into the network's state.

- **Blockchain Management**: Users can view and search blocks, exploring the blockchain's history and understanding its structure.

- **Wallet Operations**: The interface allows creating wallets, managing addresses, and sending transactions—all through an intuitive web interface.

- **Transaction Management**: Administrators can browse the mempool and view transaction history, understanding what's happening in real-time.

- **Mining Controls**: The interface provides mining information and allows generating blocks, giving full control over the mining process.

- **Health Monitoring**: System health, liveness, and readiness checks ensure administrators know the system's status at a glance.

- **Modern UI**: A dark theme with responsive design ensures the interface looks great and works well on any device.

---

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Browser (User)                          │
└───────────────────────┬─────────────────────────────────────┘
                        │
                        │ HTTP Requests
                        │ (React App + API Calls)
                        │
        ┌───────────────┴───────────────┐
        │                               │
        ▼                               ▼
┌───────────────┐               ┌───────────────┐
│  React App    │               │  Rust Server  │
│  (Frontend)   │◄─────────────-|  (Backend)    │
│               │  API Calls    │               │
│  - React      │  /api/admin/* │  - Axum       │
│  - TypeScript │               │  - NodeContext│
│  - Vite       │               │  - Blockchain │
└───────────────┘               └───────────────┘
```

### Application Flow

1. **Initial Load**: Browser requests `http://localhost:8080/`
2. **Server Response**: Rust server serves `index.html` from `bitcoin-web-ui/dist/`
3. **React Hydration**: React app loads and initializes
4. **API Configuration**: User configures API base URL and key (stored in localStorage)
5. **Data Fetching**: React Query hooks fetch data from `/api/admin/*` endpoints
6. **UI Updates**: Components render data and handle user interactions

---

## Technology Stack

### Build Tool: Vite

**Why Vite?**

Vite is a next-generation build tool that provides:
- **Lightning-fast HMR (Hot Module Replacement)**: Changes reflect instantly in development
- **Optimized Production Builds**: Uses Rollup for efficient bundling
- **Native ES Modules**: Faster development server startup
- **TypeScript Support**: Built-in TypeScript compilation

**Configuration** (`vite.config.ts`):

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

**Key Features:**
- **Proxy Configuration**: In development, `/api` requests are proxied to the Rust server
- **React Plugin**: Enables JSX transformation and React Fast Refresh
- **Build Output**: Optimized production build in `dist/` directory

### Core Framework: React 18

**Why React?**

React is chosen for:
- **Component-Based Architecture**: Reusable, composable UI components
- **Virtual DOM**: Efficient rendering and updates
- **Rich Ecosystem**: Extensive library support
- **TypeScript Integration**: Excellent type safety with TypeScript
- **Hooks System**: Powerful way to manage state and side effects

**Example Component Structure**:

```typescript
// src/components/Dashboard/Dashboard.tsx
import { useBlockchainInfo } from '../../hooks/useApi';

export function Dashboard() {
  const { data, error, isLoading } = useBlockchainInfo(5000);
  
  if (isLoading) return <LoadingSpinner />;
  if (error) return <ErrorMessage error={error} />;
  
  return (
    <div>
      <h1>Dashboard</h1>
      {/* Render data */}
    </div>
  );
}
```

### Type Safety: TypeScript

**Why TypeScript?**

TypeScript provides:
- **Type Safety**: Catch errors at compile time
- **Better IDE Support**: Autocomplete, refactoring, navigation
- **Self-Documenting Code**: Types serve as documentation
- **Refactoring Confidence**: Safe code changes

**Type Definitions** (`src/types/api.ts`):

```typescript
// API Response wrapper matching Rust API structure
export interface ApiResponse<T> {
  success: boolean;
  data?: T;
  error?: string;
  timestamp: string;
}

// Blockchain-specific types
export interface BlockchainInfo {
  height: number;
  difficulty: number;
  total_blocks: number;
  total_transactions: number;
  mempool_size: number;
  last_block_hash: string;
  last_block_timestamp: string;
}
```

**Benefits:**
- Types match Rust API responses exactly
- Prevents runtime errors from type mismatches
- Enables autocomplete in IDE

### Routing: React Router v6

**Why React Router?**

React Router enables:
- **Client-Side Routing**: No page reloads, smooth navigation
- **URL-Based Navigation**: Shareable links, browser history
- **Nested Routes**: Organized route structure
- **Code Splitting**: Lazy loading of route components

**Route Configuration** (`src/App.tsx`):

```typescript
import { BrowserRouter, Routes, Route } from 'react-router-dom';

function App() {
  return (
    <BrowserRouter>
      <Layout>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/blockchain/info" element={<BlockchainInfo />} />
          <Route path="/wallet/create" element={<CreateWallet />} />
          {/* More routes... */}
        </Routes>
      </Layout>
    </BrowserRouter>
  );
}
```

**How It Works:**
- `BrowserRouter`: Uses HTML5 history API for routing
- `Routes`: Defines route matching rules
- `Route`: Maps URL paths to components
- Fallback handling: Rust server serves `index.html` for all non-API routes

### State Management: React Query (TanStack Query)

**Why React Query?**

React Query provides:
- **Server State Management**: Handles API data fetching, caching, synchronization
- **Automatic Caching**: Reduces unnecessary API calls
- **Background Refetching**: Keeps data fresh automatically
- **Loading/Error States**: Built-in state management
- **Optimistic Updates**: Better UX for mutations

**Query Client Setup** (`src/App.tsx`):

```typescript
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,                    // Retry failed requests once
      refetchOnWindowFocus: false,  // Don't refetch on window focus
    },
  },
});

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      {/* App components */}
    </QueryClientProvider>
  );
}
```

**Key Concepts:**
- **Query Keys**: Unique identifiers for cached data (`['blockchain', 'info']`)
- **Query Functions**: Async functions that fetch data
- **Mutations**: Operations that modify server state
- **Cache Invalidation**: Refetch data after mutations

### HTTP Client: Axios

**Why Axios? (And why not just React Query?)**

**Important Distinction**: React Query (TanStack Query) does **NOT** make HTTP requests. It's a state management library that needs an HTTP client to actually fetch data.

**React Query's Role:**
- Manages caching, refetching, loading states
- Handles query invalidation
- Provides hooks for components
- **Does NOT make HTTP requests itself**

**HTTP Client's Role:**
- Actually makes the HTTP request
- Handles request/response transformation
- Manages authentication headers
- Handles errors at the HTTP level

**Why Axios over native `fetch`?**

Axios provides advantages over the native `fetch` API:

1. **Request/Response Interceptors**: Add auth headers automatically to all requests
2. **Better Error Handling**: Automatically throws on 4xx/5xx status codes
3. **Request Cancellation**: Built-in cancellation tokens
4. **Automatic JSON Parsing**: No need to call `.json()` manually
5. **Request/Response Transformation**: Built-in data transformation
6. **Better TypeScript Support**: More type-safe than fetch

**Comparison: Axios vs Fetch**

```typescript
// With Axios (what we use)
async getBlockchainInfo(): Promise<ApiResponse<BlockchainInfo>> {
  const response = await this.client.get('/api/admin/blockchain');
  return response.data; // Already parsed JSON, already typed
}

// With fetch (alternative)
async getBlockchainInfo(): Promise<ApiResponse<BlockchainInfo>> {
  const response = await fetch('/api/admin/blockchain', {
    headers: {
      'X-API-Key': this.apiKey, // Must manually add headers
      'Content-Type': 'application/json',
    },
  });
  
  if (!response.ok) {
    throw new Error('Request failed'); // Must manually check status
  }
  
  return response.json(); // Must manually parse JSON
}
```

**How They Work Together:**

```typescript
// 1. React Query hook calls API client method
export function useBlockchainInfo() {
  return useQuery({
    queryKey: ['blockchain', 'info'],
    queryFn: () => getApiClient().getBlockchainInfo(), // ← Calls Axios
    // React Query handles: caching, refetching, loading states
  });
}

// 2. API client uses Axios to make HTTP request
async getBlockchainInfo(): Promise<ApiResponse<BlockchainInfo>> {
  const response = await this.client.get('/api/admin/blockchain'); // ← Axios makes request
  return response.data;
}

// 3. Component uses React Query hook
function Dashboard() {
  const { data, error, isLoading } = useBlockchainInfo(); // ← React Query manages state
  // ...
}
```

**API Client Implementation** (`src/services/api.ts`):

```typescript
import axios, { AxiosInstance } from 'axios';

export class ApiClient {
  private client: AxiosInstance;

  constructor(baseURL: string, apiKey?: string) {
    this.client = axios.create({
      baseURL,
      headers: {
        'Content-Type': 'application/json',
        ...(apiKey && { 'X-API-Key': apiKey }),
      },
    });
  }

  updateApiKey(apiKey: string) {
    // Easy to update headers for all future requests
    this.client.defaults.headers['X-API-Key'] = apiKey;
  }

  async getBlockchainInfo(): Promise<ApiResponse<BlockchainInfo>> {
    // Axios automatically:
    // - Adds baseURL
    // - Adds headers (including X-API-Key)
    // - Parses JSON response
    // - Throws on error status codes
    const response = await this.client.get('/api/admin/blockchain');
    return response.data;
  }
}

// Singleton pattern for global API client
let apiClient: ApiClient | null = null;

export function getApiClient(): ApiClient {
  if (!apiClient) {
    const baseURL = localStorage.getItem('api_base_url') || 'http://127.0.0.1:8080';
    const apiKey = localStorage.getItem('api_key') || undefined;
    apiClient = new ApiClient(baseURL, apiKey);
  }
  return apiClient;
}
```

**Benefits:**
- **Separation of Concerns**: Axios handles HTTP, React Query handles state
- **Centralized Configuration**: API base URL and auth headers in one place
- **Automatic Authentication**: Headers added to all requests automatically
- **Type Safety**: TypeScript types flow through both layers
- **Easy Testing**: Can mock Axios client independently

### Styling: Tailwind CSS

**Why Tailwind CSS?**

Tailwind CSS provides:
- **Utility-First**: Rapid UI development with utility classes
- **Consistent Design**: Predefined design system
- **Small Bundle Size**: Only includes used styles (with JIT)
- **Customizable**: Easy theme customization

**Configuration** (`tailwind.config.js`):

```javascript
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

**Usage Example**:

```typescript
// Utility classes for styling
<div className="bg-gray-800 rounded-lg border border-gray-700 p-6">
  <h1 className="text-3xl font-bold text-white mb-6">Dashboard</h1>
  <button className="btn-primary">Click Me</button>
</div>
```

**Custom CSS Classes** (`src/index.css`):

```css
@tailwind base;
@tailwind components;
@tailwind utilities;

@layer components {
  .btn-primary {
    @apply px-4 py-2 bg-bitcoin-orange hover:bg-bitcoin-gold 
           text-white font-semibold rounded-lg transition-colors;
  }
  
  .input-field {
    @apply w-full px-4 py-2 bg-gray-800 border border-gray-700 
           rounded-lg text-white placeholder-gray-500 
           focus:outline-none focus:border-bitcoin-orange;
  }
}
```

### UI Components: Headless UI

**What is Headless UI?**

Headless UI is a library of unstyled, fully accessible UI components built by the creators of Tailwind CSS. Unlike traditional UI component libraries that come with pre-styled components, Headless UI provides the logic, behavior, and accessibility features without any default styling, giving you complete control over the visual appearance.

**Why Headless UI?**

Headless UI provides:
- **Accessible Components**: Built-in ARIA attributes and keyboard navigation
- **Unstyled Components**: Full control over styling
- **React Integration**: Designed for React
- **Common Patterns**: Menu, Dialog, Transition components

**Example Usage** (`src/components/Layout/Navbar.tsx`):

```typescript
import { Menu, Transition } from '@headlessui/react';

export function Navbar() {
  return (
    <Menu as="div" className="relative">
      <Menu.Button className="btn-secondary">
        Configure API
      </Menu.Button>
      <Transition
        enter="transition ease-out duration-100"
        enterFrom="transform opacity-0 scale-95"
        enterTo="transform opacity-100 scale-100"
      >
        <Menu.Items className="absolute right-0 mt-2 w-80 bg-gray-800">
          {/* Menu items */}
        </Menu.Items>
      </Transition>
    </Menu>
  );
}
```

**Benefits:**
- Accessibility built-in (keyboard navigation, ARIA)
- Smooth animations with Transition component
- Flexible styling with Tailwind

### Notifications: React Hot Toast

**Why React Hot Toast?**

React Hot Toast provides:
- **Lightweight**: Small bundle size
- **Customizable**: Easy to style and configure
- **Non-Blocking**: Doesn't interrupt user flow
- **Promise Support**: Works with async operations

**Setup** (`src/App.tsx`):

```typescript
import { Toaster } from 'react-hot-toast';

<Toaster
  position="top-right"
  toastOptions={{
    duration: 4000,
    style: {
      background: '#1f2937',
      color: '#fff',
      border: '1px solid #374151',
    },
  }}
/>
```

**Usage in Hooks**:

```typescript
export function useCreateWallet() {
  return useMutation({
    mutationFn: (req) => getApiClient().createWallet(req),
    onSuccess: () => {
      toast.success('Wallet created successfully!');
    },
    onError: (error) => {
      toast.error(error.response?.data?.error || 'Failed to create wallet');
    },
  });
}
```

---

## Project Structure

```
bitcoin-web-ui/
├── src/
│   ├── components/          # React components
│   │   ├── Layout/        # Navigation and layout components
│   │   ├── Dashboard/     # Dashboard page component
│   │   ├── Blockchain/    # Blockchain management components
│   │   ├── Wallet/        # Wallet operation components
│   │   ├── Transactions/  # Transaction management components
│   │   ├── Mining/        # Mining operation components
│   │   ├── Health/         # Health check components
│   │   └── common/         # Shared/reusable components
│   ├── contexts/          # React contexts (API configuration)
│   ├── hooks/             # Custom React hooks (React Query)
│   ├── services/          # API client and external services
│   ├── types/             # TypeScript type definitions
│   ├── pages/             # Page components
│   ├── App.tsx            # Main app component with routing
│   ├── main.tsx           # Application entry point
│   └── index.css          # Global styles and Tailwind imports
├── dist/                  # Production build output (generated)
├── package.json           # Dependencies and scripts
├── vite.config.ts         # Vite configuration
├── tsconfig.json          # TypeScript configuration
├── tailwind.config.js     # Tailwind CSS configuration
└── README.md              # User documentation
```

---

## Core Concepts

### React Hooks

#### What are Hooks?

Hooks are special functions in React that let you "hook into" React features like state and lifecycle methods. They were introduced in React 16.8 to allow functional components to have the same capabilities as class components.

**Key Concept**: Hooks are functions that start with `use` and can only be called from:
1. React functional components
2. Other custom hooks (functions starting with `use`)

#### Why Hooks Exist

**Before Hooks (Class Components):**

```typescript
// Old way: Class component
class Dashboard extends React.Component {
  constructor(props) {
    super(props);
    this.state = { data: null, loading: true };
  }
  
  componentDidMount() {
    fetchData().then(data => {
      this.setState({ data, loading: false });
    });
  }
  
  render() {
    if (this.state.loading) return <Loading />;
    return <div>{this.state.data}</div>;
  }
}
```

**With Hooks (Functional Components):**

```typescript
// New way: Functional component with hooks
function Dashboard({ id }) {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(true);
  
  useEffect(() => {
    setLoading(true);
    fetchData(id).then(data => {
      setData(data);
      setLoading(false);
    });
  }, [id]); // Re-run when id changes
  
  if (loading) return <Loading />;
  return <div>{data}</div>;
}
```

**Benefits:**
- **Less Boilerplate**: No `this`, `super`, or `constructor`
- **Better Reusability**: Logic can be extracted into custom hooks
- **Easier Testing**: Functions are easier to test than classes
- **Better Performance**: Functional components optimize better
- **Simpler Mental Model**: One way to write components

#### Built-in React Hooks

**1. `useState` - Component State**

Manages local component state. Returns `[currentValue, setterFunction]`.

- **Purpose**: Store and update local component state
- **Use Case**: Form inputs, UI toggles, counters, any component-local data that changes
- **Returns**: `[currentValue, setterFunction]`
- **Storage**: State is stored **in memory (RAM) only** - not in a database, localStorage, or any persistent storage. React maintains state in JavaScript memory during the component's lifetime. When the page refreshes or component unmounts, state is lost unless explicitly saved elsewhere.
- **State Lifetime**: State persists for the entire lifetime of the component instance. It is created when the component mounts and destroyed when the component unmounts. State survives re-renders (unlike regular variables which reset on each render).
- **When State Updates**: State updates when you call the setter function (`setLabel(newValue)`). Updates are **asynchronous** - React batches multiple updates and applies them before the next render.
- **How State Updates**: 
  - **Direct value**: `setState(newValue)` - Sets state to the new value
  - **Functional update**: `setState(prev => prev + 1)` - Uses previous state to calculate new state (recommended when new state depends on old state)
  - **Batching**: Multiple setState calls in the same event handler are batched together (only one re-render)
  - **Re-render trigger**: Calling setState triggers a re-render of the component and its children

```typescript
function CreateWallet() {
  const [label, setLabel] = useState(''); // State: label, setter: setLabel
  
  const handleChange = (e) => {
    setLabel(e.target.value); // Update state - triggers re-render
  };
  
  return (
    <input 
      value={label} 
      onChange={handleChange} // Update state on input change
    />
  );
}
```

- <u>**In Our Codebase:**</u>
  - Component state (`useState`) is in-memory only
  - API configuration is persisted to `localStorage` (see `ApiConfigContext.tsx`)
  - Server data is fetched from the Rust backend (not stored in React state permanently)

**2. `useEffect` - Side Effects**

Handles side effects: API calls, subscriptions, DOM manipulation, timers.

- **Purpose**: Perform side effects after render (data fetching, subscriptions, DOM updates)
- **Use Case**: Fetching data, setting up subscriptions, updating document title, cleanup on unmount
- **Runs**: After every render (or when dependencies change)

```typescript
function Component() {
  useEffect(() => {
    // This runs after component mounts
    console.log('Component mounted');
    
    // Cleanup function (runs on unmount)
    return () => {
      console.log('Component unmounting');
    };
  }, []); // Empty array = run once on mount
  
  useEffect(() => {
    // This runs whenever 'dependency' changes
    fetchData(dependency);
  }, [dependency]); // Dependency array
}
```

- <u>**In Our Codebase:**</u>
  - **Minimal usage**: We use `useEffect` sparingly because React Query handles most data fetching and side effects
  - **API Configuration**: Used in `ApiConfigContext.tsx` to update the API client when baseURL or apiKey changes
  - **React Query handles most side effects**: Data fetching, caching, refetching, and error handling are all managed by React Query hooks (`useQuery`, `useMutation`), so we rarely need `useEffect` for API calls
  - **When we do use it**: Primarily for syncing API configuration changes and updating the API client instance

**3. `useContext` - Access Context**

Accesses values from React Context providers without prop drilling.

- **Purpose**: Access values from React Context without prop drilling
- **Use Case**: Global configuration (API settings, theme, user auth), sharing data across component tree
- **Returns**: Context value (whatever was provided by Context Provider)

```typescript
function Component() {
  const { baseURL, apiKey } = useApiConfig(); // Uses context
  // ...
}
```

- <u>**In Our Codebase:**</u>
  - **API Configuration**: We use `useContext` via the `useApiConfig()` custom hook in `ApiConfigContext.tsx`
  - **Usage**: Components access API configuration (baseURL, apiKey) without prop drilling
  - **Context Provider**: `ApiConfigProvider` wraps the entire app in `App.tsx`, making API config available to all components
  - **Benefits**: Avoids passing API config through every component level (no prop drilling)

**4. `useMemo` - Memoize Expensive Calculations**

Caches expensive calculations, only recomputes when dependencies change.

- **Purpose**: Cache expensive computation results, only recompute when dependencies change
- **Use Case**: Sorting/filtering large arrays, complex calculations, creating objects/arrays used as dependencies
- **Returns**: Memoized value

```typescript
const expensiveValue = useMemo(() => {
  return computeExpensiveValue(a, b);
}, [a, b]); // Only recompute if a or b changes
```

- <u>**In Our Codebase:**</u>
  - **Not used**: We don't use `useMemo` in our codebase. React Query handles caching for server data, and our components don't perform expensive calculations that would benefit from memoization.

**5. `useCallback` - Memoize Functions**

Caches function references, only recreates when dependencies change.

- **Purpose**: Cache function references, only recreate when dependencies change
- **Use Case**: Passing functions to memoized child components, functions used as dependencies in other hooks
- **Returns**: Memoized function reference

```typescript
const handleClick = useCallback(() => {
  doSomething(id);
}, [id]); // Function only changes if id changes
```

- <u>**In Our Codebase:**</u>
  - **Not used**: We don't use `useCallback` in our codebase. Our components don't use React.memo() for child components, so function reference stability isn't necessary for performance optimization.

#### Custom Hooks

Custom hooks are functions that use other hooks. They let you extract and reuse stateful logic.

**Naming Convention:** Must start with `use` (e.g., `useBlockchainInfo`)

**Example: Our API Hooks** (`src/hooks/useApi.ts`):

```typescript
// Custom hook that combines React Query + API client
export function useBlockchainInfo(refetchInterval?: number) {
  return useQuery({
    queryKey: ['blockchain', 'info'],
    queryFn: () => getApiClient().getBlockchainInfo(), // Calls Axios
    refetchInterval,
    retry: 1,
  });
}

// Usage in component:
function Dashboard() {
  // This hook provides: data, error, isLoading, refetch
  const { data, error, isLoading } = useBlockchainInfo(5000);
  // Component doesn't need to know about React Query or Axios!
}
```

**Benefits of Custom Hooks:**

1. **Reusability**: Same logic can be used in multiple components
2. **Separation of Concerns**: Data fetching logic separate from UI
3. **Testability**: Hooks can be tested independently
4. **Composability**: Hooks can use other hooks

**Example: Custom Hook Composition**

```typescript
// Hook that uses other hooks
export function useWalletOperations(address: string) {
  const { data: info } = useWalletInfo(address);
  const { data: balance } = useBalance(address);
  const { data: history } = useAddressTransactions(address);
  
  return {
    info,
    balance,
    history,
    isLoading: !info || !balance || !history,
  };
}

// Component uses composed hook
function WalletDetails({ address }) {
  const { info, balance, history, isLoading } = useWalletOperations(address);
  // All wallet data in one hook!
}
```

#### Rules of Hooks

**1. Only call hooks at the top level**

Don't call hooks inside loops, conditions, or nested functions.

```typescript
// ✅ Correct
function Component() {
  const [state, setState] = useState(0);
  useEffect(() => { /* ... */ });
  
  if (condition) {
    // Can use state here, but can't call hooks
    return <div>{state}</div>;
  }
}

// ❌ Wrong
function Component() {
  if (condition) {
    const [state, setState] = useState(0); // Don't do this!
  }
  
  for (let i = 0; i < 10; i++) {
    useEffect(() => { /* ... */ }); // Don't do this!
  }
}
```

**Why?** React relies on the order of hook calls to track state. Conditional calls break this.

**2. Only call hooks from React functions**

Call hooks from:
- React functional components
- Custom hooks (functions starting with `use`)

```typescript
// ✅ Correct - React component
function Component() {
  const value = useSomeHook();
}

// ✅ Correct - Custom hook
function useCustomHook() {
  const value = useSomeHook();
  return value;
}

// ❌ Wrong - Regular function
function regularFunction() {
  const value = useSomeHook(); // Don't do this!
}
```

### State Management

#### State Management Layers

**1. Server State (React Query)**

**Purpose**: Manages data fetched from the API

**Example**:

```typescript
// Query: Fetch blockchain info
const { data, error, isLoading } = useBlockchainInfo();

// Mutation: Create wallet
const createWallet = useCreateWallet();
createWallet.mutate({ label: 'My Wallet' });
```

**Cache Management**:

```typescript
// Cache key structure
['blockchain', 'info']           // Blockchain info
['wallet', 'addresses']          // All addresses
['wallet', 'info', address]      // Wallet info for specific address
['transactions', 'mempool']      // Mempool transactions

// Invalidation after mutation
queryClient.invalidateQueries({ queryKey: ['wallet', 'addresses'] });
```

**2. UI State (React useState)**

**Purpose**: Manages component-local UI state

**Example**:

```typescript
function CreateWallet() {
  const [label, setLabel] = useState('');
  const createWallet = useCreateWallet();
  
  const handleSubmit = () => {
    createWallet.mutate({ label });
    setLabel(''); // Reset form
  };
  
  return (
    <input 
      value={label} 
      onChange={(e) => setLabel(e.target.value)} 
    />
  );
}
```

**3. Global Configuration (React Context)**

**Purpose**: Manages API configuration across the app

**Implementation** (`src/contexts/ApiConfigContext.tsx`):

```typescript
interface ApiConfigContextType {
  baseURL: string;
  apiKey: string;
  setBaseURL: (url: string) => void;
  setApiKey: (key: string) => void;
  isConfigured: boolean;
}

export function ApiConfigProvider({ children }) {
  const [baseURL, setBaseURLState] = useState(() => 
    localStorage.getItem('api_base_url') || 'http://127.0.0.1:8080'
  );
  const [apiKey, setApiKeyState] = useState(() => 
    localStorage.getItem('api_key') || ''
  );

  useEffect(() => {
    // Update API client when config changes
    updateApiClient(baseURL, apiKey || undefined);
  }, [baseURL, apiKey]);

  return (
    <ApiConfigContext.Provider value={{ baseURL, apiKey, ... }}>
      {children}
    </ApiConfigContext.Provider>
  );
}
```

**Usage**:

```typescript
function Navbar() {
  const { baseURL, apiKey, setBaseURL, setApiKey } = useApiConfig();
  
  return (
    <input 
      value={baseURL} 
      onChange={(e) => setBaseURL(e.target.value)} 
    />
  );
}
```

**4. Persistent State (localStorage)**

**Purpose**: Persists API configuration across sessions

**Storage Keys**:
- `api_base_url`: Base URL for API requests
- `api_key`: API key for authentication

**Implementation**:

```typescript
// Save to localStorage
localStorage.setItem('api_base_url', baseURL);
localStorage.setItem('api_key', apiKey);

// Load from localStorage
const baseURL = localStorage.getItem('api_base_url') || 'http://127.0.0.1:8080';
```

### Data Flow

#### Complete Data Flow

```
User Action (Click Button)
    ↓
Component Event Handler
    ↓
React Query Mutation/Query Hook
    ↓
API Client Service
    ↓
Axios HTTP Request
    ↓
Rust Server API Endpoint
    ↓
Response (JSON)
    ↓
React Query Cache Update
    ↓
Component Re-render
    ↓
UI Update
```

#### Complete Hook Flow Example

Here's how hooks work together in our application:

```typescript
// 1. Custom hook (uses React Query hook)
export function useBlockchainInfo() {
  return useQuery({
    queryKey: ['blockchain', 'info'],
    queryFn: () => getApiClient().getBlockchainInfo(), // Calls Axios
  });
}

// 2. Component uses custom hook
function Dashboard() {
  // React Query manages: caching, refetching, loading states
  const { data, error, isLoading } = useBlockchainInfo();
  
  // Component state for UI
  const [isExpanded, setIsExpanded] = useState(false);
  
  // Side effect (minimal, React Query handles most)
  useEffect(() => {
    document.title = 'Dashboard';
  }, []);
  
  if (isLoading) return <LoadingSpinner />;
  if (error) return <ErrorMessage error={error} />;
  
  return <div>{/* Render data */}</div>;
}
```

**Complete Flow:**
1. Component calls `useBlockchainInfo()` hook
2. Hook calls React Query's `useQuery()`
3. React Query calls `queryFn` which uses Axios
4. Axios makes HTTP request to Rust server
5. Response flows back through Axios → React Query → Custom Hook → Component
6. Component re-renders with new data
7. React Query caches the data for future use

---

## API Integration

### API Client Architecture

```
Component
    ↓
useApi Hook (React Query)
    ↓
getApiClient() → ApiClient Instance
    ↓
Axios HTTP Request
    ↓
Rust Server (/api/admin/*)
    ↓
JSON Response
    ↓
TypeScript Type Validation
    ↓
React Query Cache
```

### Authentication Flow

```typescript
// 1. User configures API key in UI
setApiKey('admin-secret');

// 2. Context updates API client
updateApiClient(baseURL, apiKey);

// 3. API client adds header to all requests
this.client = axios.create({
  headers: {
    'X-API-Key': apiKey,  // Added to every request
  },
});

// 4. Rust server validates header
// (see bitcoin/src/web/middleware/auth.rs)
```

### API Endpoint Mapping

| UI Feature | React Hook | API Client Method | Rust Endpoint |
|------------|-----------|-------------------|---------------|
| Dashboard Stats | `useBlockchainInfo()` | `getBlockchainInfo()` | `GET /api/admin/blockchain` |
| Latest Blocks | `useLatestBlocks()` | `getLatestBlocks()` | `GET /api/admin/blockchain/blocks/latest` |
| All Blocks | `useAllBlocks(page, limit)` | `getAllBlocks(page, limit)` | `GET /api/admin/blockchain/blocks?page=0&limit=100` |
| All Transactions | `useAllTransactions(page, limit)` | `getAllTransactions(page, limit)` | `GET /api/admin/transactions?page=1&limit=100` |
| Address Transactions | `useAddressTransactions(address)` | `getAddressTransactions(address)` | `GET /api/admin/transactions/address/{address}?page=1&limit=10` |
| Create Wallet | `useCreateWallet()` | `createWallet()` | `POST /api/admin/wallet` |
| Send Transaction | `useSendTransaction()` | `sendTransaction()` | `POST /api/admin/transactions` |
| Mining Info | `useMiningInfo()` | `getMiningInfo()` | `GET /api/admin/mining/info` |
| Generate Blocks | `useGenerateBlocks()` | `generateToAddress()` | `POST /api/admin/mining/generatetoaddress` |

**Note on Pagination:**
- Block pagination uses 0-indexed pages (page 0 = first page)
- Transaction pagination uses 1-indexed pages (page 1 = first page)
- Default limits: Blocks (100 per page), Transactions (100 per page), Address Transactions (10 per page)
- Backend properly slices paginated results before returning them

### Error Handling

```typescript
// In API client
async getBlockchainInfo(): Promise<ApiResponse<BlockchainInfo>> {
  try {
    const response = await this.client.get('/api/admin/blockchain');
    return response.data;
  } catch (error) {
    // Axios automatically throws on 4xx/5xx
    throw error;
  }
}

// In React Query hook
export function useBlockchainInfo() {
  return useQuery({
    queryKey: ['blockchain', 'info'],
    queryFn: () => getApiClient().getBlockchainInfo(),
    retry: 1,  // Retry once on failure
    onError: (error) => {
      // Error is available in component via error state
    },
  });
}

// In component
function Dashboard() {
  const { data, error, isLoading } = useBlockchainInfo();
  
  if (error) {
    return <ErrorMessage error={error} />;
  }
  // ...
}
```

---

## Component Architecture

### Component Overview

The application is organized into logical component groups, each serving a specific purpose:

#### Layout Components (`src/components/Layout/`)

- **`Layout.tsx`** - Main application layout wrapper that provides the overall page structure, containing Navbar and Sidebar
- **`Navbar.tsx`** - Top navigation bar displaying the app title and API configuration menu for setting base URL and API key
- **`Sidebar.tsx`** - Left sidebar navigation menu with collapsible sections for accessing different features (Blockchain, Wallet, Transactions, Mining, Health). Dropdown menus remain active/open when navigating within their sub-items and only close when navigating to a different main menu item. Uses custom state management with Transition components for smooth animations.

#### Dashboard Component (`src/components/Dashboard/`)

- **`Dashboard.tsx`** - Main dashboard page displaying real-time blockchain statistics (block height, total blocks, transactions, mempool size) with auto-refresh every 5 seconds. The total transactions count is calculated from the blockchain by counting all transactions across all blocks (not hardcoded).

#### Blockchain Components (`src/components/Blockchain/`)

- **`BlockchainInfo.tsx`** - Displays comprehensive blockchain information (height, difficulty, total blocks, etc.) with JSON viewer
- **`LatestBlocks.tsx`** - Shows the most recent blocks in a list format with block details (hash, height, timestamp, transaction count). Dates are formatted using a safe date formatting utility to handle various date formats gracefully.
- **`AllBlocks.tsx`** - Fetches and displays all blocks in the blockchain with pagination support (100 blocks per page by default). Includes "Load All Blocks" functionality to fetch all blocks across multiple pages and display them in a scrollable container (max height 80vh). Supports pagination controls (Previous/Next) and displays pagination information.
- **`BlockByHash.tsx`** - Search interface to find and display a specific block by its hash value

#### Wallet Components (`src/components/Wallet/`)

- **`CreateWallet.tsx`** - Form to create new wallets with optional label, displays the generated address upon success
- **`WalletInfo.tsx`** - Retrieves and displays detailed information for a specific wallet address
- **`Balance.tsx`** - Shows confirmed and unconfirmed balance for a wallet address
- **`SendTransaction.tsx`** - Form to send Bitcoin transactions (from address, to address, amount in satoshis)
- **`TransactionHistory.tsx`** - Displays transaction history for a specific wallet address with pagination support. Properly handles paginated responses from the API, showing transaction items from the `items` array. Displays pagination information (current page, total pages, total count) and handles empty results gracefully.
- **`AllAddresses.tsx`** - Lists all wallet addresses with quick action buttons to view info, balance, or history for each address. The history view properly handles paginated transaction responses.

#### Transaction Components (`src/components/Transactions/`)

- **`Mempool.tsx`** - Displays all transactions currently in the mempool (pending transactions)
- **`MempoolTx.tsx`** - Search interface to find and display a specific mempool transaction by transaction ID
- **`AllTransactions.tsx`** - Fetches and displays all transactions in the blockchain with pagination support (100 transactions per page by default). Includes "Load All Transactions" functionality to fetch all transactions across multiple pages and display them in a scrollable container (max height 80vh). Supports pagination controls (Previous/Next) and displays pagination information. The backend properly slices paginated results before returning them.
- **`AddressTransactions.tsx`** - Shows all transactions associated with a specific address

#### Mining Components (`src/components/Mining/`)

- **`MiningInfo.tsx`** - Displays current mining status and information
- **`GenerateBlocks.tsx`** - Form to generate new blocks with configurable mining address, number of blocks, and max tries

#### Health Components (`src/components/Health/`)

- **`HealthCheck.tsx`** - Displays overall system health status
- **`Liveness.tsx`** - Checks if the system is alive and responding
- **`Readiness.tsx`** - Checks if the system is ready to accept requests

#### Common/Shared Components (`src/components/common/`)

- **`LoadingSpinner.tsx`** - Reusable loading indicator with spinning animation
- **`ErrorMessage.tsx`** - Displays error messages in a styled error container
- **`JsonViewer.tsx`** - Displays JSON data in a formatted, expandable viewer with copy-to-clipboard functionality
- **`StatCard.tsx`** - Reusable card component for displaying statistics (label, value, optional icon)

#### Utility Functions (`src/utils/`)

- **`date.ts`** - Date formatting utilities:
  - `formatDate()` - Safely formats date strings, handling invalid dates and null/undefined inputs gracefully
  - `formatDateWithTimezone()` - Formats dates with timezone information
  - Handles various date formats including Rust's `chrono::DateTime<Utc>` format
  - Returns "N/A" for null/undefined dates and "Invalid Date" for unparseable date strings

### Component Hierarchy

```
App
├── QueryClientProvider (React Query)
├── ApiConfigProvider (Context)
├── BrowserRouter (React Router)
├── Layout
│   ├── Navbar
│   │   └── API Config Menu
│   └── Sidebar
│       └── Navigation Menu
└── Routes
    ├── Dashboard
    ├── Blockchain/*
    ├── Wallet/*
    ├── Transactions/*
    ├── Mining/*
    └── Health/*
```

### Component Patterns

#### 1. Container/Presentational Pattern

**Container Component** (handles logic):

```typescript
// src/components/Blockchain/BlockchainInfo.tsx
export function BlockchainInfo() {
  const { data, error, isLoading, refetch } = useBlockchainInfo();
  
  // Logic: data fetching, error handling
  if (isLoading) return <LoadingSpinner />;
  if (error) return <ErrorMessage error={error} />;
  
  // Presentation: render UI
  return (
    <div>
      <h1>Blockchain Info</h1>
      <JsonViewer data={data.data} />
    </div>
  );
}
```

#### 2. Reusable Components

**Common Components** (`src/components/common/`):

```typescript
// LoadingSpinner.tsx
export function LoadingSpinner() {
  return (
    <div className="flex items-center justify-center p-8">
      <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-bitcoin-orange"></div>
    </div>
  );
}

// JsonViewer.tsx
export function JsonViewer({ data, title }: JsonViewerProps) {
  const jsonString = JSON.stringify(data, null, 2);
  
  const copyToClipboard = () => {
    navigator.clipboard.writeText(jsonString);
    toast.success('Copied to clipboard!');
  };
  
  return (
    <div className="bg-gray-800 rounded-lg">
      {title && <h3>{title}</h3>}
      <button onClick={copyToClipboard}>Copy</button>
      <pre>{jsonString}</pre>
    </div>
  );
}
```

#### 3. Form Components

**Example** (`src/components/Wallet/CreateWallet.tsx`):

```typescript
export function CreateWallet() {
  const [label, setLabel] = useState('');
  const createWallet = useCreateWallet();
  
  const handleSubmit = () => {
    createWallet.mutate({ label: label.trim() });
    setLabel('');
  };
  
  return (
    <div className="card">
      <input
        type="text"
        value={label}
        onChange={(e) => setLabel(e.target.value)}
        className="input-field"
        placeholder="Wallet Label"
      />
      <button 
        onClick={handleSubmit}
        className="btn-primary"
        disabled={createWallet.isPending}
      >
        {createWallet.isPending ? 'Creating...' : 'Create Wallet'}
      </button>
      
      {createWallet.data?.data && (
        <div className="success-message">
          Wallet created: {createWallet.data.data.address}
        </div>
      )}
    </div>
  );
}
```

---

## Build and Deployment

### Development Build

```bash
# Install dependencies
npm install

# Start dev server
npm run dev
```

**What Happens:**
1. Vite starts dev server on `http://localhost:3000`
2. React app loads with Hot Module Replacement
3. API requests proxied to `http://127.0.0.1:8080`
4. Changes reflect instantly without page reload

### Production Build

```bash
# Build for production
npm run build
```

**Build Process:**
1. TypeScript compilation (`tsc`)
2. Vite bundling and optimization
3. Code splitting and tree shaking
4. Asset optimization (minification, compression)
5. Output to `dist/` directory

**Output Structure:**
```
dist/
├── index.html          # Main HTML file
├── assets/
│   ├── index-[hash].js # Main JavaScript bundle
│   ├── index-[hash].css # Styles
│   └── [other assets]  # Images, fonts, etc.
```

### Integration with Rust Server

**Rust Server Configuration** (`bitcoin/src/web/routes/web.rs`):

```rust
pub fn create_web_routes() -> Router<Arc<NodeContext>> {
    // Check for React app build
    let react_app_path = find_react_app_path();
    
    if let Some(path) = react_app_path {
        // Serve React app
        Router::new()
            .nest_service("/assets", ServeDir::new(&assets_path))
            .route("/", get(serve_react_app))
            .fallback(get(serve_react_app))  // React Router fallback
            .merge(create_swagger_ui())
    } else {
        // Show build instructions
        Router::new()
            .route("/", get(react_app_not_built))
            .merge(create_swagger_ui())
    }
}
```

**Serving Flow:**
1. Request comes to Rust server
2. If path starts with `/api` → API endpoint
3. If path starts with `/assets` → Serve static file
4. Otherwise → Serve `index.html` (React Router handles routing)

---

## Configuration

### Environment Variables

**Development** (Vite):
- `VITE_API_BASE_URL`: Override API base URL (optional)

**Runtime Configuration** (localStorage):
- `api_base_url`: Base URL for API requests
- `api_key`: API key for authentication

### TypeScript Configuration

**tsconfig.json**:

```json
{
  "compilerOptions": {
    "target": "ES2020",
    "lib": ["ES2020", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "moduleResolution": "bundler",
    "jsx": "react-jsx",
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
  },
  "include": ["src"]
}
```

**Key Settings:**
- `strict: true`: Enables all strict type checking
- `jsx: "react-jsx"`: Modern JSX transform (no React import needed)
- `moduleResolution: "bundler"`: Optimized for Vite bundler

### Tailwind Configuration

**tailwind.config.js**:

```javascript
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
}
```

**PostCSS Configuration** (`postcss.config.js`):

```javascript
export default {
  plugins: {
    tailwindcss: {},
    autoprefixer: {},
  },
}
```

---

## Recent Improvements and Bug Fixes

### Pagination Enhancements

**Backend Pagination Fixes:**
- Fixed pagination in `get_transactions()` and `get_address_transactions()` endpoints to properly slice results before creating paginated responses
- Fixed `PaginatedResponse` pagination logic: `has_next` and `has_prev` now correctly handle 1-indexed pages
- Backend now correctly applies pagination by slicing the results array before returning

**Frontend Pagination Improvements:**
- **All Blocks**: Added pagination support (100 blocks per page), "Load All Blocks" functionality, and scrollable display (max-height 80vh)
- **All Transactions**: Added pagination support (100 transactions per page), "Load All Transactions" functionality, and scrollable display (max-height 80vh)
- **Transaction History**: Fixed to properly display paginated results by accessing `data.data.items` instead of `data.data` directly
- All paginated components now display pagination information (current page, total pages, total count)

### Dashboard Improvements

**Total Transactions Fix:**
- Fixed Dashboard to calculate actual total transaction count from the blockchain instead of hardcoded zero
- Backend now calls `node.find_all_transactions().await` and counts the transactions
- Error handling added with proper logging

### Date Formatting

**Date Display Fixes:**
- Created `src/utils/date.ts` with safe date formatting utilities
- Fixed "Invalid Date" display issues by properly handling Rust's `chrono::DateTime<Utc>` format
- Dates now display correctly in Latest Blocks and other components
- Handles edge cases: null/undefined dates, invalid date strings, various date formats

### UI/UX Improvements

**Sidebar Navigation:**
- Dropdown menus now remain open when navigating within their sub-items
- Menus only close when navigating to a different main menu item
- Improved user experience with persistent menu state

**Component Display:**
- Increased default pagination limits for better data visibility (100 items per page for blocks and transactions)
- Added scrollable containers with max-height constraints for long lists
- Improved pagination controls and information display

## Summary

The Bitcoin Web UI is a modern, type-safe React application that provides a comprehensive interface for managing a Bitcoin blockchain node. It leverages:

- **React 18** for component-based UI
- **TypeScript** for type safety
- **Vite** for fast development and optimized builds
- **React Query** for efficient server state management
- **React Router** for client-side routing
- **Tailwind CSS** for rapid UI development
- **Axios** for HTTP requests
- **Headless UI** for accessible components

The architecture separates concerns cleanly:
- **Components**: UI presentation
- **Hooks**: Data fetching logic
- **Services**: API communication
- **Context**: Global configuration
- **Types**: Type safety
- **Utils**: Utility functions (date formatting, etc.)

This structure makes the codebase maintainable, testable, and scalable. Recent improvements have enhanced pagination, fixed data display issues, and improved the overall user experience.

---

<div align="center">

**📚 [← Previous: Embedded Database & Persistence](../bitcoin-wallet-ui/05-Embedded-Database.md)** | **Chapter 7: Web Admin Interface** | **[Next: Docker Compose Deployment →](../ci/docker-compose/01-Introduction.md)** 📚

</div>

---

*This chapter has explored the Bitcoin Web UI, a modern single-page application built with React and TypeScript that provides a comprehensive administrative interface for managing a Bitcoin blockchain node. We've examined how modern web technologies—React 18, TypeScript, Vite, React Query, React Router, and Tailwind CSS—come together to create a powerful, type-safe, and maintainable web application. The architecture demonstrates clean separation of concerns between components, hooks, services, context, types, and utilities, creating a scalable foundation for web-based blockchain administration. The evolution from static HTML pages to a fully-featured interactive application showcases the power of modern web development practices. In the next chapter, we'll explore [Docker Compose Deployment](../ci/docker-compose/01-Introduction.md) to understand how to deploy and manage the blockchain network using containerization.*

---

<div align="center">

**Local Navigation - Table of Contents**

| [← First Section: Overview](#overview) | [↑ Table of Contents](#table-of-contents) | [Last Section: Configuration →](#configuration) |
|:---:|:---:|:---:|
| *Start of Chapter* | *Current Chapter* | *End of Chapter* |

</div>

---
