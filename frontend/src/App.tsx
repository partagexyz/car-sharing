import React, { useState, useEffect } from 'react';
import { Routes, Route, BrowserRouter } from 'react-router-dom';
import HomePage from './pages/home';
import SetupPage from './pages/setup';
import Authenticate from './pages/login/Authenticate';
import { AccessTokenWrapper } from '@calimero-is-near/calimero-p2p-sdk';
import { getNodeUrl } from './utils/node';
import { NEARSigner, NEARSpecificSigner } from './utils/types';
import * as nearAPI from 'near-api-js';
import { JsonRpcClient, WsSubscriptionsClient } from "@calimero-is-near/calimero-p2p-sdk";

const NEAR_CONFIG = {
  networkId: 'testnet',
  nodeUrl: 'https://rpc.testnet.near.org',
  walletUrl: 'https://wallet.testnet.near.org',
  helperUrl: 'https://helper.testnet.near.org',
  explorerUrl: 'https://explorer.testnet.near.org',
  contractName: 'your-contract-name.testnet', // Replace with your actual contract name
};

interface AppProps {
  rpcClient: JsonRpcClient;
  subscriptionsClient: WsSubscriptionsClient;
}

function App({ rpcClient, subscriptionsClient }: AppProps) {
  const [near, setNear] = useState<nearAPI.Near | null>(null);
  const [account, setAccount] = useState<nearAPI.Account | null>(null);

  useEffect(() => {
    const initContract = async () => {
      const near = await nearAPI.connect(Object.assign({
        deps: { keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore() }
      }, NEAR_CONFIG));
      setNear(near);
    };
    initContract();
  }, []);

  useEffect(() => {
    if (near) {
      if ('getAccountId' in near.connection.signer) {
        (near.connection.signer as NEARSpecificSigner).getAccountId().then((accountId:string) => {
          if (accountId) {
            const nearAccount = new nearAPI.Account(near.connection, accountId);
            setAccount(nearAccount);
          }
        });
      } else {
        console.warn("Signer does not support getAccountId method");
      }
    }
  }, [near]);

  return (
    <AccessTokenWrapper getNodeUrl={getNodeUrl}>
      <BrowserRouter basename="/core-app-template/">
        <Routes>
          <Route path="/" element={<SetupPage />} />
          <Route path="/auth" element={<Authenticate />} />
          <Route path="/home" element={<HomePage near={near} account={account} />} />
        </Routes>
      </BrowserRouter>
    </AccessTokenWrapper>
  );
}

export default App;