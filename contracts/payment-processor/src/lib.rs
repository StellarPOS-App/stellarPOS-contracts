#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, log};
use soroban_token_sdk::{TokenClient, TokenUtils};
use shared::{errors::ContractError, types::PaymentStatus};

// Contract storage keys
#[contracttype]
pub enum DataKey {
    Admin,
    FeeRate,
    PaymentCounter,
    Payment(u64),
    MerchantFees(Address),
}

// Payment data structure
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Payment {
    pub id: u64,
    pub payer: Address,
    pub merchant: Address,
    pub amount: i128,
    pub asset: Address,
    pub fee: i128,
    pub status: PaymentStatus,
    pub memo: String,
    pub timestamp: u64,
    pub refund_amount: i128,
}

// Contract events
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PaymentCreatedEvent {
    pub payment_id: u64,
    pub payer: Address,
    pub merchant: Address,
    pub amount: i128,
    pub asset: Address,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RefundProcessedEvent {
    pub payment_id: u64,
    pub refund_amount: i128,
    pub reason: String,
}

#[contract]
pub struct PaymentProcessorContract;

#[contractimpl]
impl PaymentProcessorContract {
    /// Initialize the payment processor contract
    pub fn initialize(env: Env, admin: Address, fee_rate: u32) -> Result<(), ContractError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(ContractError::AlreadyInitialized);
        }

        // Validate fee rate (max 10% = 1000 basis points)
        if fee_rate > 1000 {
            return Err(ContractError::InvalidFeeRate);
        }

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::FeeRate, &fee_rate);
        env.storage().instance().set(&DataKey::PaymentCounter, &0u64);

        log!(&env, "PaymentProcessor initialized with admin: {:?}, fee_rate: {}", admin, fee_rate);
        
        Ok(())
    }

    /// Process a payment transaction
    pub fn process_payment(
        env: Env,
        payer: Address,
        merchant: Address,
        amount: i128,
        asset: Address,
        memo: String,
    ) -> Result<u64, ContractError> {
        payer.require_auth();

        if amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        // Get next payment ID
        let payment_counter: u64 = env.storage().instance().get(&DataKey::PaymentCounter).unwrap_or(0);
        let payment_id = payment_counter + 1;

        // Calculate fee
        let fee_rate: u32 = env.storage().instance().get(&DataKey::FeeRate).unwrap_or(0);
        let fee = (amount * fee_rate as i128) / 10000; // fee_rate in basis points
        let net_amount = amount - fee;

        // Create token client
        let token = TokenClient::new(&env, &asset);

        // Transfer payment amount from payer to merchant
        token.transfer(&payer, &merchant, &net_amount);

        // Transfer fee to admin if fee > 0
        if fee > 0 {
            let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
            token.transfer(&payer, &admin, &fee);
        }

        // Create payment record
        let payment = Payment {
            id: payment_id,
            payer: payer.clone(),
            merchant: merchant.clone(),
            amount,
            asset: asset.clone(),
            fee,
            status: PaymentStatus::Completed,
            memo: memo.clone(),
            timestamp: env.ledger().timestamp(),
            refund_amount: 0,
        };

        // Store payment
        env.storage().persistent().set(&DataKey::Payment(payment_id), &payment);
        env.storage().instance().set(&DataKey::PaymentCounter, &payment_id);

        // Emit event
        env.events().publish(
            ("payment_created", payment_id),
            PaymentCreatedEvent {
                payment_id,
                payer: payer.clone(),
                merchant: merchant.clone(),
                amount,
                asset,
            },
        );

        log!(&env, "Payment processed: ID {}, Amount: {}, Fee: {}", payment_id, amount, fee);

        Ok(payment_id)
    }

    /// Get payment details
    pub fn get_payment(env: Env, payment_id: u64) -> Result<Payment, ContractError> {
        env.storage()
            .persistent()
            .get(&DataKey::Payment(payment_id))
            .ok_or(ContractError::PaymentNotFound)
    }

    /// Verify payment status
    pub fn verify_payment(env: Env, payment_id: u64) -> Result<PaymentStatus, ContractError> {
        let payment: Payment = Self::get_payment(env, payment_id)?;
        Ok(payment.status)
    }

    /// Process refund (admin only)
    pub fn process_refund(
        env: Env,
        payment_id: u64,
        refund_amount: i128,
        reason: String,
    ) -> Result<u64, ContractError> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        if refund_amount <= 0 {
            return Err(ContractError::InvalidAmount);
        }

        let mut payment: Payment = Self::get_payment(env.clone(), payment_id)?;
        
        if payment.status != PaymentStatus::Completed {
            return Err(ContractError::PaymentNotCompleted);
        }

        let total_refunded = payment.refund_amount + refund_amount;
        if total_refunded > payment.amount {
            return Err(ContractError::RefundExceedsPayment);
        }

        // Process refund transfer
        let token = TokenClient::new(&env, &payment.asset);
        token.transfer(&payment.merchant, &payment.payer, &refund_amount);

        // Update payment record
        payment.refund_amount = total_refunded;
        if total_refunded == payment.amount {
            payment.status = PaymentStatus::FullyRefunded;
        } else {
            payment.status = PaymentStatus::PartiallyRefunded;
        }

        env.storage().persistent().set(&DataKey::Payment(payment_id), &payment);

        // Emit refund event
        env.events().publish(
            ("refund_processed", payment_id),
            RefundProcessedEvent {
                payment_id,
                refund_amount,
                reason,
            },
        );

        log!(&env, "Refund processed: Payment {}, Amount: {}", payment_id, refund_amount);

        Ok(payment_id)
    }

    /// Update fee rate (admin only)
    pub fn update_fee_rate(env: Env, new_fee_rate: u32) -> Result<(), ContractError> {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        admin.require_auth();

        if new_fee_rate > 1000 {
            return Err(ContractError::InvalidFeeRate);
        }

        env.storage().instance().set(&DataKey::FeeRate, &new_fee_rate);
        
        log!(&env, "Fee rate updated to: {}", new_fee_rate);
        
        Ok(())
    }

    /// Get current fee rate
    pub fn get_fee_rate(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::FeeRate).unwrap_or(0)
    }

    /// Get total payments processed
    pub fn get_payment_count(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::PaymentCounter).unwrap_or(0)
    }

    /// Get contract admin
    pub fn get_admin(env: Env) -> Result<Address, ContractError> {
        env.storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(ContractError::NotInitialized)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::{Address as _, Ledger}, Address, Env};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let admin = Address::generate(&env);
        let fee_rate = 250; // 2.5%

        let result = PaymentProcessorContract::initialize(env.clone(), admin.clone(), fee_rate);
        assert!(result.is_ok());

        assert_eq!(PaymentProcessorContract::get_admin(env.clone()).unwrap(), admin);
        assert_eq!(PaymentProcessorContract::get_fee_rate(env), fee_rate);
    }

    #[test]
    fn test_process_payment() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let payer = Address::generate(&env);
        let merchant = Address::generate(&env);
        let token_admin = Address::generate(&env);
        
        // Initialize contract
        PaymentProcessorContract::initialize(env.clone(), admin, 250).unwrap();

        // Create and mint token
        let token_id = env.register_stellar_asset_contract(token_admin);
        let token = TokenClient::new(&env, &token_id);
        token.mint(&payer, &1000);

        // Process payment
        let payment_id = PaymentProcessorContract::process_payment(
            env.clone(),
            payer.clone(),
            merchant.clone(),
            100,
            token_id.clone(),
            String::from_str(&env, "Test payment"),
        ).unwrap();

        assert_eq!(payment_id, 1);

        // Verify payment
        let payment = PaymentProcessorContract::get_payment(env.clone(), payment_id).unwrap();
        assert_eq!(payment.payer, payer);
        assert_eq!(payment.merchant, merchant);
        assert_eq!(payment.amount, 100);
        assert_eq!(payment.status, PaymentStatus::Completed);
    }
}// Payment processor update 1
// Payment processor update 7
// Payment processor update 13
// Payment processor update 19
// Payment processor update 25
// Payment processor update 31
// Payment processor update 37
// Payment processor update 43
// Payment processor update 49
// Payment processor update 55
