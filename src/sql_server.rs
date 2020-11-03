
use crate::mmsdm::*;
use futures::{AsyncRead, AsyncWrite};

impl crate::AemoFile {
    async fn log_file<S>(&self, client: &mut tiberius::Client<S>, key: &crate::FileKey) -> crate::Result<i64> 
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
    {
        // declare @header table (id bigint not null);
        // output inserted.id into @header
        let first_row = client.query(
            "insert into mmsdm.FileLog(
                data_source,
                participant_name,
                privacy_level,
                effective_date,
                serial_number,
                data_set,
                sub_type,
                version
            )
            output inserted.file_log_id
            values (@P1, @P2, @P3, @P4, @P5, @P6, @P7, @P8);",
            &[
                &self.header.data_source,
                &self.header.participant_name,
                &self.header.privacy_level,
                &self.header.get_effective(),
                &self.header.serial_number,
                &key.data_set_name.as_str(),
                &key.table_name(),
                &key.version,
            ],
        ).await?.into_row().await?;
        let row = first_row.ok_or_else(|| crate::Error::CreateFileLogError)?;
        row.try_get(0)?.ok_or_else(|| crate::Error::CreateFileLogError)

    }


    async fn batched_insert<S, D>(&self, client: &mut tiberius::Client<S>, file_key: &crate::FileKey, data: &[D], proc: &str) -> crate::Result<()>
    where
        S: AsyncRead + AsyncWrite + Unpin + Send,
        D: serde::Serialize,
    {
        let file_log_id = self.log_file(client, file_key).await?;
        
        let total = data.len();
        let mut current = 0_usize;
        for chunk in data.chunks(100_000_usize) {
            current += chunk.len();
            let json = serde_json::to_string(chunk)?;
            if let Err(e) = client
                .execute(
                    proc,
                    &[&file_log_id, &json],
                ).await {
                client.execute(
                    "update mmsdm.FileLog set [status] = 'E', message = @P1 where file_log_id = @P2",
                    &[&e.to_string(), &file_log_id],
                ).await?;
                return Err(e.into());
            } else {
                log::debug!("Progress: {} out of {} rows saved", current, total);
            }
        }
        client.execute(
            "update mmsdm.FileLog set [status] = 'C' where file_log_id = @P1",
            &[&file_log_id],
        ).await?;
        Ok(())
    }


