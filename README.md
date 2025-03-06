# Gary: The Liquidity Provider Trading Bot

**An Experimental Stab at Turning Gary Stevenson's _The Trading Game_ into a Millionaire-Making Venture**

Inspired by [Gary Stevenson](<https://en.wikipedia.org/wiki/Gary_Stevenson_(economist)>)'s [_The Trading Game_](https://www.penguin.co.uk/books/455809/the-trading-game-by-stevenson-gary/9781802062731), "Gary" is an experimental liquidity provider (LP) trading bot I'm building in Rust. It's a wild punt at providing liquidity on decentralised exchanges (think Solana's Orca Whirlpools) to earn fees and—why not?—make me a millionaire. **_Let's be clear: this is just a bit of fun._** I'm not a trader, I don't know Gary Stevenson personally (just a fan of his book), and I've no common ground with him beyond these pages. If the grand plan flops, I'll still walk away with a solid grasp of LP trading and Rust.

## Overview

"Gary" is my attempt to automate liquidity provision. It's meant to:

- Chuck assets into a pool (like SOL/devUSDC on Devnet).
- Tweak positions when the market shifts.
- Pocket trading fees as a reward.

It's a nod to Stevenson's tales of exploiting market quirks, but I'm mashing it up with crypto and coding—purely as an experiment.

## Goals

- **Primary**: Craft a bot that rakes in enough profit to make me a millionaire (dream big, eh?).
- **Fallback**: If the cash doesn't pile up, I'll still learn the ropes of liquidity provision and sharpen my Rust skills—worth it either way.

## Features

- **Automated LP**: Adds or pulls liquidity based on simple rules I'll figure out as I go.
- **Real-Time Monitoring**: Keeps an eye on pool prices and fees via Solana.
- **Risk Management**: Sets basic guards to avoid a total wipeout.
- **Scalable**: Could handle multiple pools if I get ambitious.
- **Production-Ready**: Aims for proper logging and error handling, though it's all experimental.

## Roadmap

- [ ] Hook up to Orca Whirlpools on Devnet (starting with SOL/devUSDC).
- [ ] Code a basic LP strategy (stick liquidity in a price range).
- [ ] Sort out fee collection and reinvestment.
- [ ] Tweak it with live price feeds and clever adjustments.
- [ ] Test on Mainnet with a few quid.
- [ ] Scale up for millionaire-level profits (or at least a good story).

## Disclaimer

This is purely an experimental lark inspired by _The Trading Game_. I'm not a trader—never have been—just a curious coder who read a book. I've no personal link to Gary Stevenson, just admiration for his story. Trading's a risky business, so don't chuck in money you can't lose. I'm not doling out financial advice, just messing about with code and hoping to learn something. Proceed at your own peril!

## Contributing

Fancy helping "Gary" become a millionaire-maker? Chuck in an issue or PR—I'd welcome the input!

## Licence

MIT Licence—free to use, tweak, and share.
