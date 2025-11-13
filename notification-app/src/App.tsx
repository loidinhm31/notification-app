import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';
import NotificationPopup from './components/NotificationPopup';
import { useSSE } from './hooks/useSSE';
import { NotificationEvent } from './types';
import './styles/App.css';

function App() {
  const [currentNotification, setCurrentNotification] = useState<NotificationEvent | null>(null);

  const { isConnected, reconnect } = useSSE();

  useEffect(() => {
    // Listen for notification events from Tauri
    const unlisten = listen<NotificationEvent>('new-notification', (event) => {
      console.log('Received notification:', event.payload);
      setCurrentNotification(event.payload);
    });

    // Listen for notification-server down events
    const unlistenServerDown = listen('notification-server-down', () => {
      console.log('Server is down, falling back to polling');
      reconnect();
    });

    return () => {
      unlisten.then(fn => fn());
      unlistenServerDown.then(fn => fn());
    };
  }, [reconnect]);

  const handleCloseNotification = () => {
    setCurrentNotification(null);
  };

  return (
    <div className="app">
      {currentNotification && (
        <NotificationPopup
          notification={currentNotification}
          onClose={handleCloseNotification}
        />
      )}

      <div className="status-bar">
        <div className={`status-indicator ${isConnected ? 'connected' : 'disconnected'}`}>
          {isConnected ? '● Connected' : '○ Disconnected'}
        </div>
      </div>
    </div>
  );
}

export default App;
