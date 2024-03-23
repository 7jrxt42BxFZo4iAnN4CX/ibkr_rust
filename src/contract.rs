use chrono::NaiveDate;
use std::fmt::Formatter;
use std::{num::ParseIntError, str::FromStr};

use crate::figi::{Figi, InvalidFigi};
use crate::{
    currency::Currency,
    exchange::{Primary, Routing},
    match_poly,
};
use ibapi_macros::{Security, make_getters};
use serde::{Deserialize, Serialize, Serializer};
use crate::execution::ContractType;

// =========================================================
// === Utility Types and Functions for Contract Creation ===
// =========================================================

// todo!("Ensure that includeExpired is always set to true");

#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// Wrapper enum for all possible contracts available in the API
pub enum Contract {
    /// A [`Forex`] contract.
    Forex(Forex),
    /// A [`Crypto`] contract.
    Crypto(Crypto),
    /// A [`Stock`] contract.
    Stock(Stock),
    /// An [`Index`] contract.
    Index(Index),
    //Cfd(Cfd),
    /// A [`SecFuture`] contract.
    SecFuture(SecFuture),
    /// A [`SecOption`] contract.
    SecOption(SecOption),
    //FutureSecOption(SecFutureOption),
    //Bond(Bond),
    //MutualFund(MutualFund),
    /// A [`Commodity`] contract.
    Commodity(Commodity),
    //Warrant(Warrant),
    //StructuredProduct(StructuredProduct),
}

macro_rules! contract_impl {
    ($sec_type: ty, $pat: pat_param => $exp: expr, $func_name_ref: ident, $func_name: ident) => {
        #[doc=concat!("Coerce the contract reference to a ", stringify!($sec_type), " reference.")]
        ///
        /// # Returns
        #[doc=concat!("The underlying ", stringify!($sec_type),  " reference.")]
        ///
        /// # Errors
        #[doc=concat!("Will error if the underlying contract is not a ", stringify!($sec_type), " reference.")]
        pub fn $func_name_ref(&self) -> anyhow::Result<&$sec_type> {
            match self {
                $pat => $exp,
                _ => Err(anyhow::anyhow!(
                    "Expected {}; found other contract type.",
                    stringify!($func_name)
                )),
            }
        }
        #[doc=concat!("Coerce the contract to a ", stringify!($sec_type))]
        ///
        /// # Returns
        #[doc=concat!("The underlying ", stringify!($sec_type))]
        ///
        /// # Errors
        #[doc=concat!("Will error if the underlying contract is not a ", stringify!($sec_type))]
        pub fn $func_name(self) -> anyhow::Result<$sec_type> {
            match self {
                $pat => $exp,
                _ => Err(anyhow::anyhow!(
                    "Expected {}; found other contract type.",
                    stringify!($func_name)
                )),
            }
        }
    };
}

impl Contract {
    contract_impl!(Forex, Self::Forex(t) => Ok(t), forex_ref, forex);
    contract_impl!(Crypto, Self::Crypto(t) => Ok(t), crypto_ref, crypto);
    contract_impl!(Stock, Self::Stock(t) => Ok(t), stock_ref, stock);
    contract_impl!(Index, Self::Index(t) => Ok(t), index_ref, index);
    contract_impl!(SecFuture, Self::SecFuture(t) => Ok(t), secfuture_ref, secfuture);
    contract_impl!(SecOption, Self::SecOption(t) => Ok(t), secoption_ref, secoption);
    contract_impl!(Commodity, Self::Commodity(t) => Ok(t), commodity_ref, commodity);
}

impl Serialize for Contract {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.serialize(serializer)
        )
    }
}

