# Sway Lend Protocol
The Sway Lend Protocol is an Fuel Network smart contract for supplying or borrowing assets. 

## Let's get started
First, you need to clone the repository and open a terminal inside the folder with it

To start the frontend, run the commands
The frontend will run on localhost:3000
```
cd frontend
npm i
npm start
```

To run the main test case, run the following commands
```
cd contracts/market/
forc build     
cargo test --package market --test integration_tests -- local_tests::main_test::main_test --exact --nocapture
```

If you still have any questions, please message us on Discord
https://discord.gg/eC97a9U2Pe

Also subscribe to our Twitter to keep up with the latest news
https://twitter.com/swaygangsters
