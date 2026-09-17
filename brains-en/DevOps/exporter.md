# Prometheus Exporter - Monitoring Agent

### Exporter

Prometheus collects metrics from monitoring targets.

So, where do these metrics from the monitoring targets come from?

They are provided by Exporters.

An Exporter collects metric data from a monitoring target and provides it when Prometheus connects. There are `node-exporter`s that collect CPU and memory data from host servers, `nginx-exporter`s that collect Nginx data, and various other exporters that provide metric content for different types of data.

> - For batch jobs or similar cases where using an Exporter is difficult, a push gateway can be used.
> - For web application servers, you can create metrics using client libraries and then use a custom Exporter.

In other words, the Prometheus server connects to the HTTP endpoint exposed by the Exporter to collect metrics.

This is why it's called the **Pull method**.

Common Exporters include:

- node-exporter
- mysql-exporter
- wmi-exporter
- posgre-exporter
- redis-exporter
- kafka-exporter
- jmx-exporter

and many more. The [official website](https://prometheus.io/docs/instrumenting/exporters/) has a more detailed list of exporters.