/// This function is meant to be used in conjunction with the iterator over
/// the data contained within the AemoFile struct
pub async fn load_data<S>(&self, client: &mut tiberius::Client<S>) -> crate::Result<()>
where
S: AsyncRead + AsyncWrite + Unpin + Send,
{
for file_key in self.data.keys() {
    match (
        file_key.data_set_name.as_str(),
        file_key.table_name.as_ref().map(|s| s.as_str()),
        file_key.version,
    ) {
    
            ("MCC",Some("CASESOLUTION"),1_i32) =>  {
                #[cfg(feature = "mcc_dispatch")]
                {
                    let d: Vec<mcc_dispatch::MccCasesolution1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMccCasesolution1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "mcc_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MCC",Some("CONSTRAINTSOLUTION"),1_i32) =>  {
                #[cfg(feature = "mcc_dispatch")]
                {
                    let d: Vec<mcc_dispatch::MccConstraintsolution1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMccConstraintsolution1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "mcc_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("P5MIN",Some("CONSTRAINTSOLUTION"),6_i32) =>  {
                #[cfg(feature = "p5min")]
                {
                    let d: Vec<p5min::P5minConstraintsolution6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertP5minConstraintsolution6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "p5min"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("P5MIN",Some("INTERCONNECTORSOLN"),4_i32) =>  {
                #[cfg(feature = "p5min")]
                {
                    let d: Vec<p5min::P5minInterconnectorsoln4> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertP5minInterconnectorsoln4 @P1, @P2").await?;
                }
                #[cfg(not(feature = "p5min"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("P5MIN",Some("REGIONSOLUTION"),5_i32) =>  {
                #[cfg(feature = "p5min")]
                {
                    let d: Vec<p5min::P5minRegionsolution5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertP5minRegionsolution5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "p5min"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("P5MIN",Some("BLOCKED_CONSTRAINTS"),1_i32) =>  {
                #[cfg(feature = "p5min")]
                {
                    let d: Vec<p5min::P5minBlockedConstraints1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertP5minBlockedConstraints1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "p5min"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("P5MIN",Some("LOCAL_PRICE"),1_i32) =>  {
                #[cfg(feature = "p5min")]
                {
                    let d: Vec<p5min::P5minLocalPrice1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertP5minLocalPrice1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "p5min"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("P5MIN",Some("CASESOLUTION"),2_i32) =>  {
                #[cfg(feature = "p5min")]
                {
                    let d: Vec<p5min::P5minCasesolution2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertP5minCasesolution2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "p5min"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("P5MIN",Some("UNITSOLUTION"),3_i32) =>  {
                #[cfg(feature = "p5min")]
                {
                    let d: Vec<p5min::P5minUnitsolution3> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertP5minUnitsolution3 @P1, @P2").await?;
                }
                #[cfg(not(feature = "p5min"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("UNIT_SOLUTION"),2_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchUnitSolution2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchUnitSolution2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("REGION_SOLUTION"),4_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchRegionSolution4> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchRegionSolution4 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("CONSTRAINT_SOLUTION"),5_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchConstraintSolution5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchConstraintSolution5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("BLOCKED_CONSTRAINTS"),1_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchBlockedConstraints1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchBlockedConstraints1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("INTERCONNECTR_SENS"),1_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchInterconnectrSens1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchInterconnectrSens1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("SCENARIO_DEMAND"),1_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchScenarioDemand1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchScenarioDemand1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("LOCAL_PRICE"),1_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchLocalPrice1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchLocalPrice1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("OFFERTRK"),1_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchOffertrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchOffertrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("REGION_PRICES"),1_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchRegionPrices1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchRegionPrices1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("REGIONFCASREQUIREMENT"),2_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchRegionfcasrequirement2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchRegionfcasrequirement2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("PRICESENSITIVITIES"),1_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchPricesensitivities1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchPricesensitivities1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("MNSPBIDTRK"),1_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchMnspbidtrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchMnspbidtrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("INTERCONNECTOR_SOLN"),3_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchInterconnectorSoln3> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchInterconnectorSoln3 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("CASE_SOLUTION"),1_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchCaseSolution1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchCaseSolution1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PREDISPATCH",Some("SCENARIO_DEMAND_TRK"),1_i32) =>  {
                #[cfg(feature = "pre_dispatch")]
                {
                    let d: Vec<pre_dispatch::PredispatchScenarioDemandTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPredispatchScenarioDemandTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pre_dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("ASOFFER",Some("OFFERASTRK"),1_i32) =>  {
                #[cfg(feature = "asoffer")]
                {
                    let d: Vec<asoffer::AsofferOfferastrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertAsofferOfferastrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "asoffer"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("ASOFFER",Some("OFFERLSHEDDATA"),1_i32) =>  {
                #[cfg(feature = "asoffer")]
                {
                    let d: Vec<asoffer::AsofferOfferlsheddata1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertAsofferOfferlsheddata1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "asoffer"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("ASOFFER",Some("OFFERRPOWERDATA"),1_i32) =>  {
                #[cfg(feature = "asoffer")]
                {
                    let d: Vec<asoffer::AsofferOfferrpowerdata1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertAsofferOfferrpowerdata1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "asoffer"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("ASOFFER",Some("OFFERAGCDATA"),1_i32) =>  {
                #[cfg(feature = "asoffer")]
                {
                    let d: Vec<asoffer::AsofferOfferagcdata1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertAsofferOfferagcdata1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "asoffer"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("ASOFFER",Some("OFFERRESTARTDATA"),1_i32) =>  {
                #[cfg(feature = "asoffer")]
                {
                    let d: Vec<asoffer::AsofferOfferrestartdata1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertAsofferOfferrestartdata1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "asoffer"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("GEQRHS",Some("NULL"),1_i32) =>  {
                #[cfg(feature = "generic_constraint")]
                {
                    let d: Vec<generic_constraint::GeqrhsNull1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertGeqrhsNull1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("GCRHS",Some("NULL"),1_i32) =>  {
                #[cfg(feature = "generic_constraint")]
                {
                    let d: Vec<generic_constraint::GcrhsNull1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertGcrhsNull1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("GENERIC_CONSTRAINT",Some("GENCONSETINVOKE"),2_i32) =>  {
                #[cfg(feature = "generic_constraint")]
                {
                    let d: Vec<generic_constraint::GenericConstraintGenconsetinvoke2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertGenericConstraintGenconsetinvoke2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("GENERIC_CONSTRAINT",Some("EMSMASTER"),1_i32) =>  {
                #[cfg(feature = "generic_constraint")]
                {
                    let d: Vec<generic_constraint::GenericConstraintEmsmaster1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertGenericConstraintEmsmaster1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SPDICC",Some("NULL"),1_i32) =>  {
                #[cfg(feature = "generic_constraint")]
                {
                    let d: Vec<generic_constraint::SpdiccNull1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSpdiccNull1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("GENCONDATA",Some("NULL"),6_i32) =>  {
                #[cfg(feature = "generic_constraint")]
                {
                    let d: Vec<generic_constraint::GencondataNull6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertGencondataNull6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SPDCPC",Some("NULL"),2_i32) =>  {
                #[cfg(feature = "generic_constraint")]
                {
                    let d: Vec<generic_constraint::SpdcpcNull2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSpdcpcNull2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("GENCONSETTRK",Some("NULL"),2_i32) =>  {
                #[cfg(feature = "generic_constraint")]
                {
                    let d: Vec<generic_constraint::GenconsettrkNull2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertGenconsettrkNull2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("GEQDESC",Some("NULL"),2_i32) =>  {
                #[cfg(feature = "generic_constraint")]
                {
                    let d: Vec<generic_constraint::GeqdescNull2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertGeqdescNull2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("GENCONSET",Some("NULL"),1_i32) =>  {
                #[cfg(feature = "generic_constraint")]
                {
                    let d: Vec<generic_constraint::GenconsetNull1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertGenconsetNull1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SPDRC",Some("NULL"),2_i32) =>  {
                #[cfg(feature = "generic_constraint")]
                {
                    let d: Vec<generic_constraint::SpdrcNull2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSpdrcNull2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "generic_constraint"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_NOTICE",Some("MARKETNOTICETYPE"),1_i32) =>  {
                #[cfg(feature = "market_notice")]
                {
                    let d: Vec<market_notice::MarketNoticeMarketnoticetype1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketNoticeMarketnoticetype1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_notice"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_NOTICE",Some("MARKETNOTICEDATA"),1_i32) =>  {
                #[cfg(feature = "market_notice")]
                {
                    let d: Vec<market_notice::MarketNoticeMarketnoticedata1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketNoticeMarketnoticedata1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_notice"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_NOTICE",Some("PARTICIPANTNOTICETRK"),1_i32) =>  {
                #[cfg(feature = "market_notice")]
                {
                    let d: Vec<market_notice::MarketNoticeParticipantnoticetrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketNoticeParticipantnoticetrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_notice"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("FORCE_MAJEURE",Some("IRFMEVENTS"),1_i32) =>  {
                #[cfg(feature = "force_majeure")]
                {
                    let d: Vec<force_majeure::ForceMajeureIrfmevents1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertForceMajeureIrfmevents1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("FORCE_MAJEURE",Some("IRFMAMOUNT"),1_i32) =>  {
                #[cfg(feature = "force_majeure")]
                {
                    let d: Vec<force_majeure::ForceMajeureIrfmamount1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertForceMajeureIrfmamount1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("AP",Some("APEVENT"),1_i32) =>  {
                #[cfg(feature = "force_majeure")]
                {
                    let d: Vec<force_majeure::ApApevent1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertApApevent1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("AP",Some("APEVENTREGION"),1_i32) =>  {
                #[cfg(feature = "force_majeure")]
                {
                    let d: Vec<force_majeure::ApApeventregion1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertApApeventregion1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("AP",Some("REGIONAPCINTERVALS"),1_i32) =>  {
                #[cfg(feature = "force_majeure")]
                {
                    let d: Vec<force_majeure::ApRegionapcintervals1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertApRegionapcintervals1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("FORCE_MAJEURE",Some("OVERRIDERRP"),1_i32) =>  {
                #[cfg(feature = "force_majeure")]
                {
                    let d: Vec<force_majeure::ForceMajeureOverriderrp1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertForceMajeureOverriderrp1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("FORCE_MAJEURE",Some("MARKET_SUSPEND_SCHEDULE"),1_i32) =>  {
                #[cfg(feature = "force_majeure")]
                {
                    let d: Vec<force_majeure::ForceMajeureMarketSuspendSchedule1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertForceMajeureMarketSuspendSchedule1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("FORCE_MAJEURE",Some("MARKET_SUSPEND_REGIME_SUM"),1_i32) =>  {
                #[cfg(feature = "force_majeure")]
                {
                    let d: Vec<force_majeure::ForceMajeureMarketSuspendRegimeSum1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertForceMajeureMarketSuspendRegimeSum1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("FORCE_MAJEURE",Some("MARKET_SUSPEND_SCHEDULE_TRK"),1_i32) =>  {
                #[cfg(feature = "force_majeure")]
                {
                    let d: Vec<force_majeure::ForceMajeureMarketSuspendScheduleTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertForceMajeureMarketSuspendScheduleTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("FORCE_MAJEURE",Some("MARKET_SUSPEND_REGION_SUM"),1_i32) =>  {
                #[cfg(feature = "force_majeure")]
                {
                    let d: Vec<force_majeure::ForceMajeureMarketSuspendRegionSum1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertForceMajeureMarketSuspendRegionSum1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("AP",Some("REGIONAPC"),1_i32) =>  {
                #[cfg(feature = "force_majeure")]
                {
                    let d: Vec<force_majeure::ApRegionapc1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertApRegionapc1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "force_majeure"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("INTERCONNECTION"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchInterconnection1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchInterconnection1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PRICELOAD",Some("CONSTRAINT_FCAS_OCD"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::PriceloadConstraintFcasOcd1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPriceloadConstraintFcasOcd1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("PRICE"),4_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchPrice4> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchPrice4 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PRICELOAD",Some("CONSTRAINTRELAXATION"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::PriceloadConstraintrelaxation1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPriceloadConstraintrelaxation1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("INTERMITTENT_FORECAST_TRK"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchIntermittentForecastTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchIntermittentForecastTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("UNIT_SCADA"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchUnitScada1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchUnitScada1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("FCAS_REQ"),2_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchFcasReq2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchFcasReq2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("BLOCKED_CONSTRAINTS"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchBlockedConstraints1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchBlockedConstraints1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("UNIT_SOLUTION"),2_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchUnitSolution2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchUnitSolution2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PRICELOAD",Some("PRICE_REVISION"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::PriceloadPriceRevision1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPriceloadPriceRevision1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("REGIONSUM"),4_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchRegionsum4> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchRegionsum4 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("UNIT_CONFORMANCE"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchUnitConformance1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchUnitConformance1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("LOCAL_PRICE"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchLocalPrice1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchLocalPrice1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("OFFERTRK"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchOffertrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchOffertrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("MNSPBIDTRK"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchMnspbidtrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchMnspbidtrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("NEGATIVE_RESIDUE"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchNegativeResidue1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchNegativeResidue1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("CASE_SOLUTION"),2_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchCaseSolution2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchCaseSolution2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("INTERCONNECTORRES"),3_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchInterconnectorres3> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchInterconnectorres3 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("MR_SCHEDULE_TRK"),1_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchMrScheduleTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchMrScheduleTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DISPATCH",Some("CONSTRAINT"),5_i32) =>  {
                #[cfg(feature = "dispatch")]
                {
                    let d: Vec<dispatch::DispatchConstraint5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDispatchConstraint5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "dispatch"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("IRPARTSURPLUS"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingIrpartsurplus5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingIrpartsurplus5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("DAILY_ENERGY_SUMMARY"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingDailyEnergySummary1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingDailyEnergySummary1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("BILLING_DIRECTION_RECON_OTHER"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingBillingDirectionReconOther1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingBillingDirectionReconOther1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("NMAS_TST_RECOVERY"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingNmasTstRecovery1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingNmasTstRecovery1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("SECDEP_INTEREST_PAY"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingSecdepInterestPay1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingSecdepInterestPay1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("IRAUCSURPLUSSUM"),7_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingIraucsurplussum7> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingIraucsurplussum7 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("DAYTRK"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingDaytrk5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingDaytrk5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("IRNSPSURPLUSSUM"),6_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingIrnspsurplussum6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingIrnspsurplussum6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("APC_RECOVERY"),2_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingApcRecovery2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingApcRecovery2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("IRFM"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingIrfm5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingIrfm5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("SMELTERREDUCTION"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingSmelterreduction5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingSmelterreduction5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("NMAS_TST_PAYMENTS"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingNmasTstPayments1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingNmasTstPayments1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("RUNTRK"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingRuntrk5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingRuntrk5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("MR_SHORTFALL"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingMrShortfall5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingMrShortfall5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("GENDATA"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingGendata5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingGendata5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("GST_SUMMARY"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingGstSummary5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingGstSummary5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("SECDEP_INTEREST_RATE"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingSecdepInterestRate1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingSecdepInterestRate1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("REALLOC"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingRealloc5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingRealloc5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("MR_PAYMENT"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingMrPayment5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingMrPayment5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("REGIONFIGURES"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingRegionfigures5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingRegionfigures5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("ASPAYMENTS"),6_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingAspayments6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingAspayments6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("NMAS_TST_RECVRY_RBF"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingNmasTstRecvryRbf1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingNmasTstRecvryRbf1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("MR_SUMMARY"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingMrSummary5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingMrSummary5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("REGIONEXPORTS"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingRegionexports5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingRegionexports5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("FINANCIALADJUSTMENTS"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingFinancialadjustments5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingFinancialadjustments5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("REGIONIMPORTS"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingRegionimports5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingRegionimports5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("REALLOC_DETAIL"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingReallocDetail5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingReallocDetail5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("PRIORADJUSTMENTS"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingPrioradjustments5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingPrioradjustments5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("EFTSHORTFALL_AMOUNT"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingEftshortfallAmount1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingEftshortfallAmount1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("CPDATA"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingCpdata5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingCpdata5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("MR_RECOVERY"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingMrRecovery5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingMrRecovery5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("RES_TRADER_PAYMENT"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingResTraderPayment1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingResTraderPayment1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("ASRECOVERY"),7_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingAsrecovery7> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingAsrecovery7 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("FEES"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingFees5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingFees5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("BILLING_CO2E_PUBLICATION"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingBillingCo2ePublication1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingBillingCo2ePublication1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("SECDEPOSIT_APPLICATION"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingSecdepositApplication1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingSecdepositApplication1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("GST_DETAIL"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingGstDetail5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingGstDetail5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("DIRECTION_RECONCILIATN"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingDirectionReconciliatn1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingDirectionReconciliatn1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("EFTSHORTFALL_DETAIL"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingEftshortfallDetail1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingEftshortfallDetail1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("IRNSPSURPLUS"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingIrnspsurplus5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingIrnspsurplus5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("WHITEHOLE"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingWhitehole5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingWhitehole5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("IRPARTSURPLUSSUM"),7_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingIrpartsurplussum7> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingIrpartsurplussum7 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("APC_COMPENSATION"),2_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingApcCompensation2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingApcCompensation2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("NMAS_TST_RECVRY_TRK"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingNmasTstRecvryTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingNmasTstRecvryTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("INTERRESIDUES"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingInterresidues5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingInterresidues5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("RES_TRADER_RECOVERY"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingResTraderRecovery1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingResTraderRecovery1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("IRAUCSURPLUS"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingIraucsurplus5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingIraucsurplus5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("BILLING_CO2E_PUBLICATION_TRK"),1_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingBillingCo2ePublicationTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingBillingCo2ePublicationTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING",Some("INTRARESIDUES"),5_i32) =>  {
                #[cfg(feature = "billing_run")]
                {
                    let d: Vec<billing_run::BillingIntraresidues5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingIntraresidues5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_run"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BID",Some("BIDDAYOFFER_D"),2_i32) =>  {
                #[cfg(feature = "bids")]
                {
                    let d: Vec<bids::BidBiddayofferD2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBidBiddayofferD2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BID",Some("BIDPEROFFER_D"),2_i32) =>  {
                #[cfg(feature = "bids")]
                {
                    let d: Vec<bids::BidBidperofferD2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBidBidperofferD2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("OFFER",Some("MTPASA_OFFERFILETRK"),1_i32) =>  {
                #[cfg(feature = "bids")]
                {
                    let d: Vec<bids::OfferMtpasaOfferfiletrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertOfferMtpasaOfferfiletrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("OFFER",Some("MNSP_PEROFFER"),1_i32) =>  {
                #[cfg(feature = "bids")]
                {
                    let d: Vec<bids::OfferMnspPeroffer1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertOfferMnspPeroffer1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BID",Some("MNSP_FILETRK"),1_i32) =>  {
                #[cfg(feature = "bids")]
                {
                    let d: Vec<bids::BidMnspFiletrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBidMnspFiletrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("OFFER",Some("BIDPEROFFER"),1_i32) =>  {
                #[cfg(feature = "bids")]
                {
                    let d: Vec<bids::OfferBidperoffer1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertOfferBidperoffer1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("OFFER",Some("MNSP_DAYOFFER"),2_i32) =>  {
                #[cfg(feature = "bids")]
                {
                    let d: Vec<bids::OfferMnspDayoffer2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertOfferMnspDayoffer2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("OFFER",Some("MTPASA_OFFERDATA"),1_i32) =>  {
                #[cfg(feature = "bids")]
                {
                    let d: Vec<bids::OfferMtpasaOfferdata1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertOfferMtpasaOfferdata1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("OFFER",Some("MNSP_OFFERTRK"),1_i32) =>  {
                #[cfg(feature = "bids")]
                {
                    let d: Vec<bids::OfferMnspOffertrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertOfferMnspOffertrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("OFFER",Some("BIDDAYOFFER"),2_i32) =>  {
                #[cfg(feature = "bids")]
                {
                    let d: Vec<bids::OfferBiddayoffer2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertOfferBiddayoffer2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("OFFER",Some("BIDOFFERFILETRK"),1_i32) =>  {
                #[cfg(feature = "bids")]
                {
                    let d: Vec<bids::OfferBidofferfiletrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertOfferBidofferfiletrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "bids"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("STPASA",Some("CONSTRAINTSOLUTION"),2_i32) =>  {
                #[cfg(feature = "stpasa_solution")]
                {
                    let d: Vec<stpasa_solution::StpasaConstraintsolution2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertStpasaConstraintsolution2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "stpasa_solution"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("STPASA",Some("INTERCONNECTORSOLN"),2_i32) =>  {
                #[cfg(feature = "stpasa_solution")]
                {
                    let d: Vec<stpasa_solution::StpasaInterconnectorsoln2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertStpasaInterconnectorsoln2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "stpasa_solution"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("STPASA",Some("REGIONSOLUTION"),5_i32) =>  {
                #[cfg(feature = "stpasa_solution")]
                {
                    let d: Vec<stpasa_solution::StpasaRegionsolution5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertStpasaRegionsolution5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "stpasa_solution"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("STPASA",Some("CASESOLUTION"),3_i32) =>  {
                #[cfg(feature = "stpasa_solution")]
                {
                    let d: Vec<stpasa_solution::StpasaCasesolution3> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertStpasaCasesolution3 @P1, @P2").await?;
                }
                #[cfg(not(feature = "stpasa_solution"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("GD_INSTRUCT",Some("INSTRUCTIONSUBTYPE"),1_i32) =>  {
                #[cfg(feature = "gd_instruct")]
                {
                    let d: Vec<gd_instruct::GdInstructInstructionsubtype1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertGdInstructInstructionsubtype1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "gd_instruct"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("GD_INSTRUCT",Some("GDINSTRUCT"),1_i32) =>  {
                #[cfg(feature = "gd_instruct")]
                {
                    let d: Vec<gd_instruct::GdInstructGdinstruct1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertGdInstructGdinstruct1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "gd_instruct"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("GD_INSTRUCT",Some("INSTRUCTIONTYPE"),1_i32) =>  {
                #[cfg(feature = "gd_instruct")]
                {
                    let d: Vec<gd_instruct::GdInstructInstructiontype1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertGdInstructInstructiontype1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "gd_instruct"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("VOLTAGE_INSTRUCTION",Some("TRACK"),2_i32) =>  {
                #[cfg(feature = "voltage_instructions")]
                {
                    let d: Vec<voltage_instructions::VoltageInstructionTrack2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertVoltageInstructionTrack2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "voltage_instructions"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("VOLTAGE_INSTRUCTION",Some("INSTRUCTION"),2_i32) =>  {
                #[cfg(feature = "voltage_instructions")]
                {
                    let d: Vec<voltage_instructions::VoltageInstructionInstruction2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertVoltageInstructionInstruction2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "voltage_instructions"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("NETWORK",Some("SUBSTATIONDETAIL"),1_i32) =>  {
                #[cfg(feature = "network")]
                {
                    let d: Vec<network::NetworkSubstationdetail1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertNetworkSubstationdetail1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "network"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("NETWORK",Some("REALTIMERATING"),1_i32) =>  {
                #[cfg(feature = "network")]
                {
                    let d: Vec<network::NetworkRealtimerating1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertNetworkRealtimerating1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "network"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("NETWORK",Some("OUTAGECONSTRAINTSET"),1_i32) =>  {
                #[cfg(feature = "network")]
                {
                    let d: Vec<network::NetworkOutageconstraintset1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertNetworkOutageconstraintset1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "network"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("NETWORK",Some("STATICRATING"),1_i32) =>  {
                #[cfg(feature = "network")]
                {
                    let d: Vec<network::NetworkStaticrating1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertNetworkStaticrating1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "network"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("NETWORK",Some("EQUIPMENTDETAIL"),1_i32) =>  {
                #[cfg(feature = "network")]
                {
                    let d: Vec<network::NetworkEquipmentdetail1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertNetworkEquipmentdetail1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "network"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("NETWORK",Some("OUTAGEDETAIL"),3_i32) =>  {
                #[cfg(feature = "network")]
                {
                    let d: Vec<network::NetworkOutagedetail3> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertNetworkOutagedetail3 @P1, @P2").await?;
                }
                #[cfg(not(feature = "network"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("NETWORK",Some("OUTAGESTATUSCODE"),1_i32) =>  {
                #[cfg(feature = "network")]
                {
                    let d: Vec<network::NetworkOutagestatuscode1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertNetworkOutagestatuscode1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "network"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("NETWORK",Some("RATING"),1_i32) =>  {
                #[cfg(feature = "network")]
                {
                    let d: Vec<network::NetworkRating1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertNetworkRating1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "network"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION_CONFIG",Some("AUCTION_RP_ESTIMATE"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionConfigAuctionRpEstimate1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionConfigAuctionRpEstimate1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_FINANCIAL_AUC_RECEIPTS"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraFinancialAucReceipts1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraFinancialAucReceipts1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_FINANCIAL_AUC_MARDETAIL"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraFinancialAucMardetail1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraFinancialAucMardetail1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("RESIDUE_PRICE_FUNDS_BID"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionResiduePriceFundsBid1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionResiduePriceFundsBid1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_PRUDENTIAL_COMP_POSITION"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraPrudentialCompPosition1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraPrudentialCompPosition1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_FINANCIAL_AUC_MARGIN"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraFinancialAucMargin1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraFinancialAucMargin1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("RESIDUE_CON_FUNDS"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionResidueConFunds1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionResidueConFunds1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("RESIDUE_CON_DATA"),2_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionResidueConData2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionResidueConData2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_PRUDENTIAL_CASH_SECURITY"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraPrudentialCashSecurity1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraPrudentialCashSecurity1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("VALUATIONID"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionValuationid1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionValuationid1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION_CONFIG",Some("AUCTION_REVENUE_TRACK"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionConfigAuctionRevenueTrack1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionConfigAuctionRevenueTrack1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("RESIDUECONTRACTPAYMENTS"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::SettlementConfigResiduecontractpayments1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigResiduecontractpayments1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("RESIDUE_CONTRACTS"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionResidueContracts1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionResidueContracts1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("RESIDUE_BID_TRK"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionResidueBidTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionResidueBidTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION_BIDS",Some("FILE_TRK"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionBidsFileTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionBidsFileTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION_CONFIG",Some("AUCTION_CALENDAR"),2_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionConfigAuctionCalendar2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionConfigAuctionCalendar2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION_CONFIG",Some("AUCTION_TRANCHE"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionConfigAuctionTranche1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionConfigAuctionTranche1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION_BIDS",Some("FUNDS_BID"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionBidsFundsBid1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionBidsFundsBid1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_FINANCIAL_RUNTRK"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraFinancialRuntrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraFinancialRuntrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_CASH_SECURITY"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraCashSecurity1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraCashSecurity1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION_CONFIG",Some("AUCTION_IC_ALLOCATIONS"),2_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionConfigAuctionIcAllocations2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionConfigAuctionIcAllocations2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_FINANCIAL_AUCPAY_SUM"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraFinancialAucpaySum1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraFinancialAucpaySum1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_OFFER_PROFILE"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraOfferProfile1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraOfferProfile1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_PRUDENTIAL_EXPOSURE"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraPrudentialExposure1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraPrudentialExposure1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_PRUDENTIAL_RUN"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraPrudentialRun1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraPrudentialRun1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_OFFER_PRODUCT"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraOfferProduct1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraOfferProduct1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION_CONFIG",Some("AUCTION_REVENUE_ESTIMATE"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionConfigAuctionRevenueEstimate1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionConfigAuctionRevenueEstimate1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION_BIDS",Some("PRICE_BID"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionBidsPriceBid1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionBidsPriceBid1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION_CONFIG",Some("AUCTION"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionConfigAuction1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionConfigAuction1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("RESIDUE_TRK"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionResidueTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionResidueTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("SRA_FINANCIAL_AUCPAY_DETAIL"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionSraFinancialAucpayDetail1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionSraFinancialAucpayDetail1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("RESIDUE_CON_ESTIMATES_TRK"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionResidueConEstimatesTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionResidueConEstimatesTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("IRAUCTION",Some("RESIDUE_PUBLIC_DATA"),1_i32) =>  {
                #[cfg(feature = "irauction")]
                {
                    let d: Vec<irauction::IrauctionResiduePublicData1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertIrauctionResiduePublicData1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "irauction"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("RPOWERRECOVERY"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsRpowerrecovery5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsRpowerrecovery5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("VICBOUNDARYENERGY"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsVicboundaryenergy5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsVicboundaryenergy5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("APC_COMPENSATION"),1_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsApcCompensation1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsApcCompensation1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("FCASCOMP"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsFcascomp5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsFcascomp5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("ANCILLARY_SUMMARY"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsAncillarySummary5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsAncillarySummary5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("AGCRECOVERY"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsAgcrecovery5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsAgcrecovery5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("IRFMRECOVERY"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsIrfmrecovery5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsIrfmrecovery5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("CPDATAREGION"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsCpdataregion5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsCpdataregion5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("LUNLOADRECOVERY"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsLunloadrecovery5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsLunloadrecovery5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("RPOWERPAYMENT"),6_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsRpowerpayment6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsRpowerpayment6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("APC_RECOVERY"),1_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsApcRecovery1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsApcRecovery1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("MR_RECOVERY"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsMrRecovery5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsMrRecovery5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("FCAS_RECOVERY"),6_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsFcasRecovery6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsFcasRecovery6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("MARKETFEES"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsMarketfees5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsMarketfees5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("GENDATAREGION"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsGendataregion5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsGendataregion5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("INTRAREGIONRESIDUES"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsIntraregionresidues5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsIntraregionresidues5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("IRNSPSURPLUS"),6_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsIrnspsurplus6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsIrnspsurplus6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("CPDATA"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsCpdata5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsCpdata5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("INTERVENTIONRECOVERY"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsInterventionrecovery5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsInterventionrecovery5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("LSHEDPAYMENT"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsLshedpayment5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsLshedpayment5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("RESTARTRECOVERY"),6_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsRestartrecovery6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsRestartrecovery6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("RESTARTPAYMENT"),6_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsRestartpayment6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsRestartpayment6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("AGCPAYMENT"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsAgcpayment5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsAgcpayment5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("FCAS_PAYMENT"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsFcasPayment5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsFcasPayment5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("IRSURPLUS"),6_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsIrsurplus6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsIrsurplus6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("NMAS_RECOVERY_RBF"),1_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsNmasRecoveryRbf1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsNmasRecoveryRbf1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("MR_PAYMENT"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsMrPayment5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsMrPayment5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("IRPARTSURPLUS"),6_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsIrpartsurplus6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsIrpartsurplus6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("SMALLGENDATA"),1_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsSmallgendata1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsSmallgendata1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("LUNLOADPAYMENT"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsLunloadpayment5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsLunloadpayment5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("LULOADRECOVERY"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsLuloadrecovery5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsLuloadrecovery5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("GENDATA"),6_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsGendata6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsGendata6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("FCASREGIONRECOVERY"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsFcasregionrecovery5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsFcasregionrecovery5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("IRAUCSURPLUS"),6_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsIraucsurplus6> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsIraucsurplus6 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("SET_FCAS_REGULATION_TRK"),1_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsSetFcasRegulationTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsSetFcasRegulationTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("NMAS_RECOVERY"),2_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsNmasRecovery2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsNmasRecovery2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("VICENERGYFIGURES"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsVicenergyfigures5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsVicenergyfigures5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("DAYTRACK"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsDaytrack5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsDaytrack5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("REALLOCATIONS"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsReallocations5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsReallocations5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("INTERVENTION"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsIntervention5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsIntervention5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("VICENERGYFLOW"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsVicenergyflow5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsVicenergyflow5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("RUN_PARAMETER"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsRunParameter5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsRunParameter5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENTS",Some("LSHEDRECOVERY"),5_i32) =>  {
                #[cfg(feature = "settlement_data")]
                {
                    let d: Vec<settlement_data::SettlementsLshedrecovery5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementsLshedrecovery5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETCFG",Some("REALLOCATION"),2_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SetcfgReallocation2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSetcfgReallocation2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("SETCFG_PARTICIPANT_MPF"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SettlementConfigSetcfgParticipantMpf1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigSetcfgParticipantMpf1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("ANCILLARY_RECOVERY_SPLIT"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SettlementConfigAncillaryRecoverySplit1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigAncillaryRecoverySplit1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("MARKETFEETRK"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SettlementConfigMarketfeetrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigMarketfeetrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("SETCFG_PARTICIPANT_MPFTRK"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SettlementConfigSetcfgParticipantMpftrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigSetcfgParticipantMpftrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("MARKET_FEE_CAT_EXCL"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SettlementConfigMarketFeeCatExcl1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigMarketFeeCatExcl1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("MARKET_FEE_EXCLUSION_TRK"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SettlementConfigMarketFeeExclusionTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigMarketFeeExclusionTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("MARKET_FEE_CAT_EXCL_TRK"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SettlementConfigMarketFeeCatExclTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigMarketFeeCatExclTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("PARTICIPANT_BANDFEE_ALLOC"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SettlementConfigParticipantBandfeeAlloc1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigParticipantBandfeeAlloc1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("MARKETFEE"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SettlementConfigMarketfee1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigMarketfee1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETCFG",Some("REALLOCATIONINTERVAL"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SetcfgReallocationinterval1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSetcfgReallocationinterval1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("MARKET_FEE_EXCLUSION"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SettlementConfigMarketFeeExclusion1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigMarketFeeExclusion1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("SETTLEMENT_CONFIG",Some("MARKETFEEDATA"),1_i32) =>  {
                #[cfg(feature = "settlement_config")]
                {
                    let d: Vec<settlement_config::SettlementConfigMarketfeedata1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertSettlementConfigMarketfeedata1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "settlement_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("INTERCONNECTORALLOC"),1_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigInterconnectoralloc1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigInterconnectoralloc1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("TRANSMISSIONLOSSFACTOR"),2_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigTransmissionlossfactor2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigTransmissionlossfactor2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("BIDTYPESTRK"),1_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigBidtypestrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigBidtypestrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("LOSSMODEL"),1_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigLossmodel1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigLossmodel1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("LOSSFACTORMODEL"),1_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigLossfactormodel1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigLossfactormodel1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("BIDTYPES"),1_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigBidtypes1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigBidtypes1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("MARKET_PRICE_THRESHOLDS"),1_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigMarketPriceThresholds1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigMarketPriceThresholds1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("INTERCONNECTOR"),1_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigInterconnector1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigInterconnector1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("INTRAREGIONALLOC"),1_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigIntraregionalloc1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigIntraregionalloc1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("REGIONSTANDINGDATA"),1_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigRegionstandingdata1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigRegionstandingdata1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("INTERCONNECTORCONSTRAINT"),1_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigInterconnectorconstraint1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigInterconnectorconstraint1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MARKET_CONFIG",Some("REGION"),1_i32) =>  {
                #[cfg(feature = "market_config")]
                {
                    let d: Vec<market_config::MarketConfigRegion1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMarketConfigRegion1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "market_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("METERDATA",Some("TRK"),1_i32) =>  {
                #[cfg(feature = "meter_data")]
                {
                    let d: Vec<meter_data::MeterdataTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMeterdataTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "meter_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("METERDATA",Some("INTERCONNECTOR"),1_i32) =>  {
                #[cfg(feature = "meter_data")]
                {
                    let d: Vec<meter_data::MeterdataInterconnector1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMeterdataInterconnector1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "meter_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("METERDATA",Some("INDIVIDUAL_READS"),1_i32) =>  {
                #[cfg(feature = "meter_data")]
                {
                    let d: Vec<meter_data::MeterdataIndividualReads1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMeterdataIndividualReads1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "meter_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("METERDATA",Some("AGGREGATE_READS"),1_i32) =>  {
                #[cfg(feature = "meter_data")]
                {
                    let d: Vec<meter_data::MeterdataAggregateReads1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMeterdataAggregateReads1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "meter_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PRUDENTIAL",Some("COMPANY_POSITION"),1_i32) =>  {
                #[cfg(feature = "prudentials")]
                {
                    let d: Vec<prudentials::PrudentialCompanyPosition1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPrudentialCompanyPosition1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "prudentials"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PRUDENTIAL",Some("RUNTRK"),1_i32) =>  {
                #[cfg(feature = "prudentials")]
                {
                    let d: Vec<prudentials::PrudentialRuntrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPrudentialRuntrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "prudentials"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PDPASA",Some("CASESOLUTION"),3_i32) =>  {
                #[cfg(feature = "pdpasa")]
                {
                    let d: Vec<pdpasa::PdpasaCasesolution3> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPdpasaCasesolution3 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pdpasa"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PDPASA",Some("REGIONSOLUTION"),5_i32) =>  {
                #[cfg(feature = "pdpasa")]
                {
                    let d: Vec<pdpasa::PdpasaRegionsolution5> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertPdpasaRegionsolution5 @P1, @P2").await?;
                }
                #[cfg(not(feature = "pdpasa"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("TRADING",Some("REGIONSUM"),4_i32) =>  {
                #[cfg(feature = "trading_data")]
                {
                    let d: Vec<trading_data::TradingRegionsum4> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertTradingRegionsum4 @P1, @P2").await?;
                }
                #[cfg(not(feature = "trading_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("TRADING",Some("UNIT_SOLUTION"),2_i32) =>  {
                #[cfg(feature = "trading_data")]
                {
                    let d: Vec<trading_data::TradingUnitSolution2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertTradingUnitSolution2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "trading_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("TRADING",Some("PRICE"),2_i32) =>  {
                #[cfg(feature = "trading_data")]
                {
                    let d: Vec<trading_data::TradingPrice2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertTradingPrice2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "trading_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("TRADING",Some("INTERCONNECTORRES"),2_i32) =>  {
                #[cfg(feature = "trading_data")]
                {
                    let d: Vec<trading_data::TradingInterconnectorres2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertTradingInterconnectorres2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "trading_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DEMAND",Some("INTERMITTENT_GEN_LIMIT_DAY"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::DemandIntermittentGenLimitDay1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDemandIntermittentGenLimitDay1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DEMAND",Some("TRK"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::DemandTrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDemandTrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DEMAND",Some("INTERMITTENT_GEN_LIMIT"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::DemandIntermittentGenLimit1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDemandIntermittentGenLimit1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DEMAND",Some("INTERMITTENT_DS_PRED"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::DemandIntermittentDsPred1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDemandIntermittentDsPred1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DEMAND",Some("MTPASA_INTERMITTENT_LIMIT"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::DemandMtpasaIntermittentLimit1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDemandMtpasaIntermittentLimit1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("ROOFTOP",Some("ACTUAL"),2_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::RooftopActual2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertRooftopActual2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DEMAND",Some("INTERMITTENT_CLUSTER_AVAIL_DAY"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::DemandIntermittentClusterAvailDay1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDemandIntermittentClusterAvailDay1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("OPERATIONAL_DEMAND",Some("ACTUAL"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::OperationalDemandActual1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertOperationalDemandActual1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DEMAND",Some("INTERMITTENT_CLUSTER_AVAIL"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::DemandIntermittentClusterAvail1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDemandIntermittentClusterAvail1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("FORECAST",Some("INTERMITTENT_GEN_DATA"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::ForecastIntermittentGenData1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertForecastIntermittentGenData1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DEMAND",Some("PERIOD"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::DemandPeriod1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDemandPeriod1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("FORECAST",Some("INTERMITTENT_GEN"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::ForecastIntermittentGen1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertForecastIntermittentGen1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("ROOFTOP",Some("FORECAST"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::RooftopForecast1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertRooftopForecast1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DEMAND",Some("MTPASA_INTERMITTENT_AVAIL"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::DemandMtpasaIntermittentAvail1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDemandMtpasaIntermittentAvail1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("DEMAND",Some("INTERMITTENT_DS_RUN"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::DemandIntermittentDsRun1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertDemandIntermittentDsRun1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("OPERATIONAL_DEMAND",Some("FORECAST"),1_i32) =>  {
                #[cfg(feature = "demand_forecasts")]
                {
                    let d: Vec<demand_forecasts::OperationalDemandForecast1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertOperationalDemandForecast1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "demand_forecasts"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MR",Some("EVENT"),1_i32) =>  {
                #[cfg(feature = "mrevent")]
                {
                    let d: Vec<mrevent::MrEvent1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMrEvent1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "mrevent"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MR",Some("PEROFFER_STACK"),1_i32) =>  {
                #[cfg(feature = "mrevent")]
                {
                    let d: Vec<mrevent::MrPerofferStack1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMrPerofferStack1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "mrevent"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MR",Some("DAYOFFER_STACK"),1_i32) =>  {
                #[cfg(feature = "mrevent")]
                {
                    let d: Vec<mrevent::MrDayofferStack1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMrDayofferStack1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "mrevent"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MR",Some("EVENT_SCHEDULE"),1_i32) =>  {
                #[cfg(feature = "mrevent")]
                {
                    let d: Vec<mrevent::MrEventSchedule1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMrEventSchedule1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "mrevent"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("PARTICIPANTCATEGORYALLOC"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationParticipantcategoryalloc1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcategoryalloc1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("MNSP_INTERCONNECTOR"),2_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationMnspInterconnector2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationMnspInterconnector2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("DISPATCHABLEUNIT"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationDispatchableunit1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationDispatchableunit1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("PARTICIPANTACCOUNT"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationParticipantaccount1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantaccount1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("STATIONOWNERTRK"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationStationownertrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationStationownertrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("BIDDUIDDETAILSTRK"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationBidduiddetailstrk1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationBidduiddetailstrk1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("DUDETAIL"),3_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationDudetail3> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationDudetail3 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("STADUALLOC"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationStadualloc1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationStadualloc1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("BIDDUIDDETAILS"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationBidduiddetails1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationBidduiddetails1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("DUALLOC"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationDualloc1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationDualloc1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("DUDETAILSUMMARY"),4_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationDudetailsummary4> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationDudetailsummary4 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("PARTICIPANTCREDITDETAIL"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationParticipantcreditdetail1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcreditdetail1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("MNSP_PARTICIPANT"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationMnspParticipant1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationMnspParticipant1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("PARTICIPANTCLASS"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationParticipantclass1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantclass1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("PARTICIPANT"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationParticipant1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipant1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("STATION"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationStation1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationStation1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("STATIONOWNER"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationStationowner1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationStationowner1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("GENUNITS_UNIT"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationGenunitsUnit1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationGenunitsUnit1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("GENUNITS"),2_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationGenunits2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationGenunits2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("GENMETER"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationGenmeter1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationGenmeter1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("STATIONOPERATINGSTATUS"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationStationoperatingstatus1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationStationoperatingstatus1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("PARTICIPANT_REGISTRATION",Some("PARTICIPANTCATEGORY"),1_i32) =>  {
                #[cfg(feature = "participant_registration")]
                {
                    let d: Vec<participant_registration::ParticipantRegistrationParticipantcategory1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertParticipantRegistrationParticipantcategory1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "participant_registration"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MTPASA",Some("RESERVELIMIT"),1_i32) =>  {
                #[cfg(feature = "reserve_data")]
                {
                    let d: Vec<reserve_data::MtpasaReservelimit1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMtpasaReservelimit1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "reserve_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MTPASA",Some("RESERVELIMIT_REGION"),1_i32) =>  {
                #[cfg(feature = "reserve_data")]
                {
                    let d: Vec<reserve_data::MtpasaReservelimitRegion1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMtpasaReservelimitRegion1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "reserve_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("MTPASA",Some("RESERVELIMIT_SET"),1_i32) =>  {
                #[cfg(feature = "reserve_data")]
                {
                    let d: Vec<reserve_data::MtpasaReservelimitSet1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertMtpasaReservelimitSet1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "reserve_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("RESERVE_DATA",Some("RESERVE"),1_i32) =>  {
                #[cfg(feature = "reserve_data")]
                {
                    let d: Vec<reserve_data::ReserveDataReserve1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertReserveDataReserve1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "reserve_data"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING_CONFIG",Some("GST_TRANSACTION_CLASS"),1_i32) =>  {
                #[cfg(feature = "billing_config")]
                {
                    let d: Vec<billing_config::BillingConfigGstTransactionClass1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingConfigGstTransactionClass1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING_CONFIG",Some("GST_TRANSACTION_TYPE"),1_i32) =>  {
                #[cfg(feature = "billing_config")]
                {
                    let d: Vec<billing_config::BillingConfigGstTransactionType1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingConfigGstTransactionType1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING_CONFIG",Some("GST_RATE"),1_i32) =>  {
                #[cfg(feature = "billing_config")]
                {
                    let d: Vec<billing_config::BillingConfigGstRate1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingConfigGstRate1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING_CONFIG",Some("SECDEPOSIT_INTEREST_RATE"),1_i32) =>  {
                #[cfg(feature = "billing_config")]
                {
                    let d: Vec<billing_config::BillingConfigSecdepositInterestRate1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingConfigSecdepositInterestRate1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING_CONFIG",Some("SECDEPOSIT_PROVISION"),1_i32) =>  {
                #[cfg(feature = "billing_config")]
                {
                    let d: Vec<billing_config::BillingConfigSecdepositProvision1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingConfigSecdepositProvision1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING_CONFIG",Some("BILLINGCALENDAR"),2_i32) =>  {
                #[cfg(feature = "billing_config")]
                {
                    let d: Vec<billing_config::BillingConfigBillingcalendar2> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingConfigBillingcalendar2 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
            ("BILLING_CONFIG",Some("GST_BAS_CLASS"),1_i32) =>  {
                #[cfg(feature = "billing_config")]
                {
                    let d: Vec<billing_config::BillingConfigGstBasClass1> = self.get_table()?;
                    self.batched_insert(client, file_key, &d, "exec mmsdm_proc.InsertBillingConfigGstBasClass1 @P1, @P2").await?;
                }
                #[cfg(not(feature = "billing_config"))]
                {
                    log::warn!("Unhandled file key {:?}", file_key);
                    continue;
                }
                
            }
        _ => {
            log::error!("Unexpected file key {:?}", file_key);
            continue;
        }
    }
}
Ok(())
}
}