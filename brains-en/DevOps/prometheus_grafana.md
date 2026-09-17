# Prometheus, Grafana

Let's learn about the monitoring tools Prometheus and Grafana.

### Prometheus

Prometheus is a system that can collect, store, and query various monitoring metrics from target systems.

It can support visualization through Grafana.

It supports various plugins for monitoring many systems and is widely used as the main monitoring system for Kubernetes.

Prometheus periodically reads and collects metrics from exporters (target systems to be monitored) using a pulling method.

Because it uses a pulling method, clients retrieve data when needed, allowing for the acquisition of the latest data. It also enhances resource efficiency by not fetching unnecessary data and improves security by going through authentication and authorization processes before sending requests.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FyVDU6%2FbtrazuxWNw5%2FTsCiXAkBZm9sfZW6AYXQK1%2Fimg.png)

<br>

### Grafana

Grafana is a monitoring tool that visualizes various data, including Prometheus, and typically visualizes metrics provided by data sources like Prometheus and InfluxDB.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FAMbQ0%2Fbtrax6RUsJQ%2FWmGyOAdU0tEZTp7n73OGxk%2Fimg.png)

### Difference from Kibana

Let's look at the difference with Kibana in the ELK stack.

Before that, we'll first understand the difference between logs and metrics.

Metrics refer to data that changes over time, such as **CPU usage, RAM capacity, and other hardware resource consumption**. In contrast, logs are **system-generated messages that appear in response to irregularly occurring events**. Logs come with different datasets.

For example, an application crash can generate logs, which can later be analyzed to identify correlations and resolve issues.

In other words, Grafana has fewer good features for analyzing logs. It also has fewer queries than Kibana.

Kibana is primarily used for log message analysis, while Grafana is used for metric analysis.

Grafana is specialized in visualizing system-level metrics such as `CPU, memory, and disk usage`.

While Kibana is tied to `Elasticsearch`, Grafana offers a choice of various databases.

Additionally, Grafana's alerting features, such as integration with SLACK, are **available for free, which is a great benefit** (AlertManager).
