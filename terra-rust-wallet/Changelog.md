# Changes
## 1.1
### 1.1.4 - 17-Feb-21
* Secp256k parameter passing
* switch from anyhow::Result to TerraRustWalletError
### 1.1.2
* error returns
### 1.1.1
* terra-rust-api incompatible changes
## 1.0
### 1.0.2 - 1-Oct-2021
* convenience function - wallet.get_account( ... ) gets the account associated with the stored key 
## 0.1
### 0.1.5 - devel
### 0.1.4 - 24-Aug-2021
* Upgrade to terra-rust-api 0.3
### 0.1.3 - 29-July-2021
* Fix compilations on linux around KeyringError - PR#2/3 by [@snoberg](https://github.com/snoyberg)
* new wallet returns error for keys list
* switch to anyhow/thiserror