use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use secp256k1::Secp256k1;
use terra_rust::Cli;
use terra_rust_api::core_types::Coin;
use terra_rust_api::{GasOptions, Message, MsgExecuteContract, Terra};
use terra_rust_wallet::Wallet;
/// VERSION number of package
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// NAME of package
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
/*
/// exec smart contracts with ease
#[derive(Parser)]
#[clap(name = "terra exec")]
#[clap( long_about = None)]
struct Cli {
    #[clap(
        name = "lcd",
        env = "TERRARUST_LCD",
        default_value = "https://lcd.terra.dev",
        short,
        long = "lcd-client-url",
        help = "https://lcd.terra.dev is main-net, https://bombay-lcd.terra.dev"
    )]
    // Terra cli Client daemon
    lcd: String,
    #[clap(
        name = "fcd",
        env = "TERRARUST_FCD",
        default_value = "https://fcd.terra.dev",
        long = "fcd-client-url",
        help = "https://fcd.terra.dev is main-net. currently only used to fetch gas prices"
    )]
    // Terra cli Client daemon
    fcd: String,
    #[clap(
        name = "chain",
        env = "TERRARUST_CHAIN",
        default_value = "columbus-5",
        short,
        long = "chain",
        help = "bombay-12 is testnet, columbus-5 is main-net"
    )]
    chain_id: String,
    // Wallet name
    #[clap(
        name = "wallet",
        env = "TERRARUST_WALLET",
        default_value = "default",
        short,
        long = "wallet",
        help = "the default wallet to look for keys in"
    )]
    wallet: String,
    #[clap(
        name = "seed",
        env = "TERRARUST_SEED_PHRASE",
        default_value = "",
        short,
        long = "seed",
        help = "the seed phrase to use with this private key"
    )]
    seed: String,
    #[clap(
        name = "fees",
        default_value = "",
        short,
        long = "fees",
        help = "the fees to use. This will override gas parameters if specified."
    )]
    fees: String,
    #[clap(
        name = "gas",
        default_value = "auto",
        long = "gas",
        help = "the gas amount to use 'auto' to estimate"
    )]
    gas: String,
    #[clap(
        name = "gas-prices",
        env = "TERRARUST_GAS_PRICES",
        default_value = "auto",
        long = "gas-prices",
        help = "the gas price to use to calculate fee. Format is NNNtoken eg. 1000uluna. note we only support a single price for now. if auto. it will use FCD"
    )]
    gas_price: String,
    #[clap(
        name = "gas-denom",
        env = "TERRARUST_GAS_DENOM",
        default_value = "ukrw",
        long = "gas-denom",
        help = "the denomination/currency to use to pay fee. Format is uXXXX."
    )]
    gas_price_denom: String,
    #[clap(
        name = "gas-adjustment",
        default_value = "1.4",
        env = "TERRARUST_GAS_ADJUSTMENT",
        long = "gas-adjustment",
        help = "the adjustment to multiply the estimate to calculate the fee"
    )]
    gas_adjustment: f64,

}
impl Cli {
    pub async fn gas_opts(&self) -> Result<GasOptions> {
        if self.gas_price == "auto" {
            //            let terra = Terra::lcd_client_no_tx(&self.lcd, &self.chain_id).await?;
            //      let fcd = terra.fcd(&self.fcd);
            let client = reqwest::Client::new();
            let gas_opts = GasOptions::create_with_fcd(
                &client,
                &self.fcd,
                &self.gas_price_denom,
                self.gas_adjustment,
            )
            .await?;
            if let Some(gas_price) = &gas_opts.gas_price {
                log::info!("Using Gas price of {}", gas_price);
            }

            Ok(gas_opts)
        } else {
            let fees = Coin::parse(&self.fees)?;
            let gas_str = &self.gas;
            let (estimate_gas, gas) = if gas_str == "auto" {
                (true, None)
            } else {
                let g = &self.gas.parse::<u64>()?;
                (false, Some(*g))
            };

            let gas_price = Coin::parse(&self.gas_price)?;
            let gas_adjustment = Some(self.gas_adjustment);

            Ok(GasOptions {
                fees,
                estimate_gas,
                gas,
                gas_price,
                gas_adjustment,
            })
        }
    }
}

 */

#[derive(Subcommand)]
pub enum Command {
    Exec(ExecCmd),
}
#[derive(Parser, Debug)]
pub struct ExecCmd {
    #[clap(
        name = "contract",
        help = "the contract",
        long = "contract",
        env = "TERRARUST_CONTRACT"
    )]
    pub contract: String,
    #[clap(
        name = "sender",
        long = "sender",
        help = "the sender account",
        env = "TERRARUST_SENDER"
    )]
    pub sender: String,
    #[clap(name = "coins", long = "coins")]
    pub coins: Option<String>,
    #[clap(name = "json")]
    pub json: String,
}

async fn run() -> Result<()> {
    let cli = Cli::<Command>::parse();

    let gas_opts: GasOptions = cli.gas_opts().await?;
    let terra = Terra::lcd_client(&cli.lcd, &cli.chain_id, &gas_opts, None);
    match cli.cmd {
        Command::Exec(cmd) => {
            let json: serde_json::Value = serde_json::from_str(&cmd.json)?;

            let secp = Secp256k1::new();
            let wallet = Wallet::create(&cli.wallet);

            let seed: Option<&str> = if cli.seed.is_empty() {
                None
            } else {
                Some(&cli.seed)
            };
            let sender = cmd.sender;

            let from_key = wallet.get_private_key(&secp, &sender, seed)?;
            let from_public_key = from_key.public_key(&secp);

            let coins = if let Some(coins) = cmd.coins {
                Coin::parse_coins(&coins)?
            } else {
                vec![]
            };

            let exec_message = MsgExecuteContract::create_from_value(
                &from_public_key.account()?,
                &cmd.contract,
                &json,
                &coins,
            )?;
            let messages: Vec<Message> = vec![exec_message];

            let resp = terra
                .submit_transaction_sync(
                    &secp,
                    &from_key,
                    messages,
                    Some(format!(
                        "PFC-{}/{}",
                        NAME.unwrap_or("TERRARUST"),
                        VERSION.unwrap_or("DEV")
                    )),
                )
                .await?;

            log::debug!("{:?}", &resp.txhash);
            if cli.chain_id.contains("bombay") {
                println!(
                    "https://finder.extraterrestrial.money/testnet/tx/{}",
                    resp.txhash
                );
            } else {
                println!(
                    "https://finder.extraterrestrial.money/mainnet/tx/{}",
                    resp.txhash
                );
            }
        }
    };

    Ok(())
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    if let Err(ref err) = run().await {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}