# rayflow
Yet another data flow node-graph based system with focus on performance

### Frontend (React + @xyflow/react-flow)

- **Canvas** using `@xyflow/react-flow`
- **Sidebar**:
  - Node list
  - Canvas list
- **Modals**:
  - Create/edit node (with `body`, inputs, outputs)
  - Edit metadata (for node instance and connections)
- **WebSocket client** to sync with backend
- **REST calls** to load existing nodes/canvases on page load

#### React frontend skeleton

- Canvas powered by `@xyflow/react-flow`
- Nodes and edges loaded via REST on page load
- WebSocket sending updates to the Rust backend
- Extensible base for modals (node/connection metadata editing), canvas list, and more

### Backend (Rust)

Using **Axum** framework with `tokio`, and `serde` for data serialization.

- **REST endpoints**:
  - `GET /nodes` → returns list of all node definitions
  - `GET /canvases` → returns list of canvas metadata
  - `GET /canvas/:id` → loads canvas state
- **WebSocket endpoint**:
  - `POST ws://.../sync` → receives updates from frontend (node/body updates, canvas state changes)
- **File storage** (JSON) for persistence:
  - `data/nodes.json`
  - `data/canvas_<id>.json`

### REST and WebSocket Endpoints (Backend)

| Route                    | Method | Purpose                          |
|-------------------------|--------|----------------------------------|
| `/nodes`                | GET    | Load all nodes                   |
| `/canvases`             | GET    | List available canvases          |
| `/canvas/:id`           | GET    | Load specific canvas             |
| `/sync`                 | WS     | WebSocket endpoint for updates   |
