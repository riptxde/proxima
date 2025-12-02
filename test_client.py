import asyncio
import json
import sys

import websockets


async def test_client(username):
    uri = "ws://localhost:13376"

    async with websockets.connect(uri) as websocket:
        print("Connected to Proxima WebSocket server")

        # Register with a username
        register_msg = {"type": "register", "username": username}
        await websocket.send(json.dumps(register_msg))
        print(f"Sent registration: {register_msg}")

        # Listen for messages
        print("\nWaiting for script execution commands...")
        print("(Press Ctrl+C to stop)\n")

        async for message in websocket:
            data = json.loads(message)
            print(f"Received: {data}")

            if data.get("type") == "ping":
                print("Received ping, sending pong")
                pong_msg = {"type": "pong"}
                await websocket.send(json.dumps(pong_msg))
            elif data.get("type") == "execute":
                script = data.get("script")
                print("\n--- EXECUTE SCRIPT ---")
                print(script)
                print("--- END SCRIPT ---\n")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python test_client.py <username>")
        print("Example: python test_client.py Player1")
        sys.exit(1)

    username = sys.argv[1]

    try:
        asyncio.run(test_client(username))
    except KeyboardInterrupt:
        print("\nClient stopped")
    except Exception as e:
        print(f"Error: {e}")