impl Security for Contract {
    #[inline]
    fn contract_id(&self) -> ContractId {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.contract_id()
        )
    }

    #[inline]
    fn min_tick(&self) -> f64 {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.min_tick()
        )
    }

    #[inline]
    fn symbol(&self) -> &str {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.symbol()
        )
    }

    #[inline]
    fn currency(&self) -> Currency {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.currency()
        )
    }

    #[inline]
    fn local_symbol(&self) -> &str {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.local_symbol()
        )
    }

    #[inline]
    fn long_name(&self) -> &str {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.long_name()
        )
    }

    #[inline]
    fn order_types(&self) -> &Vec<String> {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.order_types()
        )
    }

    #[inline]
    fn valid_exchanges(&self) -> &Vec<Routing> {
        match_poly!(self;
            Self::Forex(t)
            | Self::Crypto(t)
            | Self::Stock(t)
            | Self::Index(t)
            | Self::SecFuture(t)
            | Self::SecOption(t)
            | Self::Commodity(t) => t.valid_exchanges()
        )
    }
}

/// Create a new contract based on the unique IBKR contract ID. These contract IDs can be found
/// either in the Trader Workstation software or online at the
/// [IBKR Contract Information Center](https://contract.ibkr.info/v3.10/index.php).
///
/// # Arguments
/// * `client` - The client with which to send the validation request.
/// * `contract_id` - The IBKR contract ID corresponding to the contract that will be created.
///
/// # Errors
/// Returns any error encountered while writing the query string to the outgoing buffer, while
/// sending the creation signal to the client loop thread, or while receiving the complete contract
/// from the client loop thread. Additionally, this function will error if the contract does not
/// match the generic type specified in the function call.
///
/// # Returns
/// Returns a fully-defined contract that can be used for market data, placing orders, etc.
pub async fn new<S: Security>(
    client: &mut crate::client::ActiveClient,
    query: Query,
) -> anyhow::Result<S> {
    client.send_contract_query(query).await?;
    match client.recv_contract_query().await? {
        Contract::Forex(fx) => fx.try_into().map_err(|_| ()),
        Contract::Crypto(crypto) => crypto.try_into().map_err(|_| ()),
        Contract::Stock(stk) => stk.try_into().map_err(|_| ()),
        Contract::Index(ind) => ind.try_into().map_err(|_| ()),
        Contract::SecFuture(fut) => fut.try_into().map_err(|_| ()),
        Contract::SecOption(opt) => opt.try_into().map_err(|_| ()),
        Contract::Commodity(cmdty) => cmdty.try_into().map_err(|_| ()),
    }
    .map_err(|()| anyhow::anyhow!("Failed to create contract from {:?}: ", query))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// A type used to represent a query for a new contract, which can be made by providing either an
/// IBKR contract ID, or a FIGI.
pub enum Query {
    /// An IBKR contract ID with which to make a query. When parsing from a string, the routing field
    /// defaults to [`Routing::Smart`].
    IbContractId(ContractId, Routing),
    /// A FIGI.
    Figi(Figi),
}

#[derive(Debug, Clone)]
/// An error type representing the potential ways that a [`Query`] can be invalid.
pub enum InvalidQuery {
    /// An invalid [`Query::IbContractId`]
    IbContractId(ParseIntError),
    /// AN invalid [`Query::Figi`]
    Figi(InvalidFigi),
    /// Invalid in a way such that it's impossible to tell whether it was intended to be an [`Query::IbContractId`] or a [`'Query::Figi`].
    Empty,
}

impl std::fmt::Display for InvalidQuery {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid query. {self:?}")
    }
}

impl std::error::Error for InvalidQuery {}

