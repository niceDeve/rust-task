use std::collections::HashMap;
#[cfg(test)]
mod test;
fn main() {}

// A user can submit a `MultiSend` transaction (similar to bank.MultiSend in cosmos sdk) to transfer multiple
// coins (denoms) from multiple input addresses to multiple output addresses. A denom is the name or symbol
// for a coin type, e.g USDT and USDC can be considered different denoms; in cosmos ecosystem they are called
// denoms, in ethereum world they are called symbols.
// The sum of input coins and output coins must match for every transaction.
struct MultiSend {
    // inputs contain the list of accounts that want to send coins from, and how many coins from each account we want to send.
    inputs: Vec<Balance>,
    // outputs contains the list of accounts that we want to deposit coins into, and how many coins to deposit into
    // each account
    outputs: Vec<Balance>,
}

#[derive(Debug, Clone)]
pub struct Coin {
    pub denom: String,
    pub amount: i128,
}

#[derive(Debug, Clone)]
struct Balance {
    address: String,
    coins: Vec<Coin>,
}

// A Denom has a definition (`CoinDefinition`) which contains different attributes related to the denom:
struct DenomDefinition {
    // the unique identifier for the token (e.g `core`, `eth`, `usdt`, etc.)
    denom: String,
    // The address that created the token
    issuer: String,
    // burn_rate is a number between 0 and 1. If it is above zero, in every transfer,
    // some additional tokens will be burnt on top of the transferred value, from the senders address.
    // The tokens to be burnt are calculated by multiplying the TransferAmount by burn rate, and
    // rounding it up to an integer value. For example if an account sends 100 token and burn_rate is
    // 0.2, then 120 (100 + 100 * 0.2) will be deducted from sender account and 100 will be deposited to the recipient
    // account (i.e 20 tokens will be burnt)
    burn_rate: f64,
    // commission_rate is exactly same as the burn_rate, but the calculated value will be transferred to the
    // issuer's account address instead of being burnt.
    commission_rate: f64,
}

