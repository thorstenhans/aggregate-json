# Edge API Aggregation and Fan-Out

A Spin application demonstrating edge API aggregation and Backend-for-Frontend (BFF) fan-out using WebAssembly components.

The application intercepts client requests, fans out concurrent outbound HTTP calls to multiple upstream microservices using `futures::join!`, and merges the responses into a consolidated JSON payload in a single round trip.

## Building and Running

```bash
spin up --build
```

## Test the aggregated endpoint

```bash
curl -i http://localhost:3000/
```