impl FromStr for Query {
    type Err = InvalidQuery;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // A FIGI always begin with a letter
        if s.chars().nth(0).ok_or(InvalidQuery::Empty)?.is_numeric() {
            Ok(Self::IbContractId(
                s.parse().map_err(InvalidQuery::IbContractId)?,
                Routing::Smart,
            ))
        } else {
            Ok(Self::Figi(s.parse().map_err(InvalidQuery::Figi)?))
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
/// An error caused when a call to [`new`] returns a contract that differs from
/// the type defined in the initial call.
pub struct UnexpectedSecurityType(&'static str);

impl std::fmt::Display for UnexpectedSecurityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for UnexpectedSecurityType {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display"
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
/// A unique identifier used by both IBKR's trading systems and the API to define a specific
/// contract.
pub struct ContractId(pub i64);

impl FromStr for ContractId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Identifiers used by the broader industry / regulators to define a specific contract / asset.
pub enum SecurityId {
    /// For details, see:
    /// [CUSIP Description](https://www.cusip.com/identifiers.html?section=CUSIP).
    Cusip(String),
    /// For details, see:
    /// [SEDOL Description](https://www.lseg.com/en/data-indices-analytics/data/sedol).
    Sedol(String),
    /// For details, see:
    /// [ISIN Description](https://www.cusip.com/identifiers.html?section=ISIN#/ISIN).
    Isin(String),
    /// For details, see:
    /// [RIC Description](https://en.wikipedia.org/wiki/Refinitiv_Identification_Code).
    Ric(String),
}

// =================================
// === Valid Trait Definition ===
// =================================

mod indicators {
    use super::{Commodity, Contract, Crypto, Forex, Index, SecFuture, SecOption, Stock};
    use serde::Serialize;

    pub trait Valid:
        Serialize
        + Send
        + Sync
        + TryFrom<Forex>
        + TryFrom<Crypto>
        + TryFrom<Stock>
        + TryFrom<Index>
        + TryFrom<SecFuture>
        + TryFrom<SecOption>
        + TryFrom<Commodity>
        + Into<Contract>
    {
    }

    impl Valid for Contract {}
}

#[doc(alias = "Contract")]
/// Attributes shared by a tradable contract or asset. All valid contracts implement this trait.
pub trait Security: indicators::Valid {
    /// Get the security's contract ID
    ///
    /// # Returns
    /// The security's unique contract ID
    fn contract_id(&self) -> ContractId;
    /// Get the security's minimum tick size.
    ///
    /// # Returns
    /// The security's minimum tick size
    fn min_tick(&self) -> f64;
    /// Get the security's symbol.
    ///
    /// # Returns
    /// The security's symbol.
    fn symbol(&self) -> &str;
    /// Get the security's currency.
    ///
    /// # Returns
    /// The security's currency.
    fn currency(&self) -> Currency;
    /// Get the security's local symbol.
    ///
    /// # Returns
    /// The security's local symbol.
    fn local_symbol(&self) -> &str;
    /// Get the security's long name.
    ///
    /// # Returns
    /// The security's long name.
    fn long_name(&self) -> &str;
    /// Get the security's order types.
    ///
    /// # Returns
    /// The security's order types.
    fn order_types(&self) -> &Vec<String>;
    /// Get the security's valid exchanges.
    ///
    /// # Returns
    /// The security's valid exchanges..
    fn valid_exchanges(&self) -> &Vec<Routing>;
}

// =======================================
// === Definitions of Contract Structs ===
// =======================================

macro_rules! make_contract {
    ($( #[doc = $name_doc:expr] )? $name: ident $(,$trt: ident)?; $($field: ident: $f_type: ty),* $(,)?) => {
        $( #[doc = $name_doc] )?
        #[make_getters]
        #[derive(Debug, Clone, PartialEq, PartialOrd, $($trt)?)]
        pub struct $name {
            pub(crate) contract_id: ContractId,
            pub(crate) min_tick: f64,
            pub(crate) symbol: String,
            $(pub(crate) $field: $f_type,)*
            pub(crate) currency: Currency,
            pub(crate) local_symbol: String,
            pub(crate) long_name: String,
            pub(crate) order_types: Vec<String>,
            pub(crate) valid_exchanges: Vec<Routing>,
        }
    }
}

make_contract!(
    /// A [forex contract](https://interactivebrokers.github.io/tws-api/basic_contracts.html#cash), like GBPUSD.
    Forex,
    Security;
    exchange: Routing,
    trading_class: String
);
make_contract!(
    /// A [crypto contract](https://interactivebrokers.github.io/tws-api/basic_contracts.html#crypto), like BTC.
    Crypto,
    Security;
    trading_class: String
);
make_contract!(
    /// An [equity contract](https://interactivebrokers.github.io/tws-api/basic_contracts.html#stk), like AAPL.
    Stock,
    Security;
    exchange: Routing,
    primary_exchange: Primary,
    stock_type: String,
    security_ids: Vec<SecurityId>,
    sector: String,
    trading_class: String
);
make_contract!(
    /// An [index](https://interactivebrokers.github.io/tws-api/basic_contracts.html#ind), like SPX.
    Index,
    Security;
    exchange: Routing
);
make_contract!(
    /// A [commodity](https://interactivebrokers.github.io/tws-api/basic_contracts.html#Commodities), like XAUUSD.
    Commodity,
    Security;
    exchange: Routing,
    trading_class: String
);
make_contract!(
    /// A [futures contract](https://interactivebrokers.github.io/tws-api/basic_contracts.html#fut), like FGBL MAR 23.
    SecFuture,
    Security;
    exchange: Routing,
    multiplier: u32,
    expiration_date: NaiveDate,
    trading_class: String,
    underlying_contract_id: ContractId
);

make_contract!(
    /// Helper struct to hold the fields of a [`SecOption`].
    SecOptionInner;
    exchange: Routing,
    strike: f64,
    multiplier: u32,
    expiration_date: NaiveDate,
    underlying_contract_id: ContractId,
    sector: String,
    trading_class: String
);

#[derive(Debug, Clone, PartialEq, PartialOrd, Security)]
/// A [vanilla option contract](https://interactivebrokers.github.io/tws-api/basic_contracts.html#opt), like P BMW  20221216 72 M.
pub enum SecOption {
    /// A vanilla call option, defined by the following payoff function: max(S<sub>T</sub> - K, 0)
    Call(SecOptionInner),
    /// A vanilla put option, defined by the following payoff function: max(K - S<sub>T</sub>, 0)
    Put(SecOptionInner),
}

impl SecOption {
    #[must_use]
    #[inline]
    /// Return `true` if the option is a call option.
    pub fn is_call(&self) -> bool {
        matches!(self, SecOption::Call(_))
    }

    #[must_use]
    #[inline]
    /// Return `true` if the option is a put option.
    pub fn is_put(&self) -> bool {
        !self.is_call()
    }

    #[must_use]
    #[inline]
    /// Get a reference to the underlying contract's specifications.
    pub fn as_inner_ref(&self) -> &SecOptionInner {
        let (SecOption::Call(inner) | SecOption::Put(inner)) = self;
        inner
    }

    #[must_use]
    #[inline]
    /// Transform the option into its underlying specification
    pub fn into_inner(self) -> SecOptionInner {
        let (SecOption::Call(inner) | SecOption::Put(inner)) = self;
        inner
    }
}

// ===============================
// === Unimplemented Contracts ===
// ===============================

// make_contract!(Cfd; exchange: Routing);
// make_contract!(Bond; exchange: Routing);
// make_contract!(MutualFund; exchange: Routing);
// make_contract!(StructuredProduct; exchange: Routing, multiplier: u32, expiration_date: NaiveDate);

// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// pub enum SecFutureOption {
//     Call(SecOptionInner),
//     Put(SecOptionInner),
// }

// #[derive(Debug, Clone, PartialEq, PartialOrd)]
// pub enum Warrant {
//     Call(SecOptionInner),
//     Put(SecOptionInner),
// }

macro_rules! proxy_impl {
    ($sec_type: ty, $pat: pat_param => $exp: expr, $func_name: ident) => {
        #[doc=concat!("Coerce the contract to a ", stringify!($sec_type))]
        ///
        /// # Returns
        #[doc=concat!("The underlying ", stringify!($sec_type))]
        ///
        /// # Errors
        #[doc=concat!("Will error if the underlying contract is not a ", stringify!($sec_type))]
        pub fn $func_name(self) -> anyhow::Result<Proxy<$sec_type>> {
            match self.inner {
                $pat => Ok($exp),
                _ => Err(anyhow::anyhow!(
                    "Expected {}; found other contract type.",
                    stringify!($func_name)
                )),
            }
        }
    };
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
/// Holds information about a contract but lacks the information of a full [`Contract`].
pub struct Proxy<S: Security> {
    pub(crate) inner: S,
}


impl<S: Security> Proxy<S> {
    #[inline]
    /// Get the underlying Security's contract ID.
    pub fn contract_id(&self) -> ContractId {
        self.inner.contract_id()
    }

    #[inline]
    /// Get the underlying Security's symbol.
    pub fn symbol(&self) -> &str {
        self.inner.symbol()
    }

    #[inline]
    /// Get the underlying Security's currency.
    pub fn currency(&self) -> Currency {
        self.inner.currency()
    }

    #[inline]
    /// Get the underlying Security's symbol.
    pub fn local_symbol(&self) -> &str {
        self.inner.symbol()
    }
}

impl Proxy<Contract> {
    #[inline]
    #[must_use]
    /// Get the type of contract.
    pub fn contract_type(&self) -> ContractType {
        match self.inner {
            Contract::Forex(_) => ContractType::Forex,
            Contract::Crypto(_) => ContractType::Crypto,
            Contract::Stock(_) => ContractType::Stock,
            Contract::Index(_) => ContractType::Index,
            Contract::Commodity(_) => ContractType::Commodity,
            Contract::SecFuture(_) => ContractType::SecFuture,
            Contract::SecOption(_) => ContractType::SecOption,
        }
    }

    proxy_impl!(Forex, Contract::Forex(t) => Proxy::<Forex> { inner: t }, forex);
    proxy_impl!(Crypto, Contract::Crypto(t) => Proxy::<Crypto> { inner: t }, crypto);
    proxy_impl!(Stock, Contract::Stock(t) => Proxy::<Stock> { inner: t }, stock);
    proxy_impl!(Index, Contract::Index(t) => Proxy::<Index> { inner: t }, index);
    proxy_impl!(Commodity, Contract::Commodity(t) => Proxy::<Commodity> { inner: t }, commodity);
    proxy_impl!(SecFuture, Contract::SecFuture(t) => Proxy::<SecFuture> { inner: t }, sec_future);
    proxy_impl!(SecOption, Contract::SecOption(t) => Proxy::<SecOption> { inner: t }, sec_option);
}

impl Proxy<Forex> {
    #[inline]
    #[must_use]
    /// Get the [`Forex`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }
}

impl Proxy<Crypto> {
    #[inline]
    #[must_use]
    /// Get the [`Crypto`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }
}

impl Proxy<Stock> {
    #[inline]
    #[must_use]
    /// Get the [`Stock`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }

    #[inline]
    #[must_use]
    /// Get the [`Stock`] primary exchange.
    pub fn primary_exchange(&self) -> Primary {
        self.inner.primary_exchange
    }
}

impl Proxy<Commodity> {
    #[inline]
    #[must_use]
    /// Get the [`Commodity`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }
}

impl Proxy<SecFuture> {
    #[inline]
    #[must_use]
    /// Get the [`SecFuture`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.trading_class()
    }

    #[inline]
    #[must_use]
    /// Get the [`SecFuture`] `expiration_date`.
    pub fn expiration_date(&self) -> NaiveDate {
        self.inner.expiration_date
    }

    #[inline]
    #[must_use]
    /// Get the [`SecFuture`] `multiplier`.
    pub fn multiplier(&self) -> u32 {
        self.inner.multiplier
    }
}

impl Proxy<SecOption> {
    #[inline]
    #[must_use]
    /// Get the [`SecOption`] trading class.
    pub fn trading_class(&self) -> &str {
        self.inner.as_inner_ref().trading_class.as_str()
    }

    #[inline]
    #[must_use]
    /// Get the [`SecOption`] `expiration_date`.
    pub fn expiration_date(&self) -> NaiveDate {
        self.inner.as_inner_ref().expiration_date
    }

    #[inline]
    #[must_use]
    /// Get the [`SecOption`] `strike` price.
    pub fn strike(&self) -> f64 {
        self.inner.as_inner_ref().strike
    }

    #[inline]
    #[must_use]
    /// Return true if the [`SecOption`] is a call.
    pub fn is_call(&self) -> bool {
        self.inner.is_call()
    }

    #[inline]
    #[must_use]
    /// Return true if the [`SecOption`] is a put.
    pub fn is_put(&self) -> bool {
        self.inner.is_put()
    }

    #[inline]
    #[must_use]
    /// Get the [`SecOption`] `multiplier`.
    pub fn multiplier(&self) -> u32 {
        self.inner.as_inner_ref().multiplier
    }
}
