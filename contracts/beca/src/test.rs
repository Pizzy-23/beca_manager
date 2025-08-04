#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::{
    testutils::{Address as _},
    symbol_short, 
    String, 
    Env, 
};

#[test]
fn test_criar_bolsa_simples() {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, ContratoBolsaSimples);
    let client = ContratoBolsaSimplesClient::new(&env, &contract_id);
    
    let gestor_teste = Address::generate(&env);
    let nome_teste = String::from_str(&env, "Bolsa Minimalista");

    client.criar(&gestor_teste, &nome_teste);

    let bolsa_salva: BolsaSimples = env.as_contract(&contract_id, || {
        env.storage().instance().get(&symbol_short!("BOLSA")).unwrap()
    });

    assert_eq!(bolsa_salva.gestor, gestor_teste);
    assert_eq!(bolsa_salva.nome, nome_teste);
}