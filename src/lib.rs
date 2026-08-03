use std::collections::HashMap;

pub type U256 = ();
pub type Hash256 = ();
pub type Signature = ();
pub type CallKind = ();
pub type ExecutionStatus = ();
pub type Log = ();

pub type Address = [u8; 20];
pub type Bytes = Vec<u8>;
pub type Word = [u8; 32];

pub struct Wei(U256);
pub struct WeiPerGas(U256);
pub struct Gas(u64);

pub struct Ethereum {
    pub canonical_chain: Vec<Block>,
    pub state: WorldState,
}

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<SignedTransaction>,
}

pub struct BlockHeader {
    pub parent_hash: Hash256,
    pub state_root: Hash256,
    pub transactions_root: Hash256,
    pub receipts_root: Hash256,

    pub gas_limit: Gas,
    pub gas_used: Gas,
    pub base_fee_per_gas: WeiPerGas,

    pub number: u64,
    pub timestamp: u64,
    pub beneficiary: Address,
}

pub struct WorldState {
    pub accounts: HashMap<Address, Account>,
}

pub struct Account {
    pub nonce: u64,
    pub balance: Wei,
    pub code: Bytes,
    pub storage: HashMap<Word, Word>,
}

pub struct SignedTransaction {
    pub chain_id: u64,
    pub nonce: u64,

    pub gas_limit: Gas,
    pub max_fee_per_gas: WeiPerGas,
    pub max_priority_fee_per_gas: WeiPerGas,

    pub to: Option<Address>,
    pub value: Wei,
    pub input: Bytes,

    pub signature: Signature,
}

pub struct ExecutionContext {
    pub transaction_origin: Address,
    pub gas_price: WeiPerGas,

    pub block_number: u64,
    pub block_timestamp: u64,
    pub block_base_fee: WeiPerGas,
    pub block_beneficiary: Address,
    pub chain_id: u64,
}

pub struct Message {
    pub kind: CallKind,

    pub caller: Address,
    pub recipient: Address,
    pub code_address: Address,

    pub value: Wei,
    pub input: Bytes,
    pub gas: Gas,

    pub depth: u16,
    pub is_static: bool,
}

pub struct CallFrame {
    pub message: Message,

    pub program_counter: usize,
    pub stack: Vec<Word>,
    pub memory: Vec<u8>,
    pub return_data: Bytes,

    pub gas_remaining: Gas,
}

pub struct ExecutionResult {
    pub status: ExecutionStatus,
    pub gas_remaining: Gas,
    pub output: Bytes,
    pub logs: Vec<Log>,
}

pub struct Receipt {
    pub status: bool,
    pub cumulative_gas_used: Gas,
    pub logs: Vec<Log>,
}
