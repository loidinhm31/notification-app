# Cross-Platform Desktop Notification App with SSE

A desktop notification application built with Rust (Tauri) and React that receives real-time Server-Sent Events (SSE) - Axum to display popup notifications near the system tray.

## Features

- **Server-Sent Events (SSE)**: Real-time notifications from server to desktop app
- **System Tray Integration**: App runs in system tray with custom menu
- **Notification Popups**: Beautiful animated popups near the tray bar
- **Animation Support**: Click notifications to view fullscreen animations
- **Fallback Polling**: Automatic fallback to HTTP polling when server is down
- **Health Checks**: Periodic server health monitoring
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Project Structure

```
sse-app/
├── notification-server/    # Rust Axum SSE server
│   ├── src/
│   │   ├── main.rs        # Server entry point
│   │   ├── models.rs      # NotificationEvent model
│   │   └── services.rs    # EventBroadcaster service
│   └── Cargo.toml
├── notification-app/       # Tauri + React desktop app
│   ├── src/               # React frontend
│   │   ├── components/    # React components
│   │   ├── hooks/         # Custom hooks
│   │   ├── types/         # TypeScript types
│   │   └── styles/        # CSS styles
│   ├── src-tauri/         # Rust backend
│   │   └── src/
│   │       ├── main.rs          # Tauri entry point
│   │       ├── sse_client.rs    # SSE connection handler
│   │       ├── commands.rs      # Tauri commands
│   │       └── window_manager.rs # Window management
│   └── package.json
└── README.md
```

## Prerequisites

