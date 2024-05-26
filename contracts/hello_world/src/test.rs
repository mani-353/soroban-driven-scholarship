#![cfg(test)]
use super::*;
    use soroban_sdk::{testutils::Address as TestAddress, Env, Symbol};

    #[test]
    fn test_create_and_apply_scholarship() {
        let env = Env::default();
        let contract_id = env.register_contract(None, ScholarshipContract);
        let donor = TestAddress::random(&env);
        let student = TestAddress::random(&env);
        let criteria = Symbol::new(&env, "criteria");
        let client = ScholarshipContractClient::new(&env, &contract_id);
        client.create_scholarship(&donor, 100, &criteria);
        client.apply_for_scholarship(&student, &donor, &criteria);
        let recipient = client.get_recipient(&donor, &criteria);
        assert_eq!(recipient, Some(student));
    }