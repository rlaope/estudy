# Data-Driven Thinking (Observability + Capacity Planning)

Let's imagine a job interview Q&A like the following.

Interviewer: Let's say you're tracking user traffic with CloudWatch and setting up metric alarms. At what percentage should you update which settings?

+ If I were to answer heuristically, "80%? If it reaches 80%, wouldn't we curb the increase by caching API Gateway features?"

Interviewer: Isn't 80% too late? What about other methods?

Me: Hmm... then 70%? Update caching features at 30%, and turn on Lambda Provisioned Concurrency at 70%...

Interviewer: Why 30% and why 70%? Give me specific reasons.

### Data-Driven Thinking

Let's look at the interview content above. It's terrible. You shouldn't answer like that.

When considering performance, especially for high-traffic projects, heuristics must be abandoned.

In other words, for such questions, you shouldn't state a percentage (threshold) value, but rather the decision-making process (the underlying formula).

The interviewer isn't asking for actual numbers, but rather how that threshold was logically determined using what data.

```
Thresholds are set based on pre-calculated maximum throughput (capacity).

After collecting **peak traffic**, **processing time**, and **throughput** per single API Gateway/Lambda instance over the past 2-4 weeks,

a threshold is set with a buffer (usually 20-30 percent) relative to the peak.

CloudWatch alarm thresholds are not fixed values but are set as usage relative to capacity = (current throughput / pre-calculated maximum throughput).
```

Answering like this will at least make the interviewer think, "Okay, they've given an answer."

### Explain Percentage-Based Criteria Like This

Percentages must be justified. For example, you can present criteria like these:

```matlab
- 60%: Detect rising trend (Scaling warm-up phase)
- 75%: Lambda Provisioned Concurrency: Initiate pre-warm-up
- 85%: Review API Gateway cache expansion or TTL adjustment
- 90%: Urgent alarm, check auto-scaling quotas
```

And always add the following:

```
These figures are not absolute values;
they must be calculated based on the system's TPS, Lambda Duration, Cold Start rate,
and past burst traffic patterns.
```

### Alarm Response Strategy Should Be Discussed from a Time + Warm-up Perspective

The reason the interviewer said, "Isn't 80% too late?" is because

Provisioned Concurrency requires warm-up time.

Therefore,

```
Since Provisioned Concurrency requires several minutes of warm-up time to take effect,
you must proactively scale by detecting traffic trends (slope) before the actual load arrives.

Therefore, instead of an absolute value (80 percent), an Early Signal is utilized, using the slope of short-term moving averages (e.g., comparing 1-minute, 5-minute, 15-minute baselines).
```

### In Summary

```
Threshold percentages are not fixed numbers;
they are dynamically set based on the system's pre-calculated maximum throughput.

I first determine 'Max Capacity' based on Lambda's average processing time (Duration), Concurrency, API Gateway's RPS,
and peak traffic over the past 2-4 weeks.

CloudWatch metrics are set as usage relative to this Max Capacity.
For Provisioned Concurrency, which requires warm-up, pre-warming is initiated when the traffic increase slope exceeds a certain threshold
or when capacity begins to exceed approximately 60-70%.

Caching, on the other hand, is considered for TTL adjustment or on/off toggling when the RPS growth rate surges faster than CPU/memory,
specifically around the 75-85% mark.

The important point is not the percentage itself,
but that thresholds are set based on past traffic patterns and maximum throughput calculations,
providing a basis for 'when warm-up is needed and when delays occur.'
```

> Lambda Concurrency: Refers to the number of Lambda function instances running simultaneously. It's a metric indicating how many Lambda functions are executing concurrently at any given moment. Each time a request comes in, Lambda either launches a new execution environment (cold start) or reuses an existing one. Therefore, as traffic increases, the number of concurrently running Lambdas surges, and this is Concurrency.
>
> ```
> Example: If 200 requests arrive in 1 second,
> and the average processing time for each request is 0.5 seconds,
> approximately 100 Lambdas will be executing concurrently.
> ```
>
> If Concurrency becomes too high, it can exceed the Quota (limit), leading to throttling. This can be resolved with Provisioned Concurrency, which is a feature to prevent cold starts. More details can be found in AWS Lambda documentation.
