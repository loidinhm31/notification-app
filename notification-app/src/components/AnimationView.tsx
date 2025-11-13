import React, { useEffect } from 'react';
import { motion } from 'framer-motion';

interface Props {
  url: string;
  onClose: () => void;
  size?: 'small' | 'medium' | 'large' | 'popup-size';
}

const AnimationView: React.FC<Props> = ({ url, onClose, size = 'medium' }) => {
  // Size configurations
  const sizeConfig = {
    'popup-size': { width: '1650px', height: '350px', maxWidth: undefined, maxHeight: undefined }, // Match notification popup
    'small': { width: '500px', height: '400px', maxWidth: undefined, maxHeight: undefined },
    'medium': { width: '800px', height: '600px', maxWidth: undefined, maxHeight: undefined },
    'large': { width: '90%', height: '90%', maxWidth: '1200px', maxHeight: '800px' }
  };

  const currentSize = sizeConfig[size];
  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onClose();
      }
    };

    window.addEventListener('keydown', handleEscape);
    return () => window.removeEventListener('keydown', handleEscape);
  }, [onClose]);

  return (
    <motion.div
      initial={{ opacity: 0 }}
      animate={{ opacity: 1 }}
      exit={{ opacity: 0 }}
      className="animation-view"
      onClick={onClose}
    >
      <motion.div
        initial={{ scale: 0.8 }}
        animate={{ scale: 1 }}
        transition={{ type: 'spring', duration: 0.5 }}
        className="animation-container"
        style={{
          width: currentSize.width,
          height: currentSize.height,
          maxWidth: currentSize.maxWidth,
          maxHeight: currentSize.maxHeight
        }}
        onClick={(e) => e.stopPropagation()}
      >
        <iframe
          src={url}
          className="animation-iframe"
          title="Animation"
          sandbox="allow-scripts allow-same-origin"
          allow="autoplay"
        />

        <button className="close-animation-btn" onClick={onClose}>
          Close (ESC)
        </button>
      </motion.div>
    </motion.div>
  );
};

export default AnimationView;
