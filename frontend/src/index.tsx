import React from 'react';
import { createRoot }  from 'react-dom/client';
import './index.css';
import App from './App';
import { JsonRpcClient, WsSubscriptionsClient } from "@calimero-is-near/calimero-p2p-sdk";

// React 18
const rootElement = document.getElementById('root');

if (rootElement) {
  const root = createRoot(rootElement);

  // global setup for RpcClient and SubscriptionsClient
  const rpcClient = new JsonRpcClient(process.env["REACT_APP_API_URL"] || "default-url", "/jsonrpc");
  const subscriptionsClient = new WsSubscriptionsClient(process.env["REACT_APP_API_URL"] || "default-url", "/ws");

  root.render(
    <React.StrictMode>
      <App
        rpcClient={rpcClient}
        subscriptionsClient={subscriptionsClient}
      />
    </React.StrictMode>
  );
} else {
  console.error('Root element not found');
}

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
// import reportWebVitals from './reportWebVitals';
// reportWebVitals();
