COMP_IDS=(
    stock_indexer_aapl_daily
    stock_indexer_aapl_hourly
    stock_indexer_amzn_daily
    stock_indexer_amzn_hourly
    stock_indexer_goog_daily
    stock_indexer_goog_hourly
    stock_indexer_meta_daily
    stock_indexer_meta_hourly
    stock_indexer_msft_daily
    stock_indexer_msft_hourly
    stock_indexer_nvda_daily
    stock_indexer_nvda_hourly
)
for comp_id in ${COMP_IDS[@]}; do
    echo "Deploying $comp_id"
    csx deploy --network ic -c $comp_id
    csx exec --network ic -c $comp_id
done

