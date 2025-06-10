# Metadata structure and storage format for each rune

// Represents the metadata for a single Rune token

struct RuneMetadata {
    // A unique identifier for each rune (e.g., UUID, hash, or counter-based ID)
    rune_id: String,

    // The current owner of the rune — represented by their ICP principal
    owner: Principal,

    // Number of rune tokens held (can be fractional or whole depending on logic)
    quantity: u64,

    // The amount of Bitcoin collateral that backs this particular rune
    collateral_in_btc: f64,

    // Timestamp of when this rune was minted (in seconds since Unix epoch)
    minted_timestamp: u64,

    // Timestamp of the last update to this rune (e.g., transferred, burned)
    last_updated_timestamp: u64,

    // USD price of a single rune at the time of minting
    usd_price_per_rune_at_mint: f64,

    // BTC price in USD at minting time (used for price integrity/reference)
    btc_usd_price_at_mint: f64,

    // The current status of this rune (e.g., Active, Burned, Pending)
    status: RuneStatus,
}

// Represents the current state of a Rune in its lifecycle
enum RuneStatus {
    Active,     // Currently valid and transferable
    Burned,     // Permanently destroyed and no longer valid
    Pending,    // Awaiting confirmation, mint, or transfer
}

// Primary storage mapping from unique Rune ID to its metadata
let rune_store: HashMap<String, RuneMetadata>;

// Maps each principal (user) to a list of rune_ids they own
let owner_index: HashMap<Principal, Vec<String>>;

// Maps each status to rune_ids for batch querying
let status_index: HashMap<RuneStatus, Vec<String>>;
