# StellarStream Event Watcher Service

Production-grade blockchain event monitoring service for the StellarStream protocol on Stellar/Soroban.

## Features

- **Real-time Event Monitoring**: Continuously polls Stellar RPC for contract events
- **Automatic Retry Logic**: Exponential backoff with configurable retry limits
- **Graceful Shutdown**: Handles SIGINT/SIGTERM for clean process termination
- **Type-Safe**: Full TypeScript implementation with strict type checking
- **Structured Logging**: JSON-formatted logs with configurable levels
- **Rate Limit Protection**: Configurable polling intervals to avoid RPC throttling
- **Event Parsing**: Automatic XDR decoding and ScVal parsing
- **Cursor Management**: Tracks last processed ledger to avoid duplicate events

## Architecture

```
src/
├── index.ts           # Entry point and process management
├── config.ts          # Environment configuration loader
├── event-watcher.ts   # Core polling and event processing logic
├── event-parser.ts    # XDR/ScVal parsing utilities
├── logger.ts          # Structured logging
└── types.ts           # TypeScript type definitions
```

## Installation

```bash
cd backend
npm install
```

## Configuration

Copy `.env.example` to `.env` and configure:

```bash
cp .env.example .env
```

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `STELLAR_RPC_URL` | Soroban RPC endpoint | - | ✅ |
| `STELLAR_NETWORK_PASSPHRASE` | Network passphrase | Test SDF Network ; September 2015 | ❌ |
| `CONTRACT_ID` | StellarStream contract ID | - | ✅ |
| `POLL_INTERVAL_MS` | Polling interval in milliseconds | 5000 | ❌ |
| `MAX_RETRIES` | Max consecutive errors before shutdown | 3 | ❌ |
| `RETRY_DELAY_MS` | Base retry delay (exponential backoff) | 2000 | ❌ |
| `LOG_LEVEL` | Logging level (debug/info/warn/error) | info | ❌ |

### Network Configuration

**Testnet:**
```env
STELLAR_RPC_URL=https://soroban-testnet.stellar.org
STELLAR_NETWORK_PASSPHRASE=Test SDF Network ; September 2015
```

**Mainnet:**
```env
STELLAR_RPC_URL=https://soroban-mainnet.stellar.org
STELLAR_NETWORK_PASSPHRASE=Public Global Stellar Network ; September 2015
```

## Usage

### Development Mode (with hot reload)

```bash
npm run dev
```

### Production Build

```bash
npm run build
npm start
```

### Type Checking

```bash
npm run type-check
```

## Event Types

The service automatically detects and logs these StellarStream events:

- `stream_created` - New payment stream initialized
- `stream_withdrawn` - Receiver withdrew funds
- `stream_cancelled` - Stream terminated early
- `receiver_transferred` - Stream ownership transferred
- `fee_updated` - Protocol fee changed
- `paused` / `unpaused` - Contract pause state changed

## Output Format

Events are logged in structured JSON format:

```json
{
  "timestamp": "2024-02-24T10:30:45.123Z",
  "level": "INFO",
  "message": "EVENT: stream_created",
  "data": {
    "id": "0001234567890-0000000001",
    "type": "contract",
    "ledger": 1234567,
    "ledgerClosedAt": "2024-02-24T10:30:40.000Z",
    "contractId": "CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "txHash": "abc123...",
    "topics": ["AAAADwAAAA1zdHJlYW1fY3JlYXRlZA=="],
    "value": {
      "stream_id": "42",
      "sender": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
      "receiver": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
      "amount": "1000000000"
    },
    "inSuccessfulContractCall": true
  }
}
```

## Error Handling

The service implements multiple layers of error handling:

1. **Retry Logic**: Automatic retry with exponential backoff
2. **Circuit Breaker**: Stops after `MAX_RETRIES` consecutive failures
3. **Graceful Shutdown**: Cleans up resources on SIGINT/SIGTERM
4. **Error Logging**: All errors logged with context and stack traces

## Performance Considerations

- **Polling Interval**: Default 5 seconds balances freshness vs. RPC load
- **Batch Size**: Processes up to 100 events per poll
- **Memory**: Stateless design with minimal memory footprint
- **CPU**: Efficient XDR parsing with minimal overhead

## Monitoring

Monitor service health by checking:

- Log output for `ERROR` level messages
- `lastProcessedLedger` in state logs
- `errorCount` metric in logs
- Process uptime and restart frequency

## Extending the Service

### Adding Custom Event Handlers

Edit `event-watcher.ts` and extend the `handleEventByType` method:

```typescript
private async handleEventByType(
  eventType: string,
  event: ParsedContractEvent
): Promise<void> {
  switch (eventType) {
    case "your_custom_event":
      // Add your logic here
      await this.saveToDatabase(event);
      await this.sendNotification(event);
      break;
  }
}
```

### Adding Database Persistence

1. Install database client (e.g., `pg` for PostgreSQL)
2. Create a `database.ts` module
3. Call database methods from event handlers

### Adding API Endpoints

1. Install Express: `npm install express @types/express`
2. Create `api.ts` with REST endpoints
3. Expose watcher state via `/health` and `/metrics` endpoints

## Troubleshooting

### "Missing required environment variables"
- Ensure `.env` file exists and contains `STELLAR_RPC_URL` and `CONTRACT_ID`

### "Max retries exceeded"
- Check RPC endpoint is accessible
- Verify network connectivity
- Check RPC rate limits

### "CONTRACT_ID format looks unusual"
- Ensure contract ID starts with 'C' and is 56 characters
- Verify contract is deployed on the configured network

### No events appearing
- Verify contract ID is correct
- Check contract has been interacted with on-chain
- Ensure you're on the correct network (testnet vs mainnet)

## License

MIT
