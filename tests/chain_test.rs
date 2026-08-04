use hisho::settings::chain::*;



#[cfg(test)]
mod tests{

    use super::*;




    #[test]
    fn test_chain_json_deserialization(){

        let parsed: AppConfig = serde_json::from_str(CHAIN_JSON).expect("REASONS");

        assert!(!parsed.chains.is_empty(), "Error parsing chain.json: {:#?}", parsed.chains.is_empty());

        let config = parsed;

        assert_eq!(config.chains[0].name, "Ethereum")
    }
}