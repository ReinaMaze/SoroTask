#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, IntoVal, Symbol, Val, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskConfig {
    pub creator: Address,
    pub target: Address,
    pub function: Symbol,
    pub args: Vec<Val>,
    pub resolver: Option<Address>,
    pub interval: u64,
    pub last_run: u64,
    pub gas_balance: i128,
}

#[contracttype]
pub enum DataKey {
    Task(u64),
}

pub trait ResolverInterface {
    fn check_condition(env: Env, args: Vec<Val>) -> bool;
}

#[contract]
pub struct SoroTaskContract;

#[contractimpl]
impl SoroTaskContract {
    pub fn register(env: Env, task_id: u64, config: TaskConfig) {
        env.storage().persistent().set(&DataKey::Task(task_id), &config);
    }

    pub fn get_task(env: Env, task_id: u64) -> Option<TaskConfig> {
        env.storage().persistent().get(&DataKey::Task(task_id))
    }

    pub fn monitor(_env: Env) {
        // TODO: Implement task monitoring logic
    }

    /// Executes a registered task identified by `task_id`.
    ///
    /// # Flow
    /// 1. Load the [`TaskConfig`] from persistent storage (panics if absent).
    /// 2. If a `resolver` address is set, call `check_condition(args) -> bool`
    ///    on it via [`try_invoke_contract`] so that a faulty resolver never
    ///    permanently blocks execution — a failed call is treated as `false`.
    /// 3. When the condition is met (or there is no resolver), fire the
    ///    cross-contract call to `target::function(args)` using
    ///    [`invoke_contract`].
    /// 4. Only on a **successful** invocation persist the updated `last_run`
    ///    timestamp.
    ///
    /// # Safety & Atomicity
    /// Soroban transactions are fully atomic. If the target contract panics the
    /// entire transaction reverts, so SoroTask state is never left in an
    /// inconsistent half-updated form. `last_run` is written **after** the
    /// cross-contract call returns, guaranteeing it only reflects completed
    /// executions.
    pub fn execute(env: Env, task_id: u64) {
        let task_key = DataKey::Task(task_id);
        let mut config: TaskConfig = env
            .storage()
            .persistent()
            .get(&task_key)
            .expect("Task not found");

        // -- Resolver gate ----------------------------------------------------
        // When a resolver is present we use try_invoke_contract so that an
        // error inside the resolver (panic / wrong return type) degrades
        // gracefully to "skip this run" rather than aborting the whole tx.
        //
        // The resolver's interface is:  check_condition(args: Vec<Val>) -> bool
        // Its single explicit argument is the task's args vector, so we must
        // pack config.args into a one-element outer Vec<Val> -- otherwise the
        // host would unpack config.args as individual positional arguments,
        // causing an argument-count mismatch.
        let should_execute = match config.resolver {
            Some(ref resolver_address) => {
                let mut resolver_call_args = Vec::<Val>::new(&env);
                resolver_call_args.push_back(config.args.clone().into_val(&env));
                match env.try_invoke_contract::<bool, soroban_sdk::Error>(
                    resolver_address,
                    &Symbol::new(&env, "check_condition"),
                    resolver_call_args,
                ) {
                    Ok(Ok(true)) => true,
                    _ => false,
                }
            }
            None => true,
        };

        if should_execute {
            // -- Cross-contract call ------------------------------------------
            // args is Vec<Val> as stored in TaskConfig -- passed directly.
            // The return value is discarded; callers can read target state
            // independently if needed.
            env.invoke_contract::<Val>(&config.target, &config.function, config.args.clone());

            // -- State update -------------------------------------------------
            // Reached only when invoke_contract returned without panic.
            // Record the ledger timestamp of this successful execution.
            config.last_run = env.ledger().timestamp();
            env.storage().persistent().set(&task_key, &config);
        }
    }
}
