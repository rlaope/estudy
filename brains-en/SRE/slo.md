# SLO, SLI, SLA

Service Level is a term that measures the service provided to users within a certain period.

Service-Level Objectives, **SLO**, are goals that set the expected availability of a system.

Service-Level Indicators, **SLI**, are key measurements and metrics used to understand a system's availability.

Service-Level Agreements, **SLA**, are legal contracts that describe what happens if a system fails to meet its SLOs and the agreed-upon terms.

For example, if a web application's SLO is to start video playback within 2 seconds for 99% of the time over one week, the SLI measures the percentage of videos that started playback within 2 seconds. The SLA includes these SLOs, other SLOs agreed upon by the customer and service provider, the scope of application, and the SLIs used to measure performance.

Due to Site Reliability Engineering (SRE), which focuses on how to measure service performance and reliability, best practices for maintaining uptime and stability in distributed systems have become widespread.

### SLO

Service Level Objectives are goals that set the expected availability of a system, expressed as a percentage over a certain period.

Service Level Objectives help teams collaborate based on the common understanding of availability and uptime. SLOs can be used as a standard to measure reliability and availability. As explained in the previous example, this would be 99% of web videos playing within 2 seconds over one week.

### SLI

Service Level Indicators are quantitative measurements of how users experience a system's availability. They represent the success rate of a service level as a percentage.

While Service Level Indicators are explained in relation to SLOs, SLIs provide real-time signals about system stability. SLIs can yield precise values by measuring the ratio of requests faster than a threshold or the ratio of records entering a pipeline. In the previous example, the SLI measures the percentage of videos played within 2 seconds on the website, allowing us to understand the difference from the SLO.

SLIs can yield precise values by measuring the ratio of requests faster than a threshold or the ratio of records entering a pipeline. In the previous example, the SLI measures the percentage of videos that started playback within 2 seconds on the website. This allows us to know how much it deviates from the SLO.

### SLA

A Service Level Agreement defines the **expected level of service** when a customer uses the service.

A Service Level Agreement is a contract between a service provider and a customer that documents the services the provider will deliver and defines the service standards the provider must meet. SLAs often describe the penalties for violating SLOs.

In the previous example, the SLA includes all SLOs for the web application, as well as the scope of services to be included, and all SLIs used to measure performance against the SLOs. The agreement also includes the responsibilities of both the service provider and the customer.

<br>

### Who Uses These Metrics

SRE teams, reliability engineers, and cross-functional teams often face challenges in defining and measuring service reliability. Cross-functional teams need to be able to create a comprehensive view of critical metrics across all aspects of a service or system to easily measure uptime and performance.

Service levels help SRE teams and reliability engineers identify key components in applications and infrastructure. They need to know when one or more components expose functionality to external customers, and these intersection points are called **system boundaries**. System boundaries are the points where site reliability engineers apply service level indicators and objectives to metrics to describe the actual state of system performance and reliability.

In other words, setting the service scope and determining what constitutes an SLI and what the SLO compliance requirements are takes significant effort. These metrics can be seen as crucial for SRE engineers, teams, and all teams to quickly establish baselines for availability and uptime across the entire stack.
