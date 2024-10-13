import { getAccessToken } from '@calimero-is-near/calimero-p2p-sdk';

// Define storage keys
export const APP_URL = 'car-sharing-app-url';
export const CONTEXT_ID = 'car-sharing-context-id';
export const APPLICATION_ID = 'car-sharing-app-id';
export const USER_ACCOUNT = 'car-sharing-user-account';

// Storage functions for app endpoint
export const getStorageAppEndpointKey = (): string | null => {
  try {
    if (typeof window !== 'undefined' && window.localStorage) {
      let storageRecord: string | null = localStorage.getItem(APP_URL);
      if (storageRecord) {
        let url: string = JSON.parse(storageRecord);
        if (url && url.length > 0) {
          return url;
        }
      }
    }
    return null;
  } catch (e) {
    console.error('Failed to get app endpoint:', e);
    return null;
  }
};

export const setStorageAppEndpointKey = (url: string) => {
  localStorage.setItem(APP_URL, JSON.stringify(url));
};

// Context ID management
export const getStorageContextId = (): string | null => {
  if (typeof window !== 'undefined' && window.localStorage) {
    return localStorage.getItem(CONTEXT_ID);
  }
  return null;
};

export const setStorageContextId = (contextId: string) => {
  localStorage.setItem(CONTEXT_ID, contextId);
};

// Application ID
export const getApplicationId = (): string | null => {
  if (typeof window !== 'undefined' && window.localStorage) {
    return localStorage.getItem(APPLICATION_ID);
  }
  return null;
};

export const setStorageApplicationId = (applicationId: string) => {
  localStorage.setItem(APPLICATION_ID, applicationId);
};

// User account storage
export const getStorageUserAccount = (): string | null => {
  if (typeof window !== 'undefined' && window.localStorage) {
    return localStorage.getItem(USER_ACCOUNT);
  }
  return null;
};

export const setStorageUserAccount = (accountId: string) => {
  localStorage.setItem(USER_ACCOUNT, accountId);
};

// JWT handling
export interface JsonWebToken {
  context_id: string;
  token_type: string;
  exp: number;
  sub: string;
}

export interface Car {
  id: string;
  ownerId: string;
  available: boolean;
  hourlyRate: number;
}

export const getJWTObject = (): JsonWebToken | null => {
  const token = getAccessToken();
  if (!token) return null;
  const parts = token.split('.');
  if (parts.length !== 3) {
    console.error('Invalid JWT token');
    return null;
  }
  try {
    const payload = JSON.parse(atob(parts[1]));
    return payload as JsonWebToken;
  } catch (e) {
    console.error('Failed to parse JWT payload:', e);
    return null;
  }
};

export const getJWT = (): string | null => {
  return getAccessToken();
};

// Utility function to clear all stored items
export const clearAllStorage = () => {
  if (typeof window !== 'undefined' && window.localStorage) {
    [APP_URL, CONTEXT_ID, APPLICATION_ID, USER_ACCOUNT].forEach(key => localStorage.removeItem(key));
  }
};