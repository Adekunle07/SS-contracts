//! Tests for the debtor exposure cap contract (issue #737).

use super::*;
use soroban_sdk::testutils::{Address as _, Events as _};
use soroban_sdk::xdr::{ContractEvent, ContractEventBody, ScVal};
use soroban_sdk::{Address, BytesN, Env, Symbol, TryFromVal, TryIntoVal, Val, Vec};

const DEFAULT_CAP: i128 = 1_000;

/// Deployed contract and its admin.
struct Ctx<'a> {
    admin: Address,
    client: DebtorExposureCapClient<'a>,
}

fn bytes(env: &Env, seed: u8) -> BytesN<32> {
    BytesN::from_array(env, &[seed; 32])
}

/// A debtor identifier.
fn debtor(env: &Env, seed: u8) -> BytesN<32> {
    bytes(env, seed)
}

/// An invoice identifier.
fn invoice(env: &Env, seed: u8) -> BytesN<32> {
    bytes(env, seed)
}

fn setup<'a>(env: &'a Env) -> Ctx<'a> {
    env.mock_all_auths();

    let admin = Address::generate(env);
    let contract_id = env.register(DebtorExposureCap, ());
    let client = DebtorExposureCapClient::new(env, &contract_id);
    client.initialize(&admin, &DEFAULT_CAP);

    Ctx { admin, client }
}

/// Split an event into its topics and its data.
fn parse_event(env: &Env, event: &ContractEvent) -> (Vec<Val>, Val) {
    let ContractEventBody::V0(v0) = &event.body;
    let raw_topics = ScVal::Vec(Some(v0.topics.clone().into()));
    let topics = Vec::<Val>::try_from_val(env, &raw_topics).unwrap();
    let data = Val::try_from_val(env, &v0.data).unwrap();
    (topics, data)
}

/// The data of the last event with the given name.
fn last_event_data(env: &Env, name: &str) -> Option<Val> {
    let mut result = None;
    for event in env.events().all().events().iter() {
        let (topics, data) = parse_event(env, event);
        let topic = Symbol::try_from_val(env, &topics.get(0).unwrap()).unwrap();
        if topic == Symbol::new(env, name) {
            result = Some(data);
        }
    }
    result
}

/// The number of events with the given name.
fn event_count(env: &Env, name: &str) -> u32 {
    let mut count = 0;
    for event in env.events().all().events().iter() {
        let (topics, _data) = parse_event(env, event);
        let topic = Symbol::try_from_val(env, &topics.get(0).unwrap()).unwrap();
        if topic == Symbol::new(env, name) {
            count += 1;
        }
    }
    count
}

// ── Initialisation ─────────────────────────────────────────────────────────────

#[test]
fn initialising_sets_the_default_cap() {
    let env = Env::default();
    let ctx = setup(&env);

    assert_eq!(ctx.client.get_debtor_cap(&debtor(&env, 1)), DEFAULT_CAP);
    assert_eq!(ctx.client.outstanding(&debtor(&env, 1)), 0);
    assert_eq!(ctx.client.debtor_headroom(&debtor(&env, 1)), DEFAULT_CAP);
}

#[test]
fn double_initialisation_is_rejected() {
    let env = Env::default();
    let ctx = setup(&env);

    let result = ctx.client.try_initialize(&ctx.admin, &DEFAULT_CAP);
    assert_eq!(result, Err(Ok(Error::AlreadyInitialized)));
}

#[test]
fn initialising_with_a_non_positive_cap_is_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let admin = Address::generate(&env);
    let contract_id = env.register(DebtorExposureCap, ());
    let client = DebtorExposureCapClient::new(&env, &contract_id);

    let result = client.try_initialize(&admin, &0);
    assert_eq!(result, Err(Ok(Error::InvalidCap)));
}

#[test]
fn operations_before_initialisation_are_rejected() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(DebtorExposureCap, ());
    let client = DebtorExposureCapClient::new(&env, &contract_id);

    let result = client.try_list_invoice(&invoice(&env, 1), &debtor(&env, 1), &100);
    assert_eq!(result, Err(Ok(Error::NotInitialized)));
}

// ── Cap enforcement ────────────────────────────────────────────────────────────

