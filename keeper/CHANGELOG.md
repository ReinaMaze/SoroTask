# Changelog

## [1.1.0](https://github.com/ReinaMaze/SoroTask/compare/keeper-v1.0.0...keeper-v1.1.0) (2026-04-27)


### Features

* add example environment configuration for Soroban RPC and health checks ([cf4cf31](https://github.com/ReinaMaze/SoroTask/commit/cf4cf31bafae6571a9f03e5bae17257841e1a27b))
* add Metrics singleton for tracking operational statistics ([142a773](https://github.com/ReinaMaze/SoroTask/commit/142a773879a5125180e5e7f7153c6c0d851080b3))
* Add Prometheus metrics endpoint for Grafana monitoring ([e68c6b9](https://github.com/ReinaMaze/SoroTask/commit/e68c6b9f5ced8dca04918104cf6523415040665c))
* Add Prometheus metrics endpoint for Grafana monitoring ([19ff008](https://github.com/ReinaMaze/SoroTask/commit/19ff0084bbd11ac67d6e151daabd4bfd5c311cca))
* add structured json logging to keeper ([35127b9](https://github.com/ReinaMaze/SoroTask/commit/35127b955c8da34a4afba82bc4997806aa0ee4c8))
* add structured json logging to keeper- [#109](https://github.com/ReinaMaze/SoroTask/issues/109) ([db603a7](https://github.com/ReinaMaze/SoroTask/commit/db603a7d053e687ac72dbe6475c15aa629e1404a))
* add unit tests for Metrics and MetricsServer functionality ([8073b06](https://github.com/ReinaMaze/SoroTask/commit/8073b06952808bab7cf13ad1c6567d6d45d7fef2))
* created rpc connection module ([e427fb7](https://github.com/ReinaMaze/SoroTask/commit/e427fb7e4df8bf31d87bf28eb03014262fe7355c))
* enhance ExecutionQueue with metrics tracking for task execution and cycle duration ([77cf796](https://github.com/ReinaMaze/SoroTask/commit/77cf7960869f6524961595e291c19cee66b1b5be))
* Implement Docker and Docker Compose for the keeper service ([e038a53](https://github.com/ReinaMaze/SoroTask/commit/e038a533b5de310bfcc049b744683141ce342b05))
* Implement Docker and Docker Compose for the keeper service, adding configuration, build scripts, and deployment documentation. ([5293bab](https://github.com/ReinaMaze/SoroTask/commit/5293babaeabbae3304a708f1e66ebaadf5ad5505))
* implement execution queue with concurrency control and graceful shutdown ([d687745](https://github.com/ReinaMaze/SoroTask/commit/d68774596da76a7f5fcfb3d3cc253be90546f97b))
* implement execution queue with concurrency control and graceful… ([b6b61f2](https://github.com/ReinaMaze/SoroTask/commit/b6b61f252cba422170e2f7591700dc3d2740080e))
* implement keeper reward & fee mechanism ([#5](https://github.com/ReinaMaze/SoroTask/issues/5)) ([0e4f88c](https://github.com/ReinaMaze/SoroTask/commit/0e4f88c220c11b9168066129397f231f548eddba))
* implement MetricsServer for health checks and metrics exposure ([1eaefeb](https://github.com/ReinaMaze/SoroTask/commit/1eaefeba6aaa45939a2e4118c4fd382c3a6dd336))
* implement production-grade polling engine for keeper service ([5b69bc2](https://github.com/ReinaMaze/SoroTask/commit/5b69bc2ab89a2409d9b8248d7e6854df06a7b348))
* implement production-grade polling engine for keeper service ([9d81c45](https://github.com/ReinaMaze/SoroTask/commit/9d81c45248d8c029e59f89ce723fc52d408a2f2f))
* implemented a config module for loading & validation of required env vars on startup ([54b3501](https://github.com/ReinaMaze/SoroTask/commit/54b3501a8f23cca89f82c03a7cbfeaa7b71e378c))
* implemented requirement ([11969d6](https://github.com/ReinaMaze/SoroTask/commit/11969d6a80ee9c2608811e897257a1d730fc7baa))
* initialize metrics server and update health check state in polling loop ([c43a430](https://github.com/ReinaMaze/SoroTask/commit/c43a430312347c6c6da66ea9c836844c19cef148))
* initialize project structure and add polling engine performance benchmarks ([8729afa](https://github.com/ReinaMaze/SoroTask/commit/8729afa1f910636190b365c69af75a727ff797d0))
* initialize project structure and add polling engine performance… ([652eb40](https://github.com/ReinaMaze/SoroTask/commit/652eb40bdef644a92869ed4642712fc47e7c6ca2))
* integrated rpc & config modules ([6ba3422](https://github.com/ReinaMaze/SoroTask/commit/6ba342299de9b0bd2b60ab3cfc8ac150e5bc12a0))
* Keeper account loading and validation ([7da38fc](https://github.com/ReinaMaze/SoroTask/commit/7da38fc7861da931501057985debd0728bd5528d))
* Keeper account loading and validation ([799a184](https://github.com/ReinaMaze/SoroTask/commit/799a1843942e108f505435686474e1cc9127307d))
* **keeper:** add --dry-run flag for local task simulation ([c49f788](https://github.com/ReinaMaze/SoroTask/commit/c49f7883776ab3294a6b5389116f3fd7df4b7a09))
* **keeper:** add --dry-run flag for local task simulation ([75f97aa](https://github.com/ReinaMaze/SoroTask/commit/75f97aaa97d1cb47d7768d4a114899e356d14048)), closes [#112](https://github.com/ReinaMaze/SoroTask/issues/112)
* **keeper:** add comprehensive unit and integration test suite ([136f694](https://github.com/ReinaMaze/SoroTask/commit/136f6944288164dd201783b998aaa4a6ccbf914a)), closes [#41](https://github.com/ReinaMaze/SoroTask/issues/41)
* **keeper:** add health-check sidecar for high availability ([#110](https://github.com/ReinaMaze/SoroTask/issues/110)) ([fae35de](https://github.com/ReinaMaze/SoroTask/commit/fae35deec28b687b7274c5560e2d98827369b7d4))
* **keeper:** add health-check sidecar for high availability ([#110](https://github.com/ReinaMaze/SoroTask/issues/110)) ([aebe65c](https://github.com/ReinaMaze/SoroTask/commit/aebe65c9317753a6ed203a006cd847dee593cfa5))
* **keeper:** Add Node.js CI pipeline with ESLint, Jest, and Docker v… ([f642215](https://github.com/ReinaMaze/SoroTask/commit/f642215e8417133d40c6d4e1e283ec55a290792f))
* **keeper:** Add Node.js CI pipeline with ESLint, Jest, and Docker validation ([8dbcdae](https://github.com/ReinaMaze/SoroTask/commit/8dbcdae38fbde1c41590d597af1ac911ad27c63e))
* **keeper:** add structured logging with pino, log levels, and JSON output ([e52a9d8](https://github.com/ReinaMaze/SoroTask/commit/e52a9d8cd61e06ea349f8fc54f078971a08a8f3a))
* **keeper:** add structured logging with pino, log levels, and JSON output ([19288fb](https://github.com/ReinaMaze/SoroTask/commit/19288fb9b12f1f4937c32e01f5b93b5ff0ab5009)), closes [#38](https://github.com/ReinaMaze/SoroTask/issues/38)
* **keeper:** implement event-driven task registry discovery ([fb5517e](https://github.com/ReinaMaze/SoroTask/commit/fb5517ee529fb9f0684ded01fffb70692c029c1a))
* **keeper:** implement event-driven task registry discovery ([673940b](https://github.com/ReinaMaze/SoroTask/commit/673940b057faf676d60e271777a2fc7f7c54cca4))
* **keeper:** implement retry logic with exponential backoff for failed submissions ([16782fc](https://github.com/ReinaMaze/SoroTask/commit/16782fcc05aa7ab2818e85ce450391e65c9d4e8f))
* **keeper:** implement retry logic with exponential backoff for failed submissions ([c62e405](https://github.com/ReinaMaze/SoroTask/commit/c62e405e9f432500dfda9c35423209efc87f3ffc)), closes [#37](https://github.com/ReinaMaze/SoroTask/issues/37)
* **keeper:** optimize Docker image with multi-stage node:alpine build ([af44582](https://github.com/ReinaMaze/SoroTask/commit/af44582e1d504289705c57ec0368688d56e37711))
* npm install/updated dependency chain packages ([6ad9550](https://github.com/ReinaMaze/SoroTask/commit/6ad9550d77c986c33cd90ea264889baf7ad58874))


### Bug Fixes

* npm package error ([9dcd276](https://github.com/ReinaMaze/SoroTask/commit/9dcd276ce5582dbc9ed4ff33bca450c99a9c16a7))
