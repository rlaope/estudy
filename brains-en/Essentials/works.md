# Definition of Doing Work Well vs. Developing Well

Lately, I've been heavily involved in tasks related to data aggregation and extraction (such as monthly settlement data aggregation).

For tasks where batch applications already exist, a simple run automatically executes the aggregation and settlement logic, saving the results to a file. All that's left is to move the uploaded results to a bucket so they can be reported to the manager.

After that, for aggregations that need to be done via queries (for example, loans in the OOO category disbursed this month), I have to query them directly, put them into an Excel sheet, and deliver them.

Typically, such aggregated data is requested for reporting to superiors or by external audit institutions (like the Financial Supervisory Service). While standard aggregation logic can often be covered by simple DB queries, and thus run monthly as needed, a month later, one might find themselves trying to remember how the aggregation was done and re-writing the query.

Here, more astute individuals might record the query format for reuse next month or even write a batch application. The question is, while saving queries is certainly a good idea, is it always best to write a batch application or make changes to core servers for such tasks?

Of course, developing well means being able to consistently build such applications without issues. But should a task that can be covered by a few clicks and an SQL query always be solved with software?

Tasks that can be sufficiently covered manually or with a few clicks, while perhaps cumbersome, are often completed quickly before one even feels the hassle. Should I expend my resources on such minor tasks? Moreover, if a new application is developed, it would typically go through review and testing processes within a development organization.

And what if the underlying assumptions of the aggregation logic or the structure of the required data changes? What if a decision to change a column in a referenced table means having to consider the side effects on that batch application?

I would define 'developing well' as having excellent capability to implement software to solve a problem (where 'problem' here refers solely to the existence of inconvenience, not its scale).

I believe 'doing work well' is the sense to first ask if something needs to be done, even if it can be done, then prioritize tasks, consider what the other person immediately wants and how much I can provide, and then take a step further.

<br>

### Sense

Let me give an example. I received a request to aggregate monthly settlement loan amounts in an Excel sheet, with the specific instruction that aggregation wasn't needed, just the extraction and delivery of raw_data (ID and loan amount).

Here, basic 'sense' would be to deliver the data sorted by ID, or by time, or in a structured, easy-to-read format, or a format that's easy to use with Excel functions.

If a slightly higher level of 'sense' is applied, one might even apply the aggregation directly to the sheet (using functions like vlookup, sum, count, etc.) before delivering it.

Of course, the above is not strictly a developer's job; it's perfectly fine to hand it over to a PM team member or the team member who requested the data. However, if you proactively take care of this, the requester will only need to perform a simple data check, and your performance review will significantly improve.

It's not mandatory, of course. This task falls into the realm of 'sense,' and as a developer, the most crucial job is still development, and it's important to pay even more attention to avoiding mistakes in that development.

<br>

### Identifying Repetition and Automation Timing

I've heard the DevOps principle of automating everything somewhere, but I'm slightly against it. I'm not sure if 'automate everything' refers only to deployment systems,

but a certain degree of human intervention is naturally essential. One of my biggest regrets from past development was the habit of trying to automate everything.

Because of this, even for simple issues, I had to debug or fix the automation system from start to finish (though it was also my fault for creating a highly coupled automation system at the time).

Even taking the aggregation scenario above as an example, should this aggregation be automated? The loan domain is coupled with many other areas because it's a core banking system. Is there a need to add more to an already side-effect-prone area?

Conversely, since it's already a core banking system, and the domain is almost immutable in terms of changeability, wouldn't adding to it be fine? Is there enough leeway to develop this batch application?

What I usually recommend when tackling such tasks is to assess priority, set a time limit, and try it. Allocate about 30 minutes; if it takes longer, defer it to a lower priority and handle it manually. Use that time for something else.

And what's needed to determine such a time limit is an analysis of the current situation, the other party's needs, or recurring patterns.

Why was monthly settlement aggregation data requested? There are many financial incidents lately, and institutions like [redacted] are conducting many audits. Will we also be audited monthly from now on? It doesn't seem to be an intermittent occurrence. Let's observe it until the end of this month. The data structure and format change frequently; for now, developing an application might need more observation.

The ability to understand the current situation and patterns, and make appropriate decisions, is crucial. Furthermore, proactively try to understand what additional requests the other party might have, what they want, and where they are experiencing inconvenience. The more questions you ask, the more answers you'll get. The more answers you get, the more tasks will become clear.

As you clarify what needs to be done, you will become good at your job.
