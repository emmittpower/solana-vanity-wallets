# ☀️ Solana Vanity Wallets
Multithreaded generation of Solana wallets beginning with specified prefixes. Generated keypairs are saved to `keys.txt`. 

*Adjust thread count in the code for optimal performance [(src/main.rs:53)](https://github.com/emmittpower/solana-vanity-wallets/blob/bcd6010e0fe027f37a89d3e2380d881cddc4b659/src/main.rs#L53)*

## Setup
1. **Install** [Rust](https://www.rust-lang.org/tools/install)
   
3. **Clone/download** the repository:
  ```bash
  git clone https://github.com/emmittpower/solana-vanity-wallets.git
  ```
4. **Build** the project:
  ```bash
  cargo build --release
  ```
5. **Run** the program:
  ```bash
  cargo run --release
  ```

## Usage
![rdme1](/readme/1.png)

After running the program, enter as many prefixes as you want.

*Keep in mind that Solana public keys (base58) won't include:  `I, l, O, 0`*.


![rdme2](/readme/2.png)

Specify whether or not the prefixes should be case-sensitive.

![rdme3](/readme/3.png)

You'll be updated every millionth generation, and every time a prefix is detected. 

The program will run until all the specified prefixes are generated.

Keypairs are saved to `keys.txt` after being generated.

## Prefix Rarity
I generate keypairs at 18mil/min (~1.1bil/h) using a Ryzen 5600X with 6 threads running. 

The following table shows the chance of generating a specific prefix based on the amount of characters & case sensitivity.
| Prefix Length | Case-Sensitive | Case-Insensitive |
|---------------|-------------------------|---------------------------|
| 1             | 1/58                      | 1/35                        |
| 2             | 1/3,364                   | 1/1,225                     |
| 3             | 1/195,112                 | 1/42,875                    |
| 4             | 1/11,316,496              | 1/1,500,625                 |
| 5             | 1/656,356,768             | 1/52,521,875                |
| 6             | 1/38,068,692,544          | 1/1,834,265,625             |
| 7             | 1/2,207,984,167,552       | 1/64,197,281,250            |
