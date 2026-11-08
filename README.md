# Tape

A day-trading game on a market that doesn't exist.

Every price is a pure function of a seed and a minute, so the same seed always replays the same day, nothing is stored, and 6,200 symbols build in about 50 ms. You get 20 sessions and $10,000. Score is profit less half your worst drawdown, per $10,000 you started with, times a multiplier set by how hard you made the run.

## The market

- 23-octave fractional Brownian motion from a 32-bit hash, per symbol
- Shared market and sector factors computed once per day, so stocks in the same sector move together
- 15 index and sector funds built as cap-weighted baskets of the names under them
- A warped regime clock, so volatility clusters instead of spreading evenly
- Seeded earnings and news jumps, with a wire that prints them as they hit
- LULD halts, spread, slippage and sell-side fees on fills

## The game

- Order ticket with bracket stops and targets, size-to-risk and buying-power presets
- Drag a working order's line on the chart to move it, click its × to cancel, shift-click the chart to rest a limit
- Level II marks your own resting orders; the blotter shows realized P&L on every closing fill
- The clock stops on its own when an order fills, news hits a holding, a holding halts or the day is down 3%, and tells you why
- A session strip in the top bar with your fills and the earnings you hold into
- Eight run modifiers, from ×0.29 to ×6.79, with a live preview of what each setup means
- Roguelike perks every few sessions, and a recap of each session's equity, fees and rent
- 51 indicators behind `i`, with warmup bars so values are right from the first visible candle. On the price: EMA, SMA, WMA, Hull, DEMA, TEMA, VWMA, KAMA, linear regression, Bollinger, Keltner, Donchian, envelopes, Ichimoku, Supertrend and Parabolic SAR. In panes below: RSI, MACD, stochastic, Stoch RSI, ADX/DMI, CCI, Williams %R, MFI, Chande momentum, ultimate oscillator, Aroon, Vortex, ROC, momentum, TRIX, TSI, PPO, awesome oscillator, DPO, Coppock, Elder ray, ATR, OBV, A/D line, Chaikin oscillator and money flow, force index, ease of movement, historical volatility, choppiness, Bollinger %B and width, and z-score
- A Monte Carlo cone (`m`) that runs 400 random price paths from the chart's recent volatility and shades the 50% and 90% range of the next 30 bars
- A run chart sampled every five minutes, raced against a ghost of your best run
- Records of your forty best runs; click one to replay its market
- A daily challenge seeded by the date, the same for everyone
- Review mode for any finished session, and `?` for every key

## Run it

Browser, no build step:

```
open src/tape.html
```

Add `?seed=1234` to the URL to start straight into that market.

Desktop app (Tauri 2):

```
npm install
npm run tauri dev
npm run tauri build
```

Self-test, 61 in-page assertions:

```
open "src/tape.html?selftest=1"
```

## Layout

- `src/tape.html` is the whole engine and UI in one file, with no runtime dependencies
- `src/fonts/` holds IBM Plex Sans Condensed and IBM Plex Mono (SIL Open Font License), so the app looks the same offline
- `src-tauri/` wraps it as a native window with a Run and View menu
- `scripts/build.mjs` splits the file into `dist/` for the Tauri bundle

## License

MIT. The bundled fonts are under the SIL Open Font License 1.1, in `src/fonts/OFL.txt`.
