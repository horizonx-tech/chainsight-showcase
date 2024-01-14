use lens_cvi_scaler_accessors::*;

pub type LensValue = u128;
#[derive(Clone, Debug, Default, candid :: CandidType, serde :: Deserialize, serde :: Serialize)]
pub struct CalculateArgs {
    pub num_of_digits_to_scale: Option<u32>,
}
pub async fn calculate(targets: Vec<String>, args: CalculateArgs) -> LensValue {
    let result = get_cvi_latest_round(targets.get(0usize).unwrap().clone())
        .await.unwrap();
    let (value, _round_id, _timestamp) = result;
    
    // consider the scale of args
    if let Some(decs) = args.num_of_digits_to_scale {
        return (value * 10u32.pow(decs)) as u128;
    }
    value as u128
}
