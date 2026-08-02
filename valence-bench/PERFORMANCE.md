# Valence performance

Measured on AWS (`c6i.xlarge` class hosts unless noted). Numbers describe external create, query, and get-by-id paths across storage adapters applications actually configure (memory, SQLite, Postgres, Redis, MongoDB, IndraDB, and hybrid cache layouts).

## Write capacity

Single-client sustained creates scale with the adapter: in-process stores (memory, SQLite, IndraDB) sit well above remote wire adapters. Under rising concurrency, keep error rate under about 0.1% when sizing client pools. Multi-client aggregate writes on Redis fleets are the right model when many app processes share one Valence deployment.

## Query and read paths

Compiled and ORM query latency grows with graph depth and filter shape. Get-by-id hammer paths benefit when a cache tier sits in front of a durable primary. Hybrid IndraDB cache with Postgres primary improves hot get/query/hop mixes versus Postgres alone on the same hardware class.

## Privacy and overhead

Enabling privacy evaluation on reads adds measurable latency versus a forced bypass. Treat privacy-on as the production default when quoting capacity; bypass figures are upper bounds only.

## How to read these results

Prefer AWS-tagged hardware labels when comparing deployments. Developer-laptop runs are useful for harness smoke, not for fleet sizing.
