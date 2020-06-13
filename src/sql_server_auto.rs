
use crate::{mmsdm::dispatch, GetTable};
use futures::{AsyncRead, AsyncWrite};

impl crate::AemoFile {
    /// This function is meant to be used in conjunction with the iterator over
    /// the data contained within the AemoFile struct
    pub async fn load_data<S>(&self, client: &mut tiberius::Client<S>) -> crate::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        for file_key in self.data.keys() {
            match (
                file_key.data_set_name.as_str(),
                file_key.table_name.as_str(),
                file_key.version,
            ) {
            
                    ("FORCE_MAJEURE","MARKET_SUSPEND_REGIME_SUM",1_i32) =>  {
                        let d: Vec<force_majeure::ForceMajeureMarketSuspendRegimeSum1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertForceMajeureMarketSuspendRegimeSum1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("AP","REGIONAPC",1_i32) =>  {
                        let d: Vec<force_majeure::ApRegionapc1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertApRegionapc1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("AP","APEVENTREGION",1_i32) =>  {
                        let d: Vec<force_majeure::ApApeventregion1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertApApeventregion1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("FORCE_MAJEURE","OVERRIDERRP",1_i32) =>  {
                        let d: Vec<force_majeure::ForceMajeureOverriderrp1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertForceMajeureOverriderrp1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("FORCE_MAJEURE","MARKET_SUSPEND_SCHEDULE",1_i32) =>  {
                        let d: Vec<force_majeure::ForceMajeureMarketSuspendSchedule1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertForceMajeureMarketSuspendSchedule1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("FORCE_MAJEURE","MARKET_SUSPEND_SCHEDULE_TRK",1_i32) =>  {
                        let d: Vec<force_majeure::ForceMajeureMarketSuspendScheduleTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertForceMajeureMarketSuspendScheduleTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("FORCE_MAJEURE","MARKET_SUSPEND_REGION_SUM",1_i32) =>  {
                        let d: Vec<force_majeure::ForceMajeureMarketSuspendRegionSum1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertForceMajeureMarketSuspendRegionSum1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("FORCE_MAJEURE","IRFMEVENTS",1_i32) =>  {
                        let d: Vec<force_majeure::ForceMajeureIrfmevents1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertForceMajeureIrfmevents1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("AP","REGIONAPCINTERVALS",1_i32) =>  {
                        let d: Vec<force_majeure::ApRegionapcintervals1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertApRegionapcintervals1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("FORCE_MAJEURE","IRFMAMOUNT",1_i32) =>  {
                        let d: Vec<force_majeure::ForceMajeureIrfmamount1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertForceMajeureIrfmamount1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("AP","APEVENT",1_i32) =>  {
                        let d: Vec<force_majeure::ApApevent1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertApApevent1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("STPASA","CASESOLUTION",3_i32) =>  {
                        let d: Vec<stpasa_solution::StpasaCasesolution3> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertStpasaCasesolution3 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("STPASA","INTERCONNECTORSOLN",2_i32) =>  {
                        let d: Vec<stpasa_solution::StpasaInterconnectorsoln2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertStpasaInterconnectorsoln2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("STPASA","CONSTRAINTSOLUTION",2_i32) =>  {
                        let d: Vec<stpasa_solution::StpasaConstraintsolution2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertStpasaConstraintsolution2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("STPASA","REGIONSOLUTION",5_i32) =>  {
                        let d: Vec<stpasa_solution::StpasaRegionsolution5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertStpasaRegionsolution5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("NETWORK","REALTIMERATING",1_i32) =>  {
                        let d: Vec<network::NetworkRealtimerating1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertNetworkRealtimerating1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("NETWORK","OUTAGECONSTRAINTSET",1_i32) =>  {
                        let d: Vec<network::NetworkOutageconstraintset1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertNetworkOutageconstraintset1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("NETWORK","STATICRATING",1_i32) =>  {
                        let d: Vec<network::NetworkStaticrating1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertNetworkStaticrating1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("NETWORK","OUTAGEDETAIL",3_i32) =>  {
                        let d: Vec<network::NetworkOutagedetail3> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertNetworkOutagedetail3 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("NETWORK","OUTAGESTATUSCODE",1_i32) =>  {
                        let d: Vec<network::NetworkOutagestatuscode1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertNetworkOutagestatuscode1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("NETWORK","EQUIPMENTDETAIL",1_i32) =>  {
                        let d: Vec<network::NetworkEquipmentdetail1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertNetworkEquipmentdetail1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("NETWORK","SUBSTATIONDETAIL",1_i32) =>  {
                        let d: Vec<network::NetworkSubstationdetail1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertNetworkSubstationdetail1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("NETWORK","RATING",1_i32) =>  {
                        let d: Vec<network::NetworkRating1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertNetworkRating1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("RESERVE_DATA","RESERVE",1_i32) =>  {
                        let d: Vec<reserve_data::ReserveDataReserve1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertReserveDataReserve1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MTPASA","RESERVELIMIT_SET",1_i32) =>  {
                        let d: Vec<reserve_data::MtpasaReservelimitSet1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMtpasaReservelimitSet1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MTPASA","RESERVELIMIT_REGION",1_i32) =>  {
                        let d: Vec<reserve_data::MtpasaReservelimitRegion1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMtpasaReservelimitRegion1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MTPASA","RESERVELIMIT",1_i32) =>  {
                        let d: Vec<reserve_data::MtpasaReservelimit1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMtpasaReservelimit1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","CONSTRAINT_SOLUTION",5_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchConstraintSolution5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchConstraintSolution5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","CASE_SOLUTION",1_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchCaseSolution1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchCaseSolution1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","SCENARIO_DEMAND",1_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchScenarioDemand1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchScenarioDemand1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","BLOCKED_CONSTRAINTS",1_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchBlockedConstraints1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchBlockedConstraints1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","OFFERTRK",1_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchOffertrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchOffertrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","PRICESENSITIVITIES",1_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchPricesensitivities1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchPricesensitivities1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","UNIT_SOLUTION",2_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchUnitSolution2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchUnitSolution2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","SCENARIO_DEMAND_TRK",1_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchScenarioDemandTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchScenarioDemandTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","MNSPBIDTRK",1_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchMnspbidtrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchMnspbidtrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","INTERCONNECTOR_SOLN",3_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchInterconnectorSoln3> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchInterconnectorSoln3 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","REGION_SOLUTION",4_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchRegionSolution4> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchRegionSolution4 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","REGION_PRICES",1_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchRegionPrices1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchRegionPrices1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","LOCAL_PRICE",1_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchLocalPrice1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchLocalPrice1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","INTERCONNECTR_SENS",1_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchInterconnectrSens1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchInterconnectrSens1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PREDISPATCH","REGIONFCASREQUIREMENT",2_i32) =>  {
                        let d: Vec<pre_dispatch::PredispatchRegionfcasrequirement2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPredispatchRegionfcasrequirement2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PRUDENTIAL","RUNTRK",1_i32) =>  {
                        let d: Vec<prudentials::PrudentialRuntrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPrudentialRuntrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PRUDENTIAL","COMPANY_POSITION",1_i32) =>  {
                        let d: Vec<prudentials::PrudentialCompanyPosition1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPrudentialCompanyPosition1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MCC","CONSTRAINTSOLUTION",1_i32) =>  {
                        let d: Vec<mcc_dispatch::MccConstraintsolution1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMccConstraintsolution1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MCC","CASESOLUTION",1_i32) =>  {
                        let d: Vec<mcc_dispatch::MccCasesolution1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMccCasesolution1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PDPASA","REGIONSOLUTION",5_i32) =>  {
                        let d: Vec<pdpasa::PdpasaRegionsolution5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPdpasaRegionsolution5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PDPASA","CASESOLUTION",3_i32) =>  {
                        let d: Vec<pdpasa::PdpasaCasesolution3> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPdpasaCasesolution3 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("TRADING","REGIONSUM",4_i32) =>  {
                        let d: Vec<trading_data::TradingRegionsum4> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertTradingRegionsum4 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("TRADING","UNIT_SOLUTION",2_i32) =>  {
                        let d: Vec<trading_data::TradingUnitSolution2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertTradingUnitSolution2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("TRADING","PRICE",2_i32) =>  {
                        let d: Vec<trading_data::TradingPrice2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertTradingPrice2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("TRADING","INTERCONNECTORRES",2_i32) =>  {
                        let d: Vec<trading_data::TradingInterconnectorres2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertTradingInterconnectorres2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("GD_INSTRUCT","GDINSTRUCT",1_i32) =>  {
                        let d: Vec<gd_instruct::GdInstructGdinstruct1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertGdInstructGdinstruct1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("GD_INSTRUCT","INSTRUCTIONTYPE",1_i32) =>  {
                        let d: Vec<gd_instruct::GdInstructInstructiontype1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertGdInstructInstructiontype1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("GD_INSTRUCT","INSTRUCTIONSUBTYPE",1_i32) =>  {
                        let d: Vec<gd_instruct::GdInstructInstructionsubtype1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertGdInstructInstructionsubtype1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("P5MIN","CASESOLUTION",2_i32) =>  {
                        let d: Vec<p5min::P5minCasesolution2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertP5minCasesolution2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("P5MIN","LOCAL_PRICE",1_i32) =>  {
                        let d: Vec<p5min::P5minLocalPrice1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertP5minLocalPrice1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("P5MIN","CONSTRAINTSOLUTION",6_i32) =>  {
                        let d: Vec<p5min::P5minConstraintsolution6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertP5minConstraintsolution6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("P5MIN","UNITSOLUTION",3_i32) =>  {
                        let d: Vec<p5min::P5minUnitsolution3> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertP5minUnitsolution3 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("P5MIN","BLOCKED_CONSTRAINTS",1_i32) =>  {
                        let d: Vec<p5min::P5minBlockedConstraints1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertP5minBlockedConstraints1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("P5MIN","REGIONSOLUTION",5_i32) =>  {
                        let d: Vec<p5min::P5minRegionsolution5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertP5minRegionsolution5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("P5MIN","INTERCONNECTORSOLN",4_i32) =>  {
                        let d: Vec<p5min::P5minInterconnectorsoln4> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertP5minInterconnectorsoln4 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","TRANSMISSIONLOSSFACTOR",2_i32) =>  {
                        let d: Vec<market_config::MarketConfigTransmissionlossfactor2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigTransmissionlossfactor2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","INTERCONNECTOR",1_i32) =>  {
                        let d: Vec<market_config::MarketConfigInterconnector1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigInterconnector1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","BIDTYPES",1_i32) =>  {
                        let d: Vec<market_config::MarketConfigBidtypes1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigBidtypes1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","INTERCONNECTORCONSTRAINT",1_i32) =>  {
                        let d: Vec<market_config::MarketConfigInterconnectorconstraint1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigInterconnectorconstraint1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","LOSSMODEL",1_i32) =>  {
                        let d: Vec<market_config::MarketConfigLossmodel1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigLossmodel1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","LOSSFACTORMODEL",1_i32) =>  {
                        let d: Vec<market_config::MarketConfigLossfactormodel1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigLossfactormodel1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","BIDTYPESTRK",1_i32) =>  {
                        let d: Vec<market_config::MarketConfigBidtypestrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigBidtypestrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","REGION",1_i32) =>  {
                        let d: Vec<market_config::MarketConfigRegion1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigRegion1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","REGIONSTANDINGDATA",1_i32) =>  {
                        let d: Vec<market_config::MarketConfigRegionstandingdata1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigRegionstandingdata1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","INTRAREGIONALLOC",1_i32) =>  {
                        let d: Vec<market_config::MarketConfigIntraregionalloc1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigIntraregionalloc1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","MARKET_PRICE_THRESHOLDS",1_i32) =>  {
                        let d: Vec<market_config::MarketConfigMarketPriceThresholds1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigMarketPriceThresholds1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_CONFIG","INTERCONNECTORALLOC",1_i32) =>  {
                        let d: Vec<market_config::MarketConfigInterconnectoralloc1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketConfigInterconnectoralloc1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("VOLTAGE_INSTRUCTION","INSTRUCTION",2_i32) =>  {
                        let d: Vec<voltage_instructions::VoltageInstructionInstruction2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertVoltageInstructionInstruction2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("VOLTAGE_INSTRUCTION","TRACK",2_i32) =>  {
                        let d: Vec<voltage_instructions::VoltageInstructionTrack2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertVoltageInstructionTrack2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","APC_RECOVERY",1_i32) =>  {
                        let d: Vec<settlement_data::SettlementsApcRecovery1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsApcRecovery1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","FCAS_RECOVERY",6_i32) =>  {
                        let d: Vec<settlement_data::SettlementsFcasRecovery6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsFcasRecovery6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","SMALLGENDATA",1_i32) =>  {
                        let d: Vec<settlement_data::SettlementsSmallgendata1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsSmallgendata1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","FCASREGIONRECOVERY",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsFcasregionrecovery5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsFcasregionrecovery5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","LSHEDPAYMENT",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsLshedpayment5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsLshedpayment5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","REALLOCATIONS",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsReallocations5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsReallocations5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","AGCRECOVERY",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsAgcrecovery5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsAgcrecovery5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","CPDATAREGION",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsCpdataregion5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsCpdataregion5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","GENDATA",6_i32) =>  {
                        let d: Vec<settlement_data::SettlementsGendata6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsGendata6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","MR_PAYMENT",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsMrPayment5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsMrPayment5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","INTERVENTIONRECOVERY",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsInterventionrecovery5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsInterventionrecovery5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","INTRAREGIONRESIDUES",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsIntraregionresidues5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsIntraregionresidues5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","RESTARTPAYMENT",6_i32) =>  {
                        let d: Vec<settlement_data::SettlementsRestartpayment6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsRestartpayment6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","APC_COMPENSATION",1_i32) =>  {
                        let d: Vec<settlement_data::SettlementsApcCompensation1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsApcCompensation1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","AGCPAYMENT",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsAgcpayment5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsAgcpayment5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","SET_FCAS_REGULATION_TRK",1_i32) =>  {
                        let d: Vec<settlement_data::SettlementsSetFcasRegulationTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsSetFcasRegulationTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","RUN_PARAMETER",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsRunParameter5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsRunParameter5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","GENDATAREGION",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsGendataregion5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsGendataregion5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","IRAUCSURPLUS",6_i32) =>  {
                        let d: Vec<settlement_data::SettlementsIraucsurplus6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsIraucsurplus6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","IRNSPSURPLUS",6_i32) =>  {
                        let d: Vec<settlement_data::SettlementsIrnspsurplus6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsIrnspsurplus6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","MARKETFEES",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsMarketfees5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsMarketfees5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","VICBOUNDARYENERGY",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsVicboundaryenergy5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsVicboundaryenergy5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","LULOADRECOVERY",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsLuloadrecovery5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsLuloadrecovery5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","VICENERGYFLOW",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsVicenergyflow5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsVicenergyflow5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","FCASCOMP",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsFcascomp5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsFcascomp5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","LSHEDRECOVERY",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsLshedrecovery5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsLshedrecovery5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","NMAS_RECOVERY_RBF",1_i32) =>  {
                        let d: Vec<settlement_data::SettlementsNmasRecoveryRbf1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsNmasRecoveryRbf1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","NMAS_RECOVERY",2_i32) =>  {
                        let d: Vec<settlement_data::SettlementsNmasRecovery2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsNmasRecovery2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","VICENERGYFIGURES",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsVicenergyfigures5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsVicenergyfigures5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","IRSURPLUS",6_i32) =>  {
                        let d: Vec<settlement_data::SettlementsIrsurplus6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsIrsurplus6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","FCAS_PAYMENT",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsFcasPayment5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsFcasPayment5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","INTERVENTION",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsIntervention5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsIntervention5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","RESTARTRECOVERY",6_i32) =>  {
                        let d: Vec<settlement_data::SettlementsRestartrecovery6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsRestartrecovery6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","RPOWERRECOVERY",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsRpowerrecovery5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsRpowerrecovery5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","LUNLOADRECOVERY",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsLunloadrecovery5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsLunloadrecovery5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","IRFMRECOVERY",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsIrfmrecovery5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsIrfmrecovery5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","ANCILLARY_SUMMARY",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsAncillarySummary5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsAncillarySummary5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","LUNLOADPAYMENT",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsLunloadpayment5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsLunloadpayment5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","CPDATA",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsCpdata5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsCpdata5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","IRPARTSURPLUS",6_i32) =>  {
                        let d: Vec<settlement_data::SettlementsIrpartsurplus6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsIrpartsurplus6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","RPOWERPAYMENT",6_i32) =>  {
                        let d: Vec<settlement_data::SettlementsRpowerpayment6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsRpowerpayment6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","MR_RECOVERY",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsMrRecovery5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsMrRecovery5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENTS","DAYTRACK",5_i32) =>  {
                        let d: Vec<settlement_data::SettlementsDaytrack5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementsDaytrack5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BID","BIDPEROFFER_D",2_i32) =>  {
                        let d: Vec<bids::BidBidperofferD2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBidBidperofferD2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BID","MNSP_FILETRK",1_i32) =>  {
                        let d: Vec<bids::BidMnspFiletrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBidMnspFiletrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("OFFER","MNSP_OFFERTRK",1_i32) =>  {
                        let d: Vec<bids::OfferMnspOffertrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertOfferMnspOffertrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("OFFER","MTPASA_OFFERDATA",1_i32) =>  {
                        let d: Vec<bids::OfferMtpasaOfferdata1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertOfferMtpasaOfferdata1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BID","BIDDAYOFFER_D",2_i32) =>  {
                        let d: Vec<bids::BidBiddayofferD2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBidBiddayofferD2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("OFFER","BIDDAYOFFER",2_i32) =>  {
                        let d: Vec<bids::OfferBiddayoffer2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertOfferBiddayoffer2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("OFFER","MTPASA_OFFERFILETRK",1_i32) =>  {
                        let d: Vec<bids::OfferMtpasaOfferfiletrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertOfferMtpasaOfferfiletrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("OFFER","BIDOFFERFILETRK",1_i32) =>  {
                        let d: Vec<bids::OfferBidofferfiletrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertOfferBidofferfiletrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("OFFER","MNSP_DAYOFFER",2_i32) =>  {
                        let d: Vec<bids::OfferMnspDayoffer2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertOfferMnspDayoffer2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("OFFER","BIDPEROFFER",1_i32) =>  {
                        let d: Vec<bids::OfferBidperoffer1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertOfferBidperoffer1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("OFFER","MNSP_PEROFFER",1_i32) =>  {
                        let d: Vec<bids::OfferMnspPeroffer1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertOfferMnspPeroffer1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("GENERIC_CONSTRAINT","EMSMASTER",1_i32) =>  {
                        let d: Vec<generic_constraint::GenericConstraintEmsmaster1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertGenericConstraintEmsmaster1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("GENCONSET","NULL",1_i32) =>  {
                        let d: Vec<generic_constraint::GenconsetNull1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertGenconsetNull1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("GCRHS","NULL",1_i32) =>  {
                        let d: Vec<generic_constraint::GcrhsNull1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertGcrhsNull1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("GENERIC_CONSTRAINT","GENCONSETINVOKE",2_i32) =>  {
                        let d: Vec<generic_constraint::GenericConstraintGenconsetinvoke2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertGenericConstraintGenconsetinvoke2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("GENCONSETTRK","NULL",2_i32) =>  {
                        let d: Vec<generic_constraint::GenconsettrkNull2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertGenconsettrkNull2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("GEQDESC","NULL",2_i32) =>  {
                        let d: Vec<generic_constraint::GeqdescNull2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertGeqdescNull2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("GEQRHS","NULL",1_i32) =>  {
                        let d: Vec<generic_constraint::GeqrhsNull1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertGeqrhsNull1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SPDRC","NULL",2_i32) =>  {
                        let d: Vec<generic_constraint::SpdrcNull2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSpdrcNull2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SPDCPC","NULL",2_i32) =>  {
                        let d: Vec<generic_constraint::SpdcpcNull2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSpdcpcNull2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SPDICC","NULL",1_i32) =>  {
                        let d: Vec<generic_constraint::SpdiccNull1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSpdiccNull1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("GENCONDATA","NULL",6_i32) =>  {
                        let d: Vec<generic_constraint::GencondataNull6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertGencondataNull6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("FORECAST","INTERMITTENT_GEN_DATA",1_i32) =>  {
                        let d: Vec<demand_forecasts::ForecastIntermittentGenData1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertForecastIntermittentGenData1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DEMAND","MTPASA_INTERMITTENT_LIMIT",1_i32) =>  {
                        let d: Vec<demand_forecasts::DemandMtpasaIntermittentLimit1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDemandMtpasaIntermittentLimit1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("FORECAST","INTERMITTENT_GEN",1_i32) =>  {
                        let d: Vec<demand_forecasts::ForecastIntermittentGen1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertForecastIntermittentGen1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DEMAND","MTPASA_INTERMITTENT_AVAIL",1_i32) =>  {
                        let d: Vec<demand_forecasts::DemandMtpasaIntermittentAvail1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDemandMtpasaIntermittentAvail1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("ROOFTOP","FORECAST",1_i32) =>  {
                        let d: Vec<demand_forecasts::RooftopForecast1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertRooftopForecast1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("OPERATIONAL_DEMAND","ACTUAL",1_i32) =>  {
                        let d: Vec<demand_forecasts::OperationalDemandActual1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertOperationalDemandActual1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DEMAND","INTERMITTENT_CLUSTER_AVAIL",1_i32) =>  {
                        let d: Vec<demand_forecasts::DemandIntermittentClusterAvail1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDemandIntermittentClusterAvail1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DEMAND","INTERMITTENT_DS_PRED",1_i32) =>  {
                        let d: Vec<demand_forecasts::DemandIntermittentDsPred1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDemandIntermittentDsPred1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DEMAND","TRK",1_i32) =>  {
                        let d: Vec<demand_forecasts::DemandTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDemandTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DEMAND","INTERMITTENT_CLUSTER_AVAIL_DAY",1_i32) =>  {
                        let d: Vec<demand_forecasts::DemandIntermittentClusterAvailDay1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDemandIntermittentClusterAvailDay1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DEMAND","PERIOD",1_i32) =>  {
                        let d: Vec<demand_forecasts::DemandPeriod1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDemandPeriod1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DEMAND","INTERMITTENT_GEN_LIMIT",1_i32) =>  {
                        let d: Vec<demand_forecasts::DemandIntermittentGenLimit1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDemandIntermittentGenLimit1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DEMAND","INTERMITTENT_GEN_LIMIT_DAY",1_i32) =>  {
                        let d: Vec<demand_forecasts::DemandIntermittentGenLimitDay1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDemandIntermittentGenLimitDay1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("OPERATIONAL_DEMAND","FORECAST",1_i32) =>  {
                        let d: Vec<demand_forecasts::OperationalDemandForecast1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertOperationalDemandForecast1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DEMAND","INTERMITTENT_DS_RUN",1_i32) =>  {
                        let d: Vec<demand_forecasts::DemandIntermittentDsRun1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDemandIntermittentDsRun1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("ROOFTOP","ACTUAL",2_i32) =>  {
                        let d: Vec<demand_forecasts::RooftopActual2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertRooftopActual2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("ASOFFER","OFFERRESTARTDATA",1_i32) =>  {
                        let d: Vec<asoffer::AsofferOfferrestartdata1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertAsofferOfferrestartdata1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("ASOFFER","OFFERRPOWERDATA",1_i32) =>  {
                        let d: Vec<asoffer::AsofferOfferrpowerdata1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertAsofferOfferrpowerdata1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("ASOFFER","OFFERAGCDATA",1_i32) =>  {
                        let d: Vec<asoffer::AsofferOfferagcdata1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertAsofferOfferagcdata1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("ASOFFER","OFFERASTRK",1_i32) =>  {
                        let d: Vec<asoffer::AsofferOfferastrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertAsofferOfferastrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("ASOFFER","OFFERLSHEDDATA",1_i32) =>  {
                        let d: Vec<asoffer::AsofferOfferlsheddata1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertAsofferOfferlsheddata1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING_CONFIG","SECDEPOSIT_PROVISION",1_i32) =>  {
                        let d: Vec<billing_config::BillingConfigSecdepositProvision1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingConfigSecdepositProvision1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING_CONFIG","GST_BAS_CLASS",1_i32) =>  {
                        let d: Vec<billing_config::BillingConfigGstBasClass1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingConfigGstBasClass1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING_CONFIG","GST_RATE",1_i32) =>  {
                        let d: Vec<billing_config::BillingConfigGstRate1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingConfigGstRate1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING_CONFIG","GST_TRANSACTION_TYPE",1_i32) =>  {
                        let d: Vec<billing_config::BillingConfigGstTransactionType1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingConfigGstTransactionType1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING_CONFIG","BILLINGCALENDAR",2_i32) =>  {
                        let d: Vec<billing_config::BillingConfigBillingcalendar2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingConfigBillingcalendar2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING_CONFIG","GST_TRANSACTION_CLASS",1_i32) =>  {
                        let d: Vec<billing_config::BillingConfigGstTransactionClass1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingConfigGstTransactionClass1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING_CONFIG","SECDEPOSIT_INTEREST_RATE",1_i32) =>  {
                        let d: Vec<billing_config::BillingConfigSecdepositInterestRate1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingConfigSecdepositInterestRate1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","SECDEP_INTEREST_RATE",1_i32) =>  {
                        let d: Vec<billing_run::BillingSecdepInterestRate1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingSecdepInterestRate1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","MR_SUMMARY",5_i32) =>  {
                        let d: Vec<billing_run::BillingMrSummary5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingMrSummary5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","APC_RECOVERY",2_i32) =>  {
                        let d: Vec<billing_run::BillingApcRecovery2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingApcRecovery2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","WHITEHOLE",5_i32) =>  {
                        let d: Vec<billing_run::BillingWhitehole5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingWhitehole5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","NMAS_TST_PAYMENTS",1_i32) =>  {
                        let d: Vec<billing_run::BillingNmasTstPayments1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingNmasTstPayments1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","RES_TRADER_PAYMENT",1_i32) =>  {
                        let d: Vec<billing_run::BillingResTraderPayment1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingResTraderPayment1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","IRNSPSURPLUS",5_i32) =>  {
                        let d: Vec<billing_run::BillingIrnspsurplus5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingIrnspsurplus5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","IRAUCSURPLUS",5_i32) =>  {
                        let d: Vec<billing_run::BillingIraucsurplus5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingIraucsurplus5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","INTERRESIDUES",5_i32) =>  {
                        let d: Vec<billing_run::BillingInterresidues5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingInterresidues5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","EFTSHORTFALL_DETAIL",1_i32) =>  {
                        let d: Vec<billing_run::BillingEftshortfallDetail1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingEftshortfallDetail1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","RES_TRADER_RECOVERY",1_i32) =>  {
                        let d: Vec<billing_run::BillingResTraderRecovery1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingResTraderRecovery1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","IRAUCSURPLUSSUM",7_i32) =>  {
                        let d: Vec<billing_run::BillingIraucsurplussum7> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingIraucsurplussum7 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","GENDATA",5_i32) =>  {
                        let d: Vec<billing_run::BillingGendata5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingGendata5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","REALLOC_DETAIL",5_i32) =>  {
                        let d: Vec<billing_run::BillingReallocDetail5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingReallocDetail5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","SECDEPOSIT_APPLICATION",1_i32) =>  {
                        let d: Vec<billing_run::BillingSecdepositApplication1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingSecdepositApplication1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","NMAS_TST_RECOVERY",1_i32) =>  {
                        let d: Vec<billing_run::BillingNmasTstRecovery1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingNmasTstRecovery1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","REGIONFIGURES",5_i32) =>  {
                        let d: Vec<billing_run::BillingRegionfigures5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingRegionfigures5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","REALLOC",5_i32) =>  {
                        let d: Vec<billing_run::BillingRealloc5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingRealloc5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","PRIORADJUSTMENTS",5_i32) =>  {
                        let d: Vec<billing_run::BillingPrioradjustments5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingPrioradjustments5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","ASRECOVERY",7_i32) =>  {
                        let d: Vec<billing_run::BillingAsrecovery7> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingAsrecovery7 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","ASPAYMENTS",6_i32) =>  {
                        let d: Vec<billing_run::BillingAspayments6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingAspayments6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","DIRECTION_RECONCILIATN",1_i32) =>  {
                        let d: Vec<billing_run::BillingDirectionReconciliatn1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingDirectionReconciliatn1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","BILLING_CO2E_PUBLICATION",1_i32) =>  {
                        let d: Vec<billing_run::BillingBillingCo2ePublication1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingBillingCo2ePublication1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","EFTSHORTFALL_AMOUNT",1_i32) =>  {
                        let d: Vec<billing_run::BillingEftshortfallAmount1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingEftshortfallAmount1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","IRPARTSURPLUSSUM",7_i32) =>  {
                        let d: Vec<billing_run::BillingIrpartsurplussum7> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingIrpartsurplussum7 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","NMAS_TST_RECVRY_RBF",1_i32) =>  {
                        let d: Vec<billing_run::BillingNmasTstRecvryRbf1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingNmasTstRecvryRbf1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","NMAS_TST_RECVRY_TRK",1_i32) =>  {
                        let d: Vec<billing_run::BillingNmasTstRecvryTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingNmasTstRecvryTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","APC_COMPENSATION",2_i32) =>  {
                        let d: Vec<billing_run::BillingApcCompensation2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingApcCompensation2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","BILLING_CO2E_PUBLICATION_TRK",1_i32) =>  {
                        let d: Vec<billing_run::BillingBillingCo2ePublicationTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingBillingCo2ePublicationTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","IRFM",5_i32) =>  {
                        let d: Vec<billing_run::BillingIrfm5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingIrfm5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","REGIONEXPORTS",5_i32) =>  {
                        let d: Vec<billing_run::BillingRegionexports5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingRegionexports5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","IRPARTSURPLUS",5_i32) =>  {
                        let d: Vec<billing_run::BillingIrpartsurplus5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingIrpartsurplus5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","GST_DETAIL",5_i32) =>  {
                        let d: Vec<billing_run::BillingGstDetail5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingGstDetail5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","FINANCIALADJUSTMENTS",5_i32) =>  {
                        let d: Vec<billing_run::BillingFinancialadjustments5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingFinancialadjustments5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","BILLING_DIRECTION_RECON_OTHER",1_i32) =>  {
                        let d: Vec<billing_run::BillingBillingDirectionReconOther1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingBillingDirectionReconOther1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","SMELTERREDUCTION",5_i32) =>  {
                        let d: Vec<billing_run::BillingSmelterreduction5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingSmelterreduction5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","GST_SUMMARY",5_i32) =>  {
                        let d: Vec<billing_run::BillingGstSummary5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingGstSummary5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","INTRARESIDUES",5_i32) =>  {
                        let d: Vec<billing_run::BillingIntraresidues5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingIntraresidues5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","CPDATA",5_i32) =>  {
                        let d: Vec<billing_run::BillingCpdata5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingCpdata5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","MR_PAYMENT",5_i32) =>  {
                        let d: Vec<billing_run::BillingMrPayment5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingMrPayment5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","MR_SHORTFALL",5_i32) =>  {
                        let d: Vec<billing_run::BillingMrShortfall5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingMrShortfall5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","DAYTRK",5_i32) =>  {
                        let d: Vec<billing_run::BillingDaytrk5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingDaytrk5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","FEES",5_i32) =>  {
                        let d: Vec<billing_run::BillingFees5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingFees5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","SECDEP_INTEREST_PAY",1_i32) =>  {
                        let d: Vec<billing_run::BillingSecdepInterestPay1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingSecdepInterestPay1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","REGIONIMPORTS",5_i32) =>  {
                        let d: Vec<billing_run::BillingRegionimports5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingRegionimports5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","DAILY_ENERGY_SUMMARY",1_i32) =>  {
                        let d: Vec<billing_run::BillingDailyEnergySummary1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingDailyEnergySummary1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","RUNTRK",5_i32) =>  {
                        let d: Vec<billing_run::BillingRuntrk5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingRuntrk5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","MR_RECOVERY",5_i32) =>  {
                        let d: Vec<billing_run::BillingMrRecovery5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingMrRecovery5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("BILLING","IRNSPSURPLUSSUM",6_i32) =>  {
                        let d: Vec<billing_run::BillingIrnspsurplussum6> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertBillingIrnspsurplussum6 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","RESIDUE_PUBLIC_DATA",1_i32) =>  {
                        let d: Vec<irauction::IrauctionResiduePublicData1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionResiduePublicData1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","RESIDUE_TRK",1_i32) =>  {
                        let d: Vec<irauction::IrauctionResidueTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionResidueTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_FINANCIAL_RUNTRK",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraFinancialRuntrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraFinancialRuntrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_OFFER_PRODUCT",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraOfferProduct1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraOfferProduct1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","RESIDUE_CON_DATA",2_i32) =>  {
                        let d: Vec<irauction::IrauctionResidueConData2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionResidueConData2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_CASH_SECURITY",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraCashSecurity1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraCashSecurity1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_FINANCIAL_AUC_MARGIN",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraFinancialAucMargin1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraFinancialAucMargin1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_OFFER_PROFILE",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraOfferProfile1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraOfferProfile1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION_CONFIG","AUCTION_IC_ALLOCATIONS",2_i32) =>  {
                        let d: Vec<irauction::IrauctionConfigAuctionIcAllocations2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionConfigAuctionIcAllocations2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","RESIDUE_CON_FUNDS",1_i32) =>  {
                        let d: Vec<irauction::IrauctionResidueConFunds1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionResidueConFunds1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_FINANCIAL_AUC_RECEIPTS",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraFinancialAucReceipts1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraFinancialAucReceipts1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION_CONFIG","AUCTION_RP_ESTIMATE",1_i32) =>  {
                        let d: Vec<irauction::IrauctionConfigAuctionRpEstimate1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionConfigAuctionRpEstimate1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION_BIDS","FUNDS_BID",1_i32) =>  {
                        let d: Vec<irauction::IrauctionBidsFundsBid1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionBidsFundsBid1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_PRUDENTIAL_COMP_POSITION",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraPrudentialCompPosition1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraPrudentialCompPosition1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_FINANCIAL_AUCPAY_SUM",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraFinancialAucpaySum1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraFinancialAucpaySum1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_FINANCIAL_AUCPAY_DETAIL",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraFinancialAucpayDetail1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraFinancialAucpayDetail1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","RESIDUE_BID_TRK",1_i32) =>  {
                        let d: Vec<irauction::IrauctionResidueBidTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionResidueBidTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION_CONFIG","AUCTION_TRANCHE",1_i32) =>  {
                        let d: Vec<irauction::IrauctionConfigAuctionTranche1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionConfigAuctionTranche1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_PRUDENTIAL_RUN",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraPrudentialRun1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraPrudentialRun1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION_CONFIG","AUCTION_CALENDAR",2_i32) =>  {
                        let d: Vec<irauction::IrauctionConfigAuctionCalendar2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionConfigAuctionCalendar2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION_BIDS","FILE_TRK",1_i32) =>  {
                        let d: Vec<irauction::IrauctionBidsFileTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionBidsFileTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_FINANCIAL_AUC_MARDETAIL",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraFinancialAucMardetail1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraFinancialAucMardetail1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","RESIDUE_CONTRACTS",1_i32) =>  {
                        let d: Vec<irauction::IrauctionResidueContracts1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionResidueContracts1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","RESIDUE_PRICE_FUNDS_BID",1_i32) =>  {
                        let d: Vec<irauction::IrauctionResiduePriceFundsBid1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionResiduePriceFundsBid1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION_CONFIG","AUCTION_REVENUE_ESTIMATE",1_i32) =>  {
                        let d: Vec<irauction::IrauctionConfigAuctionRevenueEstimate1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionConfigAuctionRevenueEstimate1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION_CONFIG","AUCTION_REVENUE_TRACK",1_i32) =>  {
                        let d: Vec<irauction::IrauctionConfigAuctionRevenueTrack1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionConfigAuctionRevenueTrack1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION_CONFIG","AUCTION",1_i32) =>  {
                        let d: Vec<irauction::IrauctionConfigAuction1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionConfigAuction1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","RESIDUECONTRACTPAYMENTS",1_i32) =>  {
                        let d: Vec<irauction::SettlementConfigResiduecontractpayments1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigResiduecontractpayments1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION_BIDS","PRICE_BID",1_i32) =>  {
                        let d: Vec<irauction::IrauctionBidsPriceBid1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionBidsPriceBid1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","VALUATIONID",1_i32) =>  {
                        let d: Vec<irauction::IrauctionValuationid1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionValuationid1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_PRUDENTIAL_EXPOSURE",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraPrudentialExposure1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraPrudentialExposure1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","SRA_PRUDENTIAL_CASH_SECURITY",1_i32) =>  {
                        let d: Vec<irauction::IrauctionSraPrudentialCashSecurity1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionSraPrudentialCashSecurity1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("IRAUCTION","RESIDUE_CON_ESTIMATES_TRK",1_i32) =>  {
                        let d: Vec<irauction::IrauctionResidueConEstimatesTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertIrauctionResidueConEstimatesTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_NOTICE","MARKETNOTICEDATA",1_i32) =>  {
                        let d: Vec<market_notice::MarketNoticeMarketnoticedata1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketNoticeMarketnoticedata1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_NOTICE","MARKETNOTICETYPE",1_i32) =>  {
                        let d: Vec<market_notice::MarketNoticeMarketnoticetype1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketNoticeMarketnoticetype1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MARKET_NOTICE","PARTICIPANTNOTICETRK",1_i32) =>  {
                        let d: Vec<market_notice::MarketNoticeParticipantnoticetrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMarketNoticeParticipantnoticetrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PRICELOAD","CONSTRAINTRELAXATION",1_i32) =>  {
                        let d: Vec<dispatch::PriceloadConstraintrelaxation1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPriceloadConstraintrelaxation1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","REGIONSUM",4_i32) =>  {
                        let d: Vec<dispatch::DispatchRegionsum4> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchRegionsum4 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","INTERCONNECTORRES",3_i32) =>  {
                        let d: Vec<dispatch::DispatchInterconnectorres3> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchInterconnectorres3 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","OFFERTRK",1_i32) =>  {
                        let d: Vec<dispatch::DispatchOffertrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchOffertrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PRICELOAD","PRICE_REVISION",1_i32) =>  {
                        let d: Vec<dispatch::PriceloadPriceRevision1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPriceloadPriceRevision1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","UNIT_SCADA",1_i32) =>  {
                        let d: Vec<dispatch::DispatchUnitScada1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchUnitScada1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","MR_SCHEDULE_TRK",1_i32) =>  {
                        let d: Vec<dispatch::DispatchMrScheduleTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchMrScheduleTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","NEGATIVE_RESIDUE",1_i32) =>  {
                        let d: Vec<dispatch::DispatchNegativeResidue1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchNegativeResidue1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","LOCAL_PRICE",1_i32) =>  {
                        let d: Vec<dispatch::DispatchLocalPrice1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchLocalPrice1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","UNIT_CONFORMANCE",1_i32) =>  {
                        let d: Vec<dispatch::DispatchUnitConformance1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchUnitConformance1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","FCAS_REQ",2_i32) =>  {
                        let d: Vec<dispatch::DispatchFcasReq2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchFcasReq2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","INTERCONNECTION",1_i32) =>  {
                        let d: Vec<dispatch::DispatchInterconnection1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchInterconnection1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","UNIT_SOLUTION",2_i32) =>  {
                        let d: Vec<dispatch::DispatchUnitSolution2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchUnitSolution2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","MNSPBIDTRK",1_i32) =>  {
                        let d: Vec<dispatch::DispatchMnspbidtrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchMnspbidtrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","PRICE",4_i32) =>  {
                        let d: Vec<dispatch::DispatchPrice4> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchPrice4 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","CONSTRAINT",5_i32) =>  {
                        let d: Vec<dispatch::DispatchConstraint5> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchConstraint5 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","INTERMITTENT_FORECAST_TRK",1_i32) =>  {
                        let d: Vec<dispatch::DispatchIntermittentForecastTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchIntermittentForecastTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","BLOCKED_CONSTRAINTS",1_i32) =>  {
                        let d: Vec<dispatch::DispatchBlockedConstraints1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchBlockedConstraints1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("DISPATCH","CASE_SOLUTION",2_i32) =>  {
                        let d: Vec<dispatch::DispatchCaseSolution2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertDispatchCaseSolution2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PRICELOAD","CONSTRAINT_FCAS_OCD",1_i32) =>  {
                        let d: Vec<dispatch::PriceloadConstraintFcasOcd1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertPriceloadConstraintFcasOcd1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("METERDATA","INDIVIDUAL_READS",1_i32) =>  {
                        let d: Vec<meter_data::MeterdataIndividualReads1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMeterdataIndividualReads1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("METERDATA","TRK",1_i32) =>  {
                        let d: Vec<meter_data::MeterdataTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMeterdataTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("METERDATA","INTERCONNECTOR",1_i32) =>  {
                        let d: Vec<meter_data::MeterdataInterconnector1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMeterdataInterconnector1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("METERDATA","AGGREGATE_READS",1_i32) =>  {
                        let d: Vec<meter_data::MeterdataAggregateReads1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMeterdataAggregateReads1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","MARKET_FEE_CAT_EXCL_TRK",1_i32) =>  {
                        let d: Vec<settlement_config::SettlementConfigMarketFeeCatExclTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigMarketFeeCatExclTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","MARKETFEE",1_i32) =>  {
                        let d: Vec<settlement_config::SettlementConfigMarketfee1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigMarketfee1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","SETCFG_PARTICIPANT_MPFTRK",1_i32) =>  {
                        let d: Vec<settlement_config::SettlementConfigSetcfgParticipantMpftrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigSetcfgParticipantMpftrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","MARKETFEETRK",1_i32) =>  {
                        let d: Vec<settlement_config::SettlementConfigMarketfeetrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigMarketfeetrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETCFG","REALLOCATION",2_i32) =>  {
                        let d: Vec<settlement_config::SetcfgReallocation2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSetcfgReallocation2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","ANCILLARY_RECOVERY_SPLIT",1_i32) =>  {
                        let d: Vec<settlement_config::SettlementConfigAncillaryRecoverySplit1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigAncillaryRecoverySplit1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","PARTICIPANT_BANDFEE_ALLOC",1_i32) =>  {
                        let d: Vec<settlement_config::SettlementConfigParticipantBandfeeAlloc1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigParticipantBandfeeAlloc1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","MARKET_FEE_EXCLUSION_TRK",1_i32) =>  {
                        let d: Vec<settlement_config::SettlementConfigMarketFeeExclusionTrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigMarketFeeExclusionTrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETCFG","REALLOCATIONINTERVAL",1_i32) =>  {
                        let d: Vec<settlement_config::SetcfgReallocationinterval1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSetcfgReallocationinterval1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","MARKETFEEDATA",1_i32) =>  {
                        let d: Vec<settlement_config::SettlementConfigMarketfeedata1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigMarketfeedata1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","SETCFG_PARTICIPANT_MPF",1_i32) =>  {
                        let d: Vec<settlement_config::SettlementConfigSetcfgParticipantMpf1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigSetcfgParticipantMpf1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","MARKET_FEE_EXCLUSION",1_i32) =>  {
                        let d: Vec<settlement_config::SettlementConfigMarketFeeExclusion1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigMarketFeeExclusion1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("SETTLEMENT_CONFIG","MARKET_FEE_CAT_EXCL",1_i32) =>  {
                        let d: Vec<settlement_config::SettlementConfigMarketFeeCatExcl1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertSettlementConfigMarketFeeCatExcl1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","STATIONOWNERTRK",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationStationownertrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationStationownertrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","GENUNITS",2_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationGenunits2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationGenunits2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","PARTICIPANTCLASS",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationParticipantclass1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationParticipantclass1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","PARTICIPANTCATEGORY",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationParticipantcategory1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationParticipantcategory1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","BIDDUIDDETAILS",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationBidduiddetails1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationBidduiddetails1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","DUDETAIL",3_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationDudetail3> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationDudetail3 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","GENMETER",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationGenmeter1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationGenmeter1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","DUDETAILSUMMARY",4_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationDudetailsummary4> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationDudetailsummary4 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","DISPATCHABLEUNIT",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationDispatchableunit1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationDispatchableunit1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","BIDDUIDDETAILSTRK",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationBidduiddetailstrk1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationBidduiddetailstrk1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","MNSP_PARTICIPANT",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationMnspParticipant1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationMnspParticipant1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","STATION",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationStation1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationStation1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","PARTICIPANT",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationParticipant1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationParticipant1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","PARTICIPANTCREDITDETAIL",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationParticipantcreditdetail1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationParticipantcreditdetail1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","PARTICIPANTACCOUNT",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationParticipantaccount1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationParticipantaccount1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","PARTICIPANTCATEGORYALLOC",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationParticipantcategoryalloc1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationParticipantcategoryalloc1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","STATIONOPERATINGSTATUS",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationStationoperatingstatus1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationStationoperatingstatus1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","STATIONOWNER",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationStationowner1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationStationowner1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","GENUNITS_UNIT",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationGenunitsUnit1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationGenunitsUnit1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","STADUALLOC",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationStadualloc1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationStadualloc1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","MNSP_INTERCONNECTOR",2_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationMnspInterconnector2> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationMnspInterconnector2 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("PARTICIPANT_REGISTRATION","DUALLOC",1_i32) =>  {
                        let d: Vec<participant_registration::ParticipantRegistrationDualloc1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertParticipantRegistrationDualloc1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MR","PEROFFER_STACK",1_i32) =>  {
                        let d: Vec<mrevent::MrPerofferStack1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMrPerofferStack1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MR","EVENT",1_i32) =>  {
                        let d: Vec<mrevent::MrEvent1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMrEvent1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MR","DAYOFFER_STACK",1_i32) =>  {
                        let d: Vec<mrevent::MrDayofferStack1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMrDayofferStack1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                    ("MR","EVENT_SCHEDULE",1_i32) =>  {
                        let d: Vec<mrevent::MrEventSchedule1> = self.get_table()?;
                        let json = serde_json::to_string(&d)?;
                        client.execute(
                            "exec InsertMrEventSchedule1 @P1, @P2",
                            &[&self.header.get_filename(), &json],
                            ).await?;
                    }
                _ => {
                    log::error("Unhandled file key {:?}", file_key);
                    continue;
                }
            }
        }
        Ok(())
    }
}