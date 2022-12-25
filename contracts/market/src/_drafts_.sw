// SPDX-License-Identifier: BUSL-1.1
/*
███████╗██╗    ██╗ █████╗ ██╗   ██╗     ██████╗  █████╗ ███╗   ██╗ ██████╗ 
██╔════╝██║    ██║██╔══██╗╚██╗ ██╔╝    ██╔════╝ ██╔══██╗████╗  ██║██╔════╝ 
███████╗██║ █╗ ██║███████║ ╚████╔╝     ██║  ███╗███████║██╔██╗ ██║██║  ███╗
╚════██║██║███╗██║██╔══██║  ╚██╔╝      ██║   ██║██╔══██║██║╚██╗██║██║   ██║
███████║╚███╔███╔╝██║  ██║   ██║       ╚██████╔╝██║  ██║██║ ╚████║╚██████╔╝
╚══════╝ ╚══╝╚══╝ ╚═╝  ╚═╝   ╚═╝        ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═══╝ ╚═════╝                                                                         
*/

/**  General configuration constants */
storage {
    // @notice The admin of the protocol
    governor: Address = Address::from(ZERO_B256),
    // @notice The account which may trigger pauses
    // address public override immutable pauseGuardian;
    // @notice The address of the base token contract
    baseToken: ContractId = ContractId::from(ZERO_B256), //TODO: fix
    // @notice The address of the price feed for the base token
    baseTokenPriceFeed: ContractId = ContractId::from(ZERO_B256), //TODO: fix
    // @notice The address of the extension contract delegate
    // address public override immutable extensionDelegate,
    // @notice The point in the supply rates separating the low interest rate slope and the high interest rate slope (factor)
    // @dev uint64
    supplyKink: u64 = 0,
    // @notice Per second supply interest rate slope applied when utilization is below kink (factor)
    // @dev uint64
    supplyPerSecondInterestRateSlopeLow: u64 = 0,
    // @notice Per second supply interest rate slope applied when utilization is above kink (factor)
    // @dev uint64
    supplyPerSecondInterestRateSlopeHigh: u64 = 0,
    // @notice Per second supply base interest rate (factor)
    // @dev uint64
    supplyPerSecondInterestRateBase: u64 = 0,
    // @notice The point in the borrow rate separating the low interest rate slope and the high interest rate slope (factor)
    // @dev uint64
    borrowKink: u64 = 0,
    // @notice Per second borrow interest rate slope applied when utilization is below kink (factor)
    // @dev uint64
    borrowPerSecondInterestRateSlopeLow: u64 = 0,
    // @notice Per second borrow interest rate slope applied when utilization is above kink (factor)
    // @dev uint64
    borrowPerSecondInterestRateSlopeHigh: u64 = 0,
    // @notice Per second borrow base interest rate (factor)
    // @dev uint64
    borrowPerSecondInterestRateBase: u64 = 0,
    // @notice The fraction of the liquidation penalty that goes to buyers of collateral instead of the protocol
    // @dev uint64
    // storeFrontPriceFactor: u64 = 0,
    // @notice The scale for base token (must be less than 18 decimals)
    // @dev uint64
    // baseScale: u64 = 0,
    // @notice The scale for reward tracking
    // @dev uint64
    // trackingIndexScale: u64 = 0,
    // @notice The speed at which supply rewards are tracked (in trackingIndexScale)
    // @dev uint64
    // baseTrackingSupplySpeed: u64 = 0,
    // @notice The speed at which borrow rewards are tracked (in trackingIndexScale)
    // @dev uint64
    // baseTrackingBorrowSpeed: u64 = 0,
    // @notice The minimum amount of base principal wei for rewards to accrue
    // @dev This must be large enough so as to prevent division by base wei from overflowing the 64 bit indices
    // @dev uint104
    baseMinForRewards: u64 = 0,//TODO: change type to u128
    // @notice The minimum base amount required to initiate a borrow
    baseBorrowMin: u64 = 0,//TODO: change type to u128
    // @notice The minimum base token reserves which must be held before collateral is hodled
    targetReserves: u64 = 0,//TODO: change type to u128
    // @notice The number of decimals for wrapped base token
    decimals: u8 = 0,
    // @notice The number of assets this contract actually supports
    numAssets: u8 = 0,
    // @notice Factor to divide by when accruing rewards in order to preserve 6 decimals (i.e. baseScale / 1e6)
    // accrualDescaleFactor: u64 = 0,
    /** Collateral asset configuration (packed) */
    asset00_a: ContractId = ContractId::from(ZERO_B256),
    asset00_b: ContractId = ContractId::from(ZERO_B256),
    asset01_a: ContractId = ContractId::from(ZERO_B256),
    asset01_b: ContractId = ContractId::from(ZERO_B256),
    asset02_a: ContractId = ContractId::from(ZERO_B256),
    asset02_b: ContractId = ContractId::from(ZERO_B256),
    asset03_a: ContractId = ContractId::from(ZERO_B256),
    asset03_b: ContractId = ContractId::from(ZERO_B256),
    asset04_a: ContractId = ContractId::from(ZERO_B256),
    asset04_b: ContractId = ContractId::from(ZERO_B256),
    asset05_a: ContractId = ContractId::from(ZERO_B256),
    asset05_b: ContractId = ContractId::from(ZERO_B256),
    asset06_a: ContractId = ContractId::from(ZERO_B256),
    asset06_b: ContractId = ContractId::from(ZERO_B256),
    asset07_a: ContractId = ContractId::from(ZERO_B256),
    asset07_b: ContractId = ContractId::from(ZERO_B256),
    asset08_a: ContractId = ContractId::from(ZERO_B256),
    asset08_b: ContractId = ContractId::from(ZERO_B256),
    asset09_a: ContractId = ContractId::from(ZERO_B256),
    asset09_b: ContractId = ContractId::from(ZERO_B256),
    asset10_a: ContractId = ContractId::from(ZERO_B256),
    asset10_b: ContractId = ContractId::from(ZERO_B256),
    asset11_a: ContractId = ContractId::from(ZERO_B256),
    asset11_b: ContractId = ContractId::from(ZERO_B256),
    asset12_a: ContractId = ContractId::from(ZERO_B256),
    asset12_b: ContractId = ContractId::from(ZERO_B256),
    asset13_a: ContractId = ContractId::from(ZERO_B256),
    asset13_b: ContractId = ContractId::from(ZERO_B256),
    asset14_a: ContractId = ContractId::from(ZERO_B256),
    asset14_b: ContractId = ContractId::from(ZERO_B256),
}
