use std::{collections::HashMap, sync::Arc, time::Duration};

use tokio::sync::Mutex;
use trade_log_contracts::{TradeLogSbModel, TradeLogSbModelDataItem};
use yft_service_sdk::external::{
    my_service_bus_sdk::{
        abstractions::publisher::MyServiceBusPublisher, client::MyServiceBusClient,
    },
    yft_rust_extensions::chrono::Utc,
};

use crate::{ModelToDeliver, TradeLogInner};

pub struct TradeLogContext {
    inner: TradeLogInner,
    component_name: Option<String>,
}

impl TradeLogContext {
    pub fn new() -> Self {
        Self {
            inner: TradeLogInner::new(),
            component_name: None,
        }
    }
}

pub struct TradeLog {
    context: Arc<Mutex<TradeLogContext>>,
}

impl TradeLog {
    pub fn new() -> Self {
        Self {
            context: Arc::new(Mutex::new(TradeLogContext::new())),
        }
    }

    pub async fn init_component_name(&self, component_name: &str) {
        let mut write_access = self.context.lock().await;

        if write_access.inner.is_started() {
            panic!("TradeLog is already started");
        }

        write_access.component_name = Some(component_name.to_string());
    }

    pub async fn start(&self, sb_client: &MyServiceBusClient) {
        let mut write_access = self.context.lock().await;

        if write_access.inner.is_started() {
            panic!("TradeLog is already started");
        }

        let publisher = sb_client.get_publisher(true).await;
        write_access.inner.sb_publisher = Some(Arc::new(publisher));

        tokio::spawn(write_to_trade_log(self.context.clone()));
    }

    pub async fn write<'s>(
        &self,
        trader_id: impl Into<String>,
        account_id: impl Into<String>,
        process_id: Option<impl Into<String>>,
        operation_id: impl Into<String>,
        message: impl Into<String>,
        data: Option<HashMap<String, String>>,
    ) {
        let mut write_access = self.context.lock().await;
        if !write_access.inner.is_started() {
            panic!("TradeLog is not started");
        }

        let component_name = match &write_access.component_name {
            Some(src) => src.clone(),
            None => "NotSet".to_string(),
        };

        let item = TradeLogSbModel {
            trader_id: trader_id.into().to_string(),
            account_id: account_id.into().to_string(),
            operation_id: operation_id.into().to_string(),
            process_id: if let Some(process_id) = process_id {
                process_id.into().to_string()
            } else {
                "".to_string()
            },
            message: message.into().to_string(),
            data: if let Some(data) = &data {
                data.into_iter()
                    .map(|(key, value)| TradeLogSbModelDataItem {
                        key: key.clone(),
                        value: value.clone(),
                    })
                    .collect()
            } else {
                vec![]
            },
            component: component_name,
            date_time_unix_micros: Utc::now().timestamp_micros(),
        };

        write_access.inner.add(item);
    }

    pub async fn stop(&self) {
        loop {
            let (items_in_queue, items_on_delivery) = {
                let mut write_access = self.context.lock().await;

                write_access.inner.stopping = true;

                (
                    write_access.inner.get_elements_in_queue(),
                    write_access.inner.get_items_on_delivery(),
                )
            };

            if items_in_queue == 0 && items_on_delivery == 0 {
                return;
            }

            if items_in_queue > 0 {
                println!("TradeLog: {} items in queue. Waiting", items_in_queue);
            }

            if items_on_delivery > 0 {
                println!("TradeLog: {} items on delivery. Waiting", items_on_delivery);
            }

            tokio::time::sleep(std::time::Duration::from_secs(1000)).await;
        }
    }
}

async fn write_to_trade_log(context: Arc<Mutex<TradeLogContext>>) {
    loop {
        let to_write = {
            let mut write_access = context.lock().await;
            write_access.inner.get_elements_to_deliver()
        };

        match to_write {
            Some((to_write, publisher)) => {
                deliver_it(context.clone(), to_write, publisher).await;
            }
            None => {
                {
                    let write_access = context.lock().await;

                    if write_access.inner.stopping
                        && write_access.inner.get_elements_in_queue() == 0
                        && write_access.inner.get_items_on_delivery() == 0
                    {
                        return;
                    }
                }

                tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        }
    }
}

async fn deliver_it(
    inner: Arc<Mutex<TradeLogContext>>,
    to_write: Vec<ModelToDeliver>,
    publisher: Arc<MyServiceBusPublisher<TradeLogSbModel>>,
) {
    loop {
        let models = to_write.iter().map(|x| x.model.clone()).collect::<Vec<_>>();

        match publisher.publish_messages(&models).await {
            Ok(_) => {
                let mut write_access = inner.lock().await;
                write_access.inner.delivered();
                break;
            }
            Err(err) => {
                let mut account_ids = String::new();
                let mut i = 0;
                for itm in &to_write {
                    account_ids.push_str(&itm.model.account_id);
                    account_ids.push(';');
                    i += 1;
                    if i >= 10 {
                        break;
                    }
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
    }
}
