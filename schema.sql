CREATE TABLE `ActivityInvolvement`
(
    `Id`                int unsigned NOT NULL AUTO_INCREMENT,
    `FundamentalDataId` int unsigned DEFAULT NULL,
    `Activity`          varchar(25)  DEFAULT NULL,
    `Involvement`       tinyint(1)   DEFAULT NULL,
    PRIMARY KEY (`Id`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `CrossReferencedSymbols`
(
    `id`       int          NOT NULL AUTO_INCREMENT,
    `code`     varchar(12)  NOT NULL,
    `exchange` varchar(10)  NOT NULL,
    `name`     varchar(100) NOT NULL,
    `isin`     varchar(12) DEFAULT NULL,
    PRIMARY KEY (`id`),
    FOREIGN KEY (`code`) REFERENCES `ExchangeSymbol` (`code`),
    FOREIGN KEY (`exchange`) REFERENCES `Exchange` (`code`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;


CREATE TABLE `DownloadedSymbol`
(
    `code`       varchar(12) NOT NULL,
    `exchange`   varchar(10) NOT NULL,
    `updated_at` timestamp   NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    isEmpty      BOOLEAN          DEFAULT FALSE,

    PRIMARY KEY (`code`, `exchange`),
    FOREIGN KEY (`code`) REFERENCES `ExchangeSymbol` (`code`),
    FOREIGN KEY (`exchange`) REFERENCES `Exchange` (`code`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `ESGScore`
(
    `FundamentalDataId`          int unsigned DEFAULT NULL,
    `RatingDate`                 date         DEFAULT NULL,
    `TotalEsg`                   float        DEFAULT NULL,
    `TotalEsgPercentile`         float        DEFAULT NULL,
    `EnvironmentScore`           float        DEFAULT NULL,
    `EnvironmentScorePercentile` tinyint      DEFAULT NULL,
    `SocialScore`                float        DEFAULT NULL,
    `SocialScorePercentile`      tinyint      DEFAULT NULL,
    `GovernanceScore`            float        DEFAULT NULL,
    `GovernanceScorePercentile`  tinyint      DEFAULT NULL,
    `ControversyLevel`           tinyint      DEFAULT NULL,
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `EarningsAnnual`
(
    `FundamentalDataId` int unsigned NOT NULL,
    `date`              date         NOT NULL,
    `epsActual`         float DEFAULT NULL,
    PRIMARY KEY (`FundamentalDataId`, `date`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `EarningsTrend`
(
    `FundamentalDataId`                int unsigned NOT NULL,
    `date`                             date         NOT NULL,
    `period`                           varchar(4) DEFAULT NULL,
    `growth`                           float      DEFAULT NULL,
    `earningsEstimateAvg`              float      DEFAULT NULL,
    `earningsEstimateLow`              float      DEFAULT NULL,
    `earningsEstimateHigh`             float      DEFAULT NULL,
    `earningsEstimateYearAgoEps`       float      DEFAULT NULL,
    `earningsEstimateNumberOfAnalysts` mediumint  DEFAULT NULL,
    `earningsEstimateGrowth`           float      DEFAULT NULL,
    `revenueEstimateAvg`               bigint     DEFAULT NULL,
    `revenueEstimateLow`               bigint     DEFAULT NULL,
    `revenueEstimateHigh`              bigint     DEFAULT NULL,
    `revenueEstimateYearAgoEps`        bigint     DEFAULT NULL,
    `revenueEstimateNumberOfAnalysts`  mediumint  DEFAULT NULL,
    `revenueEstimateGrowth`            float      DEFAULT NULL,
    `epsTrendCurrent`                  float      DEFAULT NULL,
    `epsTrend7daysAgo`                 float      DEFAULT NULL,
    `epsTrend30daysAgo`                float      DEFAULT NULL,
    `epsTrend60daysAgo`                float      DEFAULT NULL,
    `epsTrend90daysAgo`                float      DEFAULT NULL,
    `epsRevisionsUpLast7days`          mediumint  DEFAULT NULL,
    `epsRevisionsUpLast30days`         mediumint  DEFAULT NULL,
    `epsRevisionsDownLast7days`        mediumint  DEFAULT NULL,
    `epsRevisionsDownLast30days`       mediumint  DEFAULT NULL,
    PRIMARY KEY (`FundamentalDataId`, `date`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `Exchange`
(
    `name`         varchar(64) DEFAULT NULL,
    `code`         varchar(10) NOT NULL,
    `operatingMIC` varchar(20) DEFAULT NULL,
    `country`      varchar(20) DEFAULT NULL,
    `currency`     char(3)     DEFAULT NULL,
    `countryISO2`  char(2)     DEFAULT NULL,
    `countryISO3`  char(3)     DEFAULT NULL,
    PRIMARY KEY (`code`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `ExchangeSymbol`
(
    `name`         varchar(250) DEFAULT NULL,
    `code`         varchar(12) NOT NULL,
    `exchange`     varchar(10) NOT NULL,
    `type`         varchar(50)  DEFAULT NULL,
    `country`      varchar(32)  DEFAULT NULL,
    `currency`     char(3)      DEFAULT NULL,
    `isin`         char(12)     DEFAULT NULL,
    `realExchange` varchar(10)  DEFAULT NULL,
    PRIMARY KEY (`code`, `exchange`),
    UNIQUE KEY `code` (`code`, `exchange`, `isin`, `name`, `type`),
    KEY `idx_type` (`type`),
    KEY `idx_name` (`name`),
    FOREIGN KEY (`exchange`) REFERENCES `Exchange` (`code`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FinancialBalanceSheet`
(
    `FundamentalDataId`                                int unsigned                NOT NULL,
    `date`                                             date                        NOT NULL,
    `yearlyQuarterly`                                  enum ('YEARLY','QUARTERLY') NOT NULL,
    `filingDate`                                       date   DEFAULT NULL,
    `currencySymbol`                                   date   DEFAULT NULL,
    `totalAssets`                                      bigint DEFAULT NULL,
    `intangibleAssets`                                 bigint DEFAULT NULL,
    `earningAssets`                                    bigint DEFAULT NULL,
    `otherCurrentAssets`                               bigint DEFAULT NULL,
    `totalLiab`                                        bigint DEFAULT NULL,
    `totalStockholderEquity`                           bigint DEFAULT NULL,
    `deferredLongTermLiab`                             bigint DEFAULT NULL,
    `otherCurrentLiab`                                 bigint DEFAULT NULL,
    `commonStock`                                      bigint DEFAULT NULL,
    `capitalStock`                                     bigint DEFAULT NULL,
    `retainedEarnings`                                 bigint DEFAULT NULL,
    `otherLiab`                                        bigint DEFAULT NULL,
    `goodWill`                                         bigint DEFAULT NULL,
    `otherAssets`                                      bigint DEFAULT NULL,
    `cash`                                             bigint DEFAULT NULL,
    `cashAndEquivalents`                               bigint DEFAULT NULL,
    `totalCurrentLiabilities`                          bigint DEFAULT NULL,
    `currentDeferredRevenue`                           bigint DEFAULT NULL,
    `netDebt`                                          bigint DEFAULT NULL,
    `shortTermDebt`                                    bigint DEFAULT NULL,
    `shortLongTermDebt`                                bigint DEFAULT NULL,
    `shortLongTermDebtTotal`                           bigint DEFAULT NULL,
    `otherStockholderEquity`                           bigint DEFAULT NULL,
    `propertyPlantEquipment`                           bigint DEFAULT NULL,
    `totalCurrentAssets`                               bigint DEFAULT NULL,
    `longTermInvestments`                              bigint DEFAULT NULL,
    `netTangibleAssets`                                bigint DEFAULT NULL,
    `shortTermInvestments`                             bigint DEFAULT NULL,
    `netReceivables`                                   bigint DEFAULT NULL,
    `longTermDebt`                                     bigint DEFAULT NULL,
    `inventory`                                        bigint DEFAULT NULL,
    `accountsPayable`                                  bigint DEFAULT NULL,
    `totalPermanentEquity`                             bigint DEFAULT NULL,
    `noncontrollingInterestInConsolidatedEntity`       bigint DEFAULT NULL,
    `temporaryEquityRedeemableNoncontrollingInterests` bigint DEFAULT NULL,
    `accumulatedOtherComprehensiveIncome`              bigint DEFAULT NULL,
    `additionalPaidInCapital`                          bigint DEFAULT NULL,
    `commonStockTotalEquity`                           bigint DEFAULT NULL,
    `preferredStockTotalEquity`                        bigint DEFAULT NULL,
    `retainedEarningsTotalEquity`                      bigint DEFAULT NULL,
    `treasuryStock`                                    bigint DEFAULT NULL,
    `accumulatedAmortization`                          bigint DEFAULT NULL,
    `nonCurrentAssetsOther`                            bigint DEFAULT NULL,
    `deferredLongTermAssetCharges`                     bigint DEFAULT NULL,
    `nonCurrentAssetsTotal`                            bigint DEFAULT NULL,
    `capitalLeaseObligations`                          bigint DEFAULT NULL,
    `longTermDebtTotal`                                bigint DEFAULT NULL,
    `nonCurrentLiabilitiesOther`                       bigint DEFAULT NULL,
    `nonCurrentLiabilitiesTotal`                       bigint DEFAULT NULL,
    `negativeGoodwill`                                 bigint DEFAULT NULL,
    `warrants`                                         bigint DEFAULT NULL,
    `preferredStockRedeemable`                         bigint DEFAULT NULL,
    `capitalSurplus`                                   bigint DEFAULT NULL,
    `liabilitiesAndStockholdersEquity`                 bigint DEFAULT NULL,
    `cashAndShortTermInvestments`                      bigint DEFAULT NULL,
    `propertyPlantAndEquipmentGross`                   bigint DEFAULT NULL,
    `propertyPlantAndEquipmentNet`                     bigint DEFAULT NULL,
    `accumulatedDepreciation`                          bigint DEFAULT NULL,
    `netWorkingCapital`                                bigint DEFAULT NULL,
    `commonStockSharesOutstanding`                     bigint DEFAULT NULL,
    `netInvestedCapital`                               bigint DEFAULT NULL,
    PRIMARY KEY (`FundamentalDataId`, `date`, `yearlyQuarterly`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FinancialCashFlow`
(
    `FundamentalDataId`                     int unsigned                NOT NULL,
    `date`                                  date                        NOT NULL,
    `yearlyQuarterly`                       enum ('YEARLY','QUARTERLY') NOT NULL,
    `filingDate`                            date    DEFAULT NULL,
    `currencySymbol`                        date    DEFAULT NULL,
    `filing_date`                           date    DEFAULT NULL,
    `currency_symbol`                       char(3) DEFAULT NULL,
    `investments`                           bigint  DEFAULT NULL,
    `changeToLiabilities`                   bigint  DEFAULT NULL,
    `changeToOperatingActivities`           bigint  DEFAULT NULL,
    `totalCashflowsFromInvestingActivities` bigint  DEFAULT NULL,
    `netBorrowings`                         bigint  DEFAULT NULL,
    `totalCashFromFinancingActivities`      bigint  DEFAULT NULL,
    `netIncome`                             bigint  DEFAULT NULL,
    `changeInCash`                          bigint  DEFAULT NULL,
    `beginPeriodCashFlow`                   bigint  DEFAULT NULL,
    `endPeriodCashFlow`                     bigint  DEFAULT NULL,
    `issuanceOfCapitalStock`                bigint  DEFAULT NULL,
    `totalCashFromOperatingActivities`      bigint  DEFAULT NULL,
    `depreciation`                          bigint  DEFAULT NULL,
    `otherCashflowsFromInvestingActivities` bigint  DEFAULT NULL,
    `dividendsPaid`                         bigint  DEFAULT NULL,
    `changeToInventory`                     bigint  DEFAULT NULL,
    `changeToAccountReceivables`            bigint  DEFAULT NULL,
    `salePurchaseOfStock`                   bigint  DEFAULT NULL,
    `otherCashflowsFromFinancingActivities` bigint  DEFAULT NULL,
    `changeToNetincome`                     bigint  DEFAULT NULL,
    `capitalExpenditures`                   bigint  DEFAULT NULL,
    `changeReceivables`                     bigint  DEFAULT NULL,
    `cashFlowsOtherOperating`               bigint  DEFAULT NULL,
    `exchangeRateChanges`                   bigint  DEFAULT NULL,
    `cashAndCashEquivalentsChanges`         bigint  DEFAULT NULL,
    `changeInWorkingCapital`                bigint  DEFAULT NULL,
    `stockBasedCompensation`                bigint  DEFAULT NULL,
    `otherNonCashItems`                     bigint  DEFAULT NULL,
    `freeCashFlow`                          bigint  DEFAULT NULL,
    PRIMARY KEY (`FundamentalDataId`, `date`, `yearlyQuarterly`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FinancialIncomeStatement`
(
    `FundamentalDataId`                 int unsigned                NOT NULL,
    `date`                              date                        NOT NULL,
    `yearlyQuarterly`                   enum ('YEARLY','QUARTERLY') NOT NULL,
    `filingDate`                        date   DEFAULT NULL,
    `currencySymbol`                    date   DEFAULT NULL,
    `researchDevelopment`               bigint DEFAULT NULL,
    `effectOfAccountingCharges`         bigint DEFAULT NULL,
    `incomeBeforeTax`                   bigint DEFAULT NULL,
    `minorityInterest`                  bigint DEFAULT NULL,
    `netIncome`                         bigint DEFAULT NULL,
    `sellingGeneralAdministrative`      bigint DEFAULT NULL,
    `sellingAndMarketingExpenses`       bigint DEFAULT NULL,
    `grossProfit`                       bigint DEFAULT NULL,
    `reconciledDepreciation`            bigint DEFAULT NULL,
    `ebit`                              bigint DEFAULT NULL,
    `ebitda`                            bigint DEFAULT NULL,
    `depreciationAndAmortization`       bigint DEFAULT NULL,
    `nonOperatingIncomeNetOther`        bigint DEFAULT NULL,
    `operatingIncome`                   bigint DEFAULT NULL,
    `otherOperatingExpenses`            bigint DEFAULT NULL,
    `interestExpense`                   bigint DEFAULT NULL,
    `taxProvision`                      bigint DEFAULT NULL,
    `interestIncome`                    bigint DEFAULT NULL,
    `netInterestIncome`                 bigint DEFAULT NULL,
    `extraordinaryItems`                bigint DEFAULT NULL,
    `nonRecurring`                      bigint DEFAULT NULL,
    `otherItems`                        bigint DEFAULT NULL,
    `incomeTaxExpense`                  bigint DEFAULT NULL,
    `totalRevenue`                      bigint DEFAULT NULL,
    `totalOperatingExpenses`            bigint DEFAULT NULL,
    `costOfRevenue`                     bigint DEFAULT NULL,
    `totalOtherIncomeExpenseNet`        bigint DEFAULT NULL,
    `discontinuedOperations`            bigint DEFAULT NULL,
    `netIncomeFromContinuingOps`        bigint DEFAULT NULL,
    `netIncomeApplicableToCommonShares` bigint DEFAULT NULL,
    `preferredStockAndOtherAdjustments` bigint DEFAULT NULL,
    PRIMARY KEY (`FundamentalDataId`, `date`, `yearlyQuarterly`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FundamentalAnalystRating`
(
    `Id`                int unsigned NOT NULL AUTO_INCREMENT,
    `FundamentalDataId` int unsigned      DEFAULT NULL,
    `Rating`            float             DEFAULT NULL,
    `TargetPrice`       float             DEFAULT NULL,
    `StrongBuy`         tinyint           DEFAULT NULL,
    `Buy`               tinyint           DEFAULT NULL,
    `Hold`              tinyint           DEFAULT NULL,
    `Sell`              tinyint           DEFAULT NULL,
    `StrongSell`        tinyint           DEFAULT NULL,
    `Date`              timestamp    NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`Id`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FundamentalHighlight`
(
    `FundamentalDataId`          int unsigned NOT NULL,
    `MarketCapitalization`       bigint unsigned   DEFAULT NULL,
    `MarketCapitalizationMln`    int unsigned      DEFAULT NULL,
    `EBITDA`                     bigint            DEFAULT NULL,
    `PERatio`                    float             DEFAULT NULL,
    `PEGRatio`                   float             DEFAULT NULL,
    `WallStreetTargetPrice`      float             DEFAULT NULL,
    `BookValue`                  float             DEFAULT NULL,
    `DividendShare`              float             DEFAULT NULL,
    `DividendYield`              float             DEFAULT NULL,
    `EarningsShare`              float             DEFAULT NULL,
    `EPSEstimateCurrentYear`     float             DEFAULT NULL,
    `EPSEstimateNextYear`        float             DEFAULT NULL,
    `EPSEstimateNextQuarter`     float             DEFAULT NULL,
    `EPSEstimateCurrentQuarter`  float             DEFAULT NULL,
    `MostRecentQuarter`          date         NOT NULL,
    `ProfitMargin`               float             DEFAULT NULL,
    `OperatingMarginTTM`         float             DEFAULT NULL,
    `ReturnOnAssetsTTM`          float             DEFAULT NULL,
    `ReturnOnEquityTTM`          float             DEFAULT NULL,
    `RevenueTTM`                 bigint            DEFAULT NULL,
    `RevenuePerShareTTM`         float             DEFAULT NULL,
    `QuarterlyRevenueGrowthYOY`  float             DEFAULT NULL,
    `GrossProfitTTM`             bigint            DEFAULT NULL,
    `DilutedEpsTTM`              float             DEFAULT NULL,
    `QuarterlyEarningsGrowthYOY` float             DEFAULT NULL,
    `updated`                    timestamp    NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`FundamentalDataId`, `MostRecentQuarter`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FundamentalHolder`
(
    `Id`                int unsigned NOT NULL AUTO_INCREMENT,
    `FundamentalDataId` int unsigned                DEFAULT NULL,
    `name`              varchar(100)                DEFAULT NULL,
    `date`              date                        DEFAULT NULL,
    `totalShares`       float                       DEFAULT NULL,
    `totalAssets`       float                       DEFAULT NULL,
    `currentShares`     bigint                      DEFAULT NULL,
    `change`            int                         DEFAULT NULL,
    `change_p`          float                       DEFAULT NULL,
    `institutionFund`   enum ('INSTITUTION','FUND') DEFAULT NULL,
    PRIMARY KEY (`Id`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FundamentalListing`
(
    `Id`                int unsigned NOT NULL AUTO_INCREMENT,
    `FundamentalDataId` int unsigned NOT NULL,
    `Code`              varchar(15) DEFAULT NULL,
    `Exchange`          varchar(10) DEFAULT NULL,
    PRIMARY KEY (`Id`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`),
    FOREIGN KEY (`Exchange`) REFERENCES `Exchange` (`code`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FundamentalMetadata`
(
    `Id`                    int unsigned NOT NULL AUTO_INCREMENT,
    `Code`                  varchar(12)  NOT NULL,
    `Type`                  varchar(50)  NOT NULL,
    `Name`                  varchar(100) NOT NULL,
    `Exchange`              varchar(10)  NOT NULL,
    `CurrencyCode`          char(3)      NOT NULL,
    `CurrencyName`          varchar(50)  DEFAULT NULL,
    `CurrencySymbol`        varchar(6)   DEFAULT NULL,
    `CountryName`           varchar(50)  DEFAULT NULL,
    `CountryISO`            char(2)      DEFAULT NULL,
    `OpenFigi`              char(12)     DEFAULT NULL,
    `ISIN`                  char(12)     DEFAULT NULL,
    `LEI`                   char(20)     DEFAULT NULL,
    `PrimaryTicker`         varchar(15)  DEFAULT NULL,
    `CUSIP`                 char(9)      DEFAULT NULL,
    `CIK`                   char(8)      DEFAULT NULL,
    `EmployerIdNumber`      varchar(15)  DEFAULT NULL,
    `FiscalYearEnd`         varchar(9)   DEFAULT NULL,
    `IPODate`               char(10)     DEFAULT NULL,
    `InternationalDomestic` varchar(64)  DEFAULT NULL,
    `Sector`                varchar(64)  DEFAULT NULL,
    `Industry`              varchar(64)  DEFAULT NULL,
    `GicSector`             varchar(64)  DEFAULT NULL,
    `GicGroup`              varchar(98)  DEFAULT NULL,
    `GicIndustry`           varchar(100) DEFAULT NULL,
    `GicSubIndustry`        varchar(100) DEFAULT NULL,
    `HomeCategory`          varchar(50)  DEFAULT NULL,
    `IsDelisted`            tinyint(1)   DEFAULT NULL,
    `Description`           text,
    `Address`               varchar(100) DEFAULT NULL,
    `Street`                varchar(32)  DEFAULT NULL,
    `City`                  varchar(32)  DEFAULT NULL,
    `State`                 varchar(10)  DEFAULT NULL,
    `Country`               varchar(60)  DEFAULT NULL,
    `ZIP`                   varchar(10)  DEFAULT NULL,
    `Phone`                 varchar(17)  DEFAULT NULL,
    `WebURL`                varchar(50)  DEFAULT NULL,
    `LogoURL`               varchar(50)  DEFAULT NULL,
    `FullTimeEmployees`     int unsigned DEFAULT NULL,
    `UpdatedAt`             date         DEFAULT NULL,
    PRIMARY KEY (`Id`),
    UNIQUE KEY `Code` (`Code`, `Type`, `Name`, `Exchange`),
    FOREIGN KEY (`Code`) REFERENCES `ExchangeSymbol` (`code`),
    FOREIGN KEY (`Type`) REFERENCES `ExchangeSymbol` (`type`),
    FOREIGN KEY (`Name`) REFERENCES `ExchangeSymbol` (`name`),
    FOREIGN KEY (`Exchange`) REFERENCES `Exchange` (`code`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FundamentalNumberDividendsByYear`
(
    `Id`                int unsigned NOT NULL AUTO_INCREMENT,
    `FundamentalDataId` int unsigned NOT NULL,
    `Year`              year DEFAULT NULL,
    `Count`             int  DEFAULT NULL,
    PRIMARY KEY (`Id`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FundamentalOfficer`
(
    `Id`                int unsigned NOT NULL AUTO_INCREMENT,
    `FundamentalDataId` int unsigned NOT NULL,
    `Name`              varchar(100) DEFAULT NULL,
    `Title`             varchar(50)  DEFAULT NULL,
    `YearBorn`          char(4)      DEFAULT NULL,
    PRIMARY KEY (`Id`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FundamentalSharesStat`
(
    `FundamentalDataId`       int unsigned NOT NULL,
    `SharesOutstanding`       bigint DEFAULT NULL,
    `SharesFloat`             bigint DEFAULT NULL,
    `PercentInsiders`         double DEFAULT NULL,
    `PercentInstitutions`     float  DEFAULT NULL,
    `SharesShort`             bigint DEFAULT NULL,
    `SharesShortPriorMonth`   bigint DEFAULT NULL,
    `ShortRatio`              float  DEFAULT NULL,
    `ShortPercentOutstanding` float  DEFAULT NULL,
    `ShortPercentFloat`       float  DEFAULT NULL,
    PRIMARY KEY (`FundamentalDataId`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FundamentalSplitsDividend`
(
    `Id`                         int unsigned NOT NULL AUTO_INCREMENT,
    `FundamentalDataId`          int unsigned NOT NULL,
    `ForwardAnnualDividendRate`  float       DEFAULT NULL,
    `ForwardAnnualDividendYield` float       DEFAULT NULL,
    `PayoutRatio`                float       DEFAULT NULL,
    `DividendDate`               date        DEFAULT NULL,
    `ExDividendDate`             date        DEFAULT NULL,
    `LastSplitFactor`            varchar(10) DEFAULT NULL,
    `LastSplitDate`              date        DEFAULT NULL,
    PRIMARY KEY (`Id`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FundamentalTechnical`
(
    `FundamentalDataId`     int unsigned NOT NULL,
    `Beta`                  float  DEFAULT NULL,
    `52WeekHigh`            float  DEFAULT NULL,
    `52WeekLow`             float  DEFAULT NULL,
    `50DayMA`               float  DEFAULT NULL,
    `200DayMA`              float  DEFAULT NULL,
    `SharesShort`           bigint DEFAULT NULL,
    `SharesShortPriorMonth` bigint DEFAULT NULL,
    `ShortRatio`            float  DEFAULT NULL,
    `ShortPercent`          float  DEFAULT NULL,
    PRIMARY KEY (`FundamentalDataId`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `FundamentalValuation`
(
    `FundamentalDataId`      int unsigned NOT NULL,
    `TrailingPE`             float  DEFAULT NULL,
    `ForwardPE`              float  DEFAULT NULL,
    `PriceSalesTTM`          float  DEFAULT NULL,
    `PriceBookMRQ`           float  DEFAULT NULL,
    `EnterpriseValue`        bigint DEFAULT NULL,
    `EnterpriseValueRevenue` float  DEFAULT NULL,
    `EnterpriseValueEbitda`  float  DEFAULT NULL,
    PRIMARY KEY (`FundamentalDataId`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `HistoricalEarning`
(
    `FundamentalDataId` int unsigned NOT NULL,
    `reportDate`        date                                DEFAULT NULL,
    `date`              date         NOT NULL,
    `beforeAfterMarket` enum ('BeforeMarket','AfterMarket') DEFAULT NULL,
    `currency`          char(3)                             DEFAULT NULL,
    `epsActual`         float                               DEFAULT NULL,
    `epsEstimate`       float                               DEFAULT NULL,
    `epsDifference`     float                               DEFAULT NULL,
    `suprisePercent`    float                               DEFAULT NULL,
    PRIMARY KEY (`date`, `FundamentalDataId`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `InsiderTransaction`
(
    `Id`                          int unsigned NOT NULL AUTO_INCREMENT,
    `FundamentalDataId`           int unsigned DEFAULT NULL,
    `date`                        date         DEFAULT NULL,
    `ownerName`                   varchar(50)  DEFAULT NULL,
    `transactionDate`             date         DEFAULT NULL,
    `transactionCode`             varchar(5)   DEFAULT NULL,
    `transactionAmount`           int          DEFAULT NULL,
    `transactionPrice`            float        DEFAULT NULL,
    `transactionAcquiredDisposed` varchar(5)   DEFAULT NULL,
    `postTransactionAmount`       int          DEFAULT NULL,
    `secLink`                     varchar(150) DEFAULT NULL,
    PRIMARY KEY (`Id`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `OutstandingShare`
(
    `FundamentalDataId` int unsigned NOT NULL,
    `year`              year         NOT NULL,
    `quarter`           tinyint      NOT NULL,
    `dateFormatted`     date   DEFAULT NULL,
    `sharesMln`         double DEFAULT NULL,
    `shares`            bigint DEFAULT NULL,
    PRIMARY KEY (`FundamentalDataId`, `year`, `quarter`),
    FOREIGN KEY (`FundamentalDataId`) REFERENCES `FundamentalMetadata` (`Id`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;


CREATE TABLE `StockPrice`
(
    `code`      varchar(12) NOT NULL,
    `exchange`  varchar(10) NOT NULL,
    `timestamp` timestamp   NULL DEFAULT NULL,
    `gmtoffset` tinyint          DEFAULT NULL,
    `open`      float            DEFAULT NULL,
    `high`      float            DEFAULT NULL,
    `low`       float            DEFAULT NULL,
    `close`     float            DEFAULT NULL,
    `volume`    int              DEFAULT NULL,
    FOREIGN KEY (`code`) REFERENCES `ExchangeSymbol` (`code`),
    FOREIGN KEY (`exchange`) REFERENCES `Exchange` (`code`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = utf8mb4_0900_ai_ci;

CREATE TABLE `StockPriceEOD`
(
    `code`           varchar(12) NOT NULL,
    `exchange`       varchar(10) NOT NULL,
    `date`           date  DEFAULT NULL,
    `open`           float DEFAULT NULL,
    `high`           float DEFAULT NULL,
    `low`            float DEFAULT NULL,
    `close`          float DEFAULT NULL,
    `adjusted_close` float DEFAULT NULL,
    `volume`         int   DEFAULT NULL,
    FOREIGN KEY (`code`) REFERENCES `ExchangeSymbol` (`code`),
    FOREIGN KEY (`exchange`) REFERENCES `Exchange` (`code`)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = UTF8MB4_0900_AI_CI;

CREATE TABLE `NewsArticle`
(
    id       int unsigned auto_increment primary key not null,
    title    varchar(150),
    content  text                                    not null,
    `date`   datetime,
    link     varchar(150),

    polarity float,
    neg      float,
    neu      float,
    pos      float,

    UNIQUE (title)
) ENGINE = InnoDB
  DEFAULT CHARSET = utf8mb4
  COLLATE = UTF8MB4_0900_AI_CI;

CREATE TABLE `NewsSymbol`
(
    id       int unsigned auto_increment primary key not null,
    newsId   int unsigned                            not null,
    code     varchar(12)                             not null,
    exchange varchar(10)                             not null,

    FOREIGN KEY (code) REFERENCES ExchangeSymbol (code),
    FOREIGN KEY (exchange) REFERENCES Exchange (code),
    FOREIGN KEY (newsId) REFERENCES NewsArticle (id),
    UNIQUE (newsId, code, exchange)
);

CREATE TABLE `NewsUpdated`
(
    code          varchar(12),
    exchange      varchar(10),
    `lastUpdated` timestamp NULL DEFAULT CURRENT_TIMESTAMP,

    PRIMARY KEY (code, exchange),
    FOREIGN KEY (code) REFERENCES ExchangeSymbol (code),
    FOREIGN KEY (exchange) REFERENCES Exchange (code)
);

CREATE TABLE StageDone
(
    exchange    varchar(10),
    stage       ENUM ('INTRADAY','EOD','FUNDAMENTAL','NEWS'),
    lastUpdated TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    PRIMARY KEY (exchange, stage),
    FOREIGN KEY (exchange) REFERENCES Exchange (code)
);