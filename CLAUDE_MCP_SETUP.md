# Claude MCP Setup — PostgreSQL + RedRobot Database Access

This guide connects Claude Desktop to the RedRobot PostgreSQL database so
the admin can query trade data, signal analytics, and TVL history using
natural language — no SQL knowledge required.

---

## How It Works

```
Claude Desktop (local)
    ↓  MCP protocol (stdio transport)
@modelcontextprotocol/server-postgres
    ↓  TCP connection (SSH tunnel or direct)
PostgreSQL on VPS (127.0.0.1:5432)
    ↓  Queries against
redrobot database (tables: closed_trades, aum_snapshots, equity_snapshots …)
```

Claude reads the MCP config at startup, launches the MCP server subprocess,
and can then execute SQL queries directly when answering your questions.

---

## Step 1 — Open an SSH tunnel to the VPS database

PostgreSQL listens on `127.0.0.1:5432` (loopback only — not exposed to the internet).
Use an SSH tunnel to bring it to your local machine:

```bash
# Add this to ~/.ssh/config for convenience:
Host redrobot-db
    HostName 165.232.160.43
    User root
    LocalForward 5433 127.0.0.1:5432
    ServerAliveInterval 60
    ServerAliveCountMax 3
```

Then start the tunnel:
```bash
ssh -N redrobot-db &
# Or persistently with autossh:
autossh -M 0 -f -N redrobot-db
```

The VPS database is now available at `localhost:5433` on your Mac.

---

## Step 2 — Get the database password

```bash
ssh root@165.232.160.43 "grep DATABASE_URL /etc/environment"
# Output: DATABASE_URL=postgresql://redrobot:<password>@127.0.0.1/redrobot
```

Copy the password — you'll need it in Step 3.

---

## Step 3 — Configure Claude Desktop

Open (or create) `~/Library/Application Support/Claude/claude_desktop_config.json`
and add the `mcpServers` section:

```json
{
  "mcpServers": {
    "redrobot-db": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-postgres",
        "postgresql://redrobot:<password>@localhost:5433/redrobot"
      ]
    }
  }
}
```

Replace `<password>` with the value from Step 2.

**Restart Claude Desktop** — the MCP server launches automatically.

---

## Step 4 — Verify the connection

In Claude, type:
> "List the tables in the RedRobot database"

Claude should respond with the full schema. If it can't connect, check:
- Is the SSH tunnel running? (`ssh -N redrobot-db &`)
- Is the password correct?
- Is PostgreSQL running on the VPS? (`systemctl status postgresql`)

---

## Example queries Claude can answer

Once connected, ask Claude anything about the trading data:

```
"What's the win rate for trades where the RSI signal was bullish vs bearish
 in the last 30 days?"

"Show me the top 5 most profitable symbols by total PnL all time."

"What's the average R-multiple for trades that close on a trailing stop
 vs hitting take-profit?"

"Has the total AUM been growing or declining over the last 7 days?
 Show me a summary."

"Which signals have the highest correlation with profitable trades?"

"What's our current TVL and how does it compare to last week?"

"List trades from the last 24 hours with their PnL and close reason."
```

Claude will write the SQL, execute it against the live database, and return
a plain-English answer with the data formatted as a table or chart description.

---

## Useful SQL reference (Claude uses these patterns)

```sql
-- Win rate by signal over last 30 days
SELECT
    (signal_contrib->>'rsi_bullish')::bool  AS rsi_bullish,
    count(*)                                AS trades,
    round(avg(r_multiple)::numeric, 2)      AS avg_r,
    round(100.0 * count(*) FILTER (WHERE pnl_usd > 0) / count(*), 1) AS win_pct
FROM closed_trades
WHERE closed_at > now() - INTERVAL '30 days'
  AND signal_contrib IS NOT NULL
GROUP BY 1 ORDER BY avg_r DESC;

-- TVL trend last 7 days (5-minute buckets)
SELECT
    date_trunc('hour', recorded_at)        AS hour,
    avg(total_aum)::numeric(18,2)          AS avg_aum,
    avg(pnl_pct)::numeric(6,2)            AS avg_pnl_pct
FROM aum_snapshots
WHERE recorded_at > now() - INTERVAL '7 days'
GROUP BY 1 ORDER BY 1;

-- Per-symbol performance summary
SELECT
    symbol,
    count(*)                               AS trades,
    sum(pnl_usd)::numeric(18,2)           AS total_pnl,
    avg(r_multiple)::numeric(6,2)         AS avg_r,
    round(100.0 * count(*) FILTER (WHERE pnl_usd > 0) / count(*), 1) AS win_pct
FROM closed_trades
GROUP BY symbol ORDER BY total_pnl DESC LIMIT 10;
```

---

## Ollama — On-Device Analysis (No API Costs)

Ollama runs on the VPS and can analyse trade data locally using `llama3.2`.
The bot's `db::query_ollama()` function is wired to `OLLAMA_BASE_URL=http://localhost:11434`.

To test Ollama directly:
```bash
ssh root@165.232.160.43
curl http://localhost:11434/api/generate \
  -d '{"model":"llama3.2","prompt":"What is the average win rate for a crypto trading bot?","stream":false}' \
  | jq .response
```

Future: the `daily_analyst` module will query closed trades from PostgreSQL,
build a natural-language context string, and send it to Ollama for on-device
analysis — results stored in the `ai_analyses` table and surfaced in the admin UI.

---

## Security Notes

- PostgreSQL is bound to `127.0.0.1` only — not exposed to the internet
- The SSH tunnel is encrypted end-to-end
- The MCP server runs as a local subprocess of Claude Desktop — it has no
  persistent network presence
- Never add port 5432 to UFW/iptables allow rules
- Rotate the `redrobot` database password via:
  ```bash
  ssh root@165.232.160.43
  sudo -u postgres psql -c "ALTER ROLE redrobot WITH PASSWORD 'new_password';"
  # Then update DATABASE_URL in /etc/environment and restart hedgebot
  ```