// Implement `calculate_balance_changes` with the following requirements.
// - Output of the function is the balance changes that must be applied to different accounts
//   (negative means deduction, positive means addition), or an error. the error indicates that the transaction must be rejected.
// - If sum of inputs and outputs in multi_send_tx does not match the tx must be rejected(i.e return error).
// - Apply burn_rate and commission_rate as described by their definition.
// - If the sender does not have enough balances (in the original_balances) to cover the input amount on top of burn_rate and
// commission_rate, the transaction must be rejected.
// - burn_rate and commission_rate does not apply to the issuer. So to calculate the correct values you must do this for every denom:
//      - sum all the inputs coming from accounts that are not an issuer (let's call it non_issuer_input_sum)
//      - sum all the outputs going to accounts that are not an issuer (let's call it non_issuer_output_sum)
//      - total burn amount is total_burn = min(non_issuer_input_sum, non_issuer_output_sum)
//      - total_burn is distributed between all input accounts as: account_share = roundup(total_burn * input_from_account / non_issuer_input_sum)
//      - total_burn_amounts = sum (account_shares) // notice that in previous step we rounded up, so we need to recalculate the total again.
//      - commission_rate is exactly the same, but we send the calculate value to issuer, and not burn.
//      - Example:
//          burn_rate: 10%
//
//          inputs:
//          60, 90
//          25 <-- issuer
//
//          outputs:
//          50
//          100 <-- issuer
//          25
//          In this case burn amount is: min(non_issuer_inputs, non_issuer_outputs) = min(75+75, 50+25) = 75
//          Expected burn: 75 * 10% = 7.5
//          And now we divide it proportionally between all input sender: first_sender_share  = 7.5 * 60 / 150  = 3
//                                                                        second_sender_share = 7.5 * 90 / 150  = 4.5
// - In README.md we have provided more examples to help you better understand the requirements.
// - Write different unit tests to cover all the edge cases, we would like to see how you structure your tests.
//   There are examples in README.md, you can convert them into tests, but you should add more cases.
fn calculate_balance_changes(
    original_balances: Vec<Balance>,
    definitions: Vec<DenomDefinition>,
    multi_send_tx: MultiSend,
) -> Result<Vec<Balance>, String> {
    //calculate sum of inputs and outputs in mulit_send_tx match.
    let mut input_amounts: HashMap<String, i128> = HashMap::new();
    let mut output_amounts: HashMap<String, i128> = HashMap::new();

    for input in &multi_send_tx.inputs {
        for coin in &input.coins {
            let amount = input_amounts.entry(coin.denom.clone()).or_insert(0);
            *amount += coin.amount;
        }
    }

    for output in &multi_send_tx.outputs {
        for coin in &output.coins {
            let amount = output_amounts.entry(coin.denom.clone()).or_insert(0);
            *amount += coin.amount;
        }
    }

    //check that the input and output amounts match for each denom
    for (denom, input_amount) in input_amounts.iter() {
        match output_amounts.get(denom) {
            Some(output_amount) => {
                if input_amount != output_amount {
                    return Err(format!("notice that input and output does not match"));
                }
            }
            None => {
                return Err(format!("notice that input and output does not match"));
            }
        };
    }

    for (denom, output_amount) in output_amounts.iter() {
        match input_amounts.get(denom) {
            Some(input_amount) => {
                if input_amount != output_amount {
                    return Err(format!("notice that input and output does not match"));
                }
            }
            None => {
                return Err(format!("notice that input and output does not match"));
            }
        };
    }

    //calculate the sum of input and output amounts for non-issuer accounts
    let mut non_issuer_input_amounts: HashMap<String, i128> = HashMap::new();
    let mut non_issuer_output_amounts: HashMap<String, i128> = HashMap::new();
    let mut issuers: HashMap<String, String> = HashMap::new();
    let mut burn_rates: HashMap<String, f64> = HashMap::new();
    let mut commission_rates: HashMap<String, f64> = HashMap::new();

    for definition in &definitions {
        issuers.insert(definition.denom.clone(), definition.issuer.clone());
        burn_rates.insert(definition.denom.clone(), definition.burn_rate.clone());
        commission_rates.insert(definition.denom.clone(), definition.commission_rate.clone());
    }

    for input in &multi_send_tx.inputs {
        for coin in &input.coins {
            if input.address == *(issuers.get(&coin.denom).unwrap()) {
                continue;
            }
            let amount = non_issuer_input_amounts
                .entry(coin.denom.clone())
                .or_insert(0);
            *amount += coin.amount;
        }
    }

    for output in &multi_send_tx.outputs {
        for coin in &output.coins {
            if output.address == *(issuers.get(&coin.denom).unwrap()) {
                continue;
            }
            let amount = non_issuer_output_amounts
                .entry(coin.denom.clone())
                .or_insert(0);
            *amount += coin.amount;
        }
    }

    //calculate min of non-issuer input amounts and non-issuer output amounts for each denom
    let mut min_amounts: HashMap<String, i128> = HashMap::new();
    for definition in &definitions {
        let denom = &definition.denom;
        let min = non_issuer_input_amounts
            .get(denom)
            .unwrap_or(&0)
            .min(non_issuer_output_amounts.get(denom).unwrap_or(&0));
        min_amounts.insert(denom.clone(), min.clone());
    }

    //calculate burn and commission amounts for each denom
    // let mut burn_amounts: HashMap<String, i128> = HashMap::new();
    let mut commission_amounts: HashMap<String, i128> = HashMap::new();
    let mut blance_changes: HashMap<String, HashMap<String, i128>> = HashMap::new();

    for input in &multi_send_tx.inputs {
        let mut coins: HashMap<String, i128> = HashMap::new();
        let balance = original_balances
            .iter()
            .find(|bal| bal.address == input.address);
        if balance.is_none() {
            return Err(format!(
                "No original balance specified for {}",
                input.address
            ));
        }
        let balance_coins = &balance.unwrap().coins;
        for coin in &input.coins {
            let denom = &coin.denom;
            let mut total_amount: i128 = coin.amount;
            if input.address != *issuers.get(denom).unwrap() {
                let min_amount = *min_amounts.get(denom).unwrap();
                let burn_rate = *burn_rates.get(denom).unwrap();
                let non_issuer_input_amount = *non_issuer_input_amounts.get(denom).unwrap();
                let burn_amount = (min_amount as f64 * burn_rate * coin.amount as f64
                    / non_issuer_input_amount as f64)
                    .ceil() as i128;
                let total_commission_amount = commission_amounts.entry(denom.clone()).or_insert(0);
                let commission_rate = *commission_rates.get(denom).unwrap();
                let commission_amount = (min_amount as f64 * commission_rate * coin.amount as f64
                    / non_issuer_input_amount as f64
                    - 1e-10)
                    .ceil() as i128;

                *total_commission_amount += commission_amount;
                total_amount += burn_amount + commission_amount;
            }
            let balance_coin = balance_coins.iter().find(|coin| coin.denom == *denom);
            if balance_coin.is_none() {
                return Err(format!(
                    "notice that {} does not have enough balance for {}",
                    input.address, denom
                ));
            }
            if balance_coin.unwrap().amount < total_amount {
                return Err(format!(
                    "notice that {} does not have enough balance for {}",
                    input.address, denom,
                ));
            }
            coins.insert(denom.clone(), -total_amount);
        }
        blance_changes.insert(input.address.clone(), coins);
    }

    for output in &multi_send_tx.outputs {
        let address = &output.address;
        let change_coins = blance_changes
            .entry(address.clone())
            .or_insert(HashMap::new());
        for coin in &output.coins {
            let change_coin = change_coins.entry(coin.denom.clone()).or_insert(0);
            *change_coin += coin.amount;
        }
    }

    //update balance_changes for issuers.
    for (denom, amount) in &commission_amounts {
        if *amount == 0 {
            continue;
        }
        let address = issuers.get(denom).unwrap();
        let change_coins = blance_changes
            .entry(address.clone())
            .or_insert(HashMap::new());
        let change_coin = change_coins.entry(denom.clone()).or_insert(0);
        *change_coin += amount;
    }

    // calculates the balance changes that must be applied to different accounts 
    // (negative means deduction, positive means addition)
    let mut balances: Vec<Balance> = Vec::new();
    for (address, changes) in blance_changes.iter() {
        let mut coins: Vec<Coin> = Vec::new();
        for (denom, amount) in changes.iter() {
            if *amount != 0 {
                coins.push(Coin {
                    denom: denom.clone(),
                    amount: amount.clone(),
                });
            }
        }
        if coins.len() > 0 {
            balances.push(Balance {
                address: address.clone(),
                coins: coins,
            });
        }
    }

    Ok(balances)
}