### For Server
- Rust 1.70+ ([install from rustup.rs](https://rustup.rs))

### For Desktop App
- Node.js 18+ and pnpm
- Rust 1.70+
- System dependencies (Linux only):
  ```bash
  # Arch/Manjaro
  sudo pacman -S webkit2gtk libsoup gtk3 libayatana-appindicator

  # Ubuntu/Debian
  sudo apt install libwebkit2gtk-4.0-dev libsoup2.4-dev libgtk-3-dev libayatana-appindicator3-dev

  # Fedora
  sudo dnf install webkit2gtk3-devel libsoup-devel gtk3-devel libappindicator-gtk3-devel
  ```

## Installation & Setup

### 1. Clone and Navigate
```bash
cd /path/to/notification-app
```

### 2. Build and Run Server
```bash
cd notification-server
cargo build --release
cargo run --release
```

The server will start on `http://localhost:8080`

### 3. Build and Run Desktop App

#### Install Dependencies
```bash
cd notification-app
pnpm install
```

#### Development Mode
```bash
pnpm tauri:dev
```

#### Production Build
```bash
pnpm tauri:build
```

The compiled application will be in `notification-app/src-tauri/target/release/bundle/`

## API Endpoints

### Server Endpoints

- **GET /health** - Health check endpoint
  ```bash
  curl http://localhost:8080/health
  ```

- **GET /api/events/stream** - SSE endpoint for real-time events
  ```bash
  curl -N http://localhost:8080/api/events/stream
  ```

- **POST /api/events** - Create and broadcast a new event
  ```bash
  curl -X POST http://localhost:8080/api/events \
    -H "Content-Type: application/json" \
    -d '{
      "event_type": "reminder",
      "title": "Meeting Reminder",
      "message": "Team standup in 5 minutes!"
    }'
  ```

- **GET /api/events/poll** - Polling endpoint (fallback)
  ```bash
  curl http://localhost:8080/api/events/poll
  ```

## Testing the Application

### Test Server Endpoints

1. **Health Check**:
   ```bash
   curl http://localhost:8080/health
   # Expected: OK
   ```

2. **Subscribe to SSE Stream** (in one terminal):
   ```bash
   curl -N http://localhost:8080/api/events/stream
   ```

3. **Send Test Notification** (in another terminal):
   ```bash
   curl -X POST http://localhost:8080/api/events \
     -H "Content-Type: application/json" \
     -d '{
       "event_type": "test",
       "title": "Hello!",
       "message": "This is a test notification"
     }'
   ```

You should see the event appear in the SSE stream immediately.

### Test Desktop App

1. **Run the server** (as shown above)
2. **Run the desktop app** in development mode:
   ```bash
   cd notification-app
   pnpm tauri:dev
   ```
3. The app will:
   - Start in the system tray
   - Automatically connect to the server via SSE
   - Display notifications when events are received
   - Show periodic test notifications every 30 seconds

## Architecture

### Server (Rust + Axum)
- **Axum**: Modern async web framework
- **Tokio**: Async runtime
- **SSE**: Server-Sent Events for real-time push
- **Broadcast Channel**: Distributes events to multiple SSE clients
- **CORS**: Enabled for cross-origin requests

### Desktop App (Tauri + React)
- **Tauri**: Lightweight cross-platform desktop framework
- **React**: Frontend UI framework
- **Framer Motion**: Smooth animations
- **eventsource-stream**: SSE client implementation
- **System Tray**: Native system tray integration

### Communication Flow

```
Server                Desktop App  │                        │
  │  ◄──── SSE Connect ────┤
  │                        │
  │  ──── Keep-alive ────► │ (every 15s)
  │                        │
  │  ──── Event ─────────► │ → Show Notification
  │                        │
  │  ◄──── Health Check ───┤ (every 60s)
  │                        │
  │  [Connection Lost]     │
  │                        │
  │  ◄──── HTTP Poll ──────┤ (every 30s fallback)
  │                        │
  │  [Server Back Online]  │
  │                        │
  │  ◄──── SSE Reconnect ──┤
```

## Configuration

### Server Configuration
Edit `notification-server/src/main.rs` to configure:
- Port (default: 8080)
- SSE keep-alive interval (default: 15s)
- Periodic test notification interval (default: 30s)

### Desktop App Configuration
Edit `notification-app/src/hooks/useSSE.ts` to configure:
- Server URL (default: http://localhost:8080)
- Polling interval (default: 30s)
- Health check interval (default: 60s)

Edit `notification-app/src/components/NotificationPopup.tsx` to configure:
- Auto-hide duration (default: 5s)
- Notification position and styling

## Current Build Status

✅ **Server**: Built and tested successfully
- All endpoints working
- SSE streaming operational
- Event broadcasting functional

⚠️ **Desktop App**: Requires system dependencies
- React frontend builds successfully
- Tauri requires system packages: `webkit2gtk`, `libsoup`
- Installation command provided in Prerequisites section

## Troubleshooting

### Server won't start
- Check if port 8080 is already in use: `lsof -i :8080`
- Change the port in `notification-server/src/main.rs` if needed

### Desktop app build fails
- Ensure all system dependencies are installed (see Prerequisites)
- Try `cargo clean` in `notification-app/src-tauri/` and rebuild

### SSE connection fails
- Verify server is running: `curl http://localhost:8080/health`
- Check firewall settings
- Ensure correct server URL in `notification-app/src/hooks/useSSE.ts`

### Notifications not appearing
- Check system notification permissions
- Verify SSE connection in status bar (bottom-right of app)
- Check browser console for errors in dev mode

## Development

### Adding New Event Types
1. Add to `notification-server/src/models.rs`
2. Update `notification-app/src/types/index.ts`
3. Handle in `notification-app/src/components/NotificationPopup.tsx`

### Customizing Notification UI
Edit `notification-app/src/styles/App.css` and `notification-app/src/components/NotificationPopup.tsx`

### Adding New Server Endpoints
Add routes in `notification-server/src/main.rs` and implement handlers

## Production Deployment

### Server
1. Build release binary:
   ```bash
   cd notification-server
   cargo build --release
   ```
2. Binary location: `notification-server/target/release/notification-server`
3. Deploy with systemd, Docker, or your preferred method

### Desktop App
1. Build for your platform:
   ```bash
   cd notification-app
   pnpm tauri:build
   ```
2. Installers in: `src-tauri/target/release/bundle/`
3. Distribute to users

## License

This project is provided as-is for educational and development purposes.

## Contributing

Contributions welcome! Please:
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Submit a pull request

## Support

For issues or questions, please open an issue in the repository.
