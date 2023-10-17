## How to use it


Add as a dependency to toml file

```toml

trade-log = { tag="xxx", git="https://github.com/my-cfd-platform/trade-log.git" }

```


Plug it in main.rs
```rust
trade_log::TRADE_LOG.start(&app.sb_client).await;
```




Stop it after we detect stop application event

```rust
app_ctx.app_states.wait_until_shutdown().await;
// We put it after wait_until_shutdown()
trade_log::TRADE_LOG.stop().await;
````


Write trade_log at any place of your code

```rust
// We put it after wait_until_shutdown()
trade_log::TRADE_LOG.write(trader_id, account_id, process_id, message, data).await;
````
