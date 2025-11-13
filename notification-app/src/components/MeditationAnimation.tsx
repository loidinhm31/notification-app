import React from 'react';
import { motion } from 'framer-motion';
import '../styles/MeditationAnimation.css';

const MeditationAnimation: React.FC = () => {
  return (
    <div className="meditation-container">
      <svg
        width="120"
        height="160"
        viewBox="0 0 120 160"
        fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        {/* Aura/Glow effect */}
        <motion.circle
          cx="60"
          cy="80"
          r="50"
          fill="url(#auraGradient)"
          animate={{
            opacity: [0.3, 0.6, 0.3],
            scale: [1, 1.1, 1],
          }}
          transition={{
            duration: 3,
            repeat: Infinity,
            ease: 'easeInOut',
          }}
        />

        {/* Person's body */}
        <g className="person">
          {/* Head */}
          <motion.circle
            cx="60"
            cy="35"
            r="12"
            fill="#FFD7BA"
            stroke="#333"
            strokeWidth="1.5"
            animate={{
              y: [0, -1, 0],
            }}
            transition={{
              duration: 4,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />

          {/* Eyes closed - peaceful expression */}
          <motion.path
            d="M 54 33 Q 56 34 58 33"
            stroke="#333"
            strokeWidth="1.5"
            fill="none"
            strokeLinecap="round"
          />
          <motion.path
            d="M 62 33 Q 64 34 66 33"
            stroke="#333"
            strokeWidth="1.5"
            fill="none"
            strokeLinecap="round"
          />

          {/* Smile */}
          <motion.path
            d="M 54 39 Q 60 41 66 39"
            stroke="#333"
            strokeWidth="1.2"
            fill="none"
            strokeLinecap="round"
            animate={{
              d: [
                'M 54 39 Q 60 41 66 39',
                'M 54 39 Q 60 42 66 39',
                'M 54 39 Q 60 41 66 39',
              ],
            }}
            transition={{
              duration: 4,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />

          {/* Body - torso with breathing animation */}
          <motion.ellipse
            cx="60"
            cy="70"
            rx="20"
            ry="25"
            fill="#6B8EFF"
            stroke="#333"
            strokeWidth="1.5"
            animate={{
              ry: [25, 27, 25],
              y: [0, -1, 0],
            }}
            transition={{
              duration: 4,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />

          {/* Legs in lotus position */}
          <motion.path
            d="M 45 90 Q 35 95 30 100 Q 28 103 32 105 L 50 105"
            fill="#4A5F9F"
            stroke="#333"
            strokeWidth="1.5"
            animate={{
              y: [0, 0.5, 0],
            }}
            transition={{
              duration: 4,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />
          <motion.path
            d="M 75 90 Q 85 95 90 100 Q 92 103 88 105 L 70 105"
            fill="#4A5F9F"
            stroke="#333"
            strokeWidth="1.5"
            animate={{
              y: [0, 0.5, 0],
            }}
            transition={{
              duration: 4,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />

          {/* Arms in meditation pose */}
          {/* Left arm */}
          <motion.path
            d="M 42 60 Q 30 70 35 85 L 38 87"
            stroke="#FFD7BA"
            strokeWidth="6"
            fill="none"
            strokeLinecap="round"
            animate={{
              d: [
                'M 42 60 Q 30 70 35 85 L 38 87',
                'M 42 60 Q 30 71 35 86 L 38 88',
                'M 42 60 Q 30 70 35 85 L 38 87',
              ],
            }}
            transition={{
              duration: 4,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />

          {/* Right arm */}
          <motion.path
            d="M 78 60 Q 90 70 85 85 L 82 87"
            stroke="#FFD7BA"
            strokeWidth="6"
            fill="none"
            strokeLinecap="round"
            animate={{
              d: [
                'M 78 60 Q 90 70 85 85 L 82 87',
                'M 78 60 Q 90 71 85 86 L 82 88',
                'M 78 60 Q 90 70 85 85 L 82 87',
              ],
            }}
            transition={{
              duration: 4,
              repeat: Infinity,
              ease: 'easeInOut',
            }}
          />

          {/* Hands in meditation mudra */}
          <motion.circle
            cx="37"
            cy="88"
            r="4"
            fill="#FFD7BA"
            stroke="#333"
            strokeWidth="1"
          />
          <motion.circle
            cx="83"
            cy="88"
            r="4"
            fill="#FFD7BA"
            stroke="#333"
            strokeWidth="1"
          />
        </g>

        {/* Floating meditation symbols */}
        <motion.g
          animate={{
            y: [0, -15, 0],
            opacity: [0.5, 1, 0.5],
          }}
          transition={{
            duration: 3,
            repeat: Infinity,
            ease: 'easeInOut',
            delay: 0,
          }}
        >
          <text x="20" y="30" fontSize="16" fill="#9370DB">
            ✨
          </text>
        </motion.g>

        <motion.g
          animate={{
            y: [0, -15, 0],
            opacity: [0.5, 1, 0.5],
          }}
          transition={{
            duration: 3,
            repeat: Infinity,
            ease: 'easeInOut',
            delay: 1,
          }}
        >
          <text x="90" y="40" fontSize="16" fill="#9370DB">
            ✨
          </text>
        </motion.g>

        <motion.g
          animate={{
            y: [0, -15, 0],
            opacity: [0.5, 1, 0.5],
          }}
          transition={{
            duration: 3,
            repeat: Infinity,
            ease: 'easeInOut',
            delay: 2,
          }}
        >
          <text x="15" y="120" fontSize="16" fill="#9370DB">
            🌸
          </text>
        </motion.g>

        <motion.g
          animate={{
            y: [0, -15, 0],
            opacity: [0.5, 1, 0.5],
          }}
          transition={{
            duration: 3,
            repeat: Infinity,
            ease: 'easeInOut',
            delay: 1.5,
          }}
        >
          <text x="95" y="130" fontSize="16" fill="#9370DB">
            🌸
          </text>
        </motion.g>

        {/* Gradient definitions */}
        <defs>
          <radialGradient id="auraGradient">
            <stop offset="0%" stopColor="#B8A4FF" stopOpacity="0.4" />
            <stop offset="50%" stopColor="#9370DB" stopOpacity="0.2" />
            <stop offset="100%" stopColor="#E6E6FA" stopOpacity="0" />
          </radialGradient>
        </defs>
      </svg>

      {/* Breathing guide text */}
      <motion.div
        className="breath-guide"
        animate={{
          opacity: [0.6, 1, 0.6],
        }}
        transition={{
          duration: 4,
          repeat: Infinity,
          ease: 'easeInOut',
        }}
      >
        <motion.span
          animate={{
            opacity: [1, 0, 0, 0, 1],
          }}
          transition={{
            duration: 8,
            repeat: Infinity,
            times: [0, 0.25, 0.5, 0.75, 1],
          }}
        >
          Breathe in...
        </motion.span>
        <motion.span
          animate={{
            opacity: [0, 0, 1, 0, 0],
          }}
          transition={{
            duration: 8,
            repeat: Infinity,
            times: [0, 0.25, 0.5, 0.75, 1],
          }}
        >
          Breathe out...
        </motion.span>
      </motion.div>
    </div>
  );
};

export default MeditationAnimation;
