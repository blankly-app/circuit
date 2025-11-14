# Circuit WebAssembly Bindings for React

This document describes how to use Circuit in React applications via WebAssembly.

## Building the WASM Module

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build the WASM module
cd circuit-wasm
wasm-pack build --target web

# Or for bundler environments (webpack, rollup, etc.)
wasm-pack build --target bundler
```

## React Integration

### 1. Install the Package

After building, you can publish to npm or use it locally:

```bash
# Link locally
cd circuit-wasm/pkg
npm link

# In your React project
npm link circuit-wasm
```

### 2. React Hook

Create a custom hook for Circuit:

```typescript
import { useState, useEffect } from 'react';
import init, { WasmEngine } from 'circuit-wasm';

export function useCircuit() {
  const [engine, setEngine] = useState<WasmEngine | null>(null);
  const [isReady, setIsReady] = useState(false);

  useEffect(() => {
    init().then(() => {
      const newEngine = new WasmEngine();
      setEngine(newEngine);
      setIsReady(true);
    });
  }, []);

  const loadGraph = async (graphJson: string) => {
    if (!engine) throw new Error('Engine not initialized');
    engine.loadGraph(graphJson);
  };

  const executeGraph = async (graphId: string): Promise<any> => {
    if (!engine) throw new Error('Engine not initialized');
    const resultJson = engine.executeGraph(graphId);
    return JSON.parse(resultJson);
  };

  const listBlocks = (): string[] => {
    if (!engine) return [];
    return engine.listBlocks();
  };

  const listGraphs = (): string[] => {
    if (!engine) return [];
    return engine.listGraphs();
  };

  return {
    isReady,
    loadGraph,
    executeGraph,
    listBlocks,
    listGraphs,
  };
}
```

### 3. React Component Example

```tsx
import React, { useState } from 'react';
import { useCircuit } from './hooks/useCircuit';

function CalculatorApp() {
  const { isReady, loadGraph, executeGraph, listBlocks } = useCircuit();
  const [result, setResult] = useState<any>(null);

  const runCalculation = async () => {
    const graph = {
      id: 'calc',
      name: 'Calculator',
      nodes: {
        const1: {
          id: 'const1',
          block_type: 'core.constant',
          config: {
            value: { type: 'Float', value: 5.0 }
          },
          position: null
        },
        const2: {
          id: 'const2',
          block_type: 'core.constant',
          config: {
            value: { type: 'Float', value: 3.0 }
          },
          position: null
        },
        add: {
          id: 'add',
          block_type: 'math.add',
          config: {},
          position: null
        }
      },
      connections: [
        {
          from_node: 'const1',
          from_port: 'value',
          to_node: 'add',
          to_port: 'a'
        },
        {
          from_node: 'const2',
          from_port: 'value',
          to_node: 'add',
          to_port: 'b'
        }
      ]
    };

    try {
      await loadGraph(JSON.stringify(graph));
      const results = await executeGraph('calc');
      setResult(results);
    } catch (error) {
      console.error('Execution failed:', error);
    }
  };

  if (!isReady) {
    return <div>Loading Circuit engine...</div>;
  }

  return (
    <div>
      <h1>Circuit Calculator</h1>
      <button onClick={runCalculation}>Run Calculation</button>
      
      {result && (
        <div>
          <h2>Results:</h2>
          <pre>{JSON.stringify(result, null, 2)}</pre>
        </div>
      )}

      <div>
        <h3>Available Blocks:</h3>
        <ul>
          {listBlocks().map(block => (
            <li key={block}>{block}</li>
          ))}
        </ul>
      </div>
    </div>
  );
}

export default CalculatorApp;
```

### 4. Vite Configuration

If using Vite, add this to `vite.config.ts`:

```typescript
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

export default defineConfig({
  plugins: [
    react(),
    wasm(),
    topLevelAwait(),
  ],
});
```

### 5. Webpack Configuration

For webpack projects, add to `webpack.config.js`:

```javascript
module.exports = {
  // ...
  experiments: {
    asyncWebAssembly: true,
  },
  module: {
    rules: [
      {
        test: /\.wasm$/,
        type: 'webassembly/async',
      },
    ],
  },
};
```

## TypeScript Definitions

The WASM build includes TypeScript definitions. For better type safety:

```typescript
interface GraphNode {
  id: string;
  block_type: string;
  config: Record<string, any>;
  position: [number, number] | null;
}

interface Connection {
  from_node: string;
  from_port: string;
  to_node: string;
  to_port: string;
}

interface Graph {
  id: string;
  name: string;
  description?: string;
  nodes: Record<string, GraphNode>;
  connections: Connection[];
}

interface ExecutionResult {
  [nodeId: string]: {
    [portId: string]: any;
  };
}
```

## Performance Tips

1. **Minimize serialization**: Keep graphs in memory when possible
2. **Batch operations**: Group multiple graph operations together
3. **Use Web Workers**: Run Circuit in a Web Worker for better performance
4. **Cache results**: Store execution results to avoid re-computation

## Browser Compatibility

Circuit WASM requires:
- WebAssembly support (all modern browsers)
- ES6+ JavaScript features
- Async/await support

Supported browsers:
- Chrome 57+
- Firefox 52+
- Safari 11+
- Edge 16+
