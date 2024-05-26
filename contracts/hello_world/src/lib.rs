#![no_std]

use soroban_sdk::{contractimpl, contracttype, Address, Env, Symbol, Vec, IntoVal, TryFromVal, Val};

pub struct ScholarshipContract;

#[contracttype]
#[derive(Clone)]
pub struct Scholarship {
    donor: Address,
    amount: u64,
    criteria: Symbol,
    awarded: bool,
    recipient: Option<Address>,
}

// Conversion from Val to Scholarship
impl TryFromVal<Env, Val> for Scholarship {
    type Error = soroban_sdk::ConversionError;
    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let vec = Vec::<Val>::try_from_val(env, val)?;
        Ok(Scholarship {
            donor: Address::try_from_val(env, &vec.get(0)?)?,
            amount: u64::try_from_val(env, &vec.get(1)?)?,
            criteria: Symbol::try_from_val(env, &vec.get(2)?)?,
            awarded: bool::try_from_val(env, &vec.get(3)?)?,
            recipient: if vec.get(4)?.is_void() { None } else { Some(Address::try_from_val(env, &vec.get(4)?)?) },
        })
    }
}

// Conversion from Scholarship to Val
impl IntoVal<Env, Val> for &Scholarship {
    fn into_val(self, env: &Env) -> Val {
        Vec::from(env, vec![
            self.donor.clone().into_val(env),
            self.amount.into_val(env),
            self.criteria.clone().into_val(env),
            self.awarded.into_val(env),
            self.recipient.clone().map_or(Val::void(), |r| r.into_val(env))
        ]).into_val(env)
    }
}

// Contract methods
#[contractimpl]
impl ScholarshipContract {
    pub fn create_scholarship(env: Env, donor: Address, amount: u64, criteria: Symbol) {
        let storage_key = donor.clone().into_val(&env);
        let mut scholarships: Vec<Scholarship> = env.storage().persistent().get(storage_key.clone()).unwrap_or(Vec::new(&env));
        scholarships.push(Scholarship { donor, amount, criteria, awarded: false, recipient: None });
        env.storage().persistent().set(storage_key, scholarships.into_val(&env));
    }

    pub fn apply_for_scholarship(env: Env, student: Address, donor: Address, criteria: Symbol) {
        let storage_key = donor.clone().into_val(&env);
        let mut scholarships: Vec<Scholarship> = env.storage().persistent().get(storage_key.clone()).unwrap_or(Vec::new(&env));
        for scholarship in scholarships.iter_mut() {
            if scholarship.criteria == criteria && !scholarship.awarded {
                scholarship.recipient = Some(student.clone());
                scholarship.awarded = true;
                break;
            }
        }
        env.storage().persistent().set(storage_key, scholarships.into_val(&env));
    }

    pub fn get_scholarships(env: Env, donor: Address) -> Vec<Scholarship> {
        let storage_key = donor.into_val(&env);
        env.storage().persistent().get(storage_key).unwrap_or(Vec::new(&env))
    }

    pub fn get_recipient(env: Env, donor: Address, criteria: Symbol) -> Option<Address> {
        let storage_key = donor.into_val(&env);
        let scholarships: Vec<Scholarship> = env.storage().persistent().get(storage_key).unwrap_or(Vec::new(&env));
        scholarships.iter().find(|scholarship| scholarship.criteria == criteria).and_then(|scholarship| scholarship.recipient.clone())
    }
}

// Unit tests

mod tests
