# Safe Multisig Migration Utility

**:warning: This repository contains unaudited and insufficiently tested code.
Ethereum and the Gnosis Safe give you complete custody of your assets, which
means you can permanently and irreversably lose them. Make sure you understand
the risks, the source code, and what this migration entails. In no event shall
the author or copyright holder be liable for any claim, damages or other
liability, whether in an action of contract, tort or otherwise, arising from,
out of or in connection with the software or the use or other dealings in the
software. You have been warned.**

This Safe Multisig migration tool is used to migrate Safes created with the
legacy iOS and Android apps for use with the new Multisig web interface.

If you would like to do this manually, I suggest following the official guide:
<https://help.gnosis-safe.io/en/articles/4100585-how-to-use-your-legacy-mobile-gnosis-safe-with-the-safe-multisig-web-interface>.

## How it Works

This application automates all of the steps from the aforementioned guide, with
one major difference: it uses the Gnosis Safe relay service to pay for the
transaction with funds on the Safe being migrated instead of requiring the
recovery EOA being funded. In a nutshell, the migration works by:
1. Deriving a private key from the recovery phrase, each Safe created with the
   legacy app is initialized with two owners derived from this seed phrase.
2. Estimate the gas cost for the transaction to add a new owner with the relay
   service, potentially allowing an ERC20 token to pay for the gas.
3. Sign the transaction with the recovery key and send it to the relay service.

## Running it

**:warning: Before running make sure you have read the disclaimer at the start
of this document and understand the risks involved.**

Executing the migration tool is straight forward. Command line documentation
is accessible from:
```
safe-migrate --help
```

To "migrate" the Safe and add a new owner such as a MetaMask account or hardware
wallet:
```
safe-migrate $SAFE_ADDRESS $NEW_OWNER
```

For example, to use the recovery phrase to add a new owner and pay with DAI on
rinkeby (with fake contract and recovery addresses):
```
$ safe-migrate 0x1111111111111111111111111111111111111111 0x2222222222222222222222222222222222222222 --network rinkeby --gas-token 0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa`
Legacy Safe recovery phrase: 
Using Safe 0x1111111111111111111111111111111111111111
Using Recovery accounts:
  - 0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
  - 0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
About to add 0x2222222222222222222222222222222222222222 as an owner (yes to continue)? yes
Are you sure, this will add a new owner to the Safe 0x1111111111111111111111111111111111111111? yes
Are you absolutely sure!? yes
  to: 0x1111111111111111111111111111111111111111
  value: 0
  data: 0x0d582f1300000000000000000000000022222222222222222222222222222222222222220000000000000000000000000000000000000000000000000000000000000001
  operation: call
  safe transaction gas: 75786
  base gas: 48668
  gas price: 16666666667
  gas token: 0x5592EC0cfb4dbc12D3aB100b257153436a1f0FEa
  refund receiver: 0x07fd2865c8DE725B4e1f4E2B72E5c654baA7c4b3
  nonce: 42
  hash: 0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
Are you still 100% sure? yes
Using signature 0xrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssssvv
Are absolutely positively undoubtedly sure? yes
Transaction successfully relayed:
https://rinkeby.etherscan.io/tx/0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
```
