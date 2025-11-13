import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api';

export const useSSE = () => {
  const [isConnected, setIsConnected] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const connect = useCallback(async () => {
    try {
      await invoke('start_sse_connection');
      setIsConnected(true);
      setError(null);
    } catch (err) {
      setError(err as string);
      setIsConnected(false);
      // Fallback to polling
      startPolling();
    }
  }, []);

  const disconnect = useCallback(async () => {
    try {
      await invoke('stop_sse_connection');
      setIsConnected(false);
    } catch (err) {
      console.error('Failed to disconnect:', err);
    }
  }, []);

  const reconnect = useCallback(async () => {
    await disconnect();
    setTimeout(() => connect(), 1000);
  }, [connect, disconnect]);

  const startPolling = useCallback(() => {
    const pollInterval = setInterval(async () => {
      try {
        const isHealthy = await invoke<boolean>('check_server_health');
        if (isHealthy && !isConnected) {
          reconnect();
          clearInterval(pollInterval);
        }
      } catch (err) {
        console.error('Polling error:', err);
      }
    }, 30000); // Poll every 30 seconds

    return () => clearInterval(pollInterval);
  }, [isConnected, reconnect]);

  useEffect(() => {
    connect();
    return () => {
      disconnect();
    };
  }, [connect, disconnect]);

  return {
    isConnected,
    error,
    reconnect,
  };
};
