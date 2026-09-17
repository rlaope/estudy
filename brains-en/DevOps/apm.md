# APM (Application Performance Management)

## APM
It is a service that manages the performance of applications, as its full name suggests.

It is a service that helps ensure stable operation when providing web services.

### When to Adopt APM
It is efficient to introduce and operate APM from the testing phase, once service development is complete.

Support for Various Applications | Monitoring | Performance Management | Fault Management
--|--|--|--
Integrates with open-source and commercial applications | Provides real-time dashboards, custom monitoring, and statistics | Correlation analysis, response time measurement and analysis | Alerts on failures, provides information on failure causes

### APM Architecture

![](./image/apm.png)

APM is largely composed of three components.
- APM Server: A server that analyzes data collected from APM Agents and performs actual operations.
- APM Agent: An application server where the Agent is installed to collect data for the APM Server.
- APM Client: Provides users with data collected and analyzed by the APM Server.

### APM Solutions

There are many types of monitoring solutions such as Scouter, Prometheus, and WhaTap.
