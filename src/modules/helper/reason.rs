pub mod reason{
    use actix_web::web;
    use serde_json::Value;
    use sqlx::{MySqlPool, Error};

    use crate::schemas::helper::reason_schema::ReasonRequestSchema;

    pub enum KindTransaction {
        Update,
        Delete,
    }

    pub async fn update_reason((kind_transaction, value, pool): (KindTransaction, &Value, web::Data<MySqlPool>)) -> Result<(), Error> {
        let reason_data: ReasonRequestSchema = serde_json::from_value(value.clone()).unwrap();
        
        match kind_transaction {
            KindTransaction::Update => { 
                let _data = sqlx::query("set @userName=?, @reason=?")
                    .bind(reason_data.updated_by.clone())
                    .bind(reason_data.reason.clone())
                    .execute(pool.get_ref())
                    .await?;

                Ok(())
            },
            KindTransaction::Delete => {
                let _data = sqlx::query("set @userName=?, @reason=?")
                    .bind(reason_data.deleted_by.clone())
                    .bind(reason_data.reason.clone())
                    .execute(pool.get_ref())
                    .await?;

                Ok(())
             },
        }
    }
}