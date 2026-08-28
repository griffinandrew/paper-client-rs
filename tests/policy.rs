mod common;

#[cfg(feature = "tokio")]
use paper_client::AsyncPaperClient;
use paper_client::{PaperClient, PaperPolicy};
use serial_test::serial;

const INITIAL_POLICY: PaperPolicy = PaperPolicy::Lfu;

#[test]
#[serial]
fn policy() {
	let mut client = common::init_client(true);

	// The tiered cache fixes its design at startup, so the server refuses
	// this rather than accepting it. Asserting the refusal still catches the
	// failure that matters -- a server that returns ok and changes nothing.
	let before = get_cache_policy(&mut client);

	let result = client.policy(INITIAL_POLICY);
	assert!(result.is_err(), "the tiered server should refuse a policy change");

	let after = get_cache_policy(&mut client);
	assert_eq!(after, before, "a refused policy change must not alter the policy");
}

#[cfg(feature = "tokio")]
#[tokio::test]
#[serial]
async fn policy_async() {
	let mut client = common::init_async_client(true).await;

	let before = get_cache_policy_async(&mut client).await;

	let result = client.policy(INITIAL_POLICY).await;
	assert!(result.is_err(), "the tiered server should refuse a policy change");

	let after = get_cache_policy_async(&mut client).await;
	assert_eq!(after, before, "a refused policy change must not alter the policy");
}

fn get_cache_policy(client: &mut PaperClient) -> PaperPolicy {
	let status = client
		.status()
		.expect("Could not get cache status.");

	status.policy().clone()
}

async fn get_cache_policy_async(client: &mut AsyncPaperClient) -> PaperPolicy {
	let status = client
		.status()
		.await
		.expect("Could not get cache status.");

	status.policy().clone()
}
