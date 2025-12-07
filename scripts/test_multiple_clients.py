import asyncio
import json
import sys

import websockets


async def client_handler(_client_id: int, name: str):
    """Handle a single WebSocket client connection."""
    uri = "ws://localhost:13376"

    async with websockets.connect(uri) as websocket:
        print(f"[{name}] Connected to Proxima WebSocket server")

        # Send ready message to trigger auto-execute
        ready_msg = {"type": "ready"}
        await websocket.send(json.dumps(ready_msg))
        print(f"[{name}] Sent ready: {ready_msg}")

        # Register with a username
        register_msg = {"type": "register", "username": name}
        await websocket.send(json.dumps(register_msg))
        print(f"[{name}] Sent registration: {register_msg}")

        # Listen for messages
        print(f"[{name}] Waiting for script execution commands...\n")

        async for message in websocket:
            data = json.loads(message)
            print(f"[{name}] Received: {data}")

            if data.get("type") == "ping":
                print(f"[{name}] Received ping, sending pong")
                pong_msg = {"type": "pong"}
                await websocket.send(json.dumps(pong_msg))
            elif data.get("type") == "execute":
                script = data.get("script")
                print(f"\n[{name}] --- EXECUTE SCRIPT ---")
                print(script)
                print(f"[{name}] --- END SCRIPT ---\n")


async def main(num_clients: int):
    """Start multiple WebSocket clients."""
    # Create tasks for all clients
    tasks = []
    for i in range(1, num_clients + 1):
        name = f"Player{i}"
        task = asyncio.create_task(client_handler(i, name))
        tasks.append(task)

    # Wait for all clients to finish (they run indefinitely until interrupted)
    await asyncio.gather(*tasks)


if __name__ == "__main__":
    # Get number of clients from command line argument, default to 2
    num_clients = int(sys.argv[1]) if len(sys.argv) > 1 else 2

    if num_clients < 1:
        print("Error: Number of clients must be at least 1")
        sys.exit(1)

    print(f"Starting {num_clients} client(s)...")
    print("(Press Ctrl+C to stop)\n")

    try:
        asyncio.run(main(num_clients))
    except KeyboardInterrupt:
        print("\nClients stopped")
    except Exception as e:
        print(f"Error: {e}")
