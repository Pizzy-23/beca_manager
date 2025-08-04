// ARQUIVO: src/lib.rs (NÃO PRECISA MUDAR NADA AQUI!)

#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, String
};

#[contracttype]
#[derive(Clone)]
pub struct BolsaSimples {
    pub gestor: Address,
    pub nome: String,
}

#[contract]
pub struct ContratoBolsaSimples;

#[contractimpl]
impl ContratoBolsaSimples {
    pub fn criar(env: Env, gestor: Address, nome: String) {
        if env.storage().instance().has(&symbol_short!("BOLSA")) {
            panic!("Este contrato já foi inicializado");
        }
        gestor.require_auth();
        let bolsa = BolsaSimples { gestor, nome };
        env.storage().instance().set(&symbol_short!("BOLSA"), &bolsa);
    }
}

mod test;