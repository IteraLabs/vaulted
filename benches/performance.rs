//! Performance benchmarks for USDv operations

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solana_sdk::pubkey::Pubkey;
use usdv_utils::{
    derive_vault_authority_pda,
    derive_all_addresses,
    safe_add, safe_sub, safe_mul,
    validate_deposit_amount,
    base_to_ui_amount, ui_to_base_amount,
};

fn benchmark_pda_derivation(c: &mut Criterion) {
    let program_id = Pubkey::new_unique();
    
    c.bench_function("derive_vault_authority_pda", |b| {
        b.iter(|| {
            let (pda, bump) = derive_vault_authority_pda(black_box(&program_id));
            black_box((pda, bump))
        })
    });
}

fn benchmark_address_derivation(c: &mut Criterion) {
    let program_id = Pubkey::new_unique();
    let user = Pubkey::new_unique();
    let usdc_mint = Pubkey::new_unique();
    let usdv_mint = Pubkey::new_unique();
    
    c.bench_function("derive_all_addresses", |b| {
        b.iter(|| {
            let addresses = derive_all_addresses(
                black_box(&program_id),
                black_box(&user),
                black_box(&usdc_mint),
                black_box(&usdv_mint),
            );
            black_box(addresses)
        })
    });
}

fn benchmark_math_operations(c: &mut Criterion) {
    c.bench_function("safe_add", |b| {
        b.iter(|| {
            let result = safe_add(black_box(1000), black_box(2000));
            black_box(result)
        })
    });
    
    c.bench_function("safe_mul", |b| {
        b.iter(|| {
            let result = safe_mul(black_box(1000), black_box(1000));
            black_box(result)
        })
    });
}

fn benchmark_validation(c: &mut Criterion) {
    c.bench_function("validate_deposit_amount", |b| {
        b.iter(|| {
            let result = validate_deposit_amount(black_box(1_000_000));
            black_box(result)
        })
    });
}

fn benchmark_unit_conversion(c: &mut Criterion) {
    c.bench_function("base_to_ui_amount", |b| {
        b.iter(|| {
            let result = base_to_ui_amount(black_box(1_000_000), black_box(6));
            black_box(result)
        })
    });
    
    c.bench_function("ui_to_base_amount", |b| {
        b.iter(|| {
            let result = ui_to_base_amount(black_box(1.0), black_box(6));
            black_box(result)
        })
    });
}

criterion_group!(
    benches,
    benchmark_pda_derivation,
    benchmark_address_derivation, 
    benchmark_math_operations,
    benchmark_validation,
    benchmark_unit_conversion
);
criterion_main!(benches);
