# Proxima

![GitHub Repo stars](https://img.shields.io/github/stars/riptxde/proxima)
![GitHub License](https://img.shields.io/github/license/riptxde/proxima)
![GitHub top language](https://img.shields.io/github/languages/top/riptxde/proxima)
![GitHub Release](https://img.shields.io/github/v/release/riptxde/proxima)

<img src="src-tauri/icons/icon.png" height="120" alt="PACE Logo" align="right"/>

> A modern, feature-rich interface for Roblox script execution with advanced tools for developers and power users.

Proxima is a universal custom UI that works with almost **any script executor**. There's no need to be locked into using the UI of the executor you're using. Now, you can use the same familiar, feature-rich, and powerful UI with multiple executors.

---

<p align="center">
  <a href="https://www.youtube.com/watch?v=3ijhFtBPY8U" target="_blank">
    <img src="https://github.com/user-attachments/assets/f89dbe44-a79f-4f61-b79b-e9e343e05fdf" alt="Proxima Screenshot" width="800"/>
    <br>
    <b>Watch the Demo</b>
  </a>
</p>

## Features

- **Monaco Editor:** The same editor used in VS Code, with Lua syntax highlighting and support for other languages
- **Logs:** Track what happens in your executor and redirect console output to Proxima's console instead of the in-game console
- **Script Hub:** Built-in browser for thousands of scripts from [ScriptBlox](https://scriptblox.com)
- **Instance Explorer:** Browse the Roblox game tree, view hidden properties, search for instances, and decompile local scripts
- **Remote Spy:** Monitor all `RemoteEvent`, `RemoteFunction`, and `UnreliableRemoteEvent`, and generate calling code
- **Custom Launcher:** Easily downgrade to previous Roblox versions and enable Roblox multi-instancing from inside Proxima
- **Additional Classes:** Adds useful classes your executor doesn't have, such as `ProximaRelay` for multi-instance communciation
- **HTTP API:** Execute scripts from external editors like VS Code or Sublime

---

## Quick Start

### 1. Download and Install

1. Download the latest `proxima.zip` from [GitHub Releases](https://github.com/riptxde/proxima/releases)
2. Extract the archive to a folder
3. Run the `proxima_{version}-setup.exe` setup file
4. Proceed with the installation wizard.

> Note that the installer simply creates a portable installation of Proxima and installs missing dependencies such as [WebView2](https://developer.microsoft.com/en-us/microsoft-edge/webview2) and [vcredist](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist?view=msvc-170) if you are running an outdated version of Windows.

### 2. Connect Your Executor

1. Download `proxima_client.lua` from the [latest release](https://github.com/riptxde/proxima/releases)
2. Place it in your executor's auto-execute folder (e.g., `autoexec/`, `AutoExec/`, `AutoExecute/`)
3. Join any Roblox game and attach your real executor to Roblox
4. You'll see a connection notification in Proxima

---

## Requirements

### System Requirements

- Windows 10 (64-bit) or later
- 4 GB RAM
- 200 MB disk space

### Executor Requirements
Your Roblox executor must support the following functions:

- **`WebSocket.connect`** - Connect to WebSocket servers
- **`WebSocket.OnMessage`** - Receive messages from the server
- **`WebSocket.OnClose`** - Detect connection closures

#### Remote Spy
These functions are required to use the Remote Spy:

- **`hookmetamethod`** - Intercept and modify metatable operations
- **`hookfunction`** - Intercept and modify function calls
- **`getnamecallmethod`** - Get the namecall method to only intercept Remote calls
- **`newcclosure`** - Create a closure that can be used to intercept function calls safely

#### Subfeatures
If your executor is missing any of these functions, certain subfeatures may be limited

- **`gethiddenproperty`** - If your executor is missing this function, the Explorer will not display hidden or unscriptable properties, only ordinary properties
- **`decompile`** - If your executor is missing this function, decompilation of local scripts via the Explorer or Remote Spy will not work
- **`getcallingscript`** - If your executor is missing this function, the Explorer will not display which script made a remote call
- **`getcallbackvalue`** - If your executor is missing this function, the Remote Spy will not detect *incoming* RemoteFunction calls
- **`run_on_actor`, `getactors`, `create_comm_channel`, `get_comm_channel`** - If your executor is missing any of these functions, Remote Spy will not monitor remote calls made in actor contexts

---

## Features Guide

### Editor

Your main workspace for writing, managing, and executing scripts.

#### Multi-Tab Editor

- Work on multiple scripts simultaneously
- Each tab tracks its file independently
- Syntax highlighting and autocomplete for Lua

#### File Explorer

- Organize scripts in the `Scripts/` and `AutoExec/` folders
- Create, rename, and delete files
- `AutoExec/` folder for scripts that run automatically on client connection

#### Execute Scripts

- Click "Execute" to run the current script
- Select specific clients if you have multiple connected
- Support for executing on multiple clients at once, as well as a chosen subset of attached clients

---

### Logs

Track everything happening in Proxima.

#### Features

- Real-time log updates
- Four log levels: Info, Success, Warning, Error
- Filter by log level
- Search through logs
- See log counts by level
- Clear logs when needed

#### Log Sources

- Proxima application events
- Executor connections and disconnections
- Script execution results
- Custom logs from your Lua scripts

#### Log Functions

You can send logs from Roblox to Proxima using these functions:

```lua
printconsole("This is an info message") -- Info level (blue)
warnconsole("This is a warning")        -- Warning level (yellow)
errorconsole("This is an error")        -- Error level (red)
```

> Note that `errorconsole()` does not stop execution like `error()` does.

#### Print Redirection

Enable **Print Redirection** in Settings > Execution to automatically redirect all `print()`, `warn()`, and `error()` calls to the Proxima console instead of the in-game console. This is useful when:

- You want all output in one place (Proxima logs)
- You're testing scripts and want cleaner in-game output
- You need to capture print statements from scripts you don't control

When enabled:
- `print()` sends to Proxima as Info level
- `warn()` sends to Proxima as Warning level
- `error()` sends to Proxima as Error level *and stops execution*

---

### Script Hub

Access thousands of scripts without leaving Proxima.

#### Features

- Browse scripts from ScriptBlox
- Search by game name
- Filter by category and tags
- View script details and descriptions
- Send scripts directly to editor
- Quick execute without opening editor

#### How to Use

1. Navigate to the Script Hub page
2. Search for a game or browse categories
3. Click the "Send to Editor" (the paper airplane icon) button to open it in a new tab, or click the "Info" (the *i* icon) button to view more details about the script on ScriptBlox

---

### Instance Explorer

Explore the Roblox game tree.

#### Features

- **Real-time tree view:** See the entire game structure
- **Lazy loading:** Expand folders to load children on-demand
- **Property inspector:** View all properties of any instance
- **Property search:** Filter properties by name
- **Documentation links:** Quick access to official and community docs
- **Script decompilation:** View source code of local scripts
- **Search:** Find instances by name or class name
- **Path display:** See the full Lua path to any instance

#### How to Use

1. Navigate to the Explorer page
2. Select a connected client from the dropdown
3. Click any item to view its properties
4. Use the search feature to find specific instances

#### Property Badges

- **Deprecated:** Property is deprecated by Roblox
- **ReadOnly:** Property cannot be modified
- **Hidden:** Property is hidden in Studio
- **Unscriptable:** Property cannot normally be accessed via scripts

---

### Remote Spy

Monitor all network traffic between client and server.

#### What You Can See

- `RemoteEvent:FireServer()` and `RemoteEvent:FireClient()` calls
- `RemoteFunction:InvokeServer()` and `RemoteFunction:InvokeClient()` calls
- `UnreliableRemoteEvent:FireServer()` and `UnreliableRemoteEvent:FireClient()` calls
- All arguments passed to remotes
- Return values from RemoteFunctions
- Which script fired each remote

#### Features

- **Pause/Resume:** Pause capturing without disconnecting
- **Filters:** Filter by direction (outgoing/incoming) or remote type
- **Search:** Find specific remotes by name or path
- **Pagination:** Browse through call history (50 per page)
- **Timestamps:** See exactly when each call happened
- **Code generation:** Generate code to replicate remote calls
- **Script decompilation:** View the source of calling scripts

#### How to Use

1. Navigate to the Remote Spy page
2. Select a connected client
3. Play the game to see remote calls appear
4. Click on any call to see detailed information

#### Use Cases

- Reverse engineer game mechanics
- Find exploitable remotes
- Debug your own remote calls
- Learn how games are structured

---

### Custom Launcher

Take control of how you launch Roblox.

#### Why Use the Custom Launcher?

- Choose specific Roblox versions
- Downgrade to a previous Roblox version if your executor does not yet support the latest update
- Enable multi-instance mode (run multiple Roblox clients)
- Switch between LIVE and different channels

#### Setup

1. Navigate to the Launcher page
2. Click "Register as Launcher"
3. Configure your settings
4. Click "Play" on Roblox.com as usual or run Roblox via an Alt Manager

#### Settings

**Channel**
Leave empty for LIVE (default), or enter a custom channel name

**Version Override**
Leave empty to always use latest version, or enter a specific version hash to lock to that version

**Cooldown**
How many seconds since the last launch must pass until launching another client (default: 60) - prevents authentication errors with rapid launches

**Multi-Instance**
Enable to run multiple Roblox clients simultaneously - useful for testing with multiple accounts

#### How Launching Works

1. Click "Play" on Roblox.com
2. Proxima intercepts the launch
3. Downloads your configured version (if not cached)
4. Launches Roblox with your settings
5. Progress shown in real-time

#### Queue System

- If multiple launches happen at once, they're queued and launch one-by-one, separated by the cooldown time. This prevents authentication errors when launching multiple accounts.
- See your position in the queue

---

### Additional Classes

Proxima provides additional Lua classes that extend your executor's capabilities.

#### ProximaRelay

A communication class for broadcasting messages to all other attached Roblox clients without a separate program or file operations.

**Events**

| Event | Description |
|-------|-------------|
| `OnBroadcast:Connect(function(Content: string))` | Triggered when a message is received from another client |

**Methods**

| Method | Description |
|--------|-------------|
| `Broadcast(Content: string)` | Sends a message to all other connected clients (not including the sender) |

**Notes:**
- Broadcasts are sent to all **other clients** (not back to the sender)
- Only string content is supported

**Example - Coordinating Multiple Clients**

```lua
-- Client 1: Broadcast a message to all other clients
ProximaRelay.Broadcast("start")

-- Client 2: Listen for messages and react
ProximaRelay.OnBroadcast:Connect(function(Content)
    if Content == "start" then
        print("Do stuff...")
    end
end)
```

---

## HTTP API & IDE Integration

Proxima includes an HTTP API for executing scripts from external editors and tools.

This provides endpoints `http://localhost:13377/execute` and `http://localhost:13377/execute_file` which work identically to the endpoints from the **[Roblox Executor Proxy](https://github.com/riptxde/roblox_executor_proxy)**.

Please see the repository mentioned above for:
- Detailed HTTP API documentation
- Examples of how to integrate Roblox execution into VS Code, Sublime Text, Zed, Neovim, and more
- Sample code for custom integrations

---

## Troubleshooting

### Client Won't Connect

- Make sure Proxima is running before joining Roblox
- Check if port 13376 is blocked by firewall
- Try restarting both Proxima and your executor
- Verify the client script is the latest version from releases

### Executor Not Working

- Proxima is just a UI - you still need a working executor
- The client script must be in your executor's auto-execute folder, not Proxima's
- Some executors may require specific configurations
- Check executor compatibility with your Roblox version

### Launcher Not Working

- Click "Register as Launcher" before using
- Windows may require administrator privileges
- Check that the registration status shows green checkmark
- Try unregistering and re-registering

---

## License

This project is licensed under the AGPL-3.0 License.

---

## Credits

**Author:** [riptxde](https://github.com/riptxde)

**Built With:**

- [Tauri](https://tauri.app/) - Rust desktop application framework
- [Vue 3](https://vuejs.org/) - Frontend JavaScript framework
- [Monaco Editor](https://microsoft.github.io/monaco-editor/) - Code editor (VS Code's editor)
- [ScriptBlox](https://scriptblox.com/) - Script repository API

**Special Thanks:**

- The open-source community
- Script executor developers
- Script Hub API providers