#[test]
fn funding_that_would_exceed_the_cap_reverts() {
    let env = Env::default();
    let ctx = setup(&env);
    let d = debtor(&env, 1);
    let a = invoice(&env, 1);
    let b = invoice(&env, 2);

    ctx.client.list_invoice(&a, &d, &700);
    ctx.client.fund(&a, &700);
    assert_eq!(ctx.client.outstanding(&d), 700);

    // Listing fits exactly within the remaining capacity.
    ctx.client.list_invoice(&b, &d, &300);

    // Funding beyond the cap is rejected and leaves the state untouched.
    let result = ctx.client.try_fund(&b, &400);
    assert_eq!(result, Err(Ok(Error::DebtorCapExceeded)));
    assert_eq!(ctx.client.outstanding(&d), 700);
    assert_eq!(ctx.client.debtor_headroom(&d), 300);
}

#[test]
fn funding_at_exactly_the_cap_succeeds() {
    let env = Env::default();
    let ctx = setup(&env);
    let d = debtor(&env, 1);
    let a = invoice(&env, 1);

    ctx.client.list_invoice(&a, &d, &DEFAULT_CAP);
    ctx.client.fund(&a, &DEFAULT_CAP);

    assert_eq!(ctx.client.outstanding(&d), DEFAULT_CAP);
    assert_eq!(ctx.client.debtor_headroom(&d), 0);
}

#[test]
fn a_listing_that_would_exceed_the_cap_is_rejected() {
    let env = Env::default();
    let ctx = setup(&env);
    let d = debtor(&env, 1);
    let a = invoice(&env, 1);

    let result = ctx.client.try_list_invoice(&a, &d, &(DEFAULT_CAP + 1));
    assert_eq!(result, Err(Ok(Error::DebtorCapExceeded)));

    // The rejected invoice was not recorded.
    let fund = ctx.client.try_fund(&a, &10);
    assert_eq!(fund, Err(Ok(Error::InvoiceNotFound)));
}

#[test]
fn funding_an_unlisted_invoice_is_rejected() {
    let env = Env::default();
    let ctx = setup(&env);

    let result = ctx.client.try_fund(&invoice(&env, 9), &100);
    assert_eq!(result, Err(Ok(Error::InvoiceNotFound)));
}

#[test]
fn listing_an_invoice_twice_is_rejected() {
    let env = Env::default();
    let ctx = setup(&env);
    let d = debtor(&env, 1);
    let a = invoice(&env, 1);

    ctx.client.list_invoice(&a, &d, &100);
    let result = ctx.client.try_list_invoice(&a, &d, &100);
    assert_eq!(result, Err(Ok(Error::InvoiceAlreadyListed)));
}

// ── Freeing capacity ───────────────────────────────────────────────────────────

#[test]
fn settlement_frees_capacity_equal_to_the_principal_repaid() {
    let env = Env::default();
    let ctx = setup(&env);
    let d = debtor(&env, 1);
    let a = invoice(&env, 1);

    ctx.client.list_invoice(&a, &d, &700);
    ctx.client.fund(&a, &700);
    assert_eq!(ctx.client.debtor_headroom(&d), 300);

    ctx.client.settle(&a, &400);
    assert_eq!(ctx.client.outstanding(&d), 300);
    assert_eq!(ctx.client.debtor_headroom(&d), 700);

    ctx.client.settle(&a, &300);
    assert_eq!(ctx.client.outstanding(&d), 0);
    assert_eq!(ctx.client.debtor_headroom(&d), DEFAULT_CAP);
}

#[test]
fn write_off_frees_capacity_equal_to_the_principal() {
    let env = Env::default();
    let ctx = setup(&env);
    let d = debtor(&env, 1);
    let a = invoice(&env, 1);

    ctx.client.list_invoice(&a, &d, &500);
    ctx.client.fund(&a, &500);

    ctx.client.write_off(&a, &500);
    assert_eq!(ctx.client.outstanding(&d), 0);
    assert_eq!(ctx.client.debtor_headroom(&d), DEFAULT_CAP);
}

#[test]
fn freed_capacity_can_be_reused() {
    let env = Env::default();
    let ctx = setup(&env);
    let d = debtor(&env, 1);
    let a = invoice(&env, 1);
    let b = invoice(&env, 2);

    ctx.client.list_invoice(&a, &d, &DEFAULT_CAP);
    ctx.client.fund(&a, &DEFAULT_CAP);

    // The cap is fully used, so a second invoice cannot be listed yet.
    assert_eq!(
        ctx.client.try_list_invoice(&b, &d, &100),
        Err(Ok(Error::DebtorCapExceeded))
    );

    // Settling the first invoice releases the capacity.
    ctx.client.settle(&a, &DEFAULT_CAP);
    ctx.client.list_invoice(&b, &d, &DEFAULT_CAP);
    ctx.client.fund(&b, &DEFAULT_CAP);
    assert_eq!(ctx.client.outstanding(&d), DEFAULT_CAP);
}

