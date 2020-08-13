import not_alone


# Server
a = not_alone.server.Server("Me")
a.wait_client("Hello from me", 2)
a.messaging()
