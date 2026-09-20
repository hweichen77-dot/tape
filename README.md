# Tape

A day-trading game on a market that doesn't exist.

Every price is a pure function of a seed and a minute, so the same seed always replays the same day, nothing is stored, and 6,200 symbols build in about 50 ms. You get 20 sessions and $10,000. Score is normalised by account size so a small account played well beats a big one played badly.

## What's in the market

- 23-octave fractional Brownian motion from a 32-bit hash, per symbol
- Shared market and sector factors computed once per day, so stocks in the same sector actually move together
- 15 index and sector funds built as real cap-weighted baskets of the underlying names
- A warped regime clock so volatility clusters instead of spreading evenly
- Seeded earnings and news jumps, with a news wire that reports them as they hit
- LULD halts and slippage on fills

## What's in the game

- Order ticket with bracket orders, review mode after each session
- Eight run modifiers, from 0.29x to 6.79x score multiplier
- Liquidity-ranked scanner, earnings calendar, per-session burn
- Roguelike perks between sessions
- A ghost curve of your best run drawn over the current one
- A daily challenge seeded by the date, same for everyone

## Run it

Browser, no build step:

```
open src/tape.html
```

Desktop app (Tauri 2):

```
npm install
npm run tauri dev
npm run tauri build
```

Self-test, 33 in-page assertions:

```
open "src/tape.html?selftest=1"
```

## Layout

- `src/tape.html` is the entire engine and UI, one file, no runtime dependencies
- `src-tauri/` wraps it as a native window
- `scripts/build.mjs` copies the file into `dist/` for the Tauri bundle

## License

MIT
