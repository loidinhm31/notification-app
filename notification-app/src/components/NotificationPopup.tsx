import React, { useEffect, useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { X } from 'lucide-react';
import { NotificationEvent } from '../types';
import MeditationAnimation from './MeditationAnimation';

interface Props {
  notification: NotificationEvent;
  onClose: () => void;
}

const NotificationPopup: React.FC<Props> = ({ notification, onClose }) => {
  const [isVisible, setIsVisible] = useState(true);

  useEffect(() => {
    // Auto-hide after 5 seconds
    const timer = setTimeout(() => {
      setIsVisible(false);
      setTimeout(onClose, 300); // Wait for animation
    }, 15000);

    return () => clearTimeout(timer);
  }, [onClose]);

  return (
    <AnimatePresence>
      {isVisible && (
        <motion.div
          initial={{ opacity: 0, y: -50, scale: 0.9 }}
          animate={{ opacity: 1, y: 0, scale: 1 }}
          exit={{ opacity: 0, y: -50, scale: 0.9 }}
          transition={{ duration: 0.3, type: 'spring' }}
          className="notification-popup"
        >
          <div className="notification-content">
            <button className="close-btn" onClick={(e) => {
              e.stopPropagation();
              onClose();
            }}>
              <X size={16} />
            </button>

            <div className="notification-body">
              <div className="notification-text">
                <div className="notification-header">
                  <h3>{notification.title}</h3>
                  <span className="timestamp">
                    {new Date(notification.timestamp).toLocaleTimeString()}
                  </span>
                </div>
                <p className="notification-message">{notification.message}</p>
              </div>

              <div className="notification-animation">
                <MeditationAnimation />
              </div>
            </div>
          </div>

          {/* Decorative elements */}
          <div className="decorative-flowers">
            <div className="flower flower-left">🌸</div>
            <div className="flower flower-right">🌺</div>
          </div>
        </motion.div>
      )}
    </AnimatePresence>
  );
};

export default NotificationPopup;
