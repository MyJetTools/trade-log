use std::sync::Arc;

use service_sdk::{
    my_service_bus::abstractions::publisher::MyServiceBusPublisher,
    my_telemetry::MyTelemetryContext,
};
use trade_log_contracts::TradeLogSbModel;

const ITEMS_PER_ROUNDTRIP: usize = 10;

pub struct ModelToDeliver {
    pub model: TradeLogSbModel,
    pub my_telemetry: Option<MyTelemetryContext>,
}

pub struct TradeLogInner {
    items: Vec<ModelToDeliver>,
    pub sb_publisher: Option<Arc<MyServiceBusPublisher<TradeLogSbModel>>>,
    items_on_delivery: usize,
    pub stopping: bool,
}

impl TradeLogInner {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            sb_publisher: None,
            items_on_delivery: 0,
            stopping: false,
        }
    }

    pub fn is_started(&self) -> bool {
        self.sb_publisher.is_some()
    }

    pub fn add(&mut self, item: TradeLogSbModel, my_telemetry: Option<MyTelemetryContext>) {
        self.items.push(ModelToDeliver {
            model: item,
            my_telemetry,
        });
    }

    pub fn get_elements_in_queue(&self) -> usize {
        self.items.len()
    }

    pub fn get_items_on_delivery(&self) -> usize {
        self.items_on_delivery
    }

    pub fn get_elements_to_deliver(
        &mut self,
    ) -> Option<(
        Vec<ModelToDeliver>,
        Arc<MyServiceBusPublisher<TradeLogSbModel>>,
    )> {
        if self.items.len() == 0 {
            return None;
        }

        if self.items.len() <= ITEMS_PER_ROUNDTRIP {
            let mut result = Vec::new();
            std::mem::swap(&mut result, &mut self.items);

            self.items_on_delivery = result.len();
            return Some((result, self.sb_publisher.as_ref().unwrap().clone()));
        }

        let mut result = Vec::new();

        while result.len() < ITEMS_PER_ROUNDTRIP {
            result.push(self.items.remove(0));
        }

        self.items_on_delivery = result.len();
        Some((result, self.sb_publisher.as_ref().unwrap().clone()))
    }

    pub fn delivered(&mut self) {
        self.items_on_delivery = 0;
    }
}