// ── Overrides ──────────────────────────────────────────────────────────────────

#[test]
fn a_per_debtor_cap_overrides_the_default() {
    let env = Env::default();
    let ctx = setup(&env);
    let d1 = debtor(&env, 1);
    let d2 = debtor(&env, 2);
    let a = invoice(&env, 1);

    ctx.client.set_debtor_cap(&ctx.admin, &d2, &2_000);
    assert_eq!(ctx.client.get_debtor_cap(&d2), 2_000);
    assert_eq!(ctx.client.get_debtor_cap(&d1), DEFAULT_CAP);

    // The overridden debtor can exceed what the default would allow.
    ctx.client.list_invoice(&a, &d2, &1_500);
    ctx.client.fund(&a, &1_500);
    assert_eq!(ctx.client.outstanding(&d2), 1_500);
    assert_eq!(ctx.client.debtor_headroom(&d2), 500);

    // The default is unchanged for other debtors.
    assert_eq!(ctx.client.debtor_headroom(&d1), DEFAULT_CAP);
}

#[test]
fn only_the_admin_can_set_a_debtor_cap() {
    let env = Env::default();
    let ctx = setup(&env);
    let impostor = Address::generate(&env);

    let result = ctx
        .client
        .try_set_debtor_cap(&impostor, &debtor(&env, 1), &2_000);
    assert_eq!(result, Err(Ok(Error::Unauthorized)));
    assert_eq!(ctx.client.get_debtor_cap(&debtor(&env, 1)), DEFAULT_CAP);
}

#[test]
fn setting_a_non_positive_debtor_cap_is_rejected() {
    let env = Env::default();
    let ctx = setup(&env);

    let result = ctx
        .client
        .try_set_debtor_cap(&ctx.admin, &debtor(&env, 1), &0);
    assert_eq!(result, Err(Ok(Error::InvalidCap)));
}

// ── Headroom ───────────────────────────────────────────────────────────────────

#[test]
fn debtor_headroom_equals_cap_minus_outstanding() {
    let env = Env::default();
    let ctx = setup(&env);
    let d = debtor(&env, 1);
    let a = invoice(&env, 1);

    ctx.client.set_debtor_cap(&ctx.admin, &d, &1_200);
    ctx.client.list_invoice(&a, &d, &700);
    ctx.client.fund(&a, &700);

    let cap = ctx.client.get_debtor_cap(&d);
    let outstanding = ctx.client.outstanding(&d);
    assert_eq!(ctx.client.debtor_headroom(&d), cap - outstanding);
    assert_eq!(ctx.client.debtor_headroom(&d), 500);
}

// ── Events ─────────────────────────────────────────────────────────────────────

#[test]
fn setting_a_cap_emits_debtor_cap_set() {
    let env = Env::default();
    let ctx = setup(&env);
    let d = debtor(&env, 1);

    ctx.client.set_debtor_cap(&ctx.admin, &d, &2_000);

    assert_eq!(event_count(&env, "debtor_cap_set"), 1);
    let data = last_event_data(&env, "debtor_cap_set").unwrap();
    let cap: i128 = data.try_into_val(&env).unwrap();
    assert_eq!(cap, 2_000);
}

#[test]
fn funding_to_the_cap_emits_debtor_cap_hit() {
    let env = Env::default();
    let ctx = setup(&env);
    let d = debtor(&env, 1);
    let a = invoice(&env, 1);
    let b = invoice(&env, 2);

    ctx.client.list_invoice(&a, &d, &700);
    ctx.client.fund(&a, &700);
    // Still below the cap, so the cap has not been hit yet.
    assert_eq!(event_count(&env, "debtor_cap_hit"), 0);

    ctx.client.list_invoice(&b, &d, &300);
    ctx.client.fund(&b, &300);
    // This funding brings the outstanding amount exactly to the cap.
    assert_eq!(event_count(&env, "debtor_cap_hit"), 1);
    let data = last_event_data(&env, "debtor_cap_hit").unwrap();
    let (outstanding, cap): (i128, i128) = data.try_into_val(&env).unwrap();
    assert_eq!(outstanding, DEFAULT_CAP);
    assert_eq!(cap, DEFAULT_CAP);

    assert_eq!(ctx.client.outstanding(&d), DEFAULT_CAP);
    assert_eq!(ctx.client.debtor_headroom(&d), 0);
}
