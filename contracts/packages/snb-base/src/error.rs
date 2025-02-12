use cosmwasm_std::StdError;
use thiserror::Error;

impl From<StdError> for ContractError {
    fn from(std_error: StdError) -> Self {
        Self::CustomError {
            val: std_error.to_string(),
        }
    }
}

impl From<ContractError> for StdError {
    fn from(contract_error: ContractError) -> Self {
        Self::generic_err(contract_error.to_string())
    }
}

pub fn parse_err(err: anyhow::Error) -> StdError {
    let context = format!("{:#?}", err);
    let source = err.source().map(|x| x.to_string()).unwrap_or_default();

    StdError::GenericErr {
        msg: format!("{}\n{}", context, source),
    }
}

/// Never is a placeholder to ensure we don't return any errors
#[derive(Error, Debug)]
pub enum Never {}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Custom Error val: {val:?}")]
    CustomError { val: String },

    // ------------------------------ common ----------------------------------------
    #[error("Sender does not have access permissions!")]
    Unauthorized,

    #[error("Parameters are not provided!")]
    NoParameters,

    #[error("It's too late to accept admin role!")]
    TransferAdminDeadline,

    #[error("Chain ID is not found!")]
    ChainIdIsNotFound,

    #[error("Oracle price data is outdated!")]
    OutdatedPrice,

    // ------------------------------ faucet ----------------------------------------
    #[error("Come back in {remaining_time_in_mins:?} minutes!")]
    ClaimCooldown { remaining_time_in_mins: u64 },

    #[error("Faucet is disabled!")]
    FaucetIsDisabled,

    // ------------------------------ adapter-scheduler ----------------------------------------
    #[error("Scheduler is executed too early!")]
    EarlyExecution,

    #[error("Low total available to borrow liquidity!")]
    LowAvailableLiquidity,

    #[error("Only {provided_amount:?} {denom:?} is provided while min collateral value is {min_amount:?}")]
    LowCollateral {
        provided_amount: u128,
        min_amount: u128,
        denom: String,
    },

    #[error("Bids on own collateral are not allowed!")]
    OwnCollateralBids,

    #[error("Discount is out of range!")]
    DiscountIsOutOfRange,

    #[error("Max LTV is exceeded!")]
    ExceededLtv,

    #[error("Max token amount per tx is exceeded!")]
    ExceededTokenLimit,

    #[error("Max bid amount is exceeded!")]
    ExceededBidAmount,

    #[error("Collection {val:?} price data is not found!")]
    PriceIsNotFound { val: String },

    #[error("Currency can not be changed after adding liquidity!")]
    ChangingCurrency,

    #[error("Multiple positions with same creation date are not allowed!")]
    SameCreationDate,

    #[error("Address already exists!")]
    AddressExists,

    #[error("Offer is not found!")]
    OfferIsNotFound,

    #[error("NFT is not found!")]
    NftIsNotFound,

    #[error("NFT already is added!")]
    NftDuplication,

    #[error("Exceeded liquidation bid NFT max amount!")]
    ExceededLiquidationBidNftMaxAmount,

    #[error("Zeros in prices!")]
    ZerosInPrices,

    #[error("Zero trading volume!")]
    ZeroTradingVolume,

    #[error("Zero amount to send!")]
    ZeroAmount,

    #[error("Exceeded outpost max amount!")]
    ExceededOutpostMaxAmount,

    #[error("Exceeded prices max amount!")]
    ExceededPricesMaxAmount,

    #[error("Empty raw price item vector!")]
    EmptyRawPriceItemVector,

    #[error("Empty price vector!")]
    EmptyPriceVector,

    #[error("Empty target list!")]
    EmptyTargetList,

    #[error("Collateral is not found!")]
    CollateralIsNotFound,

    #[error("Empty collection list!")]
    EmptyCollectionList,

    #[error("Empty token list!")]
    EmptyTokenList,

    #[error("Collection already exists!")]
    CollectionDuplication,

    #[error("Collection is not found!")]
    CollectionIsNotFound,

    #[error("Collection is not added!")]
    CollectionIsNotAdded,

    #[error("Collection balance is empty!")]
    CollectionBalanceIsEmpty,

    #[error("Wrong proposal status!")]
    WrongProposalStatus,

    #[error("Sum of weights is not equal one!")]
    WeightsAreUnbalanced,

    #[error("Weight is out of range!")]
    WeightIsOutOfRange,

    #[error("Exceeded available asset amount!")]
    ExceededAvailableAssetAmount,

    #[error("Undefined Reply ID!")]
    UndefinedReplyId,

    #[error("Asset is not found!")]
    AssetIsNotFound,

    #[error("Wrong asset type!")]
    WrongAssetType,

    #[error("Wrong message type!")]
    WrongMessageType,

    #[error("Wrong action type!")]
    WrongActionType,

    #[error("Wrong funds combination!")]
    WrongFundsCombination,

    #[error("{value:?} config is not found!")]
    ParameterIsNotFound { value: String },

    #[error("The contract is temporary paused")]
    ContractIsPaused,

    #[error("Denom already exists!")]
    DenomExists,

    #[error("Exceeded tokens per owner limit!")]
    TokenLimit,

    #[error("Parsing previous version error!")]
    ParsingPrevVersion,

    #[error("Parsing new version error!")]
    ParsingNewVersion,

    #[error("Msg version is not equal contract new version!")]
    ImproperMsgVersion,

    // ---------
    #[error("Outpost is not found!")]
    OutpostIsNotFound,

    #[error("Channel is not found!")]
    ChannelIsNotFound,

    #[error("User is not found!")]
    UserIsNotFound,
}
