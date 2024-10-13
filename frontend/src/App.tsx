import React from 'react';
import { JsonRpcClient, WsSubscriptionsClient } from "@calimero-is-near/calimero-p2p-sdk";
import logo from './logo.svg';
import './App.css';

interface AppProps {
  rpcClient: JsonRpcClient;
  subscriptionsClient: WsSubscriptionsClient;
}

function App({ rpcClient, subscriptionsClient }: AppProps) {
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header>
    </div>
  );
}

export default App;
