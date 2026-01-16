# Backend 


This is created to test how webhooks and websocket are handled:

## WebSocket
I have used `logsSubscribe` method from the API to handle transaction logs where my program_id exist.

1. Problem
- The WebSocket disconnects every 10 minute with this message `Protocol(ResetWithoutClosingHandshake)`
- With my reconnection mechanism i receive this `Failed to send subscription: Trying to work with closed connection`

## Webhook
- I didn't found a way to do the same as the `WebSocket` approach
- I can just subscribe to a program and specific `TRANSACTION TYPE`, but for event-listening the approach may be tricky.