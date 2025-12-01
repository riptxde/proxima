import asyncio
import json
import sys

import websockets


async def client_handler(client_id: int, name: str):
    """Handle a single WebSocket client connection."""
    uri = "ws://localhost:13376"

    try:
        async with websockets.connect(uri) as websocket:
            print(f"[{name}] Connected to {uri}")

            # Send initial registration with username
            await websocket.send(json.dumps({"type": "register", "username": name}))
            print(f"[{name}] Sent registration")

            # Handle incoming messages
            async for message in websocket:
                try:
                    data = json.loads(message)
                    msg_type = data.get("type")

                    if msg_type == "ping":
                        # Respond to ping with pong
                        await websocket.send(json.dumps({"type": "pong"}))
                        print(f"[{name}] Ponged")

                    elif msg_type == "execute":
                        # Received script to execute
                        script = data.get("script", "")
                        print(f"[{name}] Received script to execute:")
                        print(
                            f"[{name}] {script[:100]}{'...' if len(script) > 100 else ''}"
                        )

                    else:
                        print(f"[{name}] Unknown message type: {msg_type}")

                except json.JSONDecodeError:
                    print(f"[{name}] Received non-JSON message: {message}")

    except websockets.exceptions.ConnectionClosed:
        print(f"[{name}] Connection closed")
    except Exception as e:
        print(f"[{name}] Error: {e}")


async def main(num_clients: int):
    """Start multiple WebSocket clients."""
    print(f"Starting {num_clients} client(s)...")

    # Create tasks for all clients
    tasks = []
    for i in range(1, num_clients + 1):
        name = f"player{i}"
        task = asyncio.create_task(client_handler(i, name))
        tasks.append(task)

    # Wait for all clients to finish (they run indefinitely until interrupted)
    try:
        await asyncio.gather(*tasks)
    except KeyboardInterrupt:
        print("\nShutting down all clients...")


if __name__ == "__main__":
    # Get number of clients from command line argument, default to 2
    num_clients = int(sys.argv[1]) if len(sys.argv) > 1 else 2

    if num_clients < 1:
        print("Error: Number of clients must be at least 1")
        sys.exit(1)

    print("WebSocket Multiple Client Test")
    print(f"Connecting {num_clients} client(s) to ws://localhost:13376")
    print("Press Ctrl+C to stop\n")

    try:
        asyncio.run(main(num_clients))
    except KeyboardInterrupt:
        print("\nTest completed")
