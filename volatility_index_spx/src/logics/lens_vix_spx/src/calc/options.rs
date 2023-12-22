#[derive(Copy, Clone, Debug)]
pub struct Option {
    pub strike_price: f64,
    pub bid: f64,
    pub ask: f64,
    pub is_call: bool,
}
pub fn select_target_calls(k_0: f64, options: Vec<Option>) -> Vec<Option> {
    // Check call option & ATM, OTM
    let mut targets = options.iter().filter(|op| op.is_call && op.strike_price >= k_0).cloned().collect::<Vec<Option>>();
    // Sort by strike price asc to process from ATM
    targets.sort_by(|a, b| a.strike_price.partial_cmp(&b.strike_price).unwrap());

    non_zero_bid_rule(&mut targets)
}
pub fn select_target_puts(k_0: f64, options: Vec<Option>) -> Vec<Option> {
    // Check put option & ATM, OTM
    let mut targets = options.iter().filter(|op| !op.is_call && op.strike_price <= k_0).cloned().collect::<Vec<Option>>();
    // Sort by strike price desc to process from ATM
    targets.sort_by(|a, b| b.strike_price.partial_cmp(&a.strike_price).unwrap());

    non_zero_bid_rule(&mut targets)
}
fn non_zero_bid_rule(options: &mut Vec<Option>) -> Vec<Option> {
    // Non-Zero bid rule
    let mut result = vec![];
    let mut is_last_bid_zero = false;
    for target in options {
        if target.bid > 0.0 {
            result.push(*target);
            is_last_bid_zero = false;
        } else {
            if is_last_bid_zero {
                break;
            } else {
                is_last_bid_zero = true;
            }
        }
    }
    result
}
