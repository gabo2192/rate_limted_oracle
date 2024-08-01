## Quick Start

1. **Install Dependencies**

```sh
# Install Latest Ancho CLI 0.30.1 and Solana Version 1.17.3
https://www.anchor-lang.com/release-notes/0.30.1
avm install 0.30.1
solana-install init 1.18.17
```

```sh
#Install project dependencies using yarn
yarn install
```

2. **Build the program**

```sh
anchor build
```

3. **Run the tests**

```sh
anchor test
```

Oracle Rate Limit:

Summary:

- Create a smart contract program in Anchor Framework that comprises an oracle price update contract for an example asset.
- Create a public function that has rate limits built into it to ensure that the function can only be called 3 times per X period by any wallet, otherwise it should revert.
- After the period, the amount of available calls should go back to 3.
- Period should be a variable in the contract (we assume it can be changed).
- Keep security and gas efficiency in mind overall.

Result:

- I created a smart contract using anchor that contains a basic struct with Price and Time, and a public function that initializes this struct with the current time and price is set to 0.

- I create a function that can be called by any wallet to update the price of the asset, setting the time to the current timestamp, and the price to the input price.

- In this update function we create a PDA to keep track their last time called and the number of calls made in the last period.

- We then check if the current time is greater than the last time called + the period, if it is we reset the number of calls to 0, otherwise we check the amount of calls and if there are more than 3 we revert.

- I tested the contract using the anchor test command, I've only got to the first one.
